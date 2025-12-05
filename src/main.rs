//! # AffinityServiceRust
//!
//! A Windows service for managing process priority, CPU affinity, CPU sets, IO priority,
//! and memory priority based on configuration rules.
//!
//! ## Features
//! - Process priority management (Idle to Realtime)
//! - CPU affinity masks (legacy) and CPU sets (Windows 10+)
//! - **Prime Thread Scheduler**: dynamically pins most active threads to preferred cores
//! - IO and Memory priority control via undocumented NT APIs
//! - Find mode: discovers processes without custom affinity settings and not in configs
//! - **Multi-CPU group support**: Works with systems having >64 logical processors
//!
//! ## Prime Thread Scheduler
//! For processes with `prime_cpus` configured, the scheduler:
//! 1. Tracks per-thread CPU time and cycle counts across polling intervals
//! 2. Promotes threads exceeding `entry_threshold` (default 42% of max) to prime cores
//! 3. Demotes threads falling below `keep_threshold` (default 69% of max)
//! 4. Uses hysteresis (2+ consecutive active intervals) to prevent thrashing
//!
//! This is useful for hybrid CPUs (Intel 12th+ gen) where you want game threads
//! on P-cores while background threads use E-cores.
//!
//! ## Configuration Format
//! ```text
//! @KEEP_THRESHOLD=0.69           # Scheduler constants
//! *pcore = 0-7;64-71             # CPU aliases (supports >64 cores)
//! game.exe, high, *pcore, 0, *pcore, normal, normal
//! ```

mod cli;
mod config;
mod logging;
mod priority;
mod process;
mod scheduler;
mod winapi;

use chrono::Local;
use cli::{parse_args, print_help, print_help_all};
use config::{ProcessConfig, convert, cpu_indices_to_mask, format_cpu_indices, read_config, read_list};
use logging::{FAIL_SET, LOCALTIME_BUFFER, error_from_code, find_logger, log_process_find, log_to_find, logger};
use priority::MemoryPriorityInformation;
use process::ProcessSnapshot;
use scheduler::PrimeThreadScheduler;
use std::{env, io::Write, mem::size_of, thread, time::Duration};
use winapi::{
    NtQueryInformationProcess, NtSetInformationProcess, NtSetTimerResolution, cpusetids_from_indices, enable_debug_privilege, enable_inc_base_priority_privilege,
    filter_indices_by_mask, get_cpu_set_information, indices_from_cpusetids, is_affinity_unset, is_running_as_admin, request_uac_elevation, resolve_address_to_module,
};
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError},
    System::{
        Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS},
        Threading::{
            GetPriorityClass, GetProcessAffinityMask, GetProcessDefaultCpuSets, OpenProcess, OpenThread, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION,
            ProcessMemoryPriority, SetPriorityClass, SetProcessAffinityMask, SetProcessDefaultCpuSets, SetProcessInformation, SetThreadSelectedCpuSets,
            THREAD_QUERY_INFORMATION, THREAD_SET_LIMITED_INFORMATION,
        },
        WindowsProgramming::QueryThreadCycleTime,
    },
};

/// Applies all configured settings to a process: priority, affinity, CPU sets, IO/memory priority.
/// When `dry_run` is true, returns a list of changes that would be made without applying them.
///
/// For processes with `prime_cpus` set, also runs the Prime Thread Scheduler algorithm:
/// 1. Sort threads by CPU time delta to find candidates
/// 2. Query CPU cycles (frequency-independent) for accurate comparison
/// 3. Apply hysteresis to determine which threads should be "prime"
/// 4. Pin prime threads to preferred cores, demote inactive ones
fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    mut processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
) -> Vec<String> {
    let mut changes: Vec<String> = Vec::new();

    let access_flags = if dry_run {
        PROCESS_QUERY_INFORMATION
    } else {
        PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION
    };

    let open_result = unsafe { OpenProcess(access_flags, false, pid) };
    let h_prc = match open_result {
        Err(_) => {
            if dry_run {
                return vec![format!("[SKIP] Cannot open process (access denied)")];
            }
            let error_code = unsafe { GetLastError().0 };
            log_to_find(&format!("apply_config: [OPEN][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
            return changes;
        }
        Ok(h_prc) => h_prc,
    };
    if h_prc.is_invalid() {
        if dry_run {
            return vec![format!("[SKIP] Invalid handle")];
        }
        log_to_find(&format!("apply_config: [INVALID_HANDLE] {:>5}-{}", pid, config.name));
        return changes;
    }

    // Priority
    if let Some(priority_flag) = config.priority.as_win_const() {
        let current_priority = unsafe { GetPriorityClass(h_prc) };
        if current_priority != priority_flag.0 {
            if dry_run {
                changes.push(format!("Priority: current -> {}", config.priority.as_str()));
            } else {
                let set_result = unsafe { SetPriorityClass(h_prc, priority_flag) };
                if set_result.is_ok() {
                    log!("{:>5}-{} -> Priority: {}", pid, config.name, config.priority.as_str());
                } else {
                    let error_code = unsafe { GetLastError().0 };
                    log_to_find(&format!("apply_config: [SET_PRIORITY][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                }
            }
        }
    }

    // Affinity (legacy mask-based API, only for CPUs < 64)
    let mut current_mask: usize = 0;
    let mut system_mask: usize = 0;
    let affinity_mask = cpu_indices_to_mask(&config.affinity_cpus);
    let has_affinity = !config.affinity_cpus.is_empty();
    let has_prime = !config.prime_cpus.is_empty();

    if has_affinity || has_prime {
        let query_result = unsafe { GetProcessAffinityMask(h_prc, &mut current_mask, &mut system_mask) };
        match query_result {
            Err(_) => {
                if !dry_run {
                    let error_code = unsafe { GetLastError().0 };
                    log_to_find(&format!("apply_config: [QUERY_AFFINITY][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                }
            }
            Ok(_) => {
                if has_affinity && affinity_mask != 0 && affinity_mask != current_mask {
                    if dry_run {
                        changes.push(format!("Affinity: {:#X} -> {:#X}", current_mask, affinity_mask));
                    } else {
                        let set_result = unsafe { SetProcessAffinityMask(h_prc, affinity_mask) };
                        match set_result {
                            Err(_) => {
                                let error_code = unsafe { GetLastError().0 };
                                log_to_find(&format!("apply_config: [SET_AFFINITY][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                            }
                            Ok(_) => {
                                log!("{:>5}-{} affinity {:#X} -> {:#X}", pid, config.name, current_mask, affinity_mask);
                                current_mask = affinity_mask;
                            }
                        }
                    }
                }
            }
        }
    }

    // Process default CPU Set (supports >64 cores)
    if !config.cpu_set_cpus.is_empty() && !get_cpu_set_information().lock().unwrap().is_empty() {
        if dry_run {
            changes.push(format!("CPU Set: -> [{}]", format_cpu_indices(&config.cpu_set_cpus)));
        } else {
            let target_cpusetids = cpusetids_from_indices(&config.cpu_set_cpus);
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
                        log_to_find(&format!(
                            "apply_config: [QUERY_CPUSET][{}] {:>5}-{}-{}",
                            error_from_code(code),
                            pid,
                            config.name,
                            requiredidcount
                        ));
                    } else {
                        let mut current_cpusetids: Vec<u32> = vec![0u32; requiredidcount as usize];
                        let second_query = unsafe { GetProcessDefaultCpuSets(h_prc, Some(&mut current_cpusetids[..]), &mut requiredidcount) }.as_bool();
                        if !second_query {
                            let error_code = unsafe { GetLastError().0 };
                            log_to_find(&format!(
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
                        log_to_find(&format!("apply_config: [SET_CPUSET][{}] {:>5}-{}", error_from_code(error_code), pid, config.name));
                    } else {
                        log!("{:>5}-{} -> (cpu set) [{}]", pid, config.name, format_cpu_indices(&config.cpu_set_cpus));
                    }
                }
            }
        }
    }

    // Prime thread Scheduling (supports >64 cores via CPU Set APIs)
    // For dry run, just report prime CPUs would be set
    if !config.prime_cpus.is_empty() {
        if dry_run {
            changes.push(format!("Prime CPUs: -> [{}]", format_cpu_indices(&config.prime_cpus)));
        } else {
            // Filter prime CPUs to those allowed by current process affinity
            // Per MSDN: GetProcessAffinityMask returns 0 when process has threads in multiple
            // processor groups (systems with >64 cores where threads span groups), so we use
            // all specified prime CPUs since the affinity mask is meaningless in that case
            let effective_prime_cpus = if current_mask != 0 {
                filter_indices_by_mask(&config.prime_cpus, current_mask)
            } else {
                config.prime_cpus.clone()
            };

            let cpu_setids = cpusetids_from_indices(&effective_prime_cpus);
            if !cpu_setids.is_empty() {
                prime_core_scheduler.set_alive(pid);
                let process = processes.as_mut().unwrap().pid_to_process.get_mut(&pid).unwrap();
                let thread_count = process.thread_count() as usize;
                let candidate_count = get_cpu_set_information().lock().unwrap().len().min(thread_count);
                let mut candidate_tids: Vec<u32> = vec![0u32; candidate_count];
                // (tid, delta_cycles, is_prime)
                let mut tid_with_delta_cycles: Vec<(u32, u64, bool)> = vec![(0u32, 0u64, false); candidate_count];

                // Step 1: Sort threads by delta time and select top candidates
                // Also collect start addresses for logging
                let mut tid_to_start_address: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
                {
                    let mut tid_with_delta_time: Vec<(u32, i64)> = Vec::with_capacity(thread_count);
                    process.get_threads().iter().for_each(|(tid, thread)| {
                        let total_time = unsafe { thread.KernelTime.QuadPart() + thread.UserTime.QuadPart() };
                        let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);
                        tid_with_delta_time.push((*tid, total_time - thread_stats.last_total_time));
                        thread_stats.last_total_time = total_time;
                        // Store start address for later use (only populated when running elevated)
                        tid_to_start_address.insert(*tid, thread.StartAddress as usize);
                    });
                    tid_with_delta_time.sort_by_key(|&(_, delta)| delta);
                    let precandidate_len = tid_with_delta_time.len();
                    for i in 0..candidate_count {
                        candidate_tids[i] = tid_with_delta_time[precandidate_len - i - 1].0;
                    }
                }

                // Step 2: Open thread handles and query cycle times
                for i in 0..candidate_count {
                    let tid = candidate_tids[i];
                    let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
                    let process_name = &config.name;
                    match thread_stats.handle {
                        None => {
                            let open_result = unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION, false, tid) };
                            match open_result {
                                Err(_) => {
                                    let error_code = unsafe { GetLastError().0 };
                                    log_to_find(&format!(
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
                                            log_to_find(&format!(
                                                "apply_config: [QUERY_THREAD_CYCLE][{}] {:>5}-{}-{}",
                                                error_from_code(error_code),
                                                pid,
                                                tid,
                                                process_name
                                            ));
                                        }
                                        Ok(_) => {
                                            tid_with_delta_cycles[i] = (tid, cycles, false);
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
                                    log_to_find(&format!(
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

                // Step 3: Sort by delta_cycles descending and calculate thresholds
                tid_with_delta_cycles.sort_by_key(|&(_, delta_cycles, _)| std::cmp::Reverse(delta_cycles));
                let max_cycles = tid_with_delta_cycles.first().map(|&(_, c, _)| c).unwrap_or(0u64);
                let entry_min_cycles = (max_cycles as f64 * prime_core_scheduler.constants.entry_threshold) as u64;
                let keep_min_cycles = (max_cycles as f64 * prime_core_scheduler.constants.keep_threshold) as u64;
                let prime_count = cpu_setids.len().min(candidate_count);
                // Update active_streak for all candidate threads
                for &(tid, delta_cycles, _) in &tid_with_delta_cycles {
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
                    if *delta_cycles >= entry_min_cycles
                        && prime_core_scheduler.get_thread_stats(pid, *tid).active_streak >= prime_core_scheduler.constants.min_active_streak
                    {
                        *is_prime = true;
                        new_prime_count += 1;
                    }
                }

                // Step 4: Promote new threads
                for &(tid, delta_cycles, is_prime) in &tid_with_delta_cycles {
                    if !is_prime {
                        continue;
                    }
                    let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
                    if let Some(handle) = thread_stats.handle {
                        if !handle.is_invalid() && thread_stats.cpu_set_ids.is_empty() {
                            let set_result = unsafe { SetThreadSelectedCpuSets(handle, &cpu_setids) }.as_bool();
                            if !set_result {
                                let error_code = unsafe { GetLastError().0 };
                                log_to_find(&format!(
                                    "apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}",
                                    error_from_code(error_code),
                                    pid,
                                    tid,
                                    config.name
                                ));
                            } else {
                                thread_stats.cpu_set_ids = cpu_setids.clone();
                                let promoted_cpus = indices_from_cpusetids(&cpu_setids);
                                let start_module = resolve_address_to_module(pid, process.get_thread(tid).unwrap().StartAddress as usize);
                                log!(
                                    "{:>5}-{}-{} -> (promoted, [{}], cycles={}, start={})",
                                    pid,
                                    tid,
                                    config.name,
                                    format_cpu_indices(&promoted_cpus),
                                    delta_cycles,
                                    start_module
                                );
                            }
                        }
                    }
                }

                // Step 5: Demote threads that are no longer prime
                process.get_threads().iter().for_each(|(tid, thread)| {
                    let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);
                    if !tid_with_delta_cycles.iter().any(|&(t, _, p)| t == *tid && p) && !thread_stats.cpu_set_ids.is_empty() {
                        if let Some(handle) = thread_stats.handle {
                            if !handle.is_invalid() {
                                let set_result = unsafe { SetThreadSelectedCpuSets(handle, &[]) }.as_bool();
                                if !set_result {
                                    let error_code = unsafe { GetLastError().0 };
                                    log_to_find(&format!(
                                        "apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}",
                                        error_from_code(error_code),
                                        pid,
                                        tid,
                                        config.name
                                    ));
                                } else {
                                    let start_module = resolve_address_to_module(pid, thread.StartAddress as usize);
                                    log!("{:>5}-{}-{} -> (demoted, start={})", pid, tid, config.name, start_module);
                                }
                            }
                        }
                        thread_stats.cpu_set_ids = vec![];
                    };
                });
            }
        }
    }

    // IO Priority
    if config.io_priority != priority::IOPriority::None {
        if dry_run {
            changes.push(format!("IO Priority: -> {}", config.io_priority.as_str()));
        } else if let Some(io_priority_flag) = config.io_priority.as_win_const() {
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
                log_to_find(&format!(
                    "apply_config: [QUERY_IO_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                    query_result,
                    pid,
                    config.name,
                    config.io_priority.as_str()
                ));
            } else if current_io_priority != io_priority_flag {
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
                    log_to_find(&format!(
                        "apply_config: [SET_IO_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                        set_result,
                        pid,
                        config.name,
                        config.io_priority.as_str()
                    ));
                } else {
                    log!("{:>5}-{} -> IO: {}", pid, config.name, config.io_priority.as_str());
                }
            }
        }
    }

    // Memory Priority
    if config.memory_priority != priority::MemoryPriority::None {
        if dry_run {
            changes.push(format!("Memory Priority: -> {}", config.memory_priority.as_str()));
        } else if let Some(memory_priority_flag) = config.memory_priority.as_win_const() {
            let mem_prio_info = MemoryPriorityInformation(memory_priority_flag.0);
            let set_result = unsafe {
                SetProcessInformation(
                    h_prc,
                    ProcessMemoryPriority,
                    &mem_prio_info as *const _ as *const std::ffi::c_void,
                    size_of::<MemoryPriorityInformation>() as u32,
                )
            };
            match set_result {
                Ok(_) => {
                    log!("{:>5}-{} -> Memory: {}", pid, config.name, config.memory_priority.as_str());
                }
                Err(e) => {
                    log_to_find(&format!(
                        "apply_config: [SET_MEMORY_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                        e.code().0 as u32,
                        pid,
                        config.name,
                        config.memory_priority.as_str()
                    ));
                }
            }
        }
    }

    let _ = unsafe { CloseHandle(h_prc) };
    changes
}

fn main() -> windows::core::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut interval_ms = 5000;
    let mut help_mode = false;
    let mut help_all_mode = false;
    let mut convert_mode = false;
    let mut find_mode = false;
    let mut validate_mode = false;
    let mut dry_run = false;
    let mut config_file_name = "config.ini".to_string();
    let mut blacklist_file_name: Option<String> = None;
    let mut in_file_name: Option<String> = None;
    let mut out_file_name: Option<String> = None;
    let mut no_uac = false;
    let mut loop_count: Option<u32> = None;
    let mut time_resolution: u32 = 0;
    let mut log_loop = false;
    let mut skip_log_before_elevation = false;
    let mut no_debug_priv = false;
    let mut no_inc_base_priority = false;
    parse_args(
        &args,
        &mut interval_ms,
        &mut help_mode,
        &mut help_all_mode,
        &mut convert_mode,
        &mut find_mode,
        &mut validate_mode,
        &mut dry_run,
        &mut config_file_name,
        &mut blacklist_file_name,
        &mut in_file_name,
        &mut out_file_name,
        &mut no_uac,
        &mut loop_count,
        &mut time_resolution,
        &mut log_loop,
        &mut skip_log_before_elevation,
        &mut no_debug_priv,
        &mut no_inc_base_priority,
    )?;
    if help_mode {
        print_help();
        return Ok(());
    }
    if help_all_mode {
        print_help_all();
        return Ok(());
    }
    if convert_mode {
        convert(in_file_name, out_file_name);
        return Ok(());
    }
    // Always validate config first
    let config_result = read_config(&config_file_name);
    config_result.print_report();

    if !config_result.is_valid() {
        return Ok(());
    }
    // -validate flag: just validate and exit (like loop=1 with dry_run)
    if validate_mode {
        return Ok(());
    }

    // Use configs and constants from the already-validated result
    let configs = config_result.configs;
    let constants = config_result.constants;
    let blacklist = if let Some(bf) = blacklist_file_name {
        read_list(bf).unwrap_or_default()
    } else {
        Vec::new()
    };
    let is_config_empty = configs.is_empty();
    let is_blacklist_empty = blacklist.is_empty();
    if is_config_empty && is_blacklist_empty {
        if !find_mode {
            if skip_log_before_elevation {
                log!("not even a single config, existing");
            }
            return Ok(());
        }
    } else {
        log!("{} blacklist items load", blacklist.len());
    }
    if !skip_log_before_elevation {
        log!("Affinity Service started");
        log!("time interval: {}", interval_ms);
    }
    if !is_running_as_admin() {
        if no_uac {
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
    if !no_debug_priv {
        enable_debug_privilege();
    } else {
        log!("SeDebugPrivilege disabled by -noDebugPriv flag");
    }
    if !no_inc_base_priority {
        enable_inc_base_priority_privilege();
    } else {
        log!("SeIncreaseBasePriorityPrivilege disabled by -noIncBasePriority flag");
    }
    if time_resolution != 0 {
        unsafe {
            let mut current_resolution = 0u32;
            match NtSetTimerResolution(time_resolution, true, &mut current_resolution as *mut _ as *mut std::ffi::c_void).0 {
                ntstatus if ntstatus < 0 => {
                    log!("Failed to set timer resolution: 0x{:08X}", ntstatus);
                }
                ntstatus if ntstatus >= 0 => {
                    log!("Succeed to set timer resolution: {:.4}ms", time_resolution as f64 / 10000f64);
                    log!("elder timer resolution: {:.4}ms", current_resolution);
                }
                _ => {}
            };
        }
    }
    let mut prime_core_scheduler = PrimeThreadScheduler::new(constants);
    let mut current_loop = 0u32;
    let mut should_continue = true;

    while should_continue {
        if log_loop {
            log!("Loop {} started", current_loop + 1);
        }
        match ProcessSnapshot::take() {
            Ok(mut processes) => {
                if dry_run {
                    // Dry run mode: show what would be changed without applying
                    log!(
                        "[DRY RUN] Checking {} running processes against {} config rules...",
                        processes.pid_to_process.len(),
                        configs.len()
                    );
                    let mut total_changes = 0;
                    let pids_and_names: Vec<(u32, String)> = processes.pid_to_process.values().map(|p| (p.pid(), p.get_name().to_string())).collect();
                    for (pid, name) in pids_and_names {
                        if let Some(config) = configs.get(&name) {
                            let changes = apply_config(pid, config, &mut prime_core_scheduler, None, true);
                            if !changes.is_empty() {
                                log!("  {} (PID {}):", config.name, pid);
                                for change in &changes {
                                    log!("    - {}", change);
                                }
                                total_changes += changes.len();
                            }
                        }
                    }
                    if total_changes == 0 {
                        log!("[DRY RUN] No changes would be made.");
                    } else {
                        log!("[DRY RUN] {} change(s) would be made. Run without -dryrun to apply.", total_changes);
                    }
                    // Exit after one dry run iteration
                    should_continue = false;
                } else {
                    prime_core_scheduler.reset_alive();
                    let pids_and_names: Vec<(u32, String)> = processes.pid_to_process.values().map(|p| (p.pid(), p.get_name().to_string())).collect();
                    for (pid, name) in pids_and_names {
                        if let Some(config) = configs.get(&name) {
                            apply_config(pid, config, &mut prime_core_scheduler, Some(&mut processes), false);
                        }
                    }
                    prime_core_scheduler.close_dead_process_handles();
                }
                drop(processes);
            }
            Err(err) => {
                log!("Failed to take process snapshot: {}", err);
            }
        };
        if find_mode {
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
        if let Some(max_loops) = loop_count {
            if current_loop >= max_loops {
                if log_loop {
                    log!("Completed {} loops, exiting", max_loops);
                }
                should_continue = false;
            }
        }
        if should_continue {
            thread::sleep(Duration::from_millis(interval_ms));
            *LOCALTIME_BUFFER.lock().unwrap() = Local::now();
        }
    }
    Ok(())
}
