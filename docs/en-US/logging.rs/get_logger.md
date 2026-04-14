# get_logger! macro (logging.rs)

Convenience macro that acquires the mutex lock on the [LOG_FILE](LOG_FILE.md) static and returns a `MutexGuard<File>`. This provides ergonomic, lock-guarded write access to the main log file handle without requiring the caller to spell out the full `LOG_FILE.lock().unwrap()` expression.

## Syntax

```logging.rs
#[macro_export]
macro_rules! get_logger {
    () => {
        $crate::logging::LOG_FILE.lock().unwrap()
    };
}
```

## Return value

Returns `std::sync::MutexGuard<'static, std::fs::File>`. The guard holds the mutex lock for the duration of its lifetime. When the guard is dropped (typically at the end of the enclosing statement or block), the lock is released.

The returned guard dereferences to `&File` (or `&mut File`), so it can be used directly with `writeln!` and other I/O operations.

## Remarks

- The macro calls `.unwrap()` on the `lock()` result. If the mutex is poisoned (a previous holder panicked while holding the lock), this will panic. In practice, the logging functions do not panic, so mutex poisoning is not expected.
- The lock is held only for the duration of the returned `MutexGuard`'s lifetime. Callers should avoid holding the guard across long-running operations or across calls that acquire other logging mutexes, to prevent deadlock or unnecessary contention.
- This macro is marked `#[macro_export]`, which places it at the crate root. It is invoked as `get_logger!()` from any module within the crate.
- The macro is used internally by [log_message](log_message.md) and [log_pure_message](log_pure_message.md) to write formatted output to the main log file when [USE_CONSOLE](USE_CONSOLE.md) is `false`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Expands to | `$crate::logging::LOG_FILE.lock().unwrap()` |
| Depends on | [LOG_FILE](LOG_FILE.md) |
| Used by | [log_message](log_message.md), [log_pure_message](log_pure_message.md) |

## See Also

| Topic | Link |
|-------|------|
| Main log file handle | [LOG_FILE](LOG_FILE.md) |
| Find-mode log file accessor macro | [get_logger_find!](get_logger_find.md) |
| Timestamped log writing function | [log_message](log_message.md) |
| Raw log writing function | [log_pure_message](log_pure_message.md) |
| Console vs. file routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd