# LOCAL_TIME_BUFFER static (logging.rs)

Cached `DateTime<Local>` value used for log timestamps and date-based log file naming. This global static holds the current local time, which is periodically updated by the main service loop. All logging functions read from this buffer rather than calling `Local::now()` independently, ensuring consistent timestamps within a single loop iteration and avoiding redundant system calls.

## Syntax

```logging.rs
pub static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. The value is created on first access. |
| Middle | `Mutex<…>` | Provides interior mutability and thread-safe access from the main thread and any logging call sites. |
| Inner | `DateTime<Local>` | A `chrono::DateTime<Local>` representing the cached local timestamp. Initialized to `Local::now()` on first access. |

## Remarks

- The main service loop updates this buffer at the start of each iteration so that all log messages emitted during a single pass share the same timestamp. This avoids visual inconsistencies where messages produced milliseconds apart within the same logical operation show different seconds.
- The cached time is also used by [get_log_path](get_log_path.md) to derive the date portion (`YYYYMMDD`) of log file names. Because the buffer is only updated at the top of the loop, log files are not inadvertently rotated mid-iteration if the loop spans midnight.
- Access is guarded by a `Mutex`. The convenience macro [get_local_time!](get_local_time.md) acquires this lock and returns a `MutexGuard<DateTime<Local>>` for ergonomic use:

```logging.rs
#[macro_export]
macro_rules! get_local_time {
    () => {
        $crate::logging::LOCAL_TIME_BUFFER.lock().unwrap()
    };
}
```

- [log_message](log_message.md), [log_to_find](log_to_find.md), and [get_log_path](get_log_path.md) all read this buffer through the `get_local_time!` macro. The format used for log line prefixes is `%H:%M:%S` (e.g., `[14:32:07]`), while `get_log_path` extracts the year, month, and day components for file naming.
- The initial value (`Local::now()` at first access) is only meaningful for the very first log file creation. After the service loop begins, the buffer is overwritten on every iteration.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `chrono` (`DateTime`, `Local`), `std::sync::Mutex` |
| Updated by | Main service loop in [main module](../main.rs/README.md) |
| Read by | [log_message](log_message.md), [log_to_find](log_to_find.md), [get_log_path](get_log_path.md) |
| Macro accessor | [get_local_time!](get_local_time.md) |

## See Also

| Topic | Link |
|-------|------|
| Main log file handle | [LOG_FILE](LOG_FILE.md) |
| Find-mode log file handle | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Log file path construction | [get_log_path](get_log_path.md) |
| Timestamped log writing | [log_message](log_message.md) |
| Console vs. file routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| logging module overview | [logging module](README.md) |