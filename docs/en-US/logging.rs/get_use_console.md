# get_use_console! macro (logging.rs)

Convenience macro that acquires the mutex lock on the [USE_CONSOLE](USE_CONSOLE.md) static and returns a `MutexGuard<bool>`. This provides ergonomic, panic-on-poison access to the console routing flag from any call site without requiring the caller to spell out the full `lock().unwrap()` chain.

## Syntax

```logging.rs
#[macro_export]
macro_rules! get_use_console {
    () => {
        $crate::logging::USE_CONSOLE.lock().unwrap()
    };
}
```

## Return value

Returns `std::sync::MutexGuard<'static, bool>`. Dereference the guard to read the current flag value (`true` = console output, `false` = file output). The guard can also be dereferenced mutably to change the flag, though in practice the flag is only set once during startup.

## Remarks

- The macro expands to `$crate::logging::USE_CONSOLE.lock().unwrap()`, which means it will **panic** if the mutex is poisoned (i.e., a thread panicked while holding the lock). In AffinityServiceRust this is acceptable because a poisoned logging mutex indicates an unrecoverable state.
- The returned `MutexGuard` holds the lock for the duration of its lifetime. Callers should avoid holding the guard across long operations to prevent blocking other threads that need to check the console flag.
- Because `#[macro_export]` places the macro at the crate root, it is invoked as `get_use_console!()` from any module within the crate without a `use` import.
- Typical usage pattern in logging functions:

```logging.rs
if *get_use_console!() {
    let _ = writeln!(stdout(), "[{}]{}", time_prefix, args);
} else {
    let _ = writeln!(get_logger!(), "[{}]{}", time_prefix, args);
}
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` (exported at crate root via `#[macro_export]`) |
| Underlying static | [USE_CONSOLE](USE_CONSOLE.md) |
| Used by | [log_message](log_message.md), [log_pure_message](log_pure_message.md), [log_to_find](log_to_find.md) |

## See Also

| Topic | Link |
|-------|------|
| Console routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| Dust-bin mode accessor macro | [get_dust_bin_mod!](get_dust_bin_mod.md) |
| Local time accessor macro | [get_local_time!](get_local_time.md) |
| Log file accessor macro | [get_logger!](get_logger.md) |
| Find log file accessor macro | [get_logger_find!](get_logger_find.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd