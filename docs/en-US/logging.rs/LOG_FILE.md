# LOG_FILE static (logging.rs)

Global mutex-guarded file handle for the main log file. All timestamped log output produced by [log_message](log_message.md) and untimestamped output from [log_pure_message](log_pure_message.md) is written to this file when console mode is not active. The file is opened in append mode on first access, creating both the `logs/` directory and the file itself if they do not already exist.

## Syntax

```logging.rs
pub static LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. The file is opened on first access. |
| Inner | `Mutex<File>` | Provides synchronized write access to the underlying `std::fs::File` handle from any thread. |

## Remarks

- The file path is determined by [get_log_path](get_log_path.md) with an empty suffix, yielding `logs/YYYYMMDD.log` based on the date cached in [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) at the time of first access.
- The file is opened with `OpenOptions::new().append(true).create(true)`, meaning:
  - If the file already exists, new output is appended to the end.
  - If the file does not exist, it is created.
- Because the `Lazy` initializer calls `.unwrap()` on the `open` result, a failure to create or open the log file will panic on first access. In practice, this only occurs if the process lacks write permission to its working directory or if the disk is full.
- The log file handle is never explicitly closed during the lifetime of the process. It is held open until process exit, at which point the OS reclaims the handle. This avoids the overhead of repeated open/close cycles during high-frequency logging.
- The file handle does **not** rotate automatically at midnight. The date in the filename is fixed at the time the static is first initialized. Log rotation across days is handled by the service's restart or re-elevation logic, which creates a new process with a fresh `Lazy` initialization.
- The [get_logger!](get_logger.md) macro provides a convenient shorthand for `LOG_FILE.lock().unwrap()`, returning a `MutexGuard<File>` that can be written to directly.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex`, `std::fs::File`, `std::fs::OpenOptions` |
| Initialization dependency | [get_log_path](get_log_path.md), [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| Writers | [log_message](log_message.md), [log_pure_message](log_pure_message.md) |
| Accessor macro | [get_logger!](get_logger.md) |

## See Also

| Topic | Link |
|-------|------|
| Find-mode log file handle | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Log path construction | [get_log_path](get_log_path.md) |
| Timestamped log writing | [log_message](log_message.md) |
| Raw log writing (no timestamp) | [log_pure_message](log_pure_message.md) |
| Console vs. file routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| Log suppression flag | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Cached timestamp for file naming | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd