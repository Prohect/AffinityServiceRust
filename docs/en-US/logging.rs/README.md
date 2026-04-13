# logging module (AffinityServiceRust)

The `logging` module provides all logging infrastructure for AffinityServiceRust, including timestamped file and console output, find-mode process discovery logging, and a deduplication system for error reporting. It manages log file creation with date-based naming, maintains global state for log routing (console vs. file, dust-bin suppression), and tracks per-PID operation failures so that repeated errors are logged only once per session. The module exposes both direct logging functions and convenience macros for lock-guarded access to its global statics.

## Statics

| Static | Description |
|--------|-------------|
| [FINDS_SET](FINDS_SET.md) | Deduplication set of process names already logged in `-find` mode during the current session. |
| [USE_CONSOLE](USE_CONSOLE.md) | Flag controlling whether log output goes to the console (`true`) or to a log file (`false`). |
| [DUST_BIN_MODE](DUST_BIN_MODE.md) | Flag that suppresses all logging when `true`; used before UAC elevation to avoid writing to files the unprivileged process cannot own. |
| [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) | Cached `DateTime<Local>` used for log timestamps and date-based log file naming. |
| [LOG_FILE](LOG_FILE.md) | Main log file handle, opened in append mode at `logs/YYYYMMDD.log`. |
| [FIND_LOG_FILE](FIND_LOG_FILE.md) | Find-mode log file handle, opened in append mode at `logs/YYYYMMDD.find.log`. |
| [FINDS_FAIL_SET](FINDS_FAIL_SET.md) | Deduplication set for failed find operations, preventing repeated logging of the same failure. |
| [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) | Per-PID map of [ApplyFailEntry](ApplyFailEntry.md) records used to deduplicate Windows API operation errors. |

## Macros

| Macro | Description |
|-------|-------------|
| [log!](log.md) | Formats arguments and delegates to [log_message](log_message.md) with a timestamp prefix. |
| [get_use_console!](get_use_console.md) | Returns a `MutexGuard<bool>` for the [USE_CONSOLE](USE_CONSOLE.md) flag. |
| [get_dust_bin_mod!](get_dust_bin_mod.md) | Returns a `MutexGuard<bool>` for the [DUST_BIN_MODE](DUST_BIN_MODE.md) flag. |
| [get_local_time!](get_local_time.md) | Returns a `MutexGuard<DateTime<Local>>` for the [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md). |
| [get_logger!](get_logger.md) | Returns a `MutexGuard<File>` for the [LOG_FILE](LOG_FILE.md) handle. |
| [get_logger_find!](get_logger_find.md) | Returns a `MutexGuard<File>` for the [FIND_LOG_FILE](FIND_LOG_FILE.md) handle. |
| [get_fail_find_set!](get_fail_find_set.md) | Returns a `MutexGuard<HashSet<String>>` for the [FINDS_FAIL_SET](FINDS_FAIL_SET.md). |
| [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) | Returns a `MutexGuard<HashMap<u32, HashMap<ApplyFailEntry, bool>>>` for the [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md). |

## Enums

| Enum | Description |
|------|-------------|
| [Operation](Operation.md) | Identifies each Windows API operation that can fail during rule application, used as a key in failure deduplication. |

## Structs

| Struct | Description |
|--------|-------------|
| [ApplyFailEntry](ApplyFailEntry.md) | Composite key for failure deduplication: thread ID, process name, operation, and error code. |

## Functions

| Function | Description |
|----------|-------------|
| [is_new_error](is_new_error.md) | Returns `true` if this PID/operation/error combination has not been seen before, registering it for future deduplication. |
| [purge_fail_map](purge_fail_map.md) | Removes stale entries from the failure tracking map for processes that are no longer running. |
| [get_log_path](get_log_path.md) | Builds a date-prefixed log file path (`logs/YYYYMMDD<suffix>.log`). |
| [log_message](log_message.md) | Writes a `[HH:MM:SS]` timestamped message to the console or log file, respecting dust-bin mode. |
| [log_pure_message](log_pure_message.md) | Writes a message without a timestamp prefix to the console or log file. |
| [log_to_find](log_to_find.md) | Writes a timestamped message to the find-mode log file (or console). |
| [log_process_find](log_process_find.md) | Logs a discovered process in `-find` mode, deduplicated per session via [FINDS_SET](FINDS_SET.md). |

## See Also

| Topic | Link |
|-------|------|
| Error code translation used in log messages | [error_codes module](../error_codes.rs/README.md) |
| Rule application that generates operation errors | [apply module](../apply.rs/README.md) |
| Process priority / IO priority / memory priority enums | [priority module](../priority.rs/README.md) |
| Service main loop and find mode entry point | [main module](../main.rs/README.md) |
| CLI flags that control logging behavior | [cli module](../cli.rs/README.md) |