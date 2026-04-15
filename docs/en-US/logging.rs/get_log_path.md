# get_log_path function (logging.rs)

Builds a date-stamped log file path under the `logs/` directory. The function reads the current local time from the [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer) static to construct a filename in the format `YYYYMMDD<suffix>.log`, and ensures the `logs/` directory exists before returning the path.

## Syntax

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `suffix` | `&str` | A string appended to the date portion of the filename before the `.log` extension. Pass `""` for the main log file, or `".find"` for the find-mode log file. |

## Return value

Returns a `PathBuf` pointing to the log file. The path is relative to the working directory and takes the form `logs/YYYYMMDD<suffix>.log`.

### Examples

| `suffix` | Date | Resulting path |
|----------|------|----------------|
| `""` | 2025-01-15 | `logs/20250115.log` |
| `".find"` | 2025-01-15 | `logs/20250115.find.log` |

## Remarks

- This function is **module-private** (`fn` without `pub`). It is called during `Lazy` initialization of the [`LOG_FILE`](statics.md#log_file) and [`FIND_LOG_FILE`](statics.md#find_log_file) statics and is not accessible outside the `logging` module.

- The function locks the [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer) mutex to read the cached local time. The lock is explicitly dropped (via `drop(time)`) before proceeding with directory creation, minimizing lock hold time.

- The `logs/` directory is created via `std::fs::create_dir_all` if it does not already exist. If directory creation fails, the error is silently ignored (`let _ = create_dir_all(...)`) and the returned path may point to a non-existent directory. Subsequent file-open operations using this path will fail at that point.

- The date components are extracted using `chrono::Datelike` trait methods (`year()`, `month()`, `day()`) and formatted with zero-padding to ensure consistent 8-digit date strings.

### Algorithm

1. Lock the `LOCAL_TIME_BUFFER` mutex and extract `(year, month, day)`.
2. Drop the lock.
3. Construct a `PathBuf` for the `logs/` directory.
4. If the directory does not exist, attempt to create it (including parent directories).
5. Join the directory path with the formatted filename `YYYYMMDD<suffix>.log`.
6. Return the resulting `PathBuf`.

### Call sites

This function is called exactly twice during program initialization:

- `Lazy` initializer for [`LOG_FILE`](statics.md#log_file): `get_log_path("")`
- `Lazy` initializer for [`FIND_LOG_FILE`](statics.md#find_log_file): `get_log_path(".find")`

Because these statics are lazily initialized, `get_log_path` is not called until the first log message is written or the first find-log entry is recorded.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Visibility** | Private (module-internal) |
| **Callers** | `LOG_FILE` and `FIND_LOG_FILE` `Lazy` initializers |
| **Callees** | `LOCAL_TIME_BUFFER.lock()`, `chrono::Datelike` methods, `std::fs::create_dir_all`, `PathBuf::join` |
| **Dependencies** | `chrono`, `std::fs`, `std::path::PathBuf` |
| **Platform** | Cross-platform (no Windows-specific API calls) |

## See Also

| Topic | Link |
|-------|------|
| log_message function | [log_message](log_message.md) |
| log_to_find function | [log_to_find](log_to_find.md) |
| LOG_FILE static | [statics](statics.md#log_file) |
| FIND_LOG_FILE static | [statics](statics.md#find_log_file) |
| LOCAL_TIME_BUFFER static | [statics](statics.md#local_time_buffer) |
| logging module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
