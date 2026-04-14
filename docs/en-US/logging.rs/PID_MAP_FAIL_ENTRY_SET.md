# PID_MAP_FAIL_ENTRY_SET static (logging.rs)

Global per-PID map of [ApplyFailEntry](ApplyFailEntry.md) records used to deduplicate Windows API operation errors. Each entry in the map tracks whether a specific combination of process, thread, operation, and error code has already been logged, preventing repeated identical error messages from flooding the log output during the service's polling loop.

## Syntax

```logging.rs
pub static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. The map is created empty on first access. |
| Middle | `Mutex<…>` | Provides interior mutability and thread-safe access from the service loop thread. |
| Inner | `HashMap<u32, HashMap<ApplyFailEntry, bool>>` | Two-level map: the outer key is the process ID (`u32`); the inner map keys are [ApplyFailEntry](ApplyFailEntry.md) records with a `bool` value indicating whether the entry is still "alive" (the associated process is still running). |

## Remarks

This static is the core data structure behind the error deduplication system in AffinityServiceRust. The service loop repeatedly applies configuration rules to running processes, and many operations can fail with the same error on every iteration (e.g., `ACCESS_DENIED` on a protected process). Without deduplication, the log file would be dominated by these repetitive errors.

### Two-level map structure

- **Outer map (`HashMap<u32, …>`):** Keyed by process ID. Each running process that has experienced at least one error has an entry here.
- **Inner map (`HashMap<ApplyFailEntry, bool>`):** Keyed by [ApplyFailEntry](ApplyFailEntry.md), which combines `tid`, `process_name`, `operation`, and `error_code`. The `bool` value tracks liveness: `true` means the process was seen running in the most recent snapshot, `false` means it was not.

### Lifecycle

1. **Insert:** [is_new_error](is_new_error.md) inserts a new entry when it encounters an error combination not yet present for a given PID. It returns `true` to signal the caller that this error should be logged.
2. **Deduplicate:** On subsequent calls, [is_new_error](is_new_error.md) finds the existing entry and returns `false`, suppressing the duplicate log message. The entry's `alive` flag is set to `true` to indicate the process is still active.
3. **Purge:** [purge_fail_map](purge_fail_map.md) is called periodically to remove entries for processes that are no longer running. It marks all entries as dead, then re-marks entries for still-running processes as alive, and finally removes any entries that remain dead.

### PID reuse handling

If a PID is reused by a new process with a different name, [is_new_error](is_new_error.md) detects the name mismatch via an invariant check (all entries in a PID's inner map are expected to share the same `process_name`). When a mismatch is found, the inner map is cleared before inserting the new entry, preventing stale deduplication state from suppressing errors for the new process.

### Macro access

The [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) macro provides a convenience wrapper that locks the mutex and returns the `MutexGuard`:

```logging.rs
macro_rules! get_pid_map_fail_entry_set {
    () => {
        $crate::logging::PID_MAP_FAIL_ENTRY_SET.lock().unwrap()
    };
}
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex`, `std::collections::HashMap` |
| Written by | [is_new_error](is_new_error.md) |
| Purged by | [purge_fail_map](purge_fail_map.md) |
| Read by | [is_new_error](is_new_error.md), [purge_fail_map](purge_fail_map.md) |

## See Also

| Topic | Link |
|-------|------|
| Failure entry key struct | [ApplyFailEntry](ApplyFailEntry.md) |
| Error deduplication logic | [is_new_error](is_new_error.md) |
| Stale entry cleanup | [purge_fail_map](purge_fail_map.md) |
| Windows API operation identifiers | [Operation](Operation.md) |
| Macro for guarded access | [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd