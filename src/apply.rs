use crate::config::{ProcessConfig, cpu_indices_to_mask, format_cpu_indices};
use crate::logging::error_from_code;
use crate::process::ProcessSnapshot;
use crate::scheduler::PrimeThreadScheduler;
use crate::winapi::{
    NtQueryInformationProcess, NtSetInformationProcess, cpusetids_from_indices, filter_indices_by_mask, get_cpu_set_information, get_thread_ideal_processor_ex,
    get_thread_start_address, indices_from_cpusetids, resolve_address_to_module, set_thread_ideal_processor_ex,
};
use std::collections::HashSet;
use std::mem::size_of;

use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE},
    System::{
        Threading::{
            GetPriorityClass, GetProcessAffinityMask, GetProcessDefaultCpuSets, GetProcessInformation, GetThreadPriority, OpenThread, ProcessMemoryPriority,
            SetPriorityClass, SetProcessAffinityMask, SetProcessDefaultCpuSets, SetProcessInformation, SetThreadPriority, SetThreadSelectedCpuSets,
            THREAD_QUERY_INFORMATION, THREAD_SET_INFORMATION, THREAD_SET_LIMITED_INFORMATION,
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

    /// pid & name will be logged in main loop
    pub fn add_change(&mut self, change: String) {
        self.changes.push(change);
    }
    /// pid & name will not be logged in main loop
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty() && self.errors.is_empty()
    }
}

pub fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
    if let Some(priority_flag) = config.priority.as_win_const() {
        let current_priority = unsafe { GetPriorityClass(h_prc) };
        if current_priority != priority_flag.0 {
            if dry_run {
                apply_config_result.add_change(format!(
                    "Priority: {} -> {}",
                    crate::priority::ProcessPriority::from_win_const(current_priority),
                    config.priority.as_str()
                ));
            } else {
                let set_result = unsafe { SetPriorityClass(h_prc, priority_flag) };
                if set_result.is_ok() {
                    apply_config_result.add_change(format!(
                        "Priority: {} -> {}",
                        crate::priority::ProcessPriority::from_win_const(current_priority),
                        config.priority.as_str()
                    ));
                } else {
                    let error_code = unsafe { GetLastError().0 };
                    apply_config_result.add_error(format!("apply_config: [SET_PRIORITY][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                }
            }
        }
    }
}

pub fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    h_prc: HANDLE,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) {
    let mut system_mask: usize = 0;
    let affinity_mask = cpu_indices_to_mask(&config.affinity_cpus);
    let has_affinity = !config.affinity_cpus.is_empty();
    let has_prime = !config.prime_threads_cpus.is_empty();
    if has_affinity || has_prime {
        match unsafe { GetProcessAffinityMask(h_prc, &mut *current_mask, &mut system_mask) } {
            Err(_) => {
                if !dry_run {
                    let error_code = unsafe { GetLastError().0 };
                    apply_config_result.add_error(format!("apply_config: [QUERY_AFFINITY][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                }
            }
            Ok(_) => {
                if has_affinity && affinity_mask != 0 && affinity_mask != *current_mask {
                    if dry_run {
                        apply_config_result.add_change(format!("Affinity: {:#X} -> {:#X}", current_mask, affinity_mask));
                    } else {
                        match unsafe { SetProcessAffinityMask(h_prc, affinity_mask) } {
                            Err(_) => {
                                let error_code = unsafe { GetLastError().0 };
                                apply_config_result.add_error(format!("apply_config: [SET_AFFINITY][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                            }
                            Ok(_) => {
                                apply_config_result.add_change(format!("Affinity: {:#X} -> {:#X}", current_mask, affinity_mask));
                                *current_mask = affinity_mask;
                                reset_thread_ideal_processors(pid, config, false, h_prc, apply_config_result, processes);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
    if !config.cpu_set_cpus.is_empty() && !get_cpu_set_information().lock().unwrap().is_empty() {
        if dry_run {
            apply_config_result.add_change(format!("CPU Set: -> [{}]", format_cpu_indices(&config.cpu_set_cpus)));
        } else {
            let target_cpusetids = cpusetids_from_indices(&config.cpu_set_cpus);
            let mut current_cpusetids: Vec<u32> = vec![];
            if !target_cpusetids.is_empty() {
                let mut toset: bool = false;
                let mut requiredidcount: u32 = 0;
                let query_result = unsafe { GetProcessDefaultCpuSets(h_prc, None, &mut requiredidcount) }.as_bool();
                if query_result {
                    // 0 is large enough, meaning there are no default CPU sets for this process
                    toset = true;
                } else {
                    let code = unsafe { GetLastError().0 };
                    if code != 122 {
                        apply_config_result.add_error(format!(
                            "apply_config: [QUERY_CPUSET][{}] {:>5}-{}-{}",
                            error_from_code(code),
                            pid,
                            config.name,
                            requiredidcount
                        ));
                    } else {
                        current_cpusetids = vec![0u32; requiredidcount as usize];
                        let second_query = unsafe { GetProcessDefaultCpuSets(h_prc, Some(&mut current_cpusetids[..]), &mut requiredidcount) }.as_bool();
                        if !second_query {
                            let error_code = unsafe { GetLastError().0 };
                            apply_config_result.add_error(format!(
                                "apply_config: [QUERY_CPUSET][{}] {:>5}-{}-{}",
                                error_from_code(error_code),
                                pid,
                                config.name,
                                requiredidcount
                            ));
                        } else {
                            toset = current_cpusetids != target_cpusetids;
                        }
                    }
                }
                if toset {
                    let set_result = unsafe { SetProcessDefaultCpuSets(h_prc, Some(&target_cpusetids)) }.as_bool();
                    if !set_result {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!("apply_config: [SET_CPUSET][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                    } else {
                        apply_config_result.add_change(format!(
                            "CPU Set: [{}] -> [{}]",
                            format_cpu_indices(&current_cpusetids),
                            format_cpu_indices(&config.cpu_set_cpus)
                        ));
                    }
                }
            }
        }
    }
}

pub fn prefetch_all_thread_cycles(
    pid: u32,
    process_name: &str,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) {
    let mut tid_with_delta_times: Vec<(u32, i64)> = match processes.pid_to_process.get_mut(&pid) {
        Some(process) => process
            .get_threads()
            .iter()
            .map(|(tid, thread)| {
                let total = unsafe { thread.KernelTime.QuadPart() + thread.UserTime.QuadPart() };
                let thread_stats = prime_scheduler.get_thread_stats(pid, *tid);
                thread_stats.cached_total_time = total;
                (*tid, total - prime_scheduler.get_thread_stats(pid, *tid).last_total_time)
            })
            .collect(),
        None => return,
    };
    if tid_with_delta_times.is_empty() {
        return;
    }

    tid_with_delta_times.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let mut counter = 0;
    let counter_limit = (get_cpu_set_information().lock().unwrap().len() * 2).min(tid_with_delta_times.len()) - 1;
    for &(tid, _) in &tid_with_delta_times {
        let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
        if counter > counter_limit {
            break;
        }
        if thread_stats.handle.is_none() {
            match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION | THREAD_SET_INFORMATION, false, tid) } {
                Err(_) => {
                    let code = unsafe { GetLastError().0 };
                    apply_config_result.add_error(format!(
                        "prefetch_thread_cycles: [OPEN_THREAD][{}] {:>5}-{}-{}",
                        crate::logging::error_from_code(code),
                        pid,
                        tid,
                        process_name
                    ));
                    continue;
                }
                Ok(h) => thread_stats.handle = Some(h),
            }
        }
        let handle = match thread_stats.handle {
            Some(h) if !h.is_invalid() => h,
            _ => continue,
        };
        if thread_stats.start_address == 0 {
            thread_stats.start_address = get_thread_start_address(handle);
        }
        let mut cycles: u64 = 0;
        match unsafe { QueryThreadCycleTime(handle, &mut cycles) } {
            Ok(_) => {
                prime_scheduler.get_thread_stats(pid, tid).cached_cycles = cycles;
            }
            Err(_) => {
                let code = unsafe { GetLastError().0 };
                apply_config_result.add_error(format!(
                    "prefetch_thread_cycles: [QUERY_CYCLES][{}] {:>5}-{}-{}",
                    crate::logging::error_from_code(code),
                    pid,
                    tid,
                    process_name
                ));
            }
        }
        counter += 1;
    }

    let tid_with_delta_cycles: Vec<(u32, u64)> = prime_scheduler
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

pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) {
    let has_prime_cpus = !config.prime_threads_cpus.is_empty() || !config.prime_threads_prefixes.is_empty();
    let do_prime = has_prime_cpus && config.track_top_x_threads >= 0;
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

    let Some(process) = processes.pid_to_process.get_mut(&pid) else { return };
    let thread_count = process.thread_count() as usize;
    let mut tid_with_time_deltas: Vec<(u32, i64)> = Vec::with_capacity(thread_count);
    for (&tid, thread) in process.get_threads().iter() {
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        tid_with_time_deltas.push((tid, thread_stats.cached_total_time - thread_stats.last_total_time));
        if has_tracking {
            thread_stats.last_system_thread_info = Some(*thread);
        }
    }
    tid_with_time_deltas.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let prime_count = config.prime_threads_cpus.len();
    let candidate_count = (prime_count * 4).max(get_cpu_set_information().lock().unwrap().len()).min(thread_count);
    let mut candidate_tids: Vec<u32> = tid_with_time_deltas.iter().take(candidate_count).map(|&(tid, _)| tid).collect();
    if let Some(process_stats) = prime_core_scheduler.pid_to_process_stats.get(&pid) {
        process_stats.tid_to_thread_stats.iter().for_each(|(tid, thread_stats)| {
            if thread_stats.pinned_cpu_set_ids.is_empty() && !candidate_tids.contains(tid) {
                candidate_tids.push(*tid);
            };
        });
    }

    let mut tid_with_delta_cycles: Vec<(u32, u64, bool)> = candidate_tids
        .iter()
        .map(|&tid| {
            let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
            (tid, thread_stats.cached_cycles.saturating_sub(thread_stats.last_cycles), false)
        })
        .collect();

    apply_prime_threads_select(&mut tid_with_delta_cycles, prime_core_scheduler, pid, prime_count);
    apply_prime_threads_promote(&tid_with_delta_cycles, prime_core_scheduler, pid, config, current_mask, apply_config_result);
    apply_prime_threads_demote(process, &tid_with_delta_cycles, prime_core_scheduler, pid, config, apply_config_result);

    let live_set: HashSet<u32> = tid_with_time_deltas.iter().map(|&(tid, _)| tid).collect();
    if let Some(ps) = prime_core_scheduler.pid_to_process_stats.get_mut(&pid) {
        for (tid, ts) in ps.tid_to_thread_stats.iter_mut() {
            if !live_set.contains(tid)
                && let Some(h) = ts.handle.take()
            {
                unsafe {
                    let _ = CloseHandle(h);
                }
            }
        }
    }
}

pub fn apply_prime_threads_select(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) {
    prime_core_scheduler.select_top_threads_with_hysteresis(pid, tid_with_delta_cycles, prime_count, |thread_stats| !thread_stats.pinned_cpu_set_ids.is_empty());
}

pub fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) {
    for &(tid, delta_cycles, is_prime) in tid_with_delta_cycles {
        if !is_prime {
            continue;
        }
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        if let Some(handle) = thread_stats.handle
            && !handle.is_invalid()
            && thread_stats.pinned_cpu_set_ids.is_empty()
        {
            let start_module = resolve_address_to_module(pid, thread_stats.start_address);
            let mut matched = false;
            let mut prime_cpus_to_set = &config.prime_threads_cpus;
            let mut thread_priority_to_set = crate::priority::ThreadPriority::None;
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
                if !unsafe { SetThreadSelectedCpuSets(handle, &cpu_setids) }.as_bool() {
                    let error_code = unsafe { GetLastError().0 };
                    apply_config_result.add_error(format!(
                        "apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}",
                        error_from_code(error_code),
                        pid,
                        tid,
                        config.name
                    ));
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
                let current_priority = unsafe { GetThreadPriority(handle) };
                // 0x7FFFFFFF_i32 -> ThreadPriority::ErrorReturn
                if current_priority != 0x7FFFFFFF_i32 {
                    let current_priority = crate::priority::ThreadPriority::from_win_const(current_priority);
                    thread_stats.original_priority = Some(current_priority);
                    let new_priority = if thread_priority_to_set != crate::priority::ThreadPriority::None {
                        thread_priority_to_set
                    } else {
                        current_priority.boost_one()
                    };
                    if new_priority != current_priority {
                        if unsafe { SetThreadPriority(handle, new_priority.to_thread_priority_struct()) }.is_err() {
                            let error_code = unsafe { GetLastError().0 };
                            apply_config_result.add_error(format!(
                                "apply_config: [SET_THREAD_PRIORITY][{}] {:>5}-{}-{}",
                                error_from_code(error_code),
                                pid,
                                tid,
                                config.name
                            ));
                        } else {
                            let old_name = current_priority.as_str();
                            let new_name = new_priority.as_str();
                            let action = if thread_priority_to_set != crate::priority::ThreadPriority::None {
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

pub fn apply_prime_threads_demote(
    process: &mut crate::process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) {
    let prime_set: HashSet<u32> = tid_with_delta_cycles
        .iter()
        .filter_map(|&(tid, _, is_prime)| if is_prime { Some(tid) } else { None })
        .collect();

    let live_tids: Vec<u32> = process.get_threads().keys().copied().collect();

    for tid in live_tids {
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        if prime_set.contains(&tid) || thread_stats.pinned_cpu_set_ids.is_empty() {
            continue;
        }
        let Some(handle) = thread_stats.handle else { continue };
        if handle.is_invalid() {
            continue;
        }
        if !unsafe { SetThreadSelectedCpuSets(handle, &[]) }.as_bool() {
            let error_code = unsafe { GetLastError().0 };
            apply_config_result.add_error(format!(
                "apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}",
                error_from_code(error_code),
                pid,
                tid,
                config.name
            ));
        } else {
            let start_module = resolve_address_to_module(pid, thread_stats.start_address);
            apply_config_result.add_change(format!("Thread {} -> (demoted, start={})", tid, start_module));
        }
        // whether this failed or not, clear pinned_cpu_set_ids to avoid infinite retries which spam in the logs
        thread_stats.pinned_cpu_set_ids.clear();

        if let Some(original_priority) = thread_stats.original_priority.take() // same as above
            && unsafe { SetThreadPriority(handle, original_priority.to_thread_priority_struct()) }.is_err()
        {
            let error_code = unsafe { GetLastError().0 };
            apply_config_result.add_error(format!(
                "apply_config: [RESTORE_THREAD_PRIORITY][{}] {:>5}-{}-{}",
                error_from_code(error_code),
                pid,
                tid,
                config.name
            ));
        }
    }
}

pub fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
    if let Some(io_priority_flag) = config.io_priority.as_win_const() {
        const PROCESS_INFORMATION_IO_PRIORITY: u32 = 33;
        let mut current_io_priority: u32 = 0;
        let mut return_length: u32 = 0;
        let query_result = unsafe {
            NtQueryInformationProcess(
                h_prc,
                PROCESS_INFORMATION_IO_PRIORITY,
                &mut current_io_priority as *mut _ as *mut std::ffi::c_void,
                size_of::<u32>() as u32,
                &mut return_length,
            )
        }
        .0;
        if query_result < 0 {
            apply_config_result.add_error(format!(
                "apply_config: [QUERY_IO_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                query_result,
                pid,
                config.name,
                config.io_priority.as_str()
            ));
        } else if current_io_priority != io_priority_flag {
            if dry_run {
                apply_config_result.add_change(format!(
                    "IO Priority: {} -> {}",
                    crate::priority::IOPriority::from_win_const(current_io_priority),
                    config.io_priority.as_str()
                ));
            } else {
                let set_result = unsafe {
                    NtSetInformationProcess(
                        h_prc,
                        PROCESS_INFORMATION_IO_PRIORITY,
                        &io_priority_flag as *const _ as *const std::ffi::c_void,
                        size_of::<u32>() as u32,
                    )
                }
                .0;
                if set_result < 0 {
                    apply_config_result.add_error(format!(
                        "apply_config: [SET_IO_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                        set_result,
                        pid,
                        config.name,
                        config.io_priority.as_str()
                    ));
                } else {
                    apply_config_result.add_change(format!(
                        "IO Priority: {} -> {}",
                        crate::priority::IOPriority::from_win_const(current_io_priority),
                        config.io_priority.as_str()
                    ));
                }
            }
        }
    }
}

pub fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
    if let Some(memory_priority_flag) = config.memory_priority.as_win_const() {
        let mut current_mem_prio = crate::priority::MemoryPriorityInformation(0);
        match unsafe {
            GetProcessInformation(
                h_prc,
                ProcessMemoryPriority,
                &mut current_mem_prio as *mut _ as *mut std::ffi::c_void,
                size_of::<crate::priority::MemoryPriorityInformation>() as u32,
            )
        } {
            Err(_) => {
                let err = unsafe { GetLastError().0 };
                apply_config_result.add_error(format!(
                    "apply_config: [QUERY_MEMORY_PRIORITY][{}] {:>5}-{}",
                    crate::logging::error_from_code(err),
                    pid,
                    config.name
                ));
            }
            Ok(_) => {
                if current_mem_prio.0 != memory_priority_flag.0 {
                    if dry_run {
                        apply_config_result.add_change(format!("Memory Priority: -> {}", config.io_priority.as_str()));
                    } else {
                        let mem_prio_info = crate::priority::MemoryPriorityInformation(memory_priority_flag.0);
                        match unsafe {
                            SetProcessInformation(
                                h_prc,
                                ProcessMemoryPriority,
                                &mem_prio_info as *const _ as *const std::ffi::c_void,
                                size_of::<crate::priority::MemoryPriorityInformation>() as u32,
                            )
                        } {
                            Err(_) => {
                                let error_code = unsafe { GetLastError().0 };
                                apply_config_result.add_error(format!(
                                    "apply_config: [SET_MEMORY_PRIORITY][{}] {:>5}-{} -> {}",
                                    error_from_code(error_code),
                                    pid,
                                    config.name,
                                    config.memory_priority.as_str()
                                ));
                            }
                            Ok(_) => {
                                apply_config_result.add_change(format!(
                                    "Memory Priority: {} -> {}",
                                    crate::priority::MemoryPriority::from_win_const(current_mem_prio.0),
                                    config.memory_priority.as_str()
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    _h_prc: HANDLE,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) {
    if config.affinity_cpus.is_empty() {
        return;
    }

    if dry_run {
        apply_config_result.add_change(format!("Reset Ideal Processors: {} threads based on CPU time", config.affinity_cpus.len()));
        return;
    }

    let Some(process) = processes.pid_to_process.get_mut(&pid) else { return };

    let mut thread_times: Vec<(u32, i64, HANDLE)> = Vec::new();

    for (tid, thread_info) in process.get_threads() {
        let total_time = unsafe { *thread_info.KernelTime.QuadPart() + *thread_info.UserTime.QuadPart() };

        match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION | THREAD_SET_INFORMATION, false, *tid) } {
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                let open_thread_error = error_from_code(error_code);
                apply_config_result.add_error(format!(
                    "reset_ideal_processor: [OPEN][{}] {:>5}-{}-{:>5} - OpenThread failed: {}",
                    open_thread_error, pid, config.name, tid, open_thread_error,
                ));
            }
            Ok(thread_handle) => {
                thread_times.push((*tid, total_time, thread_handle));
            }
        }
    }

    if thread_times.is_empty() {
        return;
    }

    thread_times.sort_by(|a, b| b.1.cmp(&a.1));

    let target_cpu_count = config.affinity_cpus.len();
    let random_shift = rand::random::<u8>();
    let mut counter_set_success = 0;
    for (i, (tid, _cpu_time, thread_handle)) in thread_times.iter().enumerate() {
        let target_cpu_index = (i + random_shift as usize) % target_cpu_count;
        let target_cpu = config.affinity_cpus[target_cpu_index];

        match set_thread_ideal_processor_ex(*thread_handle, 0, target_cpu as u8) {
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                apply_config_result.add_error(format!(
                    "reset_ideal_processor: [SET_IDEAL][{}] {:>5}-{}-{} - SetThreadIdealProcessorEx failed",
                    error_from_code(error_code),
                    pid,
                    tid,
                    config.name
                ));
            }
            Ok(_) => {
                counter_set_success += 1;
            }
        }
    }

    apply_config_result.add_change(format!("reset ideal processor for {} threads", counter_set_success));

    for (_tid, _cpu_time, thread_handle) in thread_times {
        unsafe {
            let _ = CloseHandle(thread_handle);
        }
    }
}

pub fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
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

    let Some(process) = processes.pid_to_process.get_mut(&pid) else { return };
    let tids: Vec<u32> = process.get_threads().keys().copied().collect();
    let mut all_threads: Vec<(u32, u64, usize, String)> = Vec::new();
    for &tid in &tids {
        let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
        let cycles = thread_stats.cached_cycles;
        if cycles == 0 {
            continue;
        }
        let start_addr = thread_stats.start_address;
        all_threads.push((tid, cycles - thread_stats.last_cycles, start_addr, resolve_address_to_module(pid, start_addr)));
    }

    for rule in &config.ideal_processor_rules {
        if rule.cpus.is_empty() {
            continue;
        }

        let mut thread_infos: Vec<(u32, u64, usize, String)> = Vec::new();
        for &(tid, delta_cycles, start_addr, ref start_module) in &all_threads {
            let matches = if rule.prefixes.is_empty() {
                true
            } else {
                let lower = start_module.to_lowercase();
                rule.prefixes.iter().any(|prefix| lower.starts_with(prefix))
            };
            if matches {
                thread_infos.push((tid, delta_cycles, start_addr, start_module.clone()));
            }
        }

        let mut selection: Vec<(u32, u64, bool)> = thread_infos.iter().map(|&(tid, delta, _, _)| (tid, delta, false)).collect();
        prime_scheduler.select_top_threads_with_hysteresis(pid, &mut selection, rule.cpus.len(), |ts| ts.ideal_processor.is_assigned);
        let selected_set: HashSet<u32> = selection.iter().filter(|(_, _, p)| *p).map(|(t, _, _)| *t).collect();

        let mut claimed: HashSet<u32> = HashSet::new();
        for &(tid, _, is_prime) in &selection {
            if is_prime {
                let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
                if thread_stats.ideal_processor.is_assigned {
                    claimed.insert(thread_stats.ideal_processor.current_number as u32);
                } else {
                    let handle = match thread_stats.handle {
                        Some(h) if !h.is_invalid() => h,
                        _ => continue,
                    };
                    match get_thread_ideal_processor_ex(handle) {
                        Err(_) => {
                            let error_code = unsafe { GetLastError().0 };
                            apply_config_result.add_error(format!(
                                "apply_ideal_processor: [GET_IDEAL][{}] {:>5}-{}-{}",
                                error_from_code(error_code),
                                pid,
                                tid,
                                config.name
                            ));
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

        let free_pool: Vec<u32> = rule.cpus.iter().copied().filter(|c| !claimed.contains(c)).collect();
        let mut counter_free_pool = 0;
        for tid in &selected_set {
            let thread_stats = prime_scheduler.get_thread_stats(pid, *tid);
            if thread_stats.ideal_processor.is_assigned {
                continue;
            }
            let handle = match thread_stats.handle {
                Some(h) if !h.is_invalid() => h,
                _ => continue,
            };

            let target_cpu = if counter_free_pool < free_pool.len() {
                free_pool[counter_free_pool]
            } else {
                break;
            };
            counter_free_pool += 1;
            match set_thread_ideal_processor_ex(handle, 0, target_cpu as u8) {
                Err(_) => {
                    let error_code = unsafe { GetLastError().0 };
                    apply_config_result.add_error(format!(
                        "apply_ideal_processor: [SET_IDEAL][{}] {:>5}-{}-{}",
                        error_from_code(error_code),
                        pid,
                        tid,
                        config.name
                    ));
                }
                Ok(_) => {
                    thread_stats.ideal_processor.current_group = 0;
                    thread_stats.ideal_processor.current_number = target_cpu as u8;
                    thread_stats.ideal_processor.is_assigned = true;
                    let start_module = all_threads.iter().find(|(t, _, _, _)| *t == *tid).map(|(_, _, _, m)| m.as_str()).unwrap_or("?");
                    apply_config_result.add_change(format!("Thread {} -> ideal CPU {} (group 0) start={}", tid, target_cpu, start_module));
                }
            };
        }

        // restore old candidates who failed to meet the active streak threshold
        for &(tid, _, _, ref start_module) in &thread_infos {
            let thread_stats = prime_scheduler.get_thread_stats(pid, tid);
            if !thread_stats.ideal_processor.is_assigned || selected_set.contains(&tid) {
                continue;
            }
            let prev_group = thread_stats.ideal_processor.previous_group;
            let prev_number = thread_stats.ideal_processor.previous_number;
            let cur_group = thread_stats.ideal_processor.current_group;
            let cur_number = thread_stats.ideal_processor.current_number;
            if (prev_group != cur_group || prev_number != cur_number)
                && let Some(handle) = thread_stats.handle
                && !handle.is_invalid()
            {
                match set_thread_ideal_processor_ex(handle, prev_group, prev_number) {
                    Err(_) => {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!(
                            "apply_ideal_processor: [RESTORE_IDEAL][{}] {:>5}-{}-{}",
                            error_from_code(error_code),
                            pid,
                            tid,
                            config.name
                        ));
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
