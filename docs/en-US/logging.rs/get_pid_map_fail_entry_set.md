# get_pid_map_fail_entry_set! macro (logging.rs)

Convenience macro that locks the [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) mutex and returns a `MutexGuard<HashMap<u32, HashMap<ApplyFailEntry, bool>>>`. This provides ergonomic, consistent access to the global per-PID failure deduplication map without requiring callers to spell out the full lock-and-unwrap expression.

## Syntax

```logging.rs
#[macro_export]
macro_rules! get_pid_map_fail_entry_set {
    () => {
        $crate::logging::PID_MAP_FAIL_ENTRY_SET.lock().unwrap()
    };
}
```

## Return value

Returns a `MutexGuard<HashMap<u32, HashMap<ApplyFailEntry, bool>>>`. The guard holds the lock for the duration of its lifetime and dereferences to the inner `HashMap`. When the guard is dropped (goes out of scope), the mutex is automatically released.

## Remarks

- The macro calls `.unwrap()` on the mutex lock result. If the mutex is poisoned (a previous holder panicked while holding the lock), this will panic. In AffinityServiceRust, mutex poisoning is not expected during normal operation because the service does not use `panic`-based error handling in the logging path.
- The returned guard provides both read and write access to the two-level map. Callers such as [is_new_error](is_new_error.md) and [purge_fail_map](purge_fail_map.md) use this macro to insert, update, and remove entries in the map.
- The `#[macro_export]` attribute makes this macro available at the crate root. Callers in other modules invoke it without a module prefix — e.g., `get_pid_map_fail_entry_set!()` — rather than `logging::get_pid_map_fail_entry_set!()`.
- Because the macro acquires a mutex lock, callers should minimize the scope of the returned guard to avoid holding the lock longer than necessary. In particular, avoid calling other lock-acquiring functions (such as those accessing [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) or [USE_CONSOLE](USE_CONSOLE.md)) while holding this guard to prevent potential deadlocks.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `std::sync::Mutex`, `std::collections::HashMap` |
| Underlying static | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Used by | [is_new_error](is_new_error.md), [purge_fail_map](purge_fail_map.md) |

## See Also

| Topic | Link |
|-------|------|
| Global failure tracking map | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Failure entry key struct | [ApplyFailEntry](ApplyFailEntry.md) |
| Error deduplication check | [is_new_error](is_new_error.md) |
| Stale entry cleanup | [purge_fail_map](purge_fail_map.md) |
| Windows API operation identifiers | [Operation](Operation.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd