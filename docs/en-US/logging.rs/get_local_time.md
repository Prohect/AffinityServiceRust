# get_local_time! macro (logging.rs)

Convenience macro that acquires the mutex lock on [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) and returns a `MutexGuard<DateTime<Local>>`. This provides ergonomic, thread-safe read and write access to the cached local timestamp used for log prefixes and date-based log file naming.

## Syntax

```logging.rs
#[macro_export]
macro_rules! get_local_time {
    () => {
        $crate::logging::LOCAL_TIME_BUFFER.lock().unwrap()
    };
}
```

## Return value

Returns `std::sync::MutexGuard<'static, DateTime<Local>>`. The guard dereferences to `DateTime<Local>` for reading the cached timestamp. The caller can also dereference mutably (`*get_local_time!() = Local::now()`) to update the cached time. The mutex is released when the guard is dropped.

## Remarks

- This macro is `#[macro_export]`ed, making it available crate-wide as `get_local_time!()` without requiring a `use` import of the macro itself. Internally it references the static through the fully qualified path `$crate::logging::LOCAL_TIME_BUFFER`.
- The main service loop updates the cached time at the beginning of each iteration by assigning a fresh `Local::now()` through this macro. All subsequent log calls within the same iteration then read the same timestamp, ensuring consistent `[HH:MM:SS]` prefixes across related log lines.
- The macro calls `.unwrap()` on the `Mutex::lock()` result. If the mutex is poisoned (a previous holder panicked while holding the lock), this will panic. In practice, none of the code paths that hold this lock can panic under normal conditions.
- Callers should be mindful of lock duration. Holding the returned `MutexGuard` across a long operation will block other threads from reading or updating the time buffer. Prefer binding the guard to a short-lived variable or dropping it explicitly when the time value has been consumed.

### Usage patterns

**Reading the cached time for a log prefix:**

```logging.rs
let time_prefix = get_local_time!().format("%H:%M:%S").to_string();
```

**Updating the cached time at the top of the service loop:**

```logging.rs
*get_local_time!() = Local::now();
```

**Reading date components for log file naming (as done in [get_log_path](get_log_path.md)):**

```logging.rs
let time = get_local_time!();
let (year, month, day) = (time.year(), time.month(), time.day());
drop(time); // release lock before file I/O
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `chrono` (`DateTime`, `Local`), `std::sync::Mutex` |
| Underlying static | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| Callers | [log_message](log_message.md), [log_to_find](log_to_find.md), [get_log_path](get_log_path.md), [main](../main.rs/README.md) |

## See Also

| Topic | Link |
|-------|------|
| Cached timestamp static | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| Timestamped log output | [log_message](log_message.md) |
| Log file path construction | [get_log_path](get_log_path.md) |
| Other accessor macros | [get_use_console!](get_use_console.md), [get_dust_bin_mod!](get_dust_bin_mod.md), [get_logger!](get_logger.md) |
| logging module overview | [logging module](README.md) |