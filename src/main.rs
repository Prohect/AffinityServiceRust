mod apply;
mod cli;
mod config;
mod error_codes;
mod event_trace;
mod logging;
mod priority;
mod process;
mod scheduler;
mod winapi;

use apply::{
    ApplyConfigResult, apply_affinity, apply_ideal_processors, apply_io_priority, apply_memory_priority, apply_prime_threads, apply_priority,
    apply_process_default_cpuset, prefetch_all_thread_cycles, update_thread_stats,
};
use cli::{CliArgs, parse_args, print_help, print_help_all};
use config::{ProcessConfig, convert, hotreload_blacklist, hotreload_config, read_config, read_list, sort_and_group_config};
use event_trace::EtwProcessMonitor;
use logging::{log_message, log_process_find, log_pure_message, log_to_find, purge_fail_map};
use process::{PID_TO_PROCESS_MAP, ProcessEntry, ProcessSnapshot, SNAPSHOT_BUFFER};
use scheduler::PrimeThreadScheduler;
use winapi::{
    drop_module_cache, enable_debug_privilege, enable_inc_base_priority_privilege, get_process_handle, is_affinity_unset, is_running_as_admin,
    request_uac_elevation, set_timer_resolution, terminate_child_processes,
};

use chrono::Local;
use encoding_rs::Encoding;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{metadata, read_dir, read_to_string, write},
    io::Write,
    mem::size_of,
    process::Command,
    thread,
    time::Duration,
};
use windows::Win32::{
    Foundation::CloseHandle,
    System::{
        Console::GetConsoleOutputCP,
        Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS},
        Threading::GetProcessAffinityMask,
    },
};

/// Applies process-level settings (one-shot per process).
/// Includes: priority, affinity (with thread ideal processor reset), CPU set, IO priority, memory priority.
fn apply_config_process_level(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) {
    let Some(process_handle) = get_process_handle(pid, &config.name) else {
        return;
    };
    let mut current_mask: usize = 0;
    apply_priority(pid, config, dry_run, &process_handle, apply_config_result);
    apply_affinity(
        pid,
        config,
        dry_run,
        &mut current_mask,
        &process_handle,
        process,
        apply_config_result,
    );
    apply_process_default_cpuset(pid, config, dry_run, &process_handle, process, apply_config_result);
    apply_io_priority(pid, config, dry_run, &process_handle, apply_config_result);
    apply_memory_priority(pid, config, dry_run, &process_handle, apply_config_result);
}

/// Applies thread-level settings (every polling iteration).
/// Includes: prime thread scheduling, ideal processor assignment, cycle time tracking.
fn apply_config_thread_level(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) {
    if !config.prime_threads_cpus.is_empty()
        || !config.prime_threads_prefixes.is_empty()
        || !config.ideal_processor_rules.is_empty()
        || config.track_top_x_threads != 0
    {
        // Query current affinity mask for prime thread CPU filtering
        let mut current_mask: usize = 0;
        if (!config.prime_threads_cpus.is_empty() || !config.affinity_cpus.is_empty())
            && let Some(process_handle) = get_process_handle(pid, &config.name)
        {
            let mut system_mask: usize = 0;
            let r_handle = process_handle.r_handle.unwrap_or(process_handle.r_limited_handle);
            let _ = unsafe { GetProcessAffinityMask(r_handle, &mut current_mask, &mut system_mask) };
        }
        drop_module_cache(pid);
        prime_core_scheduler.set_alive(pid);
        prefetch_all_thread_cycles(pid, config, process, prime_core_scheduler, apply_config_result);
        apply_prime_threads(
            pid,
            config,
            dry_run,
            &mut current_mask,
            process,
            prime_core_scheduler,
            apply_config_result,
        );
        apply_ideal_processors(pid, config, dry_run, process, prime_core_scheduler, apply_config_result);
        update_thread_stats(pid, prime_core_scheduler);
    }
}

/// Processes log files from -find mode to discover new processes.
///
/// Scans .find.log files for discovered processes, filters out known ones,
/// and uses Everything search (es.exe) to locate executable paths.
/// Results are written to a text file for manual review.
fn process_logs(
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
) {
    *get_use_console!() = true;
    let logs_path = logs_path.unwrap_or("logs");
    let output_file = output_file.unwrap_or("new_processes_results.txt");

    let mut all_processes = HashSet::new();
    if let Ok(entries) = read_dir(logs_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".find.log"))
                && let Ok(content) = read_to_string(&path)
            {
                for line in content.lines() {
                    if let Some(idx) = line.find("find ") {
                        let rest = &line[idx + 5..];
                        let proc = if let Some(space_idx) = rest.find(' ') {
                            &rest[..space_idx]
                        } else {
                            rest.trim()
                        };
                        if proc.ends_with(".exe") {
                            all_processes.insert(proc.to_lowercase());
                        }
                    }
                }
            }
        }
    }

    let in_any_grade = |p: &String| configs.values().any(|grade_configs| grade_configs.contains_key(p));
    let new_processes: Vec<String> = all_processes
        .into_iter()
        .filter(|p| !in_any_grade(p) && !blacklist.contains(p))
        .collect();

    let mut output = String::new();
    let acp = unsafe { GetConsoleOutputCP() };
    let label = if acp == 936 { "gbk" } else { &format!("windows-{}", acp) };
    let encoding = Encoding::for_label_no_replacement(label.as_bytes()).unwrap_or(encoding_rs::UTF_8);
    for proc in new_processes {
        output.push_str(&format!("Process: {}\n", proc));

        let escaped_proc = proc.replace(".", r"\.");
        let es_output = Command::new("es")
            .args(["-utf8-bom", "-r", &format!("^{}$", escaped_proc)])
            .output();
        match es_output {
            Ok(output_result) if output_result.status.success() => {
                let stdout = &output_result.stdout;

                let ansi_bytes = if stdout.starts_with(&[0xEF, 0xBB, 0xBF]) {
                    &stdout[3..]
                } else {
                    stdout
                };

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

    if let Err(e) = write(output_file, output) {
        log!("Failed to write output: {}", e);
    } else {
        log!("Results saved to {}", output_file);
    }
}

fn process_find(
    cli: &CliArgs,
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
) -> Result<(), windows::core::Error> {
    let _: () = if cli.find_mode {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
            let mut pe32 = PROCESSENTRY32W {
                dwSize: size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };
            if Process32FirstW(snapshot, &mut pe32).is_ok() {
                loop {
                    let process_name =
                        String::from_utf16_lossy(&pe32.szExeFile[..pe32.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)]).to_lowercase();

                    let in_configs = configs.values().any(|grade_configs| grade_configs.contains_key(&process_name));
                    if !get_fail_find_set!().contains(&process_name)
                        && !in_configs
                        && !blacklist.contains(&process_name)
                        && is_affinity_unset(pe32.th32ProcessID, process_name.as_str())
                    {
                        log_process_find(&process_name);
                    }
                    if Process32NextW(snapshot, &mut pe32).is_err() {
                        break;
                    }
                }
            }
            let _ = CloseHandle(snapshot);
        }
    };
    Ok(())
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
    if cli.autogroup_mode {
        sort_and_group_config(cli.in_file_name, cli.out_file_name);
        return Ok(());
    }
    let config_result = read_config(&cli.config_file_name);

    *get_dust_bin_mod!() = cli.skip_log_before_elevation;
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
    if cli.process_logs_mode {
        process_logs(&configs, &blacklist, cli.in_file_name.as_deref(), cli.out_file_name.as_deref());
        return Ok(());
    }

    let mut last_config_mod_time = metadata(&cli.config_file_name).and_then(|m| m.modified()).ok();
    let mut last_blacklist_mod_time = cli
        .blacklist_file_name
        .as_ref()
        .and_then(|bf| metadata(bf).and_then(|m| m.modified()).ok());
    let is_config_empty = configs.is_empty();
    let is_blacklist_empty = blacklist.is_empty();
    if is_config_empty && is_blacklist_empty {
        if !cli.find_mode {
            log!("not even a single config, existing");
            return Ok(());
        }
    } else {
        log!("{} blacklist items load", blacklist.len());
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
        set_timer_resolution(&cli);
    }
    log!("Affinity Service started with time interval: {}", cli.interval_ms);

    if !is_running_as_admin() {
        if cli.no_uac {
            log!("Not running as administrator. UAC elevation disabled by -noUAC flag.");
            log!("Warning: May not be able to manage all processes without admin privileges.");
        } else {
            log!("Not running as administrator. Requesting UAC elevation...");
            match request_uac_elevation(*get_use_console!()) {
                Ok(_) => {
                    log!("Running with administrator privileges.");
                }
                Err(e) => {
                    log!("Failed to request elevation: {}, may not manage all processes", e);
                }
            }
        }
    }
    terminate_child_processes();
    *get_dust_bin_mod!() = false;
    let mut prime_core_scheduler = PrimeThreadScheduler::new(config_result.constants);
    let mut current_loop = 0u32;
    let mut should_continue = true;

    // Start ETW process monitor for reactive process level rule application
    let (event_trace_monitor, event_trace_receiver) = if !(cli.no_etw) {
        match EtwProcessMonitor::start() {
            Err(e) => {
                log!("ETW process monitor failed to start: {} (falling back to polling only)", e);
                (None, None)
            }
            Ok((monitor, rx)) => {
                log!("ETW process monitor started (reactive process detection enabled)");
                (Some(monitor), Some(rx))
            }
        }
    } else {
        (None, None)
    };

    let mut process_level_applied: HashSet<u32> = HashSet::new();
    let mut process_level_pending: HashSet<u32> = HashSet::new();

    while should_continue {
        if cli.log_loop {
            log!("Loop {} started", current_loop + 1);
        }
        match ProcessSnapshot::take(&mut SNAPSHOT_BUFFER.lock().unwrap(), &mut PID_TO_PROCESS_MAP.lock().unwrap()) {
            Err(err) => {
                log!("Failed to take process snapshot: {}", err);
            }
            Ok(processes) => {
                let mut total_changes = 0;
                let pids_and_names: Vec<(u32, String)> = processes
                    .pid_to_process
                    .values()
                    .map(|p| (p.pid(), p.get_name().to_string()))
                    .collect();
                prime_core_scheduler.reset_alive();

                // Grade-based scheduling: rules with higher grade values run less frequently
                // e.g., grade=1 runs every loop, grade=5 runs every 5th loop;
                for (grade, grade_configs) in &configs {
                    // process_level_pending dont respect grade being applied just in time
                    process_level_pending.retain(|pid_pending| {
                        pids_and_names.iter().any(|(pid, name)| -> bool {
                            if pid == pid_pending {
                                if let Some(config) = grade_configs.get(name)
                                    && let Some(process) = processes.pid_to_process.get_mut(pid)
                                {
                                    let mut result = ApplyConfigResult::new();
                                    apply_config_process_level(*pid, config, process, cli.dry_run, &mut result);
                                    if !result.is_empty() {
                                        for error in &result.errors {
                                            log_to_find(error);
                                        }
                                        if !result.changes.is_empty() {
                                            let first = format!("{:>5}::{}::{}", pid, config.name, result.changes[0]);
                                            log_message(&first);
                                            let padding = " ".repeat(first.len() - result.changes[0].len() + 10); //10 for time prefix, eg."[04:55:16]"
                                            for change in &result.changes[1..] {
                                                log_pure_message(&format!("{}{}", padding, change));
                                            }
                                        }
                                        total_changes += result.changes.len();
                                    }
                                    process_level_applied.insert(*pid);
                                    false
                                } else {
                                    true
                                }
                            } else {
                                true
                            }
                        })
                    });

                    if !current_loop.is_multiple_of(*grade) {
                        continue; // Skip this grade this loop
                    }

                    for (pid, name) in &pids_and_names {
                        if let Some(config) = grade_configs.get(name) {
                            let Some(process) = processes.pid_to_process.get_mut(pid) else {
                                continue;
                            };

                            let mut result = ApplyConfigResult::new();
                            if cli.continuous_process_level_apply || !process_level_applied.contains(pid) {
                                apply_config_process_level(*pid, config, process, cli.dry_run, &mut result);
                                process_level_applied.insert(*pid);
                            }
                            apply_config_thread_level(*pid, config, &mut prime_core_scheduler, process, cli.dry_run, &mut result);
                            if !result.is_empty() {
                                for error in &result.errors {
                                    log_to_find(error);
                                }
                                if !result.changes.is_empty() {
                                    let first = format!("{:>5}::{}::{}", pid, config.name, result.changes[0]);
                                    log_message(&first);
                                    let padding = " ".repeat(first.len() - result.changes[0].len() + 10); //10 for time prefix, eg."[04:55:16]"
                                    for change in &result.changes[1..] {
                                        log_pure_message(&format!("{}{}", padding, change));
                                    }
                                }
                                total_changes += result.changes.len();
                            }
                        }
                    }
                }

                if event_trace_receiver.is_none() {
                    let dead_pids: Vec<u32> = prime_core_scheduler
                        .pid_to_process_stats
                        .iter()
                        .filter_map(|(pid, process_stats)| if !process_stats.alive { Some(*pid) } else { None })
                        .collect();
                    dead_pids.into_iter().for_each(|pid| {
                        prime_core_scheduler.drop_process_by_pid(&pid);
                    });
                    purge_fail_map(&pids_and_names);
                    process_level_applied.retain(|pid| pids_and_names.iter().any(|(p, _)| p == pid));
                }
                process_level_pending.clear();
                if cli.dry_run {
                    log!("[DRY RUN] {} change(s) would be made. Run without -dryrun to apply.", total_changes);
                    should_continue = false;
                }
                drop(processes);
            }
        };
        process_find(&cli, &configs, &blacklist)?;

        let _ = get_logger_find!().flush();
        let _ = get_logger!().flush();
        current_loop += 1;
        if let Some(max_loops) = cli.loop_count
            && current_loop >= max_loops
        {
            if cli.log_loop {
                log!("Completed {} loops, exiting", max_loops);
            }
            should_continue = false;
        }
        if should_continue {
            thread::sleep(Duration::from_millis(cli.interval_ms));

            *get_local_time!() = Local::now();
            if let Some(ref event_trace_receiver) = event_trace_receiver {
                let mut pid_map_fail_entry_set = get_pid_map_fail_entry_set!();
                while let Ok(event) = event_trace_receiver.try_recv() {
                    if event.is_start {
                        process_level_pending.insert(event.pid);
                    } else {
                        process_level_pending.remove(&event.pid);
                        process_level_applied.remove(&event.pid);
                        pid_map_fail_entry_set.remove(&event.pid);
                        prime_core_scheduler.drop_process_by_pid(&event.pid);
                    }
                }
            }
            hotreload_config(
                &cli,
                &mut configs,
                &mut last_config_mod_time,
                &mut prime_core_scheduler,
                &mut process_level_applied,
            );
            hotreload_blacklist(&cli, &mut blacklist, &mut last_blacklist_mod_time);
        }
    }
    // Stop ETW process monitor
    if let Some(mut event_trace_monitor) = event_trace_monitor {
        event_trace_monitor.stop();
        log!("ETW process monitor stopped");
    }
    Ok(())
}
