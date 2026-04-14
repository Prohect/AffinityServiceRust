# main function (main.rs)

Entry point for the AffinityServiceRust service. Parses command-line arguments, loads the configuration file, handles mode dispatch (help, convert, autogroup, validate, process logs), and runs the main polling loop that enforces process-level and thread-level settings on managed processes. Integrates ETW-based reactive process detection, grade-based scheduling, and hot-reload of configuration and blacklist files.

## Syntax

```rust
fn main() -> windows::core::Result<()>
```

## Parameters

This function takes no parameters. Command-line arguments are read from the environment via `std::env::args()`.

## Return value

Returns `Ok(())` on successful completion, or a `windows::core::Error` if a critical Win32 API call fails (e.g., `CreateToolhelp32Snapshot` in [process_find](process_find.md)). Most errors are logged and handled gracefully rather than propagated — the function returns `Ok(())` even when configuration errors or elevation failures occur.

## Remarks

### Startup sequence

The function executes the following initialization steps:

1. **Parse CLI arguments** — Calls [parse_args](../cli.rs/parse_args.md) to populate a [CliArgs](../cli.rs/CliArgs.md) structure with defaults and user-supplied overrides.

2. **Mode dispatch** — Checks mode flags in priority order and exits early for non-service modes:
   - `help_mode` → calls [print_help](../cli.rs/print_help.md) and returns.
   - `help_all_mode` → calls [print_help_all](../cli.rs/print_help_all.md) and returns.
   - `convert_mode` → calls [convert](../config.rs/convert.md) and returns.
   - `autogroup_mode` → calls [sort_and_group_config](../config.rs/sort_and_group_config.md) and returns.

3. **Load configuration** — Calls [read_config](../config.rs/read_config.md) to parse the configuration file into a grade-keyed `HashMap<u32, HashMap<String, ProcessConfig>>`. The configuration report (rule counts, warnings) is printed. If any errors exist, the function logs a message and returns without entering the polling loop.

4. **Validate mode** — If `validate_mode` is set, the function returns after printing the configuration report without entering the polling loop.

5. **Load blacklist** — If a blacklist file is specified, calls [read_list](../config.rs/read_list.md) to load process names to exclude from management and discovery.

6. **Process logs mode** — If `process_logs_mode` is set, calls [process_logs](process_logs.md) with the loaded configs and blacklist, then returns.

7. **Empty config check** — If both configs and blacklist are empty and `-find` mode is not active, the function logs a message and exits. If `-find` is active, the service proceeds with an empty config to discover unmanaged processes.

8. **Privilege acquisition** — Calls [enable_debug_privilege](../winapi.rs/enable_debug_privilege.md) and [enable_inc_base_priority_privilege](../winapi.rs/enable_inc_base_priority_privilege.md) unless suppressed by the `-noDebugPriv` or `-noIncBasePriority` flags.

9. **Timer resolution** — If `-resolution` is specified with a non-zero value, calls [set_timer_resolution](../winapi.rs/set_timer_resolution.md) to adjust the system timer granularity.

10. **UAC elevation** — If the process is not running as administrator, calls [request_uac_elevation](../winapi.rs/request_uac_elevation.md) to re-launch with elevated privileges (unless `-noUAC` is set). Note that console output is not visible in the elevated session; log files should be used instead.

11. **Child process cleanup** — Calls [terminate_child_processes](../winapi.rs/terminate_child_processes.md) to clean up any orphaned child processes from a previous elevation attempt.

12. **Initialize scheduler** — Creates a [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) with the parsed [ConfigConstants](../config.rs/ConfigConstants.md) (hysteresis thresholds and minimum active streak).

13. **Start ETW monitor** — Calls [EtwProcessMonitor::start](../event_trace.rs/EtwProcessMonitor.md) to begin real-time process start/stop event tracing (unless `-no_etw` is set). If ETW fails to start (e.g., insufficient privileges or another trace session is active), the service falls back to polling-only mode.

### Main polling loop

The loop runs indefinitely (or for a fixed count if `-loop` is specified) with a sleep interval of `cli.interval_ms` milliseconds between iterations. Each iteration performs:

1. **Take process snapshot** — Calls [ProcessSnapshot::take](../process.rs/ProcessSnapshot.md) using `NtQuerySystemInformation` to enumerate all processes and their threads with cycle-time data. If the snapshot fails, the error is logged and the iteration is skipped.

2. **Reset alive flags** — Calls `prime_core_scheduler.reset_alive()` to mark all tracked processes as potentially dead. Processes that are matched in the current snapshot are re-marked alive later.

3. **Process ETW pending queue** — For each PID in `process_level_pending` (populated by ETW process-start events between iterations), the function attempts to apply process-level settings immediately, regardless of grade. This provides near-instant rule application for newly launched processes. Successfully applied PIDs are moved to `process_level_applied` and removed from the pending set.

4. **Grade-based iteration** — Iterates over all grade levels in the configuration. A grade `N` means the rules at that level are only evaluated when `current_loop` is a multiple of `N`. For each process at the current grade:
   - If the PID has not yet had process-level settings applied, calls [apply_config_process_level](apply_config_process_level.md) and records the PID in `process_level_applied`.
   - Calls [apply_config_thread_level](apply_config_thread_level.md) for thread-level settings (always re-evaluated).
   - Logs all changes and errors from the [ApplyConfigResult](../apply.rs/ApplyConfigResult.md).

5. **Dead process cleanup** — When ETW is not active, dead processes (those not marked alive) are removed from the `PrimeThreadScheduler` and the error-deduplication fail map is purged. When ETW is active, cleanup is handled reactively via process-exit events.

6. **Dry-run exit** — If `-dryrun` is active, logs the total number of changes that would have been made and exits after the first iteration.

7. **Find mode** — Calls [process_find](process_find.md) to discover unmanaged processes (if `-find` is active).

8. **Flush logs** — Flushes both the main log file and the find-log file.

9. **Loop termination** — Increments the loop counter. If `-loop <count>` was specified and the count is reached, sets `should_continue = false`.

10. **Sleep and ETW drain** — Sleeps for the configured interval, updates the local time cache, and drains the ETW event receiver channel. Process-start events add PIDs to `process_level_pending`; process-exit events remove PIDs from `process_level_pending`, `process_level_applied`, the error fail map, and the prime thread scheduler.

11. **Hot-reload** — Calls [hotreload_config](../config.rs/hotreload_config.md) and [hotreload_blacklist](../config.rs/hotreload_blacklist.md) to detect file modifications and reload the configuration and blacklist without restarting the service. When the config is reloaded, `process_level_applied` is cleared to force re-application of process-level settings.

### ETW integration

The ETW process monitor provides two key benefits:

- **Reactive application** — New processes have their settings applied on the next polling iteration after launch, rather than waiting for the grade-based schedule to reach them. The `process_level_pending` set bridges the gap between ETW event receipt (asynchronous) and the synchronous polling loop.
- **Prompt cleanup** — When a process exits, its state is immediately removed from the scheduler, fail map, and applied set, preventing stale data accumulation and ensuring PID reuse is handled correctly.

If ETW is unavailable, the service degrades gracefully to polling-only mode where dead-process cleanup happens at the end of each iteration by comparing the scheduler's tracked PIDs against the live snapshot.

### Logging

Changes are logged in a formatted layout:

```
[HH:MM:SS] <PID>::<process_name>::<first_change>
                                   <subsequent_changes>
```

Errors from apply operations are forwarded to the find-log via `log_to_find`. The padding for multi-line change entries accounts for a 10-character time prefix (e.g., `[04:55:16]`).

### Shutdown

After the polling loop exits (due to `-loop` count, `-dryrun`, or signal), the function stops the ETW monitor (if active) by calling `event_trace_monitor.stop()` and returns `Ok(())`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main` |
| Callers | Rust runtime (program entry point) |
| Callees | [parse_args](../cli.rs/parse_args.md), [print_help](../cli.rs/print_help.md), [print_help_all](../cli.rs/print_help_all.md), [read_config](../config.rs/read_config.md), [read_list](../config.rs/read_list.md), [convert](../config.rs/convert.md), [sort_and_group_config](../config.rs/sort_and_group_config.md), [hotreload_config](../config.rs/hotreload_config.md), [hotreload_blacklist](../config.rs/hotreload_blacklist.md), [enable_debug_privilege](../winapi.rs/enable_debug_privilege.md), [enable_inc_base_priority_privilege](../winapi.rs/enable_inc_base_priority_privilege.md), [set_timer_resolution](../winapi.rs/set_timer_resolution.md), [is_running_as_admin](../winapi.rs/is_running_as_admin.md), [request_uac_elevation](../winapi.rs/request_uac_elevation.md), [terminate_child_processes](../winapi.rs/terminate_child_processes.md), [EtwProcessMonitor::start](../event_trace.rs/EtwProcessMonitor.md), [ProcessSnapshot::take](../process.rs/ProcessSnapshot.md), [apply_config_process_level](apply_config_process_level.md), [apply_config_thread_level](apply_config_thread_level.md), [process_find](process_find.md), [process_logs](process_logs.md) |
| API | `NtQuerySystemInformation` (via `ProcessSnapshot`), `CreateToolhelp32Snapshot` (via `process_find`), ETW (`StartTrace`/`ProcessTrace`/`ControlTrace`), `GetProcessAffinityMask`, various `Set*` APIs via apply functions |
| Privileges | `SeDebugPrivilege` (recommended), `SeIncreaseBasePriorityPrivilege` (for High/Realtime priority), Administrator (recommended for full process access and ETW) |

## See Also

| Topic | Link |
|-------|------|
| CLI argument parsing | [parse_args](../cli.rs/parse_args.md) |
| CLI arguments structure | [CliArgs](../cli.rs/CliArgs.md) |
| Process-level settings (one-shot) | [apply_config_process_level](apply_config_process_level.md) |
| Thread-level settings (per-iteration) | [apply_config_thread_level](apply_config_thread_level.md) |
| Configuration file parsing | [read_config](../config.rs/read_config.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| ETW process monitor | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |
| Process snapshot | [ProcessSnapshot](../process.rs/ProcessSnapshot.md) |
| Find mode (runtime discovery) | [process_find](process_find.md) |
| Log processing | [process_logs](process_logs.md) |


## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd