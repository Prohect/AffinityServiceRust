# get_log_path function (logging.rs)

Constructs a date-stamped log file path under the `logs/` directory, creating the directory if it does not exist.

## Syntax

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

## Parameters

`suffix: &str`

A string inserted between the date stamp and the `.log` extension in the filename. Pass `""` for the main log file or `".find"` for the find-mode log file.

| Suffix value | Resulting filename |
|---|---|
| `""` | `logs/YYYYMMDD.log` |
| `".find"` | `logs/YYYYMMDD.find.log` |

## Return value

`PathBuf` — The fully constructed path to the log file, relative to the service's working directory. The path follows the pattern `logs/YYYYMMDD{suffix}.log` where `YYYYMMDD` is the current local date obtained from [LOCAL_TIME_BUFFER](README.md).

## Remarks

- The function reads the current date from the `LOCAL_TIME_BUFFER` static via the `get_local_time!()` macro, then immediately drops the lock before performing filesystem operations.
- If the `logs/` directory does not exist, the function creates it (and any necessary parents) via `std::fs::create_dir_all`. Directory creation failures are silently ignored; the caller will encounter the error when attempting to open the file.
- This function is **not** `pub` — it is module-private and called only during lazy initialization of the [LOG_FILE](README.md) and [FIND_LOG_FILE](README.md) statics.
- Because `LOG_FILE` and `FIND_LOG_FILE` are lazily initialized once per process lifetime, the log file date is determined at first use. The service does **not** rotate to a new file at midnight; a restart is required to begin writing to a new date's log file.

### Date format

The date portion uses zero-padded four-digit year, two-digit month, and two-digit day with no separators: `20250115` for January 15, 2025.

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Visibility** | Private (module-internal) |
| **Callers** | `LOG_FILE` static initializer, `FIND_LOG_FILE` static initializer |
| **Dependencies** | `LOCAL_TIME_BUFFER`, `chrono::Datelike`, `std::fs::create_dir_all` |
| **Privileges** | Filesystem write access to the working directory |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Main log output | [log_message](log_message.md) |
| Find log output | [log_to_find](log_to_find.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*