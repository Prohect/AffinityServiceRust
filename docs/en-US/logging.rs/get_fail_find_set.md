# get_fail_find_set! macro (logging.rs)

Convenience macro that locks the [FINDS_FAIL_SET](FINDS_FAIL_SET.md) mutex and returns a `MutexGuard<HashSet<String>>`. This macro eliminates boilerplate for the common pattern of acquiring the lock on the global find-failure deduplication set, providing a concise and consistent accessor used throughout the codebase.

## Syntax

```logging.rs
#[macro_export]
macro_rules! get_fail_find_set {
    () => {
        $crate::logging::FINDS_FAIL_SET.lock().unwrap()
    };
}
```

## Return value

Returns `std::sync::MutexGuard<'static, HashSet<String>>` — a RAII guard that dereferences to the underlying `HashSet<String>` and releases the lock when dropped.

## Remarks

- The macro calls `.unwrap()` on the `lock()` result. If the mutex is poisoned (i.e., a thread panicked while holding the lock), the macro will panic. In AffinityServiceRust, this is considered an unrecoverable state — a poisoned mutex indicates a programming error or catastrophic failure that warrants a crash rather than silent corruption.
- The returned guard holds the mutex lock for its entire lifetime. Callers should minimize the scope of the guard to avoid blocking other threads. In practice, find-failure checks are infrequent and single-threaded, so contention is negligible.
- This macro accesses [FINDS_FAIL_SET](FINDS_FAIL_SET.md), **not** [FINDS_SET](FINDS_SET.md). Despite the similar naming, `FINDS_SET` tracks successful finds and `FINDS_FAIL_SET` tracks failed finds. Direct `.lock().unwrap()` access is used for `FINDS_SET` in [log_process_find](log_process_find.md).
- The `#[macro_export]` attribute places this macro at the crate root, so callers import it as `crate::get_fail_find_set!()` rather than `crate::logging::get_fail_find_set!()`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` (exported to crate root via `#[macro_export]`) |
| Underlying static | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| Callers | [process_find](../main.rs/README.md) |

## See Also

| Topic | Link |
|-------|------|
| Guarded static for failed find dedup | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| Successful find dedup set | [FINDS_SET](FINDS_SET.md) |
| Find-mode process logging | [log_process_find](log_process_find.md) |
| Other accessor macros | [get_use_console!](get_use_console.md), [get_dust_bin_mod!](get_dust_bin_mod.md), [get_local_time!](get_local_time.md), [get_logger!](get_logger.md), [get_logger_find!](get_logger_find.md), [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd