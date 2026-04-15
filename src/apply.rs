use crate::{
    collections::{CONSUMER_CPUS, HashMap, HashSet, List, TIDS_CAPED, TIDS_FULL, list},
    config::{ProcessLevelConfig, ThreadLevelConfig, cpu_indices_to_mask, format_cpu_indices},
    error_codes::{error_from_code_win32, error_from_ntstatus},
    logging::{Operation, is_new_error},
    priority::{IOPriority, MemoryPriority, MemoryPriorityInformation, ProcessPriority, ThreadPriority},
    process::ProcessEntry,
    scheduler::PrimeThreadScheduler,
    winapi::{
        NtQueryInformationProcess, NtSetInformationProcess, ProcessHandle, cpusetids_from_indices, filter_indices_by_mask,
        get_cpu_set_information, get_thread_handle, get_thread_ideal_processor_ex, get_thread_start_address, indices_from_cpusetids,
        resolve_address_to_module, set_thread_ideal_processor_ex,
    },
};

use ntapi::ntexapi::SYSTEM_THREAD_INFORMATION;
use rand::random;
use std::{cmp::Reverse, ffi::c_void, mem::size_of};
use windows::Win32::{
    Foundation::{GetLastError, HANDLE},
    System::{
        Threading::{
            GetPriorityClass, GetProcessAffinityMask, GetProcessDefaultCpuSets, GetProcessInformation, GetThreadPriority,
            ProcessMemoryPriority, SetPriorityClass, SetProcessAffinityMask, SetProcessDefaultCpuSets, SetProcessInformation,
            SetThreadPriority, SetThreadSelectedCpuSets,
        },
        WindowsProgramming::QueryThreadCycleTime,
    },
};

#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}

impl ApplyConfigResult {
    pub fn new() -> Self {
        Self::default()
    }

    /// format: r#"$operation details"#
    /// attached to auto-generated "{pid:>5}::{config.name}::"
    #[inline(always)]
    pub fn add_change(&mut self, change: String) {
        self.changes.push(change);
    }

    /// format: r#"$fn_name: [$operation][$error_message] details"#
    #[inline(always)]
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty() && self.errors.is_empty()
    }
}

/// Extracts read and write handles from ProcessHandle, preferring full access handles over limited.
/// results in (read_handle, write_handle)
#[inline(always)]
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) {
    let r = process_handle.r_handle.or(Some(process_handle.r_limited_handle));
    let w = process_handle.w_handle.or(Some(process_handle.w_limited_handle));
    (r, w)
}

/// Logs an error if it hasn't been logged before for this pid/operation combination.
#[inline(always)]
fn log_error_if_new(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
) {
    if is_new_error(pid, tid, process_name, operation, error_code) {
        apply_config_result.add_error(format_msg());
    }
}

pub fn apply_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) {
    let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
        return;
    };
    if let Some(priority_flag) = config.priority.as_win_const() {
        let current_priority = unsafe { GetPriorityClass(r_handle) };
        if current_priority != priority_flag.0 {
            let change_msg = format!(
                "Priority: {} -> {}",
                ProcessPriority::from_win_const(current_priority),
                config.priority.as_str()
            );
            if dry_run {
                apply_config_result.add_change(change_msg);
            } else {
                let set_result = unsafe { SetPriorityClass(w_handle, priority_flag) };
                if set_result.is_ok() {
                    apply_config_result.add_change(change_msg);
                } else {
                    let error_code = unsafe { GetLastError().0 };
                    log_error_if_new(
                        pid,
                        0,
                        &config.name,
                        Operation::SetPriorityClass,
                        error_code,
                        apply_config_result,
                        || {
                            format!(
                                "apply_priority: [SET_PRIORITY_CLASS][{}] {:>5}-{}",
                                error_from_code_win32(error_code),
                                pid,
                                config.name
                            )
                        },
                    );
                }
            }
        }
    }
}

/// side effect:  fills in the affinity mask for the given process
pub fn apply_affinity<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) {
    let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
        return;
    };
    let mut system_mask: usize = 0;
    let affinity_mask = cpu_indices_to_mask(&config.affinity_cpus);
    let has_affinity = !config.affinity_cpus.is_empty();
    if has_affinity {
        match unsafe { GetProcessAffinityMask(r_handle, &mut *current_mask, &mut system_mask) } {
            Err(_) => {
                if !dry_run {
                    let error_code = unsafe { GetLastError().0 };
                    log_error_if_new(
                        pid,
                        0,
                        &config.name,
                        Operation::GetProcessAffinityMask,
                        error_code,
                        apply_config_result,
                        || {
                            format!(
                                "apply_affinity: [GET_PROCESS_AFFINITY_MASK][{}] {:>5}-{}",
                                error_from_code_win32(error_code),
                                pid,
                                config.name
                            )
                        },
                    );
                }
            }
            Ok(_) => {
                if has_affinity && affinity_mask != 0 && affinity_mask != *current_mask {
                    let change_msg = format!("Affinity: {:#X} -> {:#X}", current_mask, affinity_mask);
                    if dry_run {
                        apply_config_result.add_change(change_msg);
                    } else {
                        match unsafe { SetProcessAffinityMask(w_handle, affinity_mask) } {
                            Err(_) => {
                                let error_code = unsafe { GetLastError().0 };
                                log_error_if_new(
                                    pid,
                                    0,
                                    &config.name,
                                    Operation::SetProcessAffinityMask,
                                    error_code,
                                    apply_config_result,
                                    || {
                                        format!(
                                            "apply_affinity: [SET_PROCESS_AFFINITY_MASK][{}] {:>5}-{}",
                                            error_from_code_win32(error_code),
                                            pid,
                                            config.name
                                        )
                                    },
                                );
                            }
                            Ok(_) => {
                                apply_config_result.add_change(change_msg);
                                *current_mask = affinity_mask;
                                reset_thread_ideal_processors(pid, config, false, &config.affinity_cpus, threads, apply_config_result);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Resets ideal processors for all threads.
///
/// When process affinity is changed, Windows may reset thread ideal processors.
/// This redistributes threads across the new affinity CPUs with a random shift
/// to avoid always assigning too much threads to the same CPUs.
/// # Arguments
/// * `cpus` - The set of CPU indices to distribute thread ideal processors across. Callers pass `&config.affinity_cpus` after an affinity
///   change, or `&config.cpu_set_cpus` after a CPU-set change (when `cpu_set_reset_ideal` is set).
pub fn reset_thread_ideal_processors<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) {
    if cpus.is_empty() {
        return;
    }

    if dry_run {
        apply_config_result.add_change(format!("Reset Ideal Processors: {} threads based on CPU time", cpus.len()));
        return;
    }

    // Collect thread IDs and their CPU times
    let mut tid_time_list: List<[(u32, i64); TIDS_FULL]> = List::new();
    for (tid, thread_info) in threads() {
        let total_time = unsafe { *thread_info.KernelTime.QuadPart() + *thread_info.UserTime.QuadPart() };
        tid_time_list.push((*tid, total_time));
    }

    if tid_time_list.is_empty() {
        return;
    }

    // Sort by CPU time descending
    tid_time_list.sort_unstable_by_key(|(_, time)| Reverse(*time));

    let target_cpu_count = cpus.len();
    let random_shift = random::<u8>();
    let mut counter_set_success = 0;
    let _: () = tid_time_list
        .iter()
        .map(|&(tid, _)| {
            if let Some(thread_handle) = get_thread_handle(tid, pid, &config.name) {
                let handle = if thread_handle.w_handle.is_invalid() {
                    thread_handle.w_limited_handle
                } else {
                    thread_handle.w_handle
                };
                if !handle.is_invalid() {
                    let target_cpu_index = (counter_set_success + random_shift as usize) % target_cpu_count;
                    let target_cpu = cpus[target_cpu_index];
                    match set_thread_ideal_processor_ex(handle, 0, target_cpu as u8) {
                        Err(_) => {
                            let error_code = unsafe { GetLastError().0 };
                            log_error_if_new(
                                pid,
                                tid,
                                &config.name,
                                Operation::SetThreadIdealProcessorEx,
                                error_code,
                                apply_config_result,
                                || {
                                    format!(
                                        "reset_ideal_processor: [SET_IDEAL][{}] {:>5}-{}-{} - SetThreadIdealProcessorEx failed",
                                        error_from_code_win32(error_code),
                                        pid,
                                        tid,
                                        config.name
                                    )
                                },
                            );
                        }
                        Ok(_) => {
                            counter_set_success += 1;
                        }
                    }
                }
            }
        })
        .collect();
    apply_config_result.add_change(format!("reset ideal processor for {} threads", counter_set_success));
}

pub fn apply_process_default_cpuset<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) {
    let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
        return;
    };
    if !config.cpu_set_cpus.is_empty() && !get_cpu_set_information().lock().unwrap().is_empty() {
        if dry_run {
            apply_config_result.add_change(format!("CPU Set: -> [{}]", format_cpu_indices(&config.cpu_set_cpus)));
        } else {
            let target_cpusetids = cpusetids_from_indices(&config.cpu_set_cpus);
            let mut current_cpusetids: List<[u32; CONSUMER_CPUS]> = List::new();
            if !target_cpusetids.is_empty() {
                let mut toset: bool = false;
                let mut requiredidcount: u32 = 0;
                let query_result = unsafe { GetProcessDefaultCpuSets(r_handle, None, &mut requiredidcount) }.as_bool();
                if query_result {
                    toset = true; // query succeeded with None, it doesn't have a default CPU set. Otherwise, it has.
                } else {
                    let error_code = unsafe { GetLastError().0 };
                    if error_code != 122 {
                        log_error_if_new(
                            pid,
                            0,
                            &config.name,
                            Operation::GetProcessDefaultCpuSets,
                            error_code,
                            apply_config_result,
                            || {
                                format!(
                                    "apply_process_default_cpuset: [GET_PROCESS_DEFAULT_CPUSETS][{}] {:>5}-{}",
                                    error_from_code_win32(error_code),
                                    pid,
                                    config.name
                                )
                            },
                        );
                    } else {
                        current_cpusetids = list![0u32; requiredidcount as usize];
                        let second_query =
                            unsafe { GetProcessDefaultCpuSets(r_handle, Some(&mut current_cpusetids[..]), &mut requiredidcount) }.as_bool();
                        if !second_query {
                            let error_code = unsafe { GetLastError().0 };
                            log_error_if_new(
                                pid,
                                0,
                                &config.name,
                                Operation::GetProcessDefaultCpuSets,
                                error_code,
                                apply_config_result,
                                || {
                                    format!(
                                        "apply_process_default_cpuset: [GET_PROCESS_DEFAULT_CPUSETS][{}] {:>5}-{}",
                                        error_from_code_win32(error_code),
                                        pid,
                                        config.name
                                    )
                                },
                            );
                        } else {
                            toset = current_cpusetids != target_cpusetids;
                        }
                    }
                }
                if toset {
                    if config.cpu_set_reset_ideal {
                        reset_thread_ideal_processors(pid, config, dry_run, &config.cpu_set_cpus, threads, apply_config_result);
                    }
                    let set_result = unsafe { SetProcessDefaultCpuSets(w_handle, Some(&target_cpusetids)) }.as_bool();
                    if !set_result {
                        let error_code = unsafe { GetLastError().0 };
                        log_error_if_new(
                            pid,
                            0,
                            &config.name,
                            Operation::SetProcessDefaultCpuSets,
                            error_code,
                            apply_config_result,
                            || {
                                format!(
                                    "apply_process_default_cpuset: [SET_PROCESS_DEFAULT_CPUSETS][{}] {:>5}-{}",
                                    error_from_code_win32(error_code),
                                    pid,
                                    config.name
                                )
                            },
                        );
                    } else {
                        apply_config_result.add_change(format!(
                            "CPU Set: [{}] -> [{}]",
                            format_cpu_indices(&indices_from_cpusetids(&current_cpusetids)),
                            format_cpu_indices(&config.cpu_set_cpus)
                        ));
                    }
                }
            }
        }
    }
}

pub fn apply_io_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) {
    let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
        return;
    };
    if let Some(io_priority_flag) = config.io_priority.as_win_const() {
        const PROCESS_INFORMATION_IO_PRIORITY: u32 = 33;
        let mut current_io_priority: u32 = 0;
        let mut return_length: u32 = 0;
        let query_result = unsafe {
            NtQueryInformationProcess(
                r_handle,
                PROCESS_INFORMATION_IO_PRIORITY,
                &mut current_io_priority as *mut _ as *mut c_void,
                size_of::<u32>() as u32,
                &mut return_length,
            )
        }
        .0;
        let query_result_u32 = i32::cast_unsigned(query_result);
        let change_msg = format!(
            "IO Priority: {} -> {}",
            IOPriority::from_win_const(current_io_priority),
            config.io_priority.as_str()
        );
        if query_result < 0 {
            log_error_if_new(
                pid,
                0,
                &config.name,
                Operation::NtQueryInformationProcess2ProcessInformationIOPriority,
                query_result_u32,
                apply_config_result,
                || {
                    format!(
                        "apply_io_priority: [QUERY_IO_PRIORITY][{}] {:>5}-{} -> {}",
                        error_from_ntstatus(query_result),
                        pid,
                        config.name,
                        config.io_priority.as_str()
                    )
                },
            );
        } else if current_io_priority != io_priority_flag {
            if dry_run {
                apply_config_result.add_change(change_msg);
            } else {
                let set_result = unsafe {
                    NtSetInformationProcess(
                        w_handle,
                        PROCESS_INFORMATION_IO_PRIORITY,
                        &io_priority_flag as *const _ as *const c_void,
                        size_of::<u32>() as u32,
                    )
                }
                .0;
                let set_result_u32 = i32::cast_unsigned(set_result);
                if set_result < 0 {
                    log_error_if_new(
                        pid,
                        0,
                        &config.name,
                        Operation::NtSetInformationProcess2ProcessInformationIOPriority,
                        set_result_u32,
                        apply_config_result,
                        || {
                            format!(
                                "apply_config: [SET_IO_PRIORITY][{}] {:>5}-{} -> {}",
                                error_from_ntstatus(set_result),
                                pid,
                                config.name,
                                config.io_priority.as_str()
                            )
                        },
                    );
                } else {
                    apply_config_result.add_change(change_msg);
                }
            }
        }
    }
}

pub fn apply_memory_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) {
    let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
        return;
    };
    if let Some(memory_priority_flag) = config.memory_priority.as_win_const() {
        let mut current_mem_prio = MemoryPriorityInformation(0);
        match unsafe {
            GetProcessInformation(
                r_handle,
                ProcessMemoryPriority,
                &mut current_mem_prio as *mut _ as *mut c_void,
                size_of::<MemoryPriorityInformation>() as u32,
            )
        } {
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                log_error_if_new(
                    pid,
                    0,
                    &config.name,
                    Operation::GetProcessInformation2ProcessMemoryPriority,
                    error_code,
                    apply_config_result,
                    || {
                        format!(
                            "apply_config: [QUERY_MEMORY_PRIORITY][{}] {:>5}-{}",
                            error_from_code_win32(error_code),
                            pid,
                            config.name
                        )
                    },
                );
            }
            Ok(_) => {
                if current_mem_prio.0 != memory_priority_flag.0 {
                    let change_msg = format!(
                        "Memory Priority: {} -> {}",
                        MemoryPriority::from_win_const(current_mem_prio.0),
                        config.memory_priority.as_str()
                    );
                    if dry_run {
                        apply_config_result.add_change(format!("Memory Priority: -> {}", config.memory_priority.as_str()));
                    } else {
                        let mem_prio_info = MemoryPriorityInformation(memory_priority_flag.0);
                        match unsafe {
                            SetProcessInformation(
                                w_handle,
                                ProcessMemoryPriority,
                                &mem_prio_info as *const _ as *const c_void,
                                size_of::<MemoryPriorityInformation>() as u32,
                            )
                        } {
                            Err(_) => {
                                let error_code = unsafe { GetLastError().0 };
                                log_error_if_new(
                                    pid,
                                    0,
                                    &config.name,
                                    Operation::SetProcessInformation2ProcessMemoryPriority,
                                    error_code,
                                    apply_config_result,
                                    || {
                                        format!(
                                            "apply_config: [SET_MEMORY_PRIORITY][{}] {:>5}-{} -> {}",
                                            error_from_code_win32(error_code),
                                            pid,
                                            config.name,
                                            config.memory_priority.as_str()
                                        )
                                    },
                                );
                            }
                            Ok(_) => {
                                apply_config_result.add_change(change_msg);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Prefetches thread cycle counts for prime thread selection.
///
/// Opens handles to top CPU-consuming threads (by kernel+user time) and
/// queries their cycle counters. This establishes baseline measurements
/// for the hysteresis-based prime thread promotion/demotion algorithm.
pub fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) {
    let mut tid_with_delta_times: List<[(u32, i32); TIDS_FULL]> = {
        threads()
            .iter()
            .map(|(tid, thread)| {
                let total = unsafe { thread.KernelTime.QuadPart() + thread.UserTime.QuadPart() };
                let thread_stats = prime_scheduler.get_thread_stats(pid, *tid);
                thread_stats.cached_total_time = total;
                (*tid, (total - prime_scheduler.get_thread_stats(pid, *tid).last_total_time) as i32)
            })
            .collect()
    };
    if tid_with_delta_times.is_empty() {
        return;
    }

    tid_with_delta_times.sort_unstable_by_key(|(_, time)| Reverse(*time));
    let mut counter = 0;
    let counter_limit = (get_cpu_set_information().lock().unwrap().len() * 2)
        .min(tid_with_delta_times.len())
        .saturating_sub(1);
    let tid_with_delta_times_caped: List<[(u32, i32); TIDS_CAPED]> = tid_with_delta_times.into_iter().take(counter_limit + 1).collect();
    for &(tid, _) in &tid_with_delta_times_caped {
        let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
        if counter > counter_limit {
            break;
        }
        // Open thread handle if not already opened
        if thread_stats.handle.is_none() {
            if let Some(thread_handle) = get_thread_handle(tid, pid, &config.name) {
                thread_stats.handle = Some(thread_handle);
            } else {
                continue; // Failed to open, get_thread_handle already logged error
            }
        }
        let thread_handle = match thread_stats.handle.as_ref() {
            Some(h) => h,
            _ => continue,
        };

        let r_handle = match thread_handle.r_handle.is_invalid() {
            true => thread_handle.r_limited_handle,
            false => thread_handle.r_handle,
        };

        if thread_stats.start_address == 0 {
            thread_stats.start_address = get_thread_start_address(r_handle);
        }
        let mut cycles: u64 = 0;
        match unsafe { QueryThreadCycleTime(r_handle, &mut cycles) } {
            Ok(_) => {
                prime_scheduler.get_thread_stats(pid, tid).cached_cycles = cycles;
            }
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                log_error_if_new(
                    pid,
                    tid,
                    &config.name,
                    Operation::QueryThreadCycleTime,
                    error_code,
                    apply_config_result,
                    || {
                        format!(
                            "prefetch_thread_cycles: [QUERY_THREAD_CYCLE_TIME][{}] {:>5}-{}-{}",
                            error_from_code_win32(error_code),
                            pid,
                            tid,
                            config.name
                        )
                    },
                );
            }
        }
        counter += 1;
    }

    let tid_with_delta_cycles: List<[(u32, u64); TIDS_CAPED]> = prime_scheduler
        .pid_to_process_stats
        .get_mut(&pid)
        .map(|process_stats| {
            process_stats
                .tid_to_thread_stats
                .iter_mut()
                .filter_map(|(tid, thread_stats)| {
                    if thread_stats.cached_cycles > 0 {
                        Some((*tid, thread_stats.cached_cycles.saturating_sub(thread_stats.last_cycles)))
                    } else {
                        thread_stats.active_streak = 0;
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    prime_scheduler.update_active_streaks(pid, &tid_with_delta_cycles);
}

/// Applies prime thread scheduling to CPU-intensive threads.
///
/// The algorithm:
/// 1. Sort threads by CPU time delta to identify candidates
/// 2. Select top threads using hysteresis (entry/keep thresholds)
/// 3. Promote selected threads to prime CPUs with optional priority boost
/// 4. Demote threads that no longer qualify
///
/// Prime threads are pinned to specific CPUs via CPU Sets for better cache locality.
#[allow(clippy::too_many_arguments)]
pub fn apply_prime_threads<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) {
    let has_prime_cpus = !config.prime_threads_cpus.is_empty() || !config.prime_threads_prefixes.is_empty();
    let do_prime = has_prime_cpus && config.track_top_x_threads >= 0; // Negative track_top_x_threads disables prime
    let has_tracking = config.track_top_x_threads != 0;
    if !do_prime && !has_tracking {
        return;
    }
    if dry_run {
        if has_prime_cpus {
            apply_config_result.add_change(format!("Prime CPUs: -> [{}]", format_cpu_indices(&config.prime_threads_cpus)));
        }
        return;
    }
    if has_tracking {
        prime_core_scheduler.set_tracking_info(pid, config.track_top_x_threads, config.name.clone());
    }

    let thread_count = process.thread_count() as usize;
    let mut tid_with_time_deltas: List<[(u32, i32); TIDS_CAPED]> = List::new();
    for (&tid, thread) in threads().iter() {
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        if thread_stats.cached_cycles > 0 {
            tid_with_time_deltas.push((tid, (thread_stats.cached_total_time - thread_stats.last_total_time) as i32));
        }
        if has_tracking {
            thread_stats.last_system_thread_info = Some(*thread);
        }
    }
    tid_with_time_deltas.sort_unstable_by_key(|(_, time)| Reverse(*time));
    let prime_count = config.prime_threads_cpus.len();
    // Candidate pool: 4x prime slots or CPU count, whichever is larger, capped at thread count
    let candidate_count = (prime_count * 4)
        .max(get_cpu_set_information().lock().unwrap().len())
        .min(thread_count);
    let mut candidate_tids: List<[u32; TIDS_CAPED]> = tid_with_time_deltas.iter().take(candidate_count).map(|&(tid, _)| tid).collect();
    // Include previously-pinned threads that may have dropped out of top candidates
    // This ensures they can be properly demoted if they no longer qualify
    if let Some(process_stats) = prime_core_scheduler.pid_to_process_stats.get(&pid) {
        process_stats.tid_to_thread_stats.iter().for_each(|(tid, thread_stats)| {
            if thread_stats.pinned_cpu_set_ids.is_empty() && !candidate_tids.contains(tid) {
                candidate_tids.push(*tid);
            };
        });
    }

    let mut tid_with_delta_cycles: List<[(u32, u64, bool); TIDS_CAPED]> = candidate_tids
        .iter()
        .map(|&tid| {
            let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
            (tid, thread_stats.cached_cycles.saturating_sub(thread_stats.last_cycles), false)
        })
        .collect();

    apply_prime_threads_select(pid, prime_count, &mut tid_with_delta_cycles, prime_core_scheduler);
    apply_prime_threads_promote(
        pid,
        config,
        current_mask,
        &tid_with_delta_cycles,
        prime_core_scheduler,
        apply_config_result,
    );
    apply_prime_threads_demote(
        pid,
        config,
        threads,
        &tid_with_delta_cycles,
        prime_core_scheduler,
        apply_config_result,
    );

    // Clean up handles for threads that no longer exist in the process
    let live_set: HashSet<u32> = tid_with_time_deltas.iter().map(|&(tid, _)| tid).collect();
    if let Some(ps) = prime_core_scheduler.pid_to_process_stats.get_mut(&pid) {
        for (tid, ts) in ps.tid_to_thread_stats.iter_mut() {
            if !live_set.contains(tid) {
                // Remove handle - ThreadHandle's Drop will close handles automatically
                let _ = ts.handle.take();
            }
        }
    }
}

/// Selects top threads for prime status using hysteresis.
///
/// Hysteresis prevents threads from rapidly flipping between prime/non-prime:
/// - Currently prime threads stay prime if cycles >= keep_threshold% of max
/// - Non-prime threads become prime if cycles >= entry_threshold% of max AND active_streak >= min_active_streak
pub fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
) {
    prime_core_scheduler.select_top_threads_with_hysteresis(pid, tid_with_delta_cycles, prime_count, |thread_stats| {
        !thread_stats.pinned_cpu_set_ids.is_empty()
    });
}

/// Promotes selected threads to prime status with CPU pinning and optional priority boost.
///
/// For each thread marked as prime:
/// - Resolves start address to module name for prefix matching
/// - Applies module-specific CPU set if prefixes are configured
/// - Boosts thread priority (either explicitly configured or auto-boosted by one level)
pub fn apply_prime_threads_promote(
    pid: u32,
    config: &ThreadLevelConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) {
    for &(tid, delta_cycles, is_prime) in tid_with_delta_cycles {
        if !is_prime {
            continue;
        }
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        if let Some(ref thread_handle) = thread_stats.handle
            && thread_stats.pinned_cpu_set_ids.is_empty()
        {
            let handle: &HANDLE = match thread_handle.w_handle.is_invalid() {
                true => match thread_handle.w_limited_handle.is_invalid() {
                    true => {
                        log_error_if_new(pid, tid, &config.name, Operation::OpenThread, 0, apply_config_result, || {
                            format!(
                                "apply_prime_threads_promote: [GET_THREAD_HANDLE] Invalid handle {:>5}-{:>5}-{}",
                                pid, tid, &config.name
                            )
                        });
                        continue;
                    }
                    false => &thread_handle.w_limited_handle,
                },
                false => &thread_handle.w_handle,
            };
            let start_module = resolve_address_to_module(pid, thread_stats.start_address);
            let mut matched = false;
            let mut prime_cpus_to_set = &config.prime_threads_cpus;
            let mut thread_priority_to_set = ThreadPriority::None;
            for prefix in &config.prime_threads_prefixes {
                if start_module.to_lowercase().starts_with(&prefix.prefix.to_lowercase()) {
                    matched = true;
                    if let Some(ref cpus) = prefix.cpus {
                        prime_cpus_to_set = cpus;
                    }
                    thread_priority_to_set = prefix.thread_priority;
                    break;
                }
            }
            if !matched && !config.prime_threads_prefixes.is_empty() {
                continue;
            }
            let filtered_cpus = if *current_mask != 0 {
                filter_indices_by_mask(prime_cpus_to_set, *current_mask)
            } else {
                prime_cpus_to_set.clone()
            };
            let cpu_setids = cpusetids_from_indices(&filtered_cpus);
            if !cpu_setids.is_empty() {
                if !unsafe { SetThreadSelectedCpuSets(*handle, &cpu_setids) }.as_bool() {
                    let error_code = unsafe { GetLastError().0 };
                    log_error_if_new(
                        pid,
                        tid,
                        &config.name,
                        Operation::SetThreadSelectedCpuSets,
                        error_code,
                        apply_config_result,
                        || {
                            format!(
                                "apply_prime_threads_promote: [SET_THREAD_SELECTED_CPU_SETS][{}] {:>5}-{:>5}-{}",
                                error_from_code_win32(error_code),
                                pid,
                                tid,
                                config.name
                            )
                        },
                    );
                } else {
                    thread_stats.pinned_cpu_set_ids = cpu_setids.clone();
                    let promoted_cpus = indices_from_cpusetids(&cpu_setids);
                    apply_config_result.add_change(format!(
                        "Thread {} -> (promoted, [{}], cycles={}, start={})",
                        tid,
                        format_cpu_indices(&promoted_cpus),
                        delta_cycles,
                        start_module
                    ));
                }

                let current_priority = unsafe {
                    GetThreadPriority(match thread_handle.r_handle.is_invalid() {
                        true => thread_handle.r_limited_handle,
                        false => thread_handle.r_handle,
                    })
                };

                if current_priority != 0x7FFFFFFF_i32 {
                    let current_priority = ThreadPriority::from_win_const(current_priority);
                    thread_stats.original_priority = Some(current_priority);
                    let new_priority = if thread_priority_to_set != ThreadPriority::None {
                        thread_priority_to_set
                    } else {
                        current_priority.boost_one()
                    };
                    if new_priority != current_priority {
                        if unsafe { SetThreadPriority(*handle, new_priority.to_thread_priority_struct()) }.is_err() {
                            let error_code = unsafe { GetLastError().0 };
                            log_error_if_new(
                                pid,
                                tid,
                                &config.name,
                                Operation::SetThreadPriority,
                                error_code,
                                apply_config_result,
                                || {
                                    format!(
                                        "apply_prime_threads_promote: [SET_THREAD_PRIORITY][{}] {:>5}-{:>5}-{}",
                                        error_from_code_win32(error_code),
                                        pid,
                                        tid,
                                        config.name
                                    )
                                },
                            );
                        } else {
                            let old_name = current_priority.as_str();
                            let new_name = new_priority.as_str();
                            let action = if thread_priority_to_set != ThreadPriority::None {
                                "priority set"
                            } else {
                                "priority boosted"
                            };
                            apply_config_result.add_change(format!("Thread {} -> ({}: {} -> {})", tid, action, old_name, new_name));
                        }
                    }
                }
            }
        }
    }
}

/// Demotes threads that no longer qualify for prime status.
///
/// Removes CPU set pinning and restores original thread priority.
/// Clears pinned_cpu_set_ids even on failure to prevent infinite retry loops.
pub fn apply_prime_threads_demote<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) {
    let prime_set: HashSet<u32> = tid_with_delta_cycles
        .iter()
        .filter_map(|&(tid, _, is_prime)| if is_prime { Some(tid) } else { None })
        .collect();

    let live_tids: List<[u32; TIDS_CAPED]> = threads().keys().copied().collect();

    for tid in live_tids {
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        if prime_set.contains(&tid) || thread_stats.pinned_cpu_set_ids.is_empty() {
            continue;
        }
        let handle: &HANDLE = match &thread_stats.handle {
            Some(thread_handle) => match thread_handle.w_handle.is_invalid() {
                true => match thread_handle.w_limited_handle.is_invalid() {
                    true => {
                        log_error_if_new(pid, tid, &config.name, Operation::OpenThread, 0, apply_config_result, || {
                            format!(
                                "apply_prime_threads_demote: [GET_THREAD_HANDLE] Invalid handle {:>5}-{:>5}-{}",
                                pid, tid, &config.name
                            )
                        });
                        continue;
                    }
                    false => &thread_handle.w_limited_handle,
                },
                false => &thread_handle.w_handle,
            },
            _ => continue,
        };
        if !unsafe { SetThreadSelectedCpuSets(*handle, &[]) }.as_bool() {
            let error_code = unsafe { GetLastError().0 };
            log_error_if_new(
                pid,
                tid,
                &config.name,
                Operation::SetThreadSelectedCpuSets,
                error_code,
                apply_config_result,
                || {
                    format!(
                        "apply_prime_threads_demote: [SET_THREAD_SELECTED_CPU_SETS][{}] {:>5}-{:>5}-{}",
                        error_from_code_win32(error_code),
                        pid,
                        tid,
                        config.name
                    )
                },
            );
        } else {
            let start_module = resolve_address_to_module(pid, thread_stats.start_address);
            apply_config_result.add_change(format!("Thread {} -> (demoted, start={})", tid, start_module));
        }
        // whether this failed or not, clear pinned_cpu_set_ids to avoid infinite retries which spam in the logs
        thread_stats.pinned_cpu_set_ids.clear();

        if let Some(original_priority) = thread_stats.original_priority.take()
            && unsafe { SetThreadPriority(*handle, original_priority.to_thread_priority_struct()) }.is_err()
        {
            let error_code = unsafe { GetLastError().0 };
            log_error_if_new(
                pid,
                tid,
                &config.name,
                Operation::SetThreadPriority,
                error_code,
                apply_config_result,
                || {
                    format!(
                        "apply_prime_threads_promote: [RESTORE_SET_THREAD_PRIORITY][{}] {:>5}-{:>5}-{}",
                        error_from_code_win32(error_code),
                        pid,
                        tid,
                        config.name
                    )
                },
            );
        }
    }
}

/// Assigns ideal processors to threads based on their start module.
///
/// For each rule, identifies threads whose start module matches the prefix,
/// selects top N by cycle count (N = number of CPUs in rule), and assigns
/// each to a dedicated CPU. When a thread drops out of top N, its ideal
/// processor is restored to the previous value.
pub fn apply_ideal_processors<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) {
    if config.ideal_processor_rules.is_empty() {
        return;
    }

    if dry_run {
        for rule in &config.ideal_processor_rules {
            let cpus_str = format_cpu_indices(&rule.cpus);
            let prefixes_str = if rule.prefixes.is_empty() {
                "all modules".to_string()
            } else {
                rule.prefixes.join("; ")
            };
            apply_config_result.add_change(format!(
                "Ideal Processor: CPUs [{}] for top {} threads from [{}]",
                cpus_str,
                rule.cpus.len(),
                prefixes_str
            ));
        }
        return;
    }

    let mut module_names: Vec<String> = Vec::new();
    let mut all_threads: List<[(u32, u64, usize, usize); TIDS_CAPED]> = List::new();
    for &tid in threads().keys() {
        let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
        if thread_stats.cached_cycles == 0 {
            continue;
        }
        let start_addr = thread_stats.start_address;
        let name_idx = module_names.len();
        module_names.push(resolve_address_to_module(pid, start_addr));
        all_threads.push((tid, thread_stats.cached_cycles - thread_stats.last_cycles, start_addr, name_idx));
    }

    for rule in &config.ideal_processor_rules {
        if rule.cpus.is_empty() {
            continue;
        }

        let mut thread_infos: List<[(u32, u64, usize, usize); TIDS_CAPED]> = List::new();
        for &(tid, delta_cycles, start_addr, name_idx) in &all_threads {
            let start_module = &module_names[name_idx];
            let matches = if rule.prefixes.is_empty() {
                true
            } else {
                let lower = start_module.to_lowercase();
                rule.prefixes.iter().any(|prefix| lower.starts_with(prefix))
            };
            if matches {
                thread_infos.push((tid, delta_cycles, start_addr, name_idx));
            }
        }

        let mut selection: List<[(u32, u64, bool); TIDS_CAPED]> = thread_infos.iter().map(|&(tid, delta, _, _)| (tid, delta, false)).collect();
        prime_scheduler.select_top_threads_with_hysteresis(pid, &mut selection, rule.cpus.len(), |ts| ts.ideal_processor.is_assigned);
        let selected_set: HashSet<u32> = selection.iter().filter(|(_, _, p)| *p).map(|(t, _, _)| *t).collect();

        let mut claimed: HashSet<u32> = HashSet::default();
        for &(tid, _, is_prime) in &selection {
            if is_prime {
                let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
                if thread_stats.ideal_processor.is_assigned {
                    claimed.insert(thread_stats.ideal_processor.current_number as u32);
                } else {
                    let handle: &HANDLE = match &thread_stats.handle {
                        Some(thread_handle) => match thread_handle.w_handle.is_invalid() {
                            true => match thread_handle.w_limited_handle.is_invalid() {
                                true => {
                                    log_error_if_new(pid, tid, &config.name, Operation::OpenThread, 0, apply_config_result, || {
                                        format!(
                                            "apply_ideal_processors: [GET_THREAD_HANDLE] Invalid handle {:>5}-{:>5}-{}",
                                            pid, tid, &config.name
                                        )
                                    });
                                    continue;
                                }
                                false => &thread_handle.w_limited_handle,
                            },
                            false => &thread_handle.w_handle,
                        },
                        _ => continue,
                    };
                    match get_thread_ideal_processor_ex(*handle) {
                        Err(_) => {
                            let error_code = unsafe { GetLastError().0 };
                            log_error_if_new(
                                pid,
                                tid,
                                &config.name,
                                Operation::GetThreadIdealProcessorEx,
                                error_code,
                                apply_config_result,
                                || {
                                    format!(
                                        "apply_ideal_processor: [GET_IDEAL][{}] {:>5}-{}-{}",
                                        error_from_code_win32(error_code),
                                        pid,
                                        tid,
                                        config.name
                                    )
                                },
                            );
                        }
                        Ok(previous_processor_number) => {
                            thread_stats.ideal_processor.previous_group = previous_processor_number.Group;
                            thread_stats.ideal_processor.previous_number = previous_processor_number.Number;
                            thread_stats.ideal_processor.current_group = previous_processor_number.Group;
                            thread_stats.ideal_processor.current_number = previous_processor_number.Number;
                            if previous_processor_number.Group == 0 && rule.cpus.contains(&(previous_processor_number.Number as u32)) {
                                thread_stats.ideal_processor.is_assigned = true;
                                claimed.insert(previous_processor_number.Number as u32);
                            }
                        }
                    }
                }
            }
        }

        let free_pool: List<[u32; CONSUMER_CPUS]> = rule.cpus.iter().copied().filter(|c| !claimed.contains(c)).collect();
        let mut counter_free_pool = 0;
        for tid in &selected_set {
            let thread_stats = prime_scheduler.get_thread_stats(pid, *tid);
            if thread_stats.ideal_processor.is_assigned {
                continue;
            }
            let thread_handle = match &thread_stats.handle {
                Some(h) => h,
                _ => continue,
            };
            let handle: &HANDLE = match thread_handle.w_handle.is_invalid() {
                true => match thread_handle.w_limited_handle.is_invalid() {
                    true => {
                        log_error_if_new(pid, *tid, &config.name, Operation::OpenThread, 0, apply_config_result, || {
                            format!(
                                "apply_ideal_processors: [GET_THREAD_HANDLE] Invalid handle {:>5}-{:>5}-{}",
                                pid, tid, &config.name
                            )
                        });
                        continue;
                    }
                    false => &thread_handle.w_limited_handle,
                },
                false => &thread_handle.w_handle,
            };

            let target_cpu = if counter_free_pool < free_pool.len() {
                free_pool[counter_free_pool]
            } else {
                break;
            };
            counter_free_pool += 1;
            match set_thread_ideal_processor_ex(*handle, 0, target_cpu as u8) {
                Err(_) => {
                    let error_code = unsafe { GetLastError().0 };
                    log_error_if_new(
                        pid,
                        *tid,
                        &config.name,
                        Operation::SetThreadIdealProcessorEx,
                        error_code,
                        apply_config_result,
                        || {
                            format!(
                                "apply_ideal_processor: [SET_IDEAL][{}] {:>5}-{}-{}",
                                error_from_code_win32(error_code),
                                pid,
                                tid,
                                config.name
                            )
                        },
                    );
                }
                Ok(_) => {
                    thread_stats.ideal_processor.current_group = 0;
                    thread_stats.ideal_processor.current_number = target_cpu as u8;
                    thread_stats.ideal_processor.is_assigned = true;
                    let start_module = all_threads
                        .iter()
                        .find(|(t, _, _, _)| *t == *tid)
                        .map(|(_, _, _, idx)| module_names[*idx].as_str())
                        .unwrap_or("?");
                    apply_config_result.add_change(format!(
                        "Thread {} -> ideal CPU {} (group 0) start={}",
                        tid, target_cpu, start_module
                    ));
                }
            };
        }

        for &(tid, _, _, name_idx) in &thread_infos {
            let start_module = &module_names[name_idx];
            let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
            if !thread_stats.ideal_processor.is_assigned || selected_set.contains(&tid) {
                continue;
            }
            let prev_group = thread_stats.ideal_processor.previous_group;
            let prev_number = thread_stats.ideal_processor.previous_number;
            let cur_group = thread_stats.ideal_processor.current_group;
            let cur_number = thread_stats.ideal_processor.current_number;
            if prev_group != cur_group || prev_number != cur_number {
                let handle: &HANDLE = match &thread_stats.handle {
                    Some(thread_handle) => match thread_handle.w_handle.is_invalid() {
                        true => match thread_handle.w_limited_handle.is_invalid() {
                            true => {
                                log_error_if_new(pid, tid, &config.name, Operation::OpenThread, 0, apply_config_result, || {
                                    format!(
                                        "apply_ideal_processors: [GET_THREAD_HANDLE] Invalid handle {:>5}-{:>5}-{}",
                                        pid, tid, &config.name
                                    )
                                });
                                continue;
                            }
                            false => &thread_handle.w_limited_handle,
                        },
                        false => &thread_handle.w_handle,
                    },
                    _ => continue,
                };

                match set_thread_ideal_processor_ex(*handle, prev_group, prev_number) {
                    Err(_) => {
                        let error_code = unsafe { GetLastError().0 };
                        log_error_if_new(
                            pid,
                            tid,
                            &config.name,
                            Operation::SetThreadIdealProcessorEx,
                            error_code,
                            apply_config_result,
                            || {
                                format!(
                                    "apply_ideal_processor: [RESTORE_IDEAL][{}] {:>5}-{}-{}",
                                    error_from_code_win32(error_code),
                                    pid,
                                    tid,
                                    config.name
                                )
                            },
                        );
                    }
                    Ok(_) => {
                        thread_stats.ideal_processor.current_group = prev_group;
                        thread_stats.ideal_processor.current_number = prev_number;
                        apply_config_result.add_change(format!(
                            "Thread {} -> restored ideal CPU {} (group {}) start={}",
                            tid, prev_number, prev_group, start_module
                        ));
                    }
                }
            }
            thread_stats.ideal_processor.is_assigned = false;
        }
    }
}

pub fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) {
    if let Some(ps) = prime_scheduler.pid_to_process_stats.get_mut(&pid) {
        for ts in ps.tid_to_thread_stats.values_mut() {
            if ts.cached_cycles > 0 {
                ts.last_cycles = ts.cached_cycles;
                ts.cached_cycles = 0;
            }
            if ts.cached_total_time > 0 {
                ts.last_total_time = ts.cached_total_time;
                ts.cached_total_time = 0;
            }
        }
    }
}
