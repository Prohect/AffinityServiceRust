# main function (main.rs)

The `main` function is the program entry point for AffinityServiceRust. It parses command-line arguments, loads and validates configuration, acquires Windows privileges, optionally starts an ETW (Event Tracing for Windows) process monitor for reactive process detection, and then enters the primary polling loop where process-level and thread-level scheduling policies are applied to matching running processes. The loop supports hot-reloading of both configuration and blacklist files, ETW-driven sleep for power efficiency when no thread-level work is active, and graceful shutdown.

## Syntax

```AffinityServiceRust/src/main.rs#L302-302
fn main() -> windows::core::Result<()>
```

## Parameters

None. Command-line arguments are read via `std::env::args()` and parsed by `parse_args` into a `CliArgs` struct.

## Return value

Returns `windows::core::Result<()>`. Returns `Ok(())` on normal exit, or propagates a `windows::core::Error` if a critical Win32 call (such as `CreateToolhelp32Snapshot` in find mode) fails.

## Remarks

### Startup sequence

1. **CLI parsing** — Calls `parse_args` to populate a `CliArgs` struct. Early-exit modes (`-help`, `-helpAll`, `-convert`, `-autogroup`) return immediately after their respective operations.
2. **Configuration loading** — Calls `read_config` to parse the TOML/custom configuration file. If errors are present, the function prints them and exits. If `-validate` was specified, it exits after the report.
3. **Blacklist loading** — Optionally loads a blacklist file of process names to ignore.
4. **Process-logs mode** — If `-processLogs` is active, delegates to `process_logs` and returns.
5. **Privilege acquisition** — Calls `enable_debug_privilege` and `enable_inc_base_priority_privilege` unless suppressed by `-noDebugPriv` or `-noIncBasePriority` CLI flags.
6. **Timer resolution** — If a custom timer resolution was requested, applies it via `set_timer_resolution`.
7. **UAC elevation** — If the process is not running as administrator and `-noUAC` was not specified, attempts `request_uac_elevation`.
8. **Child process cleanup** — Calls `terminate_child_processes` to clean up stale child processes from a previous run.
9. **ETW monitor** — Unless `-no_etw` is set, starts an `EtwProcessMonitor` that delivers process-start and process-stop events through a channel receiver.

### Main polling loop

Each iteration:

1. Takes a `ProcessSnapshot` via the NT process/thread information API.
2. Resets the alive flags in `PrimeThreadScheduler`.
3. Iterates over graded process-level configs; for each matching `(pid, name)` pair, calls `apply_config` (which internally calls `apply_process_level` and optionally `apply_thread_level`).
4. Processes any ETW-pending PIDs that arrived between iterations.
5. Applies standalone thread-level configs for processes already populated in the scheduler.
6. Cleans up dead processes (handle closure, module-cache purge, optional top-thread report).
7. Runs `process_find` if find mode is active.
8. Flushes loggers.
9. Sleeps for `interval_ms` milliseconds, or blocks on the ETW receiver channel when no thread-level work is pending (power-efficient wait).
10. On wake, drains the ETW channel to update `process_level_pending` and `process_level_applied` lists.
11. Calls `hotreload_config` and `hotreload_blacklist` to pick up file changes without restarting.

### ETW integration

When the ETW monitor is active, process-start events push PIDs onto `process_level_pending` so that rules are applied as soon as the next snapshot sees the process. Process-stop events trigger immediate cleanup via `drop_process_by_pid` and removal from the applied-PID lists.

When the ETW monitor is **not** active (e.g., `-no_etw`), fallback polling detects dead processes by comparing the scheduler's alive flags after each snapshot pass.

### Shutdown

The loop exits when:
- `dry_run` is true (single iteration).
- `loop_count` is reached.
- The ETW receiver channel disconnects (sender dropped).

After the loop, the ETW monitor is stopped if it was started.

### Platform notes

- **Windows only.** Uses Win32 process/thread APIs, NT kernel information classes, and ETW.
- Requires **administrator** privileges for full functionality (adjusting priorities, setting affinity on protected processes). The function will attempt UAC self-elevation unless suppressed.
- `SeDebugPrivilege` is needed to open handles to processes owned by other users.
- `SeIncreaseBasePriorityPrivilege` is needed to set I/O priority to High.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main.rs` |
| Callers | Operating system (program entry point) |
| Callees | `parse_args`, `read_config`, `read_list`, `process_logs`, `process_find`, `apply_config`, `enable_debug_privilege`, `enable_inc_base_priority_privilege`, `set_timer_resolution`, `is_running_as_admin`, `request_uac_elevation`, `terminate_child_processes`, `EtwProcessMonitor::start`, `ProcessSnapshot::take`, `hotreload_config`, `hotreload_blacklist`, `PrimeThreadScheduler::new` |
| Win32 API | `CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`, `GetProcessAffinityMask`, `CloseHandle`, `GetConsoleOutputCP` |
| Privileges | `SeDebugPrivilege`, `SeIncreaseBasePriorityPrivilege` (optional, enabled by default) |

## See Also

| Reference | Link |
|-----------|------|
| apply_config | [apply_config](apply_config.md) |
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| process_find | [process_find](process_find.md) |
| process_logs | [process_logs](process_logs.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| cli module | [cli.rs README](../cli.rs/README.md) |
| config module | [config.rs README](../config.rs/README.md) |
| event_trace module | [event_trace.rs README](../event_trace.rs/README.md) |

---
Commit: `7221ea0694670265d4eb4975582d8ed2ae02439d`
