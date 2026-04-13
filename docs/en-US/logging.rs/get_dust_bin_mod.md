# get_dust_bin_mod! macro (logging.rs)

Convenience macro that locks the [DUST_BIN_MODE](DUST_BIN_MODE.md) mutex and returns a `MutexGuard<bool>`. This provides ergonomic, guarded access to the global flag that controls whether logging output is suppressed (e.g., before UAC elevation).

## Syntax

```logging.rs
#[macro_export]
macro_rules! get_dust_bin_mod {
    () => {
        $crate::logging::DUST_BIN_MODE.lock().unwrap()
    };
}
```

## Return value

Returns `MutexGuard<'static, bool>`. Dereference the guard to read the current value (`true` = logging suppressed, `false` = logging enabled). Dereference mutably to change the value:

```/dev/null/example.rs#L1-2
*get_dust_bin_mod!() = true;  // suppress logging
*get_dust_bin_mod!() = false; // re-enable logging
```

## Remarks

- The macro calls `.unwrap()` on the `Mutex::lock()` result. If the mutex is poisoned (a thread panicked while holding the lock), this will panic. In practice, the lock is only held for the duration of a single read or assignment, so poisoning is unlikely.
- The returned `MutexGuard` holds the lock until it is dropped. Callers should avoid holding the guard across long-running operations or across calls that acquire other logging-related mutexes to prevent deadlocks.
- This macro is `#[macro_export]`ed, making it available throughout the crate as `get_dust_bin_mod!()` without a module path prefix.
- The primary consumer is [log_message](log_message.md), which checks `*get_dust_bin_mod!()` at entry and returns immediately if the value is `true`. The main module writes to this flag during the pre-elevation phase controlled by the `--skip-log-before-elevation` CLI flag.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Backing static | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Crate dependencies | `std::sync::Mutex` (via `DUST_BIN_MODE`) |
| Used by | [log_message](log_message.md), [main](../main.rs/README.md) |

## See Also

| Topic | Link |
|-------|------|
| Backing static for this macro | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Console flag accessor macro | [get_use_console!](get_use_console.md) |
| Local time accessor macro | [get_local_time!](get_local_time.md) |
| Timestamped log function that checks this flag | [log_message](log_message.md) |
| logging module overview | [logging module](README.md) |