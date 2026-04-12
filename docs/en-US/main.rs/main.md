# main function (main.rs)

Entry point for AffinityService. Orchestrates CLI parsing, privilege acquisition, configuration loading, UAC elevation, and the main scheduling loop with grade-based process configuration application and hot-reload support.

## Syntax

```rust
fn main() -> windows::core::Result<()>
```

## Parameters

This function takes no parameters. Command-line arguments are read internally via `std::env::args()`.

## Return value

Returns `Ok(())` on successful completion (either after all loop iterations complete or after dispatching to a sub-mode that exits cleanly). Returns a `windows::core::Error` if a fatal, unrecoverable Windows API failure occurs during initialization.

## Remarks

`main` is the top-level orchestrator that ties together all other modules. Its execution proceeds through several distinct phases:

### Phase 1: CLI Parsing

Command-line arguments are collected via `std::env::args()` and parsed into a [`CliArgs`](../cli.rs/CliArgs.md) struct by [`parse_args`](../cli.rs/parse_args.md). Based on the mode flags in `CliArgs`, execution is dispatched:

- `help_mode` → [`print_help`](../cli.rs/print_help.md), then exit.
- `help_all_mode` → [`print_help_all`](../cli.rs/print_help_all.md), then exit.
- `convert_mode` → [`convert`](../config.rs/ConfigResult.md) in config.rs, then exit.
- `autogroup_mode` → [`sort_and_group_config`](../config.rs/ConfigResult.md) in config.rs, then exit.
- `validate_mode` → parse and validate config, print results, then exit.
- `process_logs_mode` → [`process_logs`](process_logs.md), then exit.

If none of the early-exit modes are set, execution continues to the main loop setup.

### Phase 2: Privilege Acquisition

Unless suppressed by CLI flags:

1. [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md) is called to acquire `SeDebugPrivilege`, enabling access to processes owned by other users.
2. [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) is called to acquire `SeIncreaseBasePriorityPrivilege`, enabling `High` and `Realtime` priority class assignment.

These calls may silently fail if the process is not yet elevated — they will succeed after UAC elevation in the next phase.

### Phase 3: Configuration Loading

The configuration file specified in [`CliArgs.config_file_name`](../cli.rs/CliArgs.md) is parsed via [`read_config`](../config.rs/ConfigResult.md), producing a [`ConfigResult`](../config.rs/ConfigResult.md) containing the process rules, constants, and any parse errors/warnings. Errors and warnings are logged.

An optional blacklist file is also loaded if specified, providing a list of process names to exclude from configuration application.

### Phase 4: UAC Elevation

If [`is_running_as_admin`](../winapi.rs/is_running_as_admin.md) returns `false` and the `--no-uac` flag is not set:

1. If `skip_log_before_elevation` is set, [`DUST_BIN_MODE`](../logging.rs/DUST_BIN_MODE.md) is enabled to suppress logging from the non-elevated instance.
2. [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md) spawns a new elevated instance of the application.
3. [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) cleans up orphaned console host processes.
4. The non-elevated instance exits.

The elevated instance starts fresh from the beginning, but this time [`is_running_as_admin`](../winapi.rs/is_running_as_admin.md) returns `true` and the elevation phase is skipped.

### Phase 5: Main Loop

The main loop runs indefinitely (or for `loop_count` iterations if specified) with each iteration performing:

1. **Timestamp update** — [`LOCAL_TIME_BUFFER`](../logging.rs/LOCAL_TIME_BUFFER.md) is refreshed for consistent log timestamps within the iteration.
2. **Process snapshot** — A [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md) is taken, enumerating all running processes and their threads.
3. **Process matching** — Each running process is matched against configuration rules. Matched processes are scheduled for configuration application based on their grade.
4. **Grade-based scheduling** — Grade 1 processes are applied every iteration, grade 2 every 2nd iteration, up to grade 5 every 5th iteration. This reduces overhead for lower-priority rules while keeping high-priority rules responsive.
5. **Configuration application** — For each scheduled process, [`apply_config`](apply_config.md) is called to apply all configured attributes.
6. **Find logging** — Discovered process names are logged via [`log_process_find`](../logging.rs/log_process_find.md) to the `.find.log` file.
7. **Fail map purge** — [`purge_fail_map`](../logging.rs/purge_fail_map.md) removes error deduplication entries for processes that are no longer alive.
8. **Module cache cleanup** — [`drop_module_cache`](../winapi.rs/drop_module_cache.md) is called for terminated processes.
9. **Config hot reload** — The modification timestamp of the configuration file is checked. If it has changed since last load, [`read_config`](../config.rs/ConfigResult.md) is called again to reload the configuration without restarting the service.
10. **Sleep** — The loop sleeps for `interval_ms` milliseconds before the next iteration.

### Grade-Based Scheduling

The grade system provides a lightweight priority mechanism within the main loop:

| Grade | Apply frequency | Typical use |
| --- | --- | --- |
| 1 | Every iteration | High-priority, latency-sensitive processes |
| 2 | Every 2nd iteration | Important but less time-critical processes |
| 3 | Every 3rd iteration | Background processes |
| 4 | Every 4th iteration | Low-priority processes |
| 5 | Every 5th iteration | Rarely-changing system processes |

The grade is determined by the configuration and allows the service to handle hundreds of process rules without excessive per-iteration overhead.

### Config Hot Reload

Each iteration, the modification time of the configuration file is compared to the last-known value. If the file has been modified:

1. The file is re-parsed via [`read_config`](../config.rs/ConfigResult.md).
2. Parse errors and warnings are logged.
3. The active configuration is atomically replaced with the new one.
4. The next iteration uses the updated rules.

This allows live tuning of process configurations without stopping and restarting the service.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/main.rs |
| **Source lines** | L196–L456 |
| **Key dependencies** | [`CliArgs`](../cli.rs/CliArgs.md), [`ProcessConfig`](../config.rs/ProcessConfig.md), [`ConfigResult`](../config.rs/ConfigResult.md), [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md), [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md), [`ProcessHandle`](../winapi.rs/ProcessHandle.md), [`ProcessEntry`](../process.rs/ProcessEntry.md) |
| **Calls (init)** | [`parse_args`](../cli.rs/parse_args.md), [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md), [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md), [`is_running_as_admin`](../winapi.rs/is_running_as_admin.md), [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md), [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md), [`read_config`](../config.rs/ConfigResult.md) |
| **Calls (loop)** | [`apply_config`](apply_config.md), [`get_process_handle`](../winapi.rs/get_process_handle.md), [`purge_fail_map`](../logging.rs/purge_fail_map.md), [`drop_module_cache`](../winapi.rs/drop_module_cache.md), [`log_process_find`](../logging.rs/log_process_find.md) |

## See also

- [apply_config function](apply_config.md)
- [process_logs function](process_logs.md)
- [CliArgs struct](../cli.rs/CliArgs.md)
- [main.rs module overview](README.md)