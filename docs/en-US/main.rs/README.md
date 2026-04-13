# main.rs Module (main.rs)

The `main` module is the entry point and top-level orchestrator for AffinityService. It handles CLI parsing, privilege escalation, configuration loading, ETW-based reactive process detection, the main scheduling loop, and hot-reload of configuration files.

## Overview

This module ties together all other modules to implement the service's core loop:

1. Parse command-line arguments via [`parse_args`](../cli.rs/parse_args.md)
2. Acquire privileges ([`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md), [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md))
3. Load configuration via [`read_config`](../config.rs/ConfigResult.md)
4. Request UAC elevation if needed via [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md)
5. Start ETW process monitor via [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) for reactive process detection
6. Enter the main loop, applying configurations to matched processes each iteration
7. Support grade-based scheduling and hot-reload of config files

### Two-level Application

Configuration application is split into two levels:

- **Process-level** ([`apply_config_process_level`](apply_config_process_level.md)) — One-shot settings (priority, affinity, CPU set, IO priority, memory priority) applied once per process. Tracked via `process_level_applied: HashSet<u32>`.
- **Thread-level** ([`apply_config_thread_level`](apply_config_thread_level.md)) — Per-iteration settings (prime thread scheduling, ideal processor assignment) applied every loop.

### Reactive Process Detection

ETW events from [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) enable immediate rule application:
- **Process start** → PID added to `process_level_pending` → process-level rules applied on next snapshot, bypassing grade scheduling
- **Process stop** → PID cleaned from scheduler, error tracking, and applied sets

## Items

### Functions

| Name | Description |
| --- | --- |
| [apply_config_process_level](apply_config_process_level.md) | Applies process-level settings (one-shot): priority, affinity, CPU set, IO/memory priority. |
| [apply_config_thread_level](apply_config_thread_level.md) | Applies thread-level settings (every iteration): prime scheduling, ideal processors, cycle tracking. |
| [process_find](process_find.md) | Scans for unmanaged processes in `-find` mode. |
| [process_logs](process_logs.md) | Processes `.find.log` files to discover executable paths using `es.exe` (Everything). |
| [main](main.md) | Entry point — CLI, privileges, config, UAC, ETW, main loop. |

## Execution Flow

1. **CLI parsing** — [`parse_args`](../cli.rs/parse_args.md) populates a [`CliArgs`](../cli.rs/CliArgs.md) struct.
2. **Mode dispatch** — help, convert, validate, autogroup, process-logs modes exit early.
3. **Privileges** — [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md) and [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) are called unless suppressed.
4. **Config loading** — [`read_config`](../config.rs/ConfigResult.md) parses the configuration file.
5. **UAC elevation** — if not admin and not suppressed, [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md) re-launches elevated.
6. **Timer resolution** — [`set_timer_resolution`](../winapi.rs/set_timer_resolution.md) if configured.
7. **Cleanup** — [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) removes inherited children.
8. **ETW start** — [`EtwProcessMonitor::start()`](../event_trace.rs/EtwProcessMonitor.md) begins reactive monitoring (falls back to polling if unavailable).
9. **Main loop** — each iteration:
   - Snapshot processes via [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md)
   - Drain ETW events: start events → `process_level_pending`, stop events → cleanup
   - Apply process-level rules from `process_level_pending` immediately
   - Grade-based iteration: apply process-level (if not yet applied) and thread-level rules
   - Find mode scanning via [`process_find`](process_find.md)
   - Hot-reload via [`hotreload_config`](../config.rs/hotreload_config.md) and [`hotreload_blacklist`](../config.rs/hotreload_blacklist.md)
   - Sleep for configured interval
10. **ETW stop** — Clean shutdown of ETW monitor.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/main.rs` |
| **Key dependencies** | [`CliArgs`](../cli.rs/CliArgs.md), [`ProcessConfig`](../config.rs/ProcessConfig.md), [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md), [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md), [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) |