# logging module (AffinityServiceRust)

The `logging` module provides the logging infrastructure for AffinityServiceRust, including date-based file rotation, optional console output, find-mode discovery logging, and error deduplication. All log output is gated by a dust-bin mode flag (which suppresses logging entirely during UAC elevation) and a console flag (which redirects output to stdout instead of files). Log files are written to the `logs/` directory with date-stamped filenames in the format `YYYYMMDD.log` and `YYYYMMDD.find.log`.

## Statics

| Name | Type | Description |
|------|------|-------------|
| `FINDS_SET` | `Lazy<Mutex<HashSet<String>>>` | Deduplication set for find-mode process discovery. Each process name is logged at most once per session. |
| `USE_CONSOLE` | `Lazy<Mutex<bool>>` | When `true`, all log output is written to stdout instead of log files. Used in CLI/interactive mode. |
| `DUST_BIN_MODE` | `Lazy<Mutex<bool>>` | When `true`, suppresses all logging. Enabled during UAC elevation to avoid writing to files from a transient elevated process. |
| `LOCAL_TIME_BUFFER` | `Lazy<Mutex<DateTime<Local>>>` | Cached local timestamp, updated externally each tick. Used for log prefixes and date-based file rotation. |
| `LOG_FILE` | `Lazy<Mutex<File>>` | Main log file handle, opened in append mode at `logs/YYYYMMDD.log`. |
| `FIND_LOG_FILE` | `Lazy<Mutex<File>>` | Find-mode log file handle, opened in append mode at `logs/YYYYMMDD.find.log`. |
| `FINDS_FAIL_SET` | `Lazy<Mutex<HashSet<String>>>` | Error deduplication tracking for find-mode failures. |
| `PID_MAP_FAIL_ENTRY_SET` | `Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>` | Per-PID map of [ApplyFailEntry](ApplyFailEntry.md) to alive flag. Used by [is_new_error](is_new_error.md) and [purge_fail_map](purge_fail_map.md) to deduplicate error logging. |

## Macros

| Name | Description |
|------|-------------|
| `log!()` | Formats arguments and delegates to [log_message](log_message.md). Usage: `log!("value: {}", x)`. |
| `get_use_console!()` | Locks and returns the `USE_CONSOLE` mutex guard. |
| `get_dust_bin_mod!()` | Locks and returns the `DUST_BIN_MODE` mutex guard. |
| `get_local_time!()` | Locks and returns the `LOCAL_TIME_BUFFER` mutex guard. |
| `get_logger!()` | Locks and returns the `LOG_FILE` mutex guard. |
| `get_logger_find!()` | Locks and returns the `FIND_LOG_FILE` mutex guard. |
| `get_fail_find_set!()` | Locks and returns the `FINDS_FAIL_SET` mutex guard. |
| `get_pid_map_fail_entry_set!()` | Locks and returns the `PID_MAP_FAIL_ENTRY_SET` mutex guard. |

## Enums

| Name | Description |
|------|-------------|
| [Operation](Operation.md) | Enumerates all Windows API operations the service performs, used as keys for error deduplication. |

## Structs

| Name | Description |
|------|-------------|
| [ApplyFailEntry](ApplyFailEntry.md) | Composite key representing a unique failure: thread ID, process name, operation, and error code. |

## Functions

| Name | Description |
|------|-------------|
| [is_new_error](is_new_error.md) | Returns `true` if a failure combination has not been seen before for a given PID. Tracks per-PID error history. |
| [purge_fail_map](purge_fail_map.md) | Removes stale entries from the error deduplication map, retaining only currently running processes. |
| [get_log_path](get_log_path.md) | Builds a date-stamped log file path under the `logs/` directory. |
| [log_message](log_message.md) | Writes a timestamped `[HH:MM:SS]msg` line to the log file or console. |
| [log_pure_message](log_pure_message.md) | Writes a line without a timestamp prefix to the log file or console. |
| [log_to_find](log_to_find.md) | Writes a timestamped line to the find-mode log file or console. |
| [log_process_find](log_process_find.md) | Logs a discovered process name in find mode, deduplicated per session. |

## See Also

| Topic | Link |
|-------|------|
| Apply module (primary consumer) | [apply.rs](../apply.rs/README.md) |
| Error logging helper | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| Configuration types | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) |
| Main service loop | [main.rs](../main.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*