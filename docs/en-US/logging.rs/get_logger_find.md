# get_logger_find! macro (logging.rs)

Convenience macro that acquires the mutex lock on the [FIND_LOG_FILE](FIND_LOG_FILE.md) static and returns a `MutexGuard<File>`. This provides ergonomic, short-lived access to the find-mode log file handle for writing find-mode log entries.

## Syntax

```logging.rs
#[macro_export]
macro_rules! get_logger_find {
    () => {
        $crate::logging::FIND_LOG_FILE.lock().unwrap()
    };
}
```

## Return value

Returns `std::sync::MutexGuard<'static, std::fs::File>` — a RAII guard that dereferences to the underlying `File` handle. The lock is released when the guard is dropped (typically at the end of the enclosing statement or block).

## Remarks

- This macro is a thin wrapper around `FIND_LOG_FILE.lock().unwrap()`. It calls `unwrap()` on the lock result, which means it will panic if the mutex is poisoned (i.e., a previous holder panicked while the lock was held). In practice, this does not occur because the logging code paths do not panic.
- The returned `MutexGuard<File>` implements `DerefMut` to `File`, so callers can use it directly with `writeln!` and other I/O write macros.
- The macro is annotated with `#[macro_export]`, making it available crate-wide as `crate::get_logger_find!()` without requiring a `use` import of the `logging` module.
- [log_to_find](log_to_find.md) is the primary consumer of this macro. It writes timestamped messages to the find-mode log file when [USE_CONSOLE](USE_CONSOLE.md) is `false`.
- The lock scope should be kept as short as possible to avoid holding the mutex across expensive operations or other lock acquisitions that could lead to contention or deadlock.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Wraps | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Used by | [log_to_find](log_to_find.md) |

## See Also

| Topic | Link |
|-------|------|
| Find-mode log file static | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Main log file accessor macro | [get_logger!](get_logger.md) |
| Find-mode log writing function | [log_to_find](log_to_find.md) |
| Console vs. file routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| logging module overview | [logging module](README.md) |