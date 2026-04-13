# logging.rs Module (logging.rs)

The `logging` module provides centralized logging infrastructure, error deduplication, and process-find tracking for the application. It manages log files, console output, and maintains state to suppress duplicate error messages across loop iterations.

## Overview

This module handles all logging output for the application through several mechanisms:

- **General logging** — [`log_message`](log_message.md) writes timestamped messages to the log file and optionally to the console.
- **Pure logging** — [`log_pure_message`](log_pure_message.md) writes messages without timestamp prefix.
- **Find logging** — [`log_to_find`](log_to_find.md) and [`log_process_find`](log_process_find.md) write to a separate `.find.log` file for process discovery tracking.
- **Error deduplication** — [`is_new_error`](is_new_error.md) prevents the same error from being logged repeatedly across loop iterations.

The `log!` macro is the primary logging interface used throughout the codebase, which delegates to [`log_message`](log_message.md).

## Items

### Statics

| Name | Description |
| --- | --- |
| [FINDS_SET](FINDS_SET.md) | Tracks process names already logged to the find log to avoid duplicates. |
| [USE_CONSOLE](USE_CONSOLE.md) | Controls whether log output is also written to the console. |
| [DUST_BIN_MODE](DUST_BIN_MODE.md) | Suppresses logging before UAC elevation to avoid writing to a log file that will be abandoned. |
| [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) | Caches the current local time to ensure consistent timestamps within a single loop iteration. |
| [LOG_FILE](LOG_FILE.md) | The main log file handle. |
| [FIND_LOG_FILE](FIND_LOG_FILE.md) | The find log file handle for process discovery output. |
| [FINDS_FAIL_SET](FINDS_FAIL_SET.md) | Tracks process names that failed to be found, for deduplication. |
| [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) | Per-PID map of deduplicated error entries, keyed by (tid, name, operation, error_code), outer map keyed by PID. |

### Enums

| Name | Description |
| --- | --- |
| [Operation](Operation.md) | Enumerates all Windows API operations that can produce errors during configuration application. |

### Structs

| Name | Description |
| --- | --- |
| [ApplyFailEntry](ApplyFailEntry.md) | Composite key for error deduplication, combining tid, process name, operation, and error code. |

### Functions

| Name | Description |
| --- | --- |
| [is_new_error](is_new_error.md) | Checks whether an error has already been logged for the given pid/tid/operation/error combination. |
| [get_log_path](get_log_path.md) | Constructs the log file path with the given suffix next to the executable. |
| [log_message](log_message.md) | Writes a timestamped log message to the log file and optionally to the console. |
| [log_pure_message](log_pure_message.md) | Writes a log message without timestamp prefix. |
| [log_to_find](log_to_find.md) | Writes a message to the `.find.log` file. |
| [log_process_find](log_process_find.md) | Logs a discovered process name to the find log, with deduplication. |

## Error Deduplication

The deduplication system prevents log spam when the same error occurs every loop iteration for the same process/thread/operation combination:

1. Each error is represented as an [`ApplyFailEntry`](ApplyFailEntry.md) keyed by `(tid, process_name, operation, error_code)`.
2. The [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) stores these entries in a two-level map: PID → set of fail entries.
3. [`is_new_error`](is_new_error.md) returns `true` only the first time a particular error combination is seen.
4. Process exit cleanup is handled reactively via ETW events — when a process stops, its entries are removed directly from the map by the main loop.

## Dust Bin Mode

When [`DUST_BIN_MODE`](DUST_BIN_MODE.md) is enabled, log output is suppressed. This is used during the pre-UAC-elevation phase: since the process will be relaunched with elevated privileges, any log output written before elevation would go to a log file that is immediately abandoned. The `skip_log_before_elevation` CLI flag controls this behavior.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/logging.rs` |
| **Called by** | All modules via `log!` macro; [`apply_config_process_level`](../apply.rs/apply_config_process_level.md)/[`apply_config_thread_level`](../apply.rs/apply_config_thread_level.md) indirectly via apply functions |
| **Key dependencies** | [`Operation`](Operation.md), [`ApplyFailEntry`](ApplyFailEntry.md), `chrono::Local`, `once_cell::sync::Lazy` |