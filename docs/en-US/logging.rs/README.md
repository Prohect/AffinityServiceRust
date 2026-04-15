# logging module (AffinityServiceRust)

The `logging` module provides file-based and console-based logging facilities for AffinityServiceRust. It manages daily rotating log files, deduplicates process-find log entries, and tracks per-PID operation failures to suppress repeated error messages. The module exposes global static state for log configuration (console mode, dust-bin mode, time buffers, file handles) and helper macros for convenient access.

## Functions

| Function | Description |
|----------|-------------|
| [is_new_error](is_new_error.md) | Tracks operation failures per PID/TID/process/operation/error-code tuple; returns `true` only on the first occurrence. |
| [purge_fail_map](purge_fail_map.md) | Removes stale entries from the failure-tracking map based on a list of currently running processes. |
| [get_log_path](get_log_path.md) | Builds a date-stamped log file path under the `logs/` directory with an optional suffix. |
| [log_message](log_message.md) | Writes a timestamped message to the main log file or stdout, respecting dust-bin mode. |
| [log_pure_message](log_pure_message.md) | Writes a message without a timestamp prefix to the main log file or stdout. |
| [log_to_find](log_to_find.md) | Writes a timestamped message to the `.find` log file or stdout. |
| [log_process_find](log_process_find.md) | Logs a discovered process name, deduplicated per session via `FINDS_SET`. |

## Structs / Enums

| Item | Description |
|------|-------------|
| [Operation](Operation.md) | Enum of Windows API operations whose failures are tracked by `is_new_error`. |
| [ApplyFailEntry](ApplyFailEntry.md) | Composite key struct representing a unique failure event (TID, process name, operation, error code). |

## Statics

| Static | Description |
|--------|-------------|
| [FINDS_SET](statics.md#finds_set) | `Lazy<Mutex<HashSet<String>>>` — set of process names already logged by `log_process_find` this session. |
| [USE_CONSOLE](statics.md#use_console) | `Lazy<Mutex<bool>>` — when `true`, all logging goes to stdout instead of files. |
| [DUST_BIN_MODE](statics.md#dust_bin_mode) | `Lazy<Mutex<bool>>` — when `true`, `log_message` discards output silently. |
| [LOCAL_TIME_BUFFER](statics.md#local_time_buffer) | `Lazy<Mutex<DateTime<Local>>>` — cached local time used for timestamp formatting. |
| [LOG_FILE](statics.md#log_file) | `Lazy<Mutex<File>>` — handle to the main daily log file (append mode). |
| [FIND_LOG_FILE](statics.md#find_log_file) | `Lazy<Mutex<File>>` — handle to the `.find` daily log file (append mode). |
| [FINDS_FAIL_SET](statics.md#finds_fail_set) | `Lazy<Mutex<HashSet<String>>>` — set of process names that failed `-find` mode access checks. |
| [PID_MAP_FAIL_ENTRY_SET](statics.md#pid_map_fail_entry_set) | `Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>` — per-PID failure tracking map with alive flags. |

## Macros

| Macro | Description |
|-------|-------------|
| `log!` | Convenience wrapper around `log_message` that accepts `format!`-style arguments. |
| `get_use_console!` | Locks and returns the `USE_CONSOLE` mutex guard. |
| `get_dust_bin_mod!` | Locks and returns the `DUST_BIN_MODE` mutex guard. |
| `get_local_time!` | Locks and returns the `LOCAL_TIME_BUFFER` mutex guard. |
| `get_logger!` | Locks and returns the `LOG_FILE` mutex guard. |
| `get_logger_find!` | Locks and returns the `FIND_LOG_FILE` mutex guard. |
| `get_fail_find_set!` | Locks and returns the `FINDS_FAIL_SET` mutex guard. |
| `get_pid_map_fail_entry_set!` | Locks and returns the `PID_MAP_FAIL_ENTRY_SET` mutex guard. |

## See Also

| Link | Description |
|------|-------------|
| [collections module](../collections.rs/README.md) | Custom `HashMap` and `HashSet` type aliases used by this module. |
| [error_codes module](../error_codes.rs/README.md) | Win32/NTSTATUS error code translation used alongside logging. |
| [winapi module](../winapi.rs/README.md) | Windows API wrappers that call into logging for error reporting. |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
