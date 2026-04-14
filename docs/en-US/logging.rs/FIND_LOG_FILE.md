# FIND_LOG_FILE static (logging.rs)

Global mutex-guarded file handle for the find-mode log file. This static holds the `File` opened in append mode at the path `logs/YYYYMMDD.find.log`, where the date prefix is determined at initialization time from [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md). All find-mode log output written by [log_to_find](log_to_find.md) and [log_process_find](log_process_find.md) is directed to this file when console mode is not active.

## Syntax

```logging.rs
pub static FIND_LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. The file is opened on first access. |
| Inner | `Mutex<File>` | Provides synchronized write access to the underlying file handle from any thread. |

## Remarks

- The file is opened with `OpenOptions::new().append(true).create(true)`, which creates the file if it does not exist and positions all writes at the end if it does. This ensures that log entries are never lost due to overwrites, even across service restarts on the same day.
- The file path is constructed by [get_log_path](get_log_path.md) with the suffix `".find"`, producing paths of the form `logs/YYYYMMDD.find.log`. The date component is derived from the [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) value at initialization time; if the service runs past midnight, the find log continues writing to the file named after the startup date.
- The `unwrap()` in the initializer will panic if the `logs/` directory cannot be created or the file cannot be opened. In practice, [get_log_path](get_log_path.md) calls `create_dir_all` to ensure the directory exists before the file is opened.
- The [get_logger_find!](get_logger_find.md) macro provides a convenient shorthand for `FIND_LOG_FILE.lock().unwrap()`, returning a `MutexGuard<File>`.
- This file handle is separate from [LOG_FILE](LOG_FILE.md), which serves the main operational log. The separation allows `-find` mode discovery output to be reviewed independently without commingling with standard service diagnostics.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex`, `std::fs::File`, `std::fs::OpenOptions` |
| Initialized by | `Lazy` on first access (typically the first call to [log_to_find](log_to_find.md)) |
| Accessor macro | [get_logger_find!](get_logger_find.md) |
| Path builder | [get_log_path](get_log_path.md) |

## See Also

| Topic | Link |
|-------|------|
| Main log file handle | [LOG_FILE](LOG_FILE.md) |
| Find-mode logging function | [log_to_find](log_to_find.md) |
| Deduplicated process discovery logging | [log_process_find](log_process_find.md) |
| Log path construction | [get_log_path](get_log_path.md) |
| Cached local time for date prefix | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| Console vs. file routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd