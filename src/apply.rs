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

/// Prefetches `QueryThreadCycleTime` results for every live thread of a process into
/// `ThreadStats::cached_cycles`, opening thread handles and capturing start addresses
/// on first visit.  Called once per loop iteration before both `apply_prime_threads`
/// and `apply_ideal_processors` so that neither consumer needs to call
/// `OpenThread` / `QueryThreadCycleTime` / `get_thread_start_address` itself.
///
/// `cached_cycles == 0` is the sentinel for "no data this iteration"; threads whose
/// prefetch fails are silently skipped by both consumers.
pub fn prefetch_all_thread_cycles(
    pid: u32,
    process_name: &str,
    processes: &mut Option<&mut ProcessSnapshot>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) {
    // Collect (tid, total_cpu_time) from the snapshot — zero extra syscalls, the data is
    // already in the NtQuerySystemInformation buffer held by ProcessSnapshot.
    let tid_times: Vec<(u32, i64)> = match processes {
        Some(ps) => match ps.pid_to_process.get_mut(&pid) {
            Some(p) => p
                .get_threads()
                .iter()
                .map(|(tid, thread)| {
                    let total = unsafe { thread.KernelTime.QuadPart() + thread.UserTime.QuadPart() };
                    (*tid, total)
                })
                .collect(),
            None => return,
        },
        None => return,
    };

    if tid_times.is_empty() {
        return;
    }

    // Pre-filter: rank threads by CPU-time delta (cycles are linear with CPU time, so the
    // top-N by time also rank highest by cycles).  Cap at the logical-CPU count — no
    // assignment algorithm ever needs more slots than there are logical CPUs.
    // NOTE: last_total_time is intentionally NOT updated here; apply_prime_threads_select_candidates
    // remains responsible for that so its own delta computation stays correct.
    let cpu_count = get_cpu_set_information().lock().unwrap().len();
    let n = cpu_count.min(tid_times.len());
    let mut tid_with_delta: Vec<(u32, i64)> = Vec::with_capacity(tid_times.len());
    for &(tid, total) in &tid_times {
        let last = prime_scheduler.get_thread_stats(pid, tid).last_total_time;
        tid_with_delta.push((tid, total - last));
    }
    tid_with_delta.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    tid_with_delta.truncate(n);

    for &(tid, _) in &tid_with_delta {
        // Step 1: Open handle if not yet cached.
        {
            let ts = prime_scheduler.get_thread_stats(pid, tid);
            if ts.handle.is_none() {
                match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION | THREAD_SET_INFORMATION, false, tid) } {
                    Err(_) => {
                        let error_code = unsafe { GetLastError().0 };
                        apply_config_result.add_error(format!(
                            "prefetch_thread_cycles: [OPEN_THREAD][{}] {:>5}-{}-{}",
                            crate::logging::error_from_code(error_code),
                            pid,
                            tid,
                            process_name
                        ));
                        continue;
                    }
                    Ok(handle) => {
                        ts.handle = Some(handle);
                    }
                }
            }
        }

        // Step 2: Copy handle value; skip if invalid.
        let handle = {
            let ts = prime_scheduler.get_thread_stats(pid, tid);
            match ts.handle {
                Some(h) if !h.is_invalid() => h,
                _ => continue,
            }
        };

        // Step 3: Populate start address if not yet cached.
        {
            let ts = prime_scheduler.get_thread_stats(pid, tid);
            if ts.start_address == 0 {
                ts.start_address = get_thread_start_address(handle);
            }
        }

        // Step 4: Query cycles into cached_cycles; on error leave cached_cycles unchanged.
        let mut cycles: u64 = 0;
        match unsafe { QueryThreadCycleTime(handle, &mut cycles) } {
            Ok(_) => {
                prime_scheduler.get_thread_stats(pid, tid).cached_cycles = cycles;
            }
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                apply_config_result.add_error(format!(
                    "prefetch_thread_cycles: [QUERY_CYCLES][{}] {:>5}-{}-{}",
                    crate::logging::error_from_code(error_code),
                    pid,
                    tid,
                    process_name
                ));
                // Do not update cached_cycles — leave at previous value or 0 (no data sentinel).
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
    // Cycles are pre-fetched by prefetch_all_thread_cycles; just read cached_cycles.
    let _ = process_name;
    let _ = apply_config_result;
    for i in 0..candidate_tids.len() {
        let tid = candidate_tids[i];
        let ts = prime_core_scheduler.get_thread_stats(pid, tid);
        let cached_cycles = ts.cached_cycles;
        if cached_cycles == 0 {
            continue;
        }
        let delta = cached_cycles.saturating_sub(ts.last_cycles);
        ts.last_cycles = cached_cycles;
        tid_with_delta_cycles[i] = (tid, delta, false);
    }
}

/// Delegates to [`PrimeThreadScheduler::select_top_threads_with_hysteresis`] using the
/// prime-thread fields (`active_streak`, `cpu_set_ids`).
pub fn apply_prime_threads_update_streaks(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) {
    prime_core_scheduler.select_top_threads_with_hysteresis(pid, tid_with_delta_cycles, prime_count, |ts| &mut ts.active_streak, |ts| !ts.cpu_set_ids.is_empty());
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
/// Uses the same hysteresis-based filter algorithm as prime-thread scheduling
/// (`MIN_ACTIVE_STREAK`, `KEEP_THRESHOLD`, `ENTRY_THRESHOLD`) via the shared
/// [`PrimeThreadScheduler::select_top_threads_with_hysteresis`] method.
///
/// # Per-iteration flow
///
/// **Phase 1** (once, before the rule loop):
/// - For every thread in the process snapshot: open/cache the handle, populate
///   `start_address`, call `QueryThreadCycleTime` and compute
///   `delta = current − ThreadStats::last_ideal_cycles` (separate from the
///   `last_cycles` field used by prime-thread scheduling).
/// - Builds `all_threads: Vec<(tid, delta_cycles, start_addr, start_module)>`.
///
/// **Phase 2** (per rule):
/// 1. Filter `all_threads` by the rule's module prefixes → `thread_infos`.
/// 2. Call `select_top_threads_with_hysteresis` with `ideal_active_streak` /
///    `ideal_processor.is_assigned`:
///    - Pass 1 – keep already-assigned threads above `keep_threshold` (no syscall).
///    - Pass 2 – promote threads above `entry_threshold` with streak satisfied.
/// 3. **Promote**: for each newly-selected thread, capture its current ideal CPU
///    (for later restoration), then call `SetThreadIdealProcessorEx` with the
///    next free CPU from the rule's list.  Logs `start=module+offset`.
/// 4. **Demote**: scan `all_threads` for threads with `is_assigned` that are no
///    longer selected; restore their original ideal CPU via `SetThreadIdealProcessorEx`.
///
/// Keep-threads (pass 1 survivors) require **zero write syscalls** this iteration.
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

    // Phase 1: Pre-collect (tid, delta_cycles, start_addr, start_module) for every thread once,
    // before the rule loop.  Thread handles, start addresses, and raw cycle counts are now
    // provided by `prefetch_all_thread_cycles` (called before both this function and
    // `apply_prime_threads`).  delta_cycles is derived from ThreadStats::last_ideal_cycles so
    // the sort reflects actual CPU usage since the last query rather than cumulative totals.
    let tids: Vec<u32> = process.get_threads().keys().copied().collect();
    let mut all_threads: Vec<(u32, u64, usize, String)> = Vec::new();
    for &tid in &tids {
        let ts = prime_scheduler.get_thread_stats(pid, tid);
        let cycles = ts.cached_cycles;
        if cycles == 0 {
            continue;
        }
        let delta = cycles.saturating_sub(ts.last_ideal_cycles);
        ts.last_ideal_cycles = cycles;
        let start_addr = ts.start_address;
        let start_module = resolve_address_to_module(pid, start_addr);
        all_threads.push((tid, delta, start_addr, start_module));
    }

    // Phase 2: Per-rule hysteresis-based selection + assign/restore (least write syscalls).
    for rule in &config.ideal_processor_rules {
        if rule.cpus.is_empty() {
            continue;
        }

        // Filter threads matching this rule's prefixes.
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

        // Build parallel (tid, delta, is_prime=false) vec for the shared hysteresis algorithm.
        // select_top_threads_with_hysteresis will sort this by delta descending in-place.
        let mut selection: Vec<(u32, u64, bool)> = thread_infos.iter().map(|&(tid, delta, _, _)| (tid, delta, false)).collect();

        // Apply hysteresis: updates ideal_active_streak, marks up to slot_count as is_prime.
        //   Pass 1 – keep already-assigned threads above keep_threshold  (no syscall).
        //   Pass 2 – promote new threads above entry_threshold with MIN_ACTIVE_STREAK met.
        prime_scheduler.select_top_threads_with_hysteresis(pid, &mut selection, slot_count, |ts| &mut ts.active_streak, |ts| ts.ideal_processor.is_assigned);

        // Quick-lookup set of threads that won a slot this iteration.
        let is_prime_set: HashSet<u32> = selection.iter().filter(|(_, _, p)| *p).map(|(t, _, _)| *t).collect();

        // CPUs already held by keep-threads (is_prime AND already assigned) — no syscall needed.
        let mut claimed: HashSet<u32> = HashSet::new();
        for &(tid, _, is_prime) in &selection {
            if is_prime {
                let ts = prime_scheduler.get_thread_stats(pid, tid);
                if ts.ideal_processor.is_assigned {
                    claimed.insert(ts.ideal_processor.current_number as u32);
                }
            }
        }

        // Free pool: rule CPUs not held by keep-threads, handed out in rule order.
        // Stored as a Vec so we can search by value and remove a specific element when
        // the lazy-set path claims a CPU that is not at the front of the pool.
        let mut free_pool: Vec<u32> = rule.cpus.iter().copied().filter(|c| !claimed.contains(c)).collect();

        // Promote: assign a free CPU to each newly-selected thread.
        // selection is already sorted delta-desc by select_top_threads_with_hysteresis,
        // so the hottest thread always gets the first (highest-priority) CPU in the rule.
        for &(tid, _, is_prime) in &selection {
            if !is_prime {
                continue;
            }
            let ts = prime_scheduler.get_thread_stats(pid, tid);
            if ts.ideal_processor.is_assigned {
                continue; // keep-thread: already on its CPU, no syscall.
            }

            let handle = match ts.handle {
                Some(h) if !h.is_invalid() => h,
                _ => continue,
            };

            // Capture the current ideal CPU for restoration on demotion and lazy-set check.
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

            // If the thread already sits on one of the free-pool CPUs, claim that slot
            // in-place and skip the write syscall.  Otherwise pop the front of the pool
            // (rule-order priority) and call SetThreadIdealProcessorEx.
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

        // Demote: scan all_threads for threads that are assigned but no longer selected.
        // Using all_threads (not just thread_infos) catches edge cases where a thread
        // was assigned in a prior iteration but no longer matches the prefix filter.
        for &(tid, _, _, ref start_module) in &all_threads {
            let ts = prime_scheduler.get_thread_stats(pid, tid);
            if !ts.ideal_processor.is_assigned || is_prime_set.contains(&tid) {
                continue;
            }
            if let Some(handle) = ts.handle
                && !handle.is_invalid()
            {
                let prev_group = ts.ideal_processor.previous_group;
                let prev_number = ts.ideal_processor.previous_number;
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
