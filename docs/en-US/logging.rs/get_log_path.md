# get_log_path function (logging.rs)

Constructs a date-prefixed log file path under the `logs/` directory. The function reads the cached local time from [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) to derive the `YYYYMMDD` date component and appends an optional suffix before the `.log` extension. If the `logs/` directory does not exist, it is created automatically.

## Syntax

```logging.rs
fn get_log_path(suffix: &str) -> PathBuf
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `suffix` | `&str` | A string appended between the date prefix and the `.log` extension. Pass `""` for the main log file or `".find"` for the find-mode log file. The suffix is inserted verbatim — if a dot separator is desired, it must be included in the string (e.g., `".find"` not `"find"`). |

## Return value

A `PathBuf` representing the fully qualified relative path to the log file, e.g., `logs/20250114.log` or `logs/20250114.find.log`.

## Remarks

- The function acquires the [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) mutex lock via the [get_local_time!](get_local_time.md) macro, extracts the `year`, `month`, and `day` components using `chrono::Datelike`, and then explicitly drops the guard before performing any file-system operations. This is important because `create_dir_all` could block on I/O, and holding the time buffer lock during I/O would unnecessarily delay other threads.
- The `logs/` directory is created with `std::fs::create_dir_all` if it does not already exist. The result of `create_dir_all` is silently discarded (`let _ = …`), so a failure to create the directory does not produce an error at this point — it will instead surface when the caller attempts to open the file.
- This function is **not** `pub` — it is module-private (`fn`, not `pub fn`). It is called only during the lazy initialization of [LOG_FILE](LOG_FILE.md) and [FIND_LOG_FILE](FIND_LOG_FILE.md), and is not intended for use outside the `logging` module.
- The date portion of the filename is determined by the value in [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) at the time the function is called. Because `LOG_FILE` and `FIND_LOG_FILE` are `Lazy` statics, the path is computed exactly once — on first access — and the resulting file handle is reused for the entire process lifetime. Log files are not automatically rotated at midnight; a new process (or service restart) is required to start a new dated log file.

### Path format

The generated path follows this pattern:

```/dev/null/example.txt#L1-1
logs/{YYYY}{MM}{DD}{suffix}.log
```

Examples:

| Suffix | Resulting path |
|--------|---------------|
| `""` | `logs/20250114.log` |
| `".find"` | `logs/20250114.find.log` |

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Visibility | Module-private (`fn`, not `pub fn`) |
| Callers | [LOG_FILE](LOG_FILE.md) initializer, [FIND_LOG_FILE](FIND_LOG_FILE.md) initializer |
| Callees | [get_local_time!](get_local_time.md), `std::fs::create_dir_all` |
| Crate dependencies | `chrono` (`Datelike`), `std::path::PathBuf`, `std::fs::create_dir_all` |

## See Also

| Topic | Link |
|-------|------|
| Main log file handle initialized by this function | [LOG_FILE](LOG_FILE.md) |
| Find-mode log file handle initialized by this function | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Cached local time used for date derivation | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| Timestamped log writing | [log_message](log_message.md) |
| Find-mode log writing | [log_to_find](log_to_find.md) |
| logging module overview | [logging module](README.md) |