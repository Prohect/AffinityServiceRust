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
2. **Configuration loading** — Calls `read_config` to parse the TOML/custom configuration file into a `ConfigResult` struct (containing `process_level_configs`, `thread_level_configs`, `constants`, and `errors`). If errors are present, the function prints them and exits. If `-validate` was specified, it exits after the report.
3. **Blacklist loading** — Optionally loads a blacklist file of process names to ignore.
4. **Process-logs mode** — If `-processLogs` is active, delegates to `process_logs` and returns.
5. **Privilege acquisition** — Calls `enable_debug_privilege` and `enable_inc_base_priority_privilege` unless suppressed by `-noDebugPriv` or `-noIncBasePriority` CLI flags.
6. **Timer resolution** — If a custom timer resolution was requested, applies it via `set_timer_resolution`.
7. **UAC elevation** — If the process is not running as administrator and `-noUAC` was not specified, attempts `request_uac_elevation`.
8. **Child process cleanup** — Calls `terminate_child_processes` to clean up stale child processes from a previous run.
9. **ETW monitor** — Unless `-no_etw` is set, starts an `EtwProcessMonitor` that delivers process-start and process-stop events through a channel receiver.

### Main polling loop

The loop maintains three PID tracking lists:

- `process_level_applied` — PIDs that have already had process-level settings applied (deduped each iteration, retained across iterations).
- `thread_level_applied` — PIDs that have already had thread-level settings applied in the **current** iteration (cleared at the end of each iteration so thread-level configs are re-applied every poll).
- `process_level_pending` — PIDs queued by ETW events for just-in-time application regardless of grade.

Each iteration:

1. Takes a `ProcessSnapshot` via the NT process/thread information API.
2. Builds `pids_and_names` as a `List<[(u32, &str); PIDS]>` (stack-allocated small-vec of PID/name pairs borrowed from the snapshot).
3. Resets the alive flags in `PrimeThreadScheduler`.
4. **Process-level config pass** — Iterates over `configs.process_level_configs` by grade:
   - First, processes any ETW-pending PIDs via `process_level_pending.retain(...)`, calling `apply_config` for each match (just-in-time, grade-independent).
   - Then, for PIDs matching the grade schedule (`current_loop.is_multiple_of(*grade)`), calls `apply_config` for each `(pid, name)` that matches a `ProcessLevelConfig` entry—provided the PID has not already been applied (or `continuous_process_level_apply` is set).
   - `apply_config` internally also looks up and applies any matching `ThreadLevelConfig` for the same grade and name, pushing the PID into both `process_level_applied` and `thread_level_applied`. This "both-level apply" path exists to reduce `get_threads()` calls and merge log output for the same process.
5. **Standalone thread-level config pass** — If the scheduler has any tracked processes (`pid_to_process_stats` is non-empty), iterates over `configs.thread_level_configs` by grade. For each matching `(pid, name)`, if the PID was **not** already handled by the both-level apply in step 4 (i.e., not in `thread_level_applied`), creates its own `OnceCell`-backed thread cache and calls `apply_thread_level` directly, followed by `log_apply_results`.
6. Cleans up dead processes (handle closure, module-cache purge, optional top-thread report) when ETW is not active.
7. Runs `process_find` if find mode is active.
8. In `dry_run` mode, sets `should_continue = false` to exit after a single iteration (no longer logs a total change count).
9. Drops `pids_and_names` and `processes` explicitly before sleep.
10. Flushes loggers.
11. **Sleep** — Either blocks on the ETW receiver channel (power-efficient wait) when no thread-level work is pending, or falls back to `thread::sleep(interval_ms)`.
12. On wake, drains the ETW channel to update `process_level_pending` and `process_level_applied` lists.
13. Calls `hotreload_config` and `hotreload_blacklist` to pick up file changes without restarting.
14. Calls `process_level_applied.dedup()` and `process_level_pending.dedup()` for compaction; clears `thread_level_applied` so thread-level rules are re-evaluated next iteration.

### ETW-driven sleep

When the ETW monitor is active and the prime-thread scheduler has no tracked processes (`pid_to_process_stats` is empty), the loop enters a power-efficient wait instead of a fixed `thread::sleep`. It calls `event_trace_receiver.recv_timeout(...)` in a loop with a timeout of `(interval_ms + 16) / 2` milliseconds. The loop breaks when either:
- The receiver disconnects (sets `should_continue = false` to trigger shutdown).
- Enough wall-clock time has elapsed (checked via `Local::now() - last_time > TimeDelta::milliseconds(interval_ms)`).

This reduces CPU wake-ups when the service has no thread-level work to perform and is only waiting for new processes to appear via ETW events.

When the ETW monitor is **not** active (e.g., `-no_etw`), fallback polling detects dead processes by comparing the scheduler's alive flags after each snapshot pass, and the loop always uses `thread::sleep`.

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
| Callees | `parse_args`, `read_config`, `read_list`, `process_logs`, `process_find`, `apply_config`, `apply_thread_level`, `log_apply_results`, `enable_debug_privilege`, `enable_inc_base_priority_privilege`, `set_timer_resolution`, `is_running_as_admin`, `request_uac_elevation`, `terminate_child_processes`, `EtwProcessMonitor::start`, `ProcessSnapshot::take`, `hotreload_config`, `hotreload_blacklist`, `PrimeThreadScheduler::new` |
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
Commit: `b0df9da35213b050501fab02c3020ad4dbd6c4e0`
