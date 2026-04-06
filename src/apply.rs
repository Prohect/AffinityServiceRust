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

    let tid_with_delta: Vec<(u32, u64)> = prime_scheduler
        .pid_to_process_stats
        .get(&pid)
        .map(|process_stats| {
            process_stats
                .tid_to_thread_stats
                .iter()
                .filter(|(_, ts)| ts.cached_cycles > 0)
                .map(|(&tid, ts)| (tid, ts.cached_cycles.saturating_sub(ts.last_cycles)))
                .collect()
        })
        .unwrap_or_default();
    prime_scheduler.update_active_streaks(pid, &tid_with_delta, |ts| &mut ts.active_streak);
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
    let has_prime_cpus = !config.prime_threads_cpus.is_empty();
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

    prime_core_scheduler.set_alive(pid);
    if has_tracking {
        prime_core_scheduler.set_tracking_info(pid, config.track_top_x_threads, config.name.clone());
    }

    let Some(process) = processes.pid_to_process.get_mut(&pid) else { return };

    let mut tid_time_deltas: Vec<(u32, i64)> = Vec::with_capacity(process.thread_count() as usize);
    for (&tid, thread) in process.get_threads().iter() {
        let ts = prime_core_scheduler.get_thread_stats(pid, tid);
        tid_time_deltas.push((tid, ts.cached_total_time - ts.last_total_time));
        if has_tracking {
            ts.last_system_thread_info = Some(*thread);
        }
    }

    if !do_prime {
        return;
    }

    let thread_count = tid_time_deltas.len();
    if thread_count == 0 {
        return;
    }

    let prime_count = config.prime_threads_cpus.len();
    let cpu_count = get_cpu_set_information().lock().unwrap().len();
    let k = (prime_count.saturating_mul(4)).max(cpu_count).min(thread_count);

    tid_time_deltas.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let mut candidate_set: Vec<u32> = tid_time_deltas.iter().take(k).map(|&(tid, _)| tid).collect();

    let promoted_tids: Vec<u32> = {
        match prime_core_scheduler.pid_to_process_stats.get(&pid) {
            Some(ps) => ps
                .tid_to_thread_stats
                .iter()
                .filter(|(_, ts)| !ts.cpu_set_ids.is_empty())
                .map(|(&tid, _)| tid)
                .collect(),
            None => vec![],
        }
    };
    for tid in promoted_tids {
        if !candidate_set.contains(&tid) {
            candidate_set.push(tid);
        }
    }

    let mut tid_with_delta_cycles: Vec<(u32, u64, bool)> = candidate_set
        .iter()
        .map(|&tid| {
            let ts = prime_core_scheduler.get_thread_stats(pid, tid);
            let cached = ts.cached_cycles;
            let delta = cached.saturating_sub(ts.last_cycles);
            (tid, delta, false)
        })
        .collect();

    apply_prime_threads_select(&mut tid_with_delta_cycles, prime_core_scheduler, pid, prime_count);

    apply_prime_threads_promote(&tid_with_delta_cycles, prime_core_scheduler, pid, config, current_mask, apply_config_result);
    apply_prime_threads_demote(process, &tid_with_delta_cycles, prime_core_scheduler, pid, config, apply_config_result);

    let live_set: HashSet<u32> = tid_time_deltas.iter().map(|&(tid, _)| tid).collect();
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
    prime_core_scheduler.select_top_threads_with_hysteresis(pid, tid_with_delta_cycles, prime_count, |ts| &mut ts.active_streak, |ts| !ts.cpu_set_ids.is_empty());
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
            && thread_stats.cpu_set_ids.is_empty()
        {
            let start_module = resolve_address_to_module(pid, thread_stats.start_address);
            let mut matched = false;
            let mut effective_prime_cpus = &config.prime_threads_cpus;
            let mut effective_thread_priority = crate::priority::ThreadPriority::None;
            for prefix in &config.prime_threads_prefixes {
                if start_module.to_lowercase().starts_with(&prefix.prefix.to_lowercase()) {
                    matched = true;
                    if let Some(ref cpus) = prefix.cpus {
                        effective_prime_cpus = cpus;
                    }
                    effective_thread_priority = prefix.thread_priority;
                    break;
                }
            }
            if !matched && !config.prime_threads_prefixes.is_empty() {
                continue;
            }
            let filtered_cpus = if *current_mask != 0 {
                filter_indices_by_mask(effective_prime_cpus, *current_mask)
            } else {
                effective_prime_cpus.clone()
            };
            let cpu_setids = cpusetids_from_indices(&filtered_cpus);
            if !cpu_setids.is_empty() {
                let set_result = unsafe { SetThreadSelectedCpuSets(handle, &cpu_setids) }.as_bool();
                if !set_result {
                    let error_code = unsafe { GetLastError().0 };
                    apply_config_result.add_error(format!(
                        "apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}",
                        error_from_code(error_code),
                        pid,
                        tid,
                        config.name
                    ));
                } else {
                    thread_stats.cpu_set_ids = cpu_setids.clone();
                    let promoted_cpus = indices_from_cpusetids(&cpu_setids);
                    apply_config_result.add_change(format!(
                        "Thread {} -> (promoted, [{}], cycles={}, start={})",
                        tid,
                        format_cpu_indices(&promoted_cpus),
                        delta_cycles,
                        start_module
                    ));
                }
                let current_prio = unsafe { GetThreadPriority(handle) };
                if current_prio != 0x7FFFFFFF_i32 {
                    let current_prio = crate::priority::ThreadPriority::from_win_const(current_prio);
                    thread_stats.original_priority = Some(current_prio);

                    let new_prio = if effective_thread_priority != crate::priority::ThreadPriority::None {
                        effective_thread_priority
                    } else {
                        current_prio.boost_one()
                    };

                    if new_prio != current_prio {
                        if unsafe { SetThreadPriority(handle, new_prio.to_thread_priority_struct()) }.is_err() {
                            let error_code = unsafe { GetLastError().0 };
                            apply_config_result.add_error(format!(
                                "apply_config: [SET_THREAD_PRIORITY][{}] {:>5}-{}-{}",
                                error_from_code(error_code),
                                pid,
                                tid,
                                config.name
                            ));
                        } else {
                            let old_name = current_prio.as_str();
                            let new_name = new_prio.as_str();
                            let action = if effective_thread_priority != crate::priority::ThreadPriority::None {
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
        let needs_demote = {
            let ts = prime_core_scheduler.get_thread_stats(pid, tid);
            !ts.cpu_set_ids.is_empty() && !prime_set.contains(&tid)
        };
        if !needs_demote {
            continue;
        }

        let (handle_opt, start_address, original_priority_opt) = {
            let ts = prime_core_scheduler.get_thread_stats(pid, tid);
            let h = ts.handle;
            let addr = ts.start_address;
            let prio = ts.original_priority.take(); // also clears ts.original_priority
            ts.cpu_set_ids.clear();
            (h, addr, prio)
        };

        let Some(handle) = handle_opt else { continue };
        if handle.is_invalid() {
            continue;
        }

        let set_result = unsafe { SetThreadSelectedCpuSets(handle, &[]) }.as_bool();
        if !set_result {
            let error_code = unsafe { GetLastError().0 };
            apply_config_result.add_error(format!(
                "apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}",
                error_from_code(error_code),
                pid,
                tid,
                config.name
            ));
        } else {
            let start_module = resolve_address_to_module(pid, start_address);
            apply_config_result.add_change(format!("Thread {} -> (demoted, start={})", tid, start_module));
        }

        if let Some(original_priority) = original_priority_opt
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

    let process = match processes.pid_to_process.get_mut(&pid) {
        Some(p) => p,
        None => return,
    };

    let mut thread_times: Vec<(u32, i64, HANDLE)> = Vec::new();

    for (tid, thread_info) in process.get_threads() {
        let total_time = unsafe { *thread_info.KernelTime.QuadPart() + *thread_info.UserTime.QuadPart() };

        match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION | THREAD_SET_INFORMATION, false, *tid) } {
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                let open_thread_error = error_from_code(error_code);
                apply_config_result.add_error(format!(
                    "reset_ideal_processor: [OPEN][{}] {:>5}-{} - OpenThread failed: {}",
                    open_thread_error, pid, tid, open_thread_error,
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

    let process = match processes.pid_to_process.get_mut(&pid) {
        Some(p) => p,
        None => return,
    };

    let tids: Vec<u32> = process.get_threads().keys().copied().collect();
    let mut all_threads: Vec<(u32, u64, usize, String)> = Vec::new();
    for &tid in &tids {
        let ts = prime_scheduler.get_thread_stats(pid, tid);
        let cycles = ts.cached_cycles;
        if cycles == 0 {
            continue;
        }
        let delta = cycles.saturating_sub(ts.last_cycles);
        let start_addr = ts.start_address;
        let start_module = resolve_address_to_module(pid, start_addr);
        all_threads.push((tid, delta, start_addr, start_module));
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

        let slot_count = rule.cpus.len();

        let mut selection: Vec<(u32, u64, bool)> = thread_infos.iter().map(|&(tid, delta, _, _)| (tid, delta, false)).collect();

        prime_scheduler.select_top_threads_with_hysteresis(pid, &mut selection, slot_count, |ts| &mut ts.active_streak, |ts| ts.ideal_processor.is_assigned);

        let is_prime_set: HashSet<u32> = selection.iter().filter(|(_, _, p)| *p).map(|(t, _, _)| *t).collect();

        let mut claimed: HashSet<u32> = HashSet::new();
        for &(tid, _, is_prime) in &selection {
            if is_prime {
                let ts = prime_scheduler.get_thread_stats(pid, tid);
                if ts.ideal_processor.is_assigned {
                    claimed.insert(ts.ideal_processor.current_number as u32);
                }
            }
        }

        let mut free_pool: Vec<u32> = rule.cpus.iter().copied().filter(|c| !claimed.contains(c)).collect();

        for &(tid, _, is_prime) in &selection {
            if !is_prime {
                continue;
            }
            let ts = prime_scheduler.get_thread_stats(pid, tid);
            if ts.ideal_processor.is_assigned {
                continue;
            }

            let handle = match ts.handle {
                Some(h) if !h.is_invalid() => h,
                _ => continue,
            };

            let mut got_current = false;
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
                Ok(prev) => {
                    ts.ideal_processor.previous_group = prev.Group;
                    ts.ideal_processor.previous_number = prev.Number;
                    ts.ideal_processor.current_group = prev.Group;
                    ts.ideal_processor.current_number = prev.Number;
                    got_current = true;
                }
            }

            let free_pos = if got_current && ts.ideal_processor.current_group == 0 {
                free_pool.iter().position(|&c| c == ts.ideal_processor.current_number as u32)
            } else {
                None
            };

            let (target_cpu, need_set) = if let Some(pos) = free_pos {
                (free_pool.remove(pos), false)
            } else {
                if free_pool.is_empty() {
                    break;
                }
                (free_pool.remove(0), true)
            };

            let set_ok = if need_set {
                match set_thread_ideal_processor_ex(handle, 0, target_cpu as u8) {
                    Ok(_) => true,
                    Err(_) => {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!(
                            "apply_ideal_processor: [SET_IDEAL][{}] {:>5}-{}-{}",
                            error_from_code(error_code),
                            pid,
                            tid,
                            config.name
                        ));
                        false
                    }
                }
            } else {
                true
            };

            if set_ok {
                ts.ideal_processor.current_group = 0;
                ts.ideal_processor.current_number = target_cpu as u8;
                ts.ideal_processor.is_assigned = true;
                let start_module = all_threads.iter().find(|(t, _, _, _)| *t == tid).map(|(_, _, _, m)| m.as_str()).unwrap_or("?");
                apply_config_result.add_change(format!("Thread {} -> ideal CPU {} (group 0) start={}", tid, target_cpu, start_module));
            }
        }

        for &(tid, _, _, ref start_module) in &thread_infos {
            let ts = prime_scheduler.get_thread_stats(pid, tid);
            if !ts.ideal_processor.is_assigned || is_prime_set.contains(&tid) {
                continue;
            }
            let prev_group = ts.ideal_processor.previous_group;
            let prev_number = ts.ideal_processor.previous_number;
            let cur_group = ts.ideal_processor.current_group;
            let cur_number = ts.ideal_processor.current_number;
            if (prev_group != cur_group || prev_number != cur_number)
                && let Some(handle) = ts.handle
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
                        ts.ideal_processor.current_group = prev_group;
                        ts.ideal_processor.current_number = prev_number;
                        apply_config_result.add_change(format!(
                            "Thread {} -> restored ideal CPU {} (group {}) start={}",
                            tid, prev_number, prev_group, start_module
                        ));
                    }
                }
            }
            ts.ideal_processor.is_assigned = false;
        }
    }
}
