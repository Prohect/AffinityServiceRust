mod cli;
mod config;
mod logging;
mod priority;
mod process;
mod scheduler;
mod winapi;

use chrono::Local;
use cli::{CliArgs, parse_args, print_help, print_help_all};
use config::{ProcessConfig, convert, cpu_indices_to_mask, format_cpu_indices, read_config, read_list};
use encoding_rs::Encoding;
use logging::{FAIL_SET, LOCALTIME_BUFFER, error_from_code, find_logger, log_message, log_process_find, log_pure_message, log_to_find, logger, use_console};
use process::ProcessSnapshot;
use scheduler::PrimeThreadScheduler;
use std::{
    collections::{HashMap, HashSet},
    env,
    io::Write,
    mem::size_of,
    process::Command,
    thread,
    time::Duration,
};
use winapi::{
    NtQueryInformationProcess, NtSetInformationProcess, NtSetTimerResolution, cpusetids_from_indices, enable_debug_privilege, enable_inc_base_priority_privilege,
    filter_indices_by_mask, get_cpu_set_information, get_thread_start_address, indices_from_cpusetids, is_affinity_unset, is_running_as_admin, request_uac_elevation,
    resolve_address_to_module, set_symbol_proxy,
};
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE},
    System::{
        Console::GetConsoleOutputCP,
        Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS},
        Threading::{
            GetPriorityClass, GetProcessAffinityMask, GetProcessDefaultCpuSets, GetProcessInformation, GetThreadPriority, OpenProcess, OpenThread,
            PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION, ProcessMemoryPriority, SetPriorityClass, SetProcessAffinityMask, SetProcessDefaultCpuSets,
            SetProcessInformation, SetThreadPriority, SetThreadSelectedCpuSets, THREAD_QUERY_INFORMATION, THREAD_SET_LIMITED_INFORMATION,
        },
        WindowsProgramming::QueryThreadCycleTime,
    },
};

#[derive(Debug, Default)]
struct ApplyConfigResult {
    changes: Vec<String>,
    errors: Vec<String>,
}

impl ApplyConfigResult {
    fn new() -> Self {
        Self::default()
    }

    /// pid & name will be logged in main loop
    fn add_change(&mut self, change: String) {
        self.changes.push(change);
    }
    /// pid & name will not be logged in main loop
    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    fn is_empty(&self) -> bool {
        self.changes.is_empty() && self.errors.is_empty()
    }
}

fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
    if let Some(priority_flag) = config.priority.as_win_const() {
        let current_priority = unsafe { GetPriorityClass(h_prc) };
        if current_priority != priority_flag.0 {
            if dry_run {
                apply_config_result.add_change(format!(
                    "Priority: {} -> {}",
                    priority::ProcessPriority::from_win_const(current_priority),
                    config.priority.as_str()
                ));
            } else {
                let set_result = unsafe { SetPriorityClass(h_prc, priority_flag) };
                if set_result.is_ok() {
                    apply_config_result.add_change(format!(
                        "Priority: {} -> {}",
                        priority::ProcessPriority::from_win_const(current_priority),
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

fn apply_affinity(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, current_mask: &mut usize, apply_config_result: &mut ApplyConfigResult) {
    let mut system_mask: usize = 0;
    let affinity_mask = cpu_indices_to_mask(&config.affinity_cpus);
    let has_affinity = !config.affinity_cpus.is_empty();
    let has_prime = !config.prime_threads_cpus.is_empty();
    if has_affinity || has_prime {
        let query_result = unsafe { GetProcessAffinityMask(h_prc, &mut *current_mask, &mut system_mask) };
        match query_result {
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
                        let set_result = unsafe { SetProcessAffinityMask(h_prc, affinity_mask) };
                        match set_result {
                            Err(_) => {
                                let error_code = unsafe { GetLastError().0 };
                                apply_config_result.add_error(format!("apply_config: [SET_AFFINITY][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                            }
                            Ok(_) => {
                                apply_config_result.add_change(format!("Affinity: {:#X} -> {:#X}", current_mask, affinity_mask));
                                *current_mask = affinity_mask;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
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

fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    mut processes: Option<&mut ProcessSnapshot>,
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
                thread_stats.last_system_thread_info = Some(thread.clone());
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
            if !alive_tids.contains_key(tid) {
                if let Some(handle) = stats.handle.take() {
                    unsafe {
                        let _ = windows::Win32::Foundation::CloseHandle(handle);
                    }
                }
            }
        }
    }
}

fn apply_prime_threads_select_candidates(process: &mut process::ProcessEntry, candidate_tids: &mut [u32], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32) {
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

fn apply_prime_threads_query_cycles(
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
                let open_result = unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION, false, tid) };
                match open_result {
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
                        let query_result = unsafe { QueryThreadCycleTime(handle, &mut cycles) };
                        match query_result {
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
                let query_result = unsafe { QueryThreadCycleTime(handle, &mut cycles) };
                match query_result {
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

fn apply_prime_threads_update_streaks(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) {
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

fn apply_prime_threads_promote(
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
        if let Some(handle) = thread_stats.handle {
            if !handle.is_invalid() && thread_stats.cpu_set_ids.is_empty() {
                let start_module = resolve_address_to_module(pid, thread_stats.start_address);
                // Find effective prime CPUs and thread priority for this thread based on prefix matching
                let mut matched = false;
                let mut effective_prime_cpus = &config.prime_threads_cpus;
                let mut effective_thread_priority = priority::ThreadPriority::None;
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
                        let current_prio = priority::ThreadPriority::from_win_const(current_prio);
                        thread_stats.original_priority = Some(current_prio);

                        let new_prio = if effective_thread_priority != priority::ThreadPriority::None {
                            effective_thread_priority
                        } else {
                            current_prio.boost_one()
                        };

                        if let Err(e) = unsafe { SetThreadPriority(handle, new_prio.to_thread_priority_struct()) } {
                            apply_config_result.add_error(format!(
                                "apply_config: [SET_THREAD_PRIORITY][{}] {:>5}-{}-{}",
                                error_from_code(e.code().0 as u32),
                                pid,
                                tid,
                                config.name
                            ));
                        } else {
                            let old_name = current_prio.as_str();
                            let new_name = new_prio.as_str();
                            let action = if effective_thread_priority != priority::ThreadPriority::None {
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

fn apply_prime_threads_demote(
    process: &mut process::ProcessEntry,
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
                    if let Err(e) = unsafe { SetThreadPriority(handle, thread_stats.original_priority.as_mut().unwrap().to_thread_priority_struct()) } {
                        apply_config_result.add_error(format!(
                            "apply_config: [RESTORE_THREAD_PRIORITY][{}] {:>5}-{}-{}",
                            error_from_code(e.code().0 as u32),
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

fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
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
                    priority::IOPriority::from_win_const(current_io_priority),
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
                        priority::IOPriority::from_win_const(current_io_priority),
                        config.io_priority.as_str()
                    ));
                }
            }
        }
    }
}

fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) {
    if let Some(memory_priority_flag) = config.memory_priority.as_win_const() {
        let mut current_mem_prio = priority::MemoryPriorityInformation(0);
        let query_result = unsafe {
            GetProcessInformation(
                h_prc,
                ProcessMemoryPriority,
                &mut current_mem_prio as *mut _ as *mut std::ffi::c_void,
                size_of::<priority::MemoryPriorityInformation>() as u32,
            )
        };
        match query_result {
            Err(_) => {
                let err = unsafe { GetLastError().0 };
                apply_config_result.add_error(format!(
                    "apply_config: [QUERY_MEMORY_PRIORITY][{}] {:>5}-{}",
                    logging::error_from_code(err),
                    pid,
                    config.name
                ));
            }
            Ok(_) => {
                if current_mem_prio.0 != memory_priority_flag.0 {
                    if dry_run {
                        apply_config_result.add_change(format!("Memory Priority: -> {}", config.io_priority.as_str()));
                    } else {
                        let mem_prio_info = priority::MemoryPriorityInformation(memory_priority_flag.0);
                        let set_result = unsafe {
                            SetProcessInformation(
                                h_prc,
                                ProcessMemoryPriority,
                                &mem_prio_info as *const _ as *const std::ffi::c_void,
                                size_of::<priority::MemoryPriorityInformation>() as u32,
                            )
                        };
                        match set_result {
                            Err(e) => {
                                apply_config_result.add_error(format!(
                                    "apply_config: [SET_MEMORY_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                                    e.code().0 as u32,
                                    pid,
                                    config.name,
                                    config.memory_priority.as_str()
                                ));
                            }
                            Ok(_) => {
                                apply_config_result.add_change(format!(
                                    "Memory Priority: {} -> {}",
                                    priority::MemoryPriority::from_win_const(current_mem_prio.0),
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

fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
) -> ApplyConfigResult {
    let mut apply_config_result = ApplyConfigResult::new();
    let access_flags = if dry_run {
        PROCESS_QUERY_INFORMATION
    } else {
        PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION
    };
    let open_result = unsafe { OpenProcess(access_flags, false, pid) };
    let h_prc = match open_result {
        Err(_) => {
            let error_code = unsafe { GetLastError().0 };
            if dry_run {
                apply_config_result.add_change(format!("[SKIP] OpenProcess failed"));
            } else {
                apply_config_result.add_error(format!("apply_config: [OPEN][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
            }
            return apply_config_result;
        }
        Ok(h_prc) => h_prc,
    };
    if h_prc.is_invalid() {
        if dry_run {
            apply_config_result.add_change(format!("[SKIP] Invalid handle"));
            return apply_config_result;
        }
        apply_config_result.add_error(format!("apply_config: [INVALID_HANDLE] {:>5}-{}", pid, config.name));
        return apply_config_result;
    }
    let mut current_mask: usize = 0;
    apply_priority(pid, config, dry_run, h_prc, &mut apply_config_result);
    apply_affinity(pid, config, dry_run, h_prc, &mut current_mask, &mut apply_config_result);
    apply_process_default_cpuset(pid, config, dry_run, h_prc, &mut apply_config_result);
    apply_prime_threads(pid, config, prime_core_scheduler, processes, dry_run, &mut current_mask, &mut apply_config_result);
    apply_io_priority(pid, config, dry_run, h_prc, &mut apply_config_result);
    apply_memory_priority(pid, config, dry_run, h_prc, &mut apply_config_result);
    let _ = unsafe { CloseHandle(h_prc) };
    apply_config_result
}

fn process_logs(configs: &HashMap<String, ProcessConfig>, blacklist: &Vec<String>, logs_path: Option<&str>, output_file: Option<&str>) {
    // Processes logs to discover new processes not yet in config or blacklist, and searches for their file paths.
    //
    // # Requirements
    // - Everything search tool installed with `es.exe` in PATH
    // - Log files in the logs directory (generated by `-find` mode)
    *use_console().lock().unwrap() = true;
    let logs_path = logs_path.unwrap_or("logs");
    let output_file = output_file.unwrap_or("new_processes_results.txt");

    // Get all unique processes from logs
    let mut all_processes = HashSet::new();
    if let Ok(entries) = std::fs::read_dir(logs_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().and_then(|n| n.to_str()).map_or(false, |s| s.ends_with(".find.log")) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    for line in content.lines() {
                        if let Some(idx) = line.find("find ") {
                            let rest = &line[idx + 5..];
                            let proc = if let Some(space_idx) = rest.find(' ') { &rest[..space_idx] } else { rest.trim() };
                            if proc.ends_with(".exe") {
                                all_processes.insert(proc.to_lowercase());
                            }
                        }
                    }
                }
            }
        }
    }
    let new_processes: Vec<String> = all_processes.into_iter().filter(|p| !configs.contains_key(p) && !blacklist.contains(p)).collect();

    // Search with es
    let mut output = String::new();
    let acp = unsafe { GetConsoleOutputCP() };
    let label = if acp == 936 { "gbk" } else { &format!("windows-{}", acp) };
    let encoding = Encoding::for_label_no_replacement(label.as_bytes()).unwrap_or(encoding_rs::UTF_8);
    for proc in new_processes {
        output.push_str(&format!("Process: {}\n", proc));
        // Escape dots for regex
        let escaped_proc = proc.replace(".", r"\.");
        let es_output = Command::new("es").args(&["-utf8-bom", "-r", &format!("^{}$", escaped_proc)]).output();
        match es_output {
            Ok(output_result) if output_result.status.success() => {
                let stdout = &output_result.stdout;
                // Strip UTF-8 BOM if present
                let ansi_bytes = if stdout.starts_with(&[0xEF, 0xBB, 0xBF]) { &stdout[3..] } else { stdout };
                // Decode from console output codepage to UTF-8
                let (result, _, _) = encoding.decode(ansi_bytes);
                if !result.trim().is_empty() {
                    output.push_str("Found:\n");
                    for line in result.lines() {
                        output.push_str(&format!("  {}\n", line));
                    }
                } else {
                    output.push_str("Not found, result empty\n");
                }
            }
            _ => {
                output.push_str("Not found, es failed\n");
            }
        }
        output.push_str("---\n");
    }

    if let Err(e) = std::fs::write(output_file, output) {
        log!("Failed to write output: {}", e);
    } else {
        log!("Results saved to {}", output_file);
    }
}

fn main() -> windows::core::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut cli = CliArgs::new();
    parse_args(&args, &mut cli)?;
    if cli.help_mode {
        print_help();
        return Ok(());
    }
    if cli.help_all_mode {
        print_help_all();
        return Ok(());
    }
    if cli.convert_mode {
        convert(cli.in_file_name, cli.out_file_name);
        return Ok(());
    }
    let config_result = read_config(&cli.config_file_name);
    config_result.print_report();
    if !config_result.errors.is_empty() {
        log!("Configuration file has errors, please fix them before running the service.");
        return Ok(());
    }
    if cli.validate_mode {
        return Ok(());
    }
    let mut configs = config_result.configs;
    let mut blacklist = if let Some(ref bf) = cli.blacklist_file_name {
        read_list(bf).unwrap_or_default()
    } else {
        Vec::new()
    };

    let mut last_config_mod_time = std::fs::metadata(&cli.config_file_name).and_then(|m| m.modified()).ok();
    let mut last_blacklist_mod_time = cli.blacklist_file_name.as_ref().and_then(|bf| std::fs::metadata(bf).and_then(|m| m.modified()).ok());
    let is_config_empty = configs.is_empty();
    let is_blacklist_empty = blacklist.is_empty();
    if cli.process_logs_mode {
        process_logs(&configs, &blacklist, cli.in_file_name.as_deref(), cli.out_file_name.as_deref());
        return Ok(());
    }
    if is_config_empty && is_blacklist_empty {
        if !cli.find_mode {
            if cli.skip_log_before_elevation {
                log!("not even a single config, existing");
            }
            return Ok(());
        }
    } else {
        log!("{} blacklist items load", blacklist.len());
    }
    // Set proxy for symbol downloads if provided
    if let Some(ref proxy) = cli.proxy {
        set_symbol_proxy(Some(proxy.clone()));
        log!("Using proxy for symbol downloads: {}", proxy);
    }
    if !cli.no_debug_priv {
        enable_debug_privilege();
    } else {
        log!("SeDebugPrivilege disabled by -noDebugPriv flag");
    }
    if !cli.no_inc_base_priority {
        enable_inc_base_priority_privilege();
    } else {
        log!("SeIncreaseBasePriorityPrivilege disabled by -noIncBasePriority flag");
    }
    if cli.time_resolution != 0 {
        unsafe {
            let mut current_resolution = 0u32;
            match NtSetTimerResolution(cli.time_resolution, true, &mut current_resolution as *mut _ as *mut std::ffi::c_void).0 {
                ntstatus if ntstatus < 0 => {
                    log!("Failed to set timer resolution: 0x{:08X}", ntstatus);
                }
                ntstatus if ntstatus >= 0 => {
                    log!("Succeed to set timer resolution: {:.4}ms", cli.time_resolution as f64 / 10000f64);
                    log!("elder timer resolution: {:.4}ms", current_resolution);
                }
                _ => {}
            };
        }
    }
    if !cli.skip_log_before_elevation {
        log!("Affinity Service started with time interval: {}", cli.interval_ms);
    }
    if !is_running_as_admin() {
        if cli.no_uac {
            log!("Not running as administrator. UAC elevation disabled by -noUAC flag.");
            log!("Warning: May not be able to manage all processes without admin privileges.");
        } else {
            log!("Not running as administrator. Requesting UAC elevation...");
            match request_uac_elevation() {
                Ok(_) => {
                    log!("Running with administrator privileges.");
                }
                Err(e) => {
                    log!("Failed to request elevation: {}, may not manage all processes", e);
                }
            }
        }
    }
    let mut prime_core_scheduler = PrimeThreadScheduler::new(config_result.constants);
    let mut current_loop = 0u32;
    let mut should_continue = true;

    while should_continue {
        if cli.log_loop {
            log!("Loop {} started", current_loop + 1);
        }
        match ProcessSnapshot::take() {
            Ok(mut processes) => {
                let mut total_changes = 0;
                let pids_and_names: Vec<(u32, String)> = processes.pid_to_process.values().map(|p| (p.pid(), p.get_name().to_string())).collect();
                prime_core_scheduler.reset_alive();
                for (pid, name) in pids_and_names {
                    if let Some(config) = configs.get(&name) {
                        let result = apply_config(pid, config, &mut prime_core_scheduler, Some(&mut processes), cli.dry_run);
                        if !result.is_empty() {
                            // Log errors to find log
                            for error in &result.errors {
                                log_to_find(error);
                            }
                            // Log changes to main log
                            if !result.changes.is_empty() {
                                let first = format!("{:>5}::{}::{}", pid, config.name, result.changes[0]);
                                log_message(&first);
                                let padding = " ".repeat(first.len() - result.changes[0].len() + 10);
                                for change in &result.changes[1..] {
                                    log_pure_message(&format!("{}{}", padding, change));
                                }
                            }

                            total_changes += result.changes.len();
                        }
                    }
                }
                prime_core_scheduler.close_dead_process_handles();
                if cli.dry_run {
                    log!(
                        "[DRY RUN] Checking {} running processes against {} config rules...",
                        processes.pid_to_process.len(),
                        configs.len()
                    );
                    log!("[DRY RUN] {} change(s) would be made. Run without -dryrun to apply.", total_changes);
                    should_continue = false;
                }

                drop(processes);
            }
            Err(err) => {
                log!("Failed to take process snapshot: {}", err);
            }
        };
        if cli.find_mode {
            unsafe {
                let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
                let mut pe32 = PROCESSENTRY32W::default();
                pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
                if Process32FirstW(snapshot, &mut pe32).is_ok() {
                    loop {
                        let process_name = String::from_utf16_lossy(&pe32.szExeFile[..pe32.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)]).to_lowercase();
                        if !FAIL_SET.lock().unwrap().contains(&process_name) && !configs.contains_key(&process_name) && !blacklist.contains(&process_name) {
                            if is_affinity_unset(pe32.th32ProcessID, process_name.as_str()) {
                                log_process_find(&process_name);
                            }
                        }
                        if !Process32NextW(snapshot, &mut pe32).is_ok() {
                            break;
                        }
                    }
                }
                let _ = CloseHandle(snapshot);
            }
        }
        let _ = find_logger().lock().unwrap().flush();
        let _ = logger().lock().unwrap().flush();
        current_loop += 1;
        if let Some(max_loops) = cli.loop_count {
            if current_loop >= max_loops {
                if cli.log_loop {
                    log!("Completed {} loops, exiting", max_loops);
                }
                should_continue = false;
            }
        }
        if should_continue {
            thread::sleep(Duration::from_millis(cli.interval_ms));
            *LOCALTIME_BUFFER.lock().unwrap() = Local::now();

            // Check for config reload
            if let Ok(metadata) = std::fs::metadata(&cli.config_file_name) {
                if let Ok(mod_time) = metadata.modified() {
                    if Some(mod_time) != last_config_mod_time {
                        last_config_mod_time = Some(mod_time);
                        log!("Configuration file '{}' changed, reloading...", cli.config_file_name);
                        let new_config_result = read_config(&cli.config_file_name);
                        if new_config_result.errors.is_empty() {
                            new_config_result.print_report();
                            configs = new_config_result.configs;
                            prime_core_scheduler.constants = new_config_result.constants;
                            log!("Configuration reload complete: {} rules loaded.", configs.len());
                        } else {
                            log!("Configuration file '{}' has errors, keeping previous configuration.", cli.config_file_name);
                            for error in &new_config_result.errors {
                                log!("  - {}", error);
                            }
                        }
                    }
                }
            }

            // Check for blacklist reload
            if let Some(ref bf) = cli.blacklist_file_name {
                match std::fs::metadata(bf) {
                    Ok(metadata) => {
                        if let Ok(mod_time) = metadata.modified() {
                            if Some(mod_time) != last_blacklist_mod_time {
                                last_blacklist_mod_time = Some(mod_time);
                                log!("Blacklist file '{}' changed, reloading...", bf);
                                blacklist = read_list(bf).unwrap_or_default();
                                log!("Blacklist reload complete: {} items loaded.", blacklist.len());
                            }
                        }
                    }
                    Err(_) => {
                        if last_blacklist_mod_time.is_some() {
                            last_blacklist_mod_time = None;
                            log!("Blacklist file '{}' no longer accessible, clearing blacklist.", bf);
                            blacklist.clear();
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
