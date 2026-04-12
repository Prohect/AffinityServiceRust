# main.rs Module (main.rs)

The `main` module is the entry point and top-level orchestrator for AffinityService. It handles CLI parsing, privilege escalation, configuration loading, the main scheduling loop, and hot-reload of configuration files.

## Overview

This module ties together all other modules to implement the service's core loop:

1. Parse command-line arguments via [`parse_args`](../cli.rs/parse_args.md)
2. Acquire privileges ([`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md), [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md))
3. Load configuration via [`read_config`](../config.rs/ConfigResult.md)
4. Request UAC elevation if needed via [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md)
5. Enter the main loop, applying configurations to matched processes each iteration
6. Support grade-based scheduling and hot-reload of config files

The main loop uses **grade-based scheduling**: grade 1 processes are applied every loop iteration, grade 5 processes every 5th iteration, reducing overhead for lower-priority rules.

**Config hot reload** checks file modification times each loop iteration and re-parses the configuration file when changes are detected, allowing live tuning without restarting the service.

## Items

### Functions

| Name | Description |
| --- | --- |
| [apply_config](apply_config.md) | Orchestrates all configuration application steps for a single process. |
| [process_logs](process_logs.md) | Processes `.find.log` files to discover executable paths using `es.exe` (Everything). |
| [main](main.md) | Entry point — CLI, privileges, config, UAC, main loop. |

## Execution Flow

A typical run proceeds as follows:

1. **CLI parsing** — [`parse_args`](../cli.rs/parse_args.md) populates a [`CliArgs`](../cli.rs/CliArgs.md) struct.
2. **Mode dispatch** — help, convert, validate, autogroup, find, and process-logs modes exit early.
3. **Privileges** — [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md) and [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) are called unless suppressed by CLI flags.
4. **Config loading** — [`read_config`](../config.rs/ConfigResult.md) parses the configuration file.
5. **UAC elevation** — if not already admin and `--no-uac` is not set, [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md) re-launches elevated and [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) cleans up.
6. **Main loop** — each iteration:
   - Snapshots running processes via [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md)
   - Matches processes against config rules
   - Calls [`apply_config`](apply_config.md) for each matched process
   - Checks config file modification times for hot reload
   - Sleeps for the configured interval
7. **Logging** — changes and errors are emitted via the [`log!`](../logging.rs/log_message.md) macro.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/main.rs` |
| **Key dependencies** | [`CliArgs`](../cli.rs/CliArgs.md), [`ProcessConfig`](../config.rs/ProcessConfig.md), [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md), [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md), [`ProcessHandle`](../winapi.rs/ProcessHandle.md) |