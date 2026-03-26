use crate::config::{ProcessConfig, cpu_indices_to_mask, format_cpu_indices};
use crate::logging::error_from_code;
use crate::process::ProcessSnapshot;
use crate::scheduler::PrimeThreadScheduler;
use crate::winapi::{
    NtQueryInformationProcess, NtSetInformationProcess, cpusetids_from_indices, filter_indices_by_mask, get_cpu_set_information, get_thread_ideal_processor_ex,
    get_thread_start_address, indices_from_cpusetids, resolve_address_to_module, set_thread_ideal_processor_ex,
};
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
    processes: &mut Option<&mut ProcessSnapshot>,
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

pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut Option<&mut ProcessSnapshot>,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) {
    // For dry run, just report prime CPUs would be set
    if !config.prime_threads_cpus.is_empty() || config.track_top_x_threads != 0 {
        if dry_run {
            if !config.prime_threads_cpus.is_empty() {
                apply_config_result.add_change(format!("Prime CPUs: -> [{}]", format_cpu_indices(&config.prime_threads_cpus)));
            }
            return;
        }
        prime_core_scheduler.set_alive(pid);
        if config.track_top_x_threads != 0 {
            prime_core_scheduler.set_tracking_info(pid, config.track_top_x_threads, config.name.clone());
            // Ensure module cache is populated while process is alive
            crate::winapi::resolve_address_to_module(pid, 1);
        }

        let process = processes.as_mut().unwrap().pid_to_process.get_mut(&pid).unwrap();
        let thread_count = process.thread_count() as usize;

        let mut all_tids: Vec<u32> = Vec::new();
        let mut all_delta_cycles: Vec<(u32, u64, bool)> = Vec::new();

        if config.track_top_x_threads != 0 {
            all_tids = process.get_threads().keys().copied().collect();
            all_delta_cycles = vec![(0u32, 0u64, false); all_tids.len()];
            apply_prime_threads_query_cycles(&all_tids, &mut all_delta_cycles, prime_core_scheduler, pid, &config.name, apply_config_result);
            for (tid, thread) in process.get_threads().iter() {
                let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);
                thread_stats.last_system_thread_info = Some(*thread);
            }
        }

        if !config.prime_threads_cpus.is_empty() && config.track_top_x_threads >= 0 {
            let candidate_count = get_cpu_set_information().lock().unwrap().len().min(thread_count);
            let mut candidate_tids: Vec<u32> = vec![0u32; candidate_count];
            let mut tid_with_delta_cycles: Vec<(u32, u64, bool)> = vec![(0u32, 0u64, false); candidate_count];

            apply_prime_threads_select_candidates(process, &mut candidate_tids, prime_core_scheduler, pid);

            if config.track_top_x_threads != 0 {
                // We already queried cycles for all threads, just copy the deltas
                for i in 0..candidate_count {
                    let tid = candidate_tids[i];
                    if let Some(idx) = all_tids.iter().position(|&t| t == tid) {
                        tid_with_delta_cycles[i] = all_delta_cycles[idx];
                    }
                }
            } else {
                apply_prime_threads_query_cycles(
                    &candidate_tids,
                    &mut tid_with_delta_cycles,
                    prime_core_scheduler,
                    pid,
                    &config.name,
                    apply_config_result,
                );
            }

            apply_prime_threads_update_streaks(&mut tid_with_delta_cycles, prime_core_scheduler, pid, candidate_count);
            apply_prime_threads_promote(&tid_with_delta_cycles, prime_core_scheduler, pid, config, current_mask, apply_config_result);
            apply_prime_threads_demote(process, &tid_with_delta_cycles, prime_core_scheduler, pid, config, apply_config_result);
        }

        // Cleanup handles for dead threads
        let process_stats = prime_core_scheduler.pid_to_process_stats.get_mut(&pid).unwrap();
        let alive_tids = process.get_threads();
        for (tid, stats) in process_stats.tid_to_thread_stats.iter_mut() {
            if !alive_tids.contains_key(tid)
                && let Some(handle) = stats.handle.take()
            {
                unsafe {
                    let _ = CloseHandle(handle);
                }
            }
        }
    }
}

pub fn apply_prime_threads_select_candidates(
    process: &mut crate::process::ProcessEntry,
    candidate_tids: &mut [u32],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
) {
    let thread_count = process.thread_count() as usize;
    let candidate_count = candidate_tids.len();
    let mut tid_with_delta_time: Vec<(u32, i64)> = Vec::with_capacity(thread_count);
    process.get_threads().iter().for_each(|(tid, thread)| {
        let total_time = unsafe { thread.KernelTime.QuadPart() + thread.UserTime.QuadPart() };
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);
        tid_with_delta_time.push((*tid, total_time - thread_stats.last_total_time));
        thread_stats.last_total_time = total_time;
    });
    tid_with_delta_time.sort_unstable_by_key(|&(_, delta)| delta);
    let precandidate_len = tid_with_delta_time.len();
    for i in 0..candidate_count {
        candidate_tids[i] = tid_with_delta_time[precandidate_len - i - 1].0;
    }
}

pub fn apply_prime_threads_query_cycles(
    candidate_tids: &[u32],
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    process_name: &str,
    apply_config_result: &mut ApplyConfigResult,
) {
    for i in 0..candidate_tids.len() {
        let tid = candidate_tids[i];
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        match thread_stats.handle {
            None => {
                match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION | THREAD_SET_INFORMATION, false, tid) } {
                    Err(_) => {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!(
                            "apply_config: [OPEN_THREAD][{}] {:>5}-{}-{}",
                            error_from_code(error_code),
                            pid,
                            tid,
                            process_name
                        ));
                    }
                    Ok(handle) => {
                        let mut cycles: u64 = 0;
                        match unsafe { QueryThreadCycleTime(handle, &mut cycles) } {
                            Err(_) => {
                                let error_code = unsafe { GetLastError().0 };
                                apply_config_result.add_error(format!(
                                    "apply_config: [QUERY_THREAD_CYCLE][{}] {:>5}-{}-{}",
                                    error_from_code(error_code),
                                    pid,
                                    tid,
                                    process_name
                                ));
                            }
                            Ok(_) => {
                                tid_with_delta_cycles[i] = (tid, cycles, false);
                                thread_stats.start_address = get_thread_start_address(handle);
                            }
                        };
                        thread_stats.handle = Some(handle);
                    }
                };
            }
            Some(handle) => {
                let mut cycles: u64 = 0;
                match unsafe { QueryThreadCycleTime(handle, &mut cycles) } {
                    Err(_) => {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!(
                            "apply_config: [QUERY_THREAD_CYCLE][{}] {:>5}-{}-{}",
                            error_from_code(error_code),
                            pid,
                            tid,
                            process_name
                        ));
                    }
                    Ok(_) => {
                        tid_with_delta_cycles[i] = (tid, cycles - thread_stats.last_cycles, false);
                        thread_stats.last_cycles = cycles;
                    }
                };
            }
        }
    }
}

pub fn apply_prime_threads_update_streaks(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) {
    tid_with_delta_cycles.sort_unstable_by_key(|&(_, delta_cycles, _)| std::cmp::Reverse(delta_cycles));
    let max_cycles = tid_with_delta_cycles.first().map(|&(_, c, _)| c).unwrap_or(0u64);
    let entry_min_cycles = (max_cycles as f64 * prime_core_scheduler.constants.entry_threshold) as u64;
    let keep_min_cycles = (max_cycles as f64 * prime_core_scheduler.constants.keep_threshold) as u64;
    for &(tid, delta_cycles, _) in &*tid_with_delta_cycles {
        if tid == 0 {
            continue;
        }
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
        if delta_cycles >= entry_min_cycles {
            thread_stats.active_streak = thread_stats.active_streak.saturating_add(1).min(254);
        } else {
            thread_stats.active_streak = 0;
        }
    }
    let mut new_prime_count: usize = 0;
    // First pass: mark protected prime threads
    for (tid, delta_cycles, is_prime) in tid_with_delta_cycles.iter_mut() {
        if *tid == 0 || new_prime_count >= prime_count {
            continue;
        }
        if !prime_core_scheduler.get_thread_stats(pid, *tid).cpu_set_ids.is_empty() && *delta_cycles >= keep_min_cycles {
            *is_prime = true;
            new_prime_count += 1;
        }
    }
    // Second pass: mark new candidates
    for (tid, delta_cycles, is_prime) in tid_with_delta_cycles.iter_mut() {
        if new_prime_count >= prime_count {
            break;
        }
        if *tid == 0 || *is_prime {
            continue;
        }
        if *delta_cycles >= entry_min_cycles && prime_core_scheduler.get_thread_stats(pid, *tid).active_streak >= prime_core_scheduler.constants.min_active_streak {
            *is_prime = true;
            new_prime_count += 1;
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
            // Find effective prime CPUs and thread priority for this thread based on prefix matching
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
            // If prefixes are defined but none match, skip promotion
            if !matched && !config.prime_threads_prefixes.is_empty() {
                continue;
            }
            // Filter by current process affinity mask
            let filtered_cpus = if *current_mask != 0 {
                filter_indices_by_mask(effective_prime_cpus, *current_mask)
            } else {
                effective_prime_cpus.clone()
            };
            let cpu_setids = cpusetids_from_indices(&filtered_cpus);
            if !cpu_setids.is_empty() {
                // Set the thread selected CPU sets
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
                // Set thread priority (use explicit priority from config or boost by one tier)
                let current_prio = unsafe { GetThreadPriority(handle) };
                if current_prio != 0x7FFFFFFF_i32 {
                    let current_prio = crate::priority::ThreadPriority::from_win_const(current_prio);
                    thread_stats.original_priority = Some(current_prio);

                    let new_prio = if effective_thread_priority != crate::priority::ThreadPriority::None {
                        effective_thread_priority
                    } else {
                        current_prio.boost_one()
                    };

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

pub fn apply_prime_threads_demote(
    process: &mut crate::process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) {
    process.get_threads().iter().for_each(|(tid, _)| {
        let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);
        if !tid_with_delta_cycles.iter().any(|&(t, _, p)| t == *tid && p) && !thread_stats.cpu_set_ids.is_empty() {
            if let Some(handle) = thread_stats.handle {
                if !handle.is_invalid() {
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
                        let start_module = resolve_address_to_module(pid, thread_stats.start_address);
                        apply_config_result.add_change(format!("Thread {} -> (demoted, start={})", tid, start_module));
                    }
                }

                // Restore priority
                if thread_stats.original_priority.is_some() {
                    if unsafe { SetThreadPriority(handle, thread_stats.original_priority.as_mut().unwrap().to_thread_priority_struct()) }.is_err() {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!(
                            "apply_config: [RESTORE_THREAD_PRIORITY][{}] {:>5}-{}-{}",
                            error_from_code(error_code),
                            pid,
                            tid,
                            config.name
                        ));
                    }
                    thread_stats.original_priority = None;
                }
            }
            thread_stats.cpu_set_ids = vec![];
        };
    });
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

/// Resets ideal processor assignments for all threads based on CPU time/cycle sorting.
///
/// This function is called when process affinity changes to avoid Windows kernel's
/// behavior of clamping ideal processor values toward the new range, which can cause
/// too many threads to have the same ideal processor.
///
/// Strategy:
/// 1. Gather all threads with their total CPU time and cycle count
/// 2. Sort threads by total CPU time (descending) as primary key
/// 3. For threads with similar CPU time, use cycle count as secondary key
/// 4. Assign ideal processors round-robin across the available CPUs in config
///
/// # Arguments
/// * `pid` - Process ID
/// * `config` - Process configuration with target CPUs
/// * `dry_run` - If true, only log what would be done
/// * `h_prc` - Process handle
/// * `apply_config_result` - Result collector for changes and errors
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    _h_prc: HANDLE,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut Option<&mut ProcessSnapshot>,
) {
    if config.affinity_cpus.is_empty() {
        return;
    }

    if dry_run {
        apply_config_result.add_change(format!("Reset Ideal Processors: {} threads based on CPU time", config.affinity_cpus.len()));
        return;
    }

    // Check if processes contains the target process
    let processes = match processes {
        Some(p) => p,
        None => return,
    };

    let process = match processes.pid_to_process.get_mut(&pid) {
        Some(p) => p,
        None => return,
    };

    // Get all threads with their total CPU time from the snapshot
    let mut thread_times: Vec<(u32, i64, HANDLE)> = Vec::new();

    // Get threads from the process snapshot
    for (tid, thread_info) in process.get_threads() {
        // Get total CPU time from SYSTEM_THREAD_INFORMATION (already available in snapshot)
        let total_time = unsafe { *thread_info.KernelTime.QuadPart() + *thread_info.UserTime.QuadPart() };

        // Open thread handle
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

    // Sort by total CPU time descending, then by cycle time descending
    thread_times.sort_by(|a, b| b.1.cmp(&a.1));

    // Assign ideal processors round-robin across target CPUs
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

    // Close all thread handles
    for (_tid, _cpu_time, thread_handle) in thread_times {
        unsafe {
            let _ = CloseHandle(thread_handle);
        }
    }
}

/// Applies ideal processor assignments to threads based on configuration rules.
///
/// For each ideal processor rule:
/// 1. Find threads whose start module matches the rule's prefixes
/// 2. Sort matching threads by total CPU time (descending)
/// 3. Assign ideal processors from the rule's CPU list to top N threads
/// 4. Store previous ideal processor for restoration when thread falls out of top N
pub fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut Option<&mut ProcessSnapshot>,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) {
    if config.ideal_processor_rules.is_empty() {
        return;
    }

    // For dry run, just report what would be done
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

    let processes = match processes {
        Some(p) => p,
        None => return,
    };

    let process = match processes.pid_to_process.get_mut(&pid) {
        Some(p) => p,
        None => return,
    };

    // Get all threads with their total CPU time
    let mut thread_times: Vec<(u32, i64)> = Vec::new();
    for (tid, thread_info) in process.get_threads() {
        let total_time = unsafe { *thread_info.KernelTime.QuadPart() + *thread_info.UserTime.QuadPart() };
        thread_times.push((*tid, total_time));
    }

    // Process each ideal processor rule
    for rule in &config.ideal_processor_rules {
        if rule.cpus.is_empty() {
            continue;
        }

        // First pass: collect thread info without holding mutable borrows
        let mut thread_infos: Vec<(u32, i64, usize, String)> = Vec::new();
        for (tid, total_time) in &thread_times {
            // Get start address from scheduler or resolve it
            let (start_addr, start_module) = {
                let thread_stats = prime_scheduler.get_thread_stats(pid, *tid);

                if thread_stats.start_address == 0 {
                    // Need to open thread to get start address
                    if thread_stats.handle.is_none() {
                        match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION | THREAD_SET_INFORMATION, false, *tid) } {
                            Ok(handle) if !handle.is_invalid() => {
                                let addr = get_thread_start_address(handle);
                                // Store handle for later use
                                prime_scheduler.get_thread_stats(pid, *tid).handle = Some(handle);
                                let module = resolve_address_to_module(pid, addr);
                                (addr, module)
                            }
                            _ => (0, String::new()),
                        }
                    } else {
                        let addr = thread_stats.start_address;
                        let module = resolve_address_to_module(pid, addr);
                        (addr, module)
                    }
                } else {
                    let addr = thread_stats.start_address;
                    let module = resolve_address_to_module(pid, addr);
                    (addr, module)
                }
            };

            // Check if module matches any prefix
            let matches = if rule.prefixes.is_empty() {
                true // No prefixes = match all
            } else {
                let start_module_lower = start_module.to_lowercase();
                rule.prefixes.iter().any(|prefix| start_module_lower.starts_with(prefix))
            };

            if matches {
                thread_infos.push((*tid, *total_time, start_addr, start_module));
            }
        }

        // Sort by total CPU time descending
        thread_infos.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top N threads where N = number of CPUs in the rule
        let top_n = thread_infos.len().min(rule.cpus.len());
        let top_threads: Vec<_> = thread_infos.iter().take(top_n).cloned().collect();

        // Apply ideal processors to top N threads
        for (i, (tid, _, _start_addr, _)) in top_threads.iter().enumerate() {
            let target_cpu = rule.cpus[i];
            let thread_stats = prime_scheduler.get_thread_stats(pid, *tid);

            // Open thread if we don't have a handle
            if thread_stats.handle.is_none() {
                match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION | THREAD_SET_INFORMATION, false, *tid) } {
                    Err(_) => {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!(
                            "apply_ideal_processor: [OPEN_THREAD][{}] {:>5}-{}-{}",
                            error_from_code(error_code),
                            pid,
                            tid,
                            config.name
                        ));
                        continue;
                    }
                    Ok(handle) if !handle.is_invalid() => {
                        thread_stats.handle = Some(handle);
                    }
                    _ => continue,
                }
            }

            if let Some(handle) = thread_stats.handle {
                if handle.is_invalid() {
                    continue;
                }

                // Get current ideal processor if not already stored
                if !thread_stats.ideal_processor.is_assigned {
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
                            thread_stats.ideal_processor.previous_group = prev.Group;
                            thread_stats.ideal_processor.current_group = prev.Group;
                            thread_stats.ideal_processor.previous_number = prev.Number;
                            thread_stats.ideal_processor.current_number = prev.Number;
                        }
                    }
                }

                // Set new ideal processor (always group 0 for now, extend if >64 cores needed)
                let target_group: u16 = 0;
                let target_number: u8 = target_cpu as u8;

                // Only set if different from current
                if !thread_stats.ideal_processor.is_assigned || thread_stats.ideal_processor.current_number != target_number {
                    match set_thread_ideal_processor_ex(handle, target_group, target_number) {
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
                            thread_stats.ideal_processor.current_group = target_group;
                            thread_stats.ideal_processor.current_number = target_number;
                            thread_stats.ideal_processor.is_assigned = true;

                            apply_config_result.add_change(format!("Thread {} -> ideal CPU {} (group {})", tid, target_number, target_group));
                        }
                    }
                }
            }
        }

        // Restore previous ideal processor for threads that fell out of top N
        for (tid, _, _, _) in thread_infos.iter().skip(top_n) {
            let thread_stats = prime_scheduler.get_thread_stats(pid, *tid);

            if thread_stats.ideal_processor.is_assigned {
                if let Some(handle) = thread_stats.handle
                    && !handle.is_invalid()
                {
                    let prev_group = thread_stats.ideal_processor.previous_group;
                    let prev_number = thread_stats.ideal_processor.previous_number;

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
                            apply_config_result.add_change(format!("Thread {} -> restored ideal CPU {} (group {})", tid, prev_number, prev_group));
                            thread_stats.ideal_processor.current_group = prev_group;
                            thread_stats.ideal_processor.current_number = prev_number;
                        }
                    }
                }
                thread_stats.ideal_processor.is_assigned = false;
            }
        }
    }
}
