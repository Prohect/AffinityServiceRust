# main function (main.rs)

Application entry point for AffinityServiceRust. Parses command-line arguments, dispatches to the appropriate operational mode, and — for the default service mode — runs the main polling loop that enforces process and thread configuration on running Windows processes.

## Syntax

```rust
fn main() -> windows::core::Result<()>
```

## Return value

`windows::core::Result<()>` — Returns `Ok(())` on graceful shutdown. Propagates Windows errors from snapshot creation or CLI parsing.

## Remarks

The function implements a multi-phase startup followed by a continuous enforcement loop.

### Phase 1 — CLI dispatch

1. **Parse arguments** — Calls [`parse_args`](../cli.rs/parse_args.md) to populate a [`CliArgs`](../cli.rs/CliArgs.md) struct.
2. **Mode dispatch** — Checks for early-exit modes in order:
   - `-help` → [`print_help`](../cli.rs/print_help.md) and return.
   - `-helpAll` → [`print_help_all`](../cli.rs/print_help_all.md) and return.
   - `-convert` → [`convert`](../config.rs/convert.md) and return.
   - `-autogroup` → [`sort_and_group_config`](../config.rs/sort_and_group_config.md) and return.
   - `-validate` → Reads config, prints validation report, and returns.
   - `-processLogs` → [`process_logs`](process_logs.md) and return.

### Phase 2 — Configuration and privilege setup

3. **Read config** — Calls [`read_config`](../config.rs/read_config.md) to parse the INI-format configuration file into a [`ConfigResult`](../config.rs/ConfigResult.md). Prints the configuration report. If errors are found, exits.
4. **Read blacklist** — Optionally reads a blacklist file of process names to ignore.
5. **Empty check** — If both config and blacklist are empty and find mode is not enabled, exits.
6. **Enable privileges** — Calls [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md) and [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) for the current process token.
7. **Timer resolution** — Calls [`set_timer_resolution`](../winapi.rs/set_timer_resolution.md) to set the system timer to the configured resolution.
8. **UAC elevation** — If not running as administrator and `-noUAC` is not set, calls [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md) to re-launch the process elevated.
9. **Terminate children** — Calls [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) to clean up prior non-elevated instances.

### Phase 3 — ETW monitor

10. **Start ETW** — Unless `-noETW` is set, starts an [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) that delivers process start/stop events over an `mpsc` channel. If ETW fails, falls back to polling-only mode.

### Phase 4 — Main loop

The loop repeats until shutdown (dry-run completes after one iteration, `-loop` caps iterations, or ETW channel disconnects):

11. **Take snapshot** — Calls [`ProcessSnapshot::take`](../process.rs/ProcessSnapshot.md) to enumerate all running processes.
12. **Match rules** — Iterates graded config rules. For each grade and process name match, calls [`apply_config`](apply_config.md) which handles both process-level and thread-level application.
    - **ETW pending list** — Processes received from ETW events are applied eagerly via `process_level_pending`, which is drained by matching against snapshot data in the retain loop.
    - **Full match vs. graded** — The first loop iteration (and after config reload) does a full match against all processes. Subsequent iterations only match processes at their configured grade interval.
    - **Continuous apply** — When `-continuousProcessLevelApply` is set, process-level configs are reapplied every iteration. Otherwise, `process_level_applied` tracks PIDs that have already been configured.
13. **Thread-level pass** — After the combined pass, runs a dedicated thread-level-only pass for processes already initialized by the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), applying cycle-time-based scheduling at each grade interval.
14. **Cleanup** — Removes dead PIDs from `prime_core_scheduler`, `process_level_applied`, and the fail map.
15. **Find mode** — Calls [`process_find`](process_find.md) to log unmanaged processes.
16. **Flush logs** — Flushes both the main logger and find logger.

### Phase 5 — Sleep and hot-reload

17. **ETW-reactive sleep** — When no thread-level tracking is active and ETW is available, the loop blocks on the ETW channel with a timeout of `(interval_ms + 16) / 2` milliseconds. Process start events are queued into `process_level_pending`; stop events remove PIDs from tracking. The loop breaks when pending items have accumulated long enough (approximately `interval_ms`).
18. **Polling sleep** — When ETW sleep is not applicable (thread-level tracking active, ETW disabled, or `-continuousProcessLevelApply`), falls back to a simple `thread::sleep` for `interval_ms`.
19. **Hot-reload** — Calls [`hotreload_config`](../config.rs/hotreload_config.md) and [`hotreload_blacklist`](../config.rs/hotreload_blacklist.md) to detect file modifications and reload when changed. On config reload, resets `process_level_applied` and triggers a full match on the next iteration.

### Phase 6 — Shutdown

20. **Stop ETW** — Calls `EtwProcessMonitor::stop()` to tear down the ETW trace session.

### Key state variables

| Variable | Type | Purpose |
|----------|------|---------|
| `process_level_applied` | `SmallVec<[u32; PIDS]>` | PIDs that have already received process-level config. Prevents redundant re-application. |
| `thread_level_applied` | `SmallVec<[u32; PENDING]>` | PIDs that received thread-level config in the current iteration. Cleared each loop. |
| `process_level_pending` | `SmallVec<[u32; PENDING]>` | PIDs received from ETW process start events awaiting application. |
| `full_process_level_match` | `bool` | When `true`, all processes are matched regardless of grade. Set on first loop and config reload. |
| `current_loop` | `u32` | Monotonically increasing loop counter. Used with grade-based modulo scheduling. |

### ETW sleep algorithm

The ETW-reactive sleep avoids fixed-interval polling when only process-level configs are active. Instead of sleeping for `interval_ms`, the loop waits on the ETW channel:

- On **timeout** with an empty pending list, continues waiting.
- On **timeout** with a non-empty pending list, breaks to process pending items.
- On **process start event**, adds the PID to pending; breaks when enough wall-clock time has elapsed.
- On **process stop event**, removes the PID from all tracking structures.
- On **channel disconnect**, sets `should_continue = false` (another instance may have taken over the ETW session).

This results in lower CPU usage during idle periods while maintaining fast reaction to new process launches.

## Requirements

| | |
|---|---|
| **Module** | `src/main.rs` |
| **Callers** | Rust runtime (`fn main`) |
| **Callees** | [`parse_args`](../cli.rs/parse_args.md), [`read_config`](../config.rs/read_config.md), [`apply_config`](apply_config.md), [`apply_thread_level`](apply_thread_level.md), [`process_find`](process_find.md), [`process_logs`](process_logs.md), [`log_apply_results`](log_apply_results.md), [`EtwProcessMonitor::start`](../event_trace.rs/EtwProcessMonitor.md), [`ProcessSnapshot::take`](../process.rs/ProcessSnapshot.md), [`hotreload_config`](../config.rs/hotreload_config.md), [`hotreload_blacklist`](../config.rs/hotreload_blacklist.md), [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md), [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md), [`set_timer_resolution`](../winapi.rs/set_timer_resolution.md), [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) |
| **Win32 API** | [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot) (via `ProcessSnapshot`), ETW via [`EVENT_TRACE_PROPERTIES`](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/) |
| **Privileges** | `SeDebugPrivilege`, `SeIncreaseBasePriorityPrivilege`, Administrator (via UAC) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [main.rs](README.md) |
| CLI arguments | [CliArgs](../cli.rs/CliArgs.md) |
| Configuration result | [ConfigResult](../config.rs/ConfigResult.md) |
| Apply engine | [apply.rs](../apply.rs/README.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| ETW process monitor | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*