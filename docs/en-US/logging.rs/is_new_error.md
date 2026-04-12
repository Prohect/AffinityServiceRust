# is_new_error function (logging.rs)

Checks whether a specific error combination has already been recorded, returning `true` only the first time a particular (pid, tid, process_name, operation, error_code) tuple is encountered. Used to suppress duplicate error logging across loop iterations.

## Syntax

```rust
pub fn is_new_error(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
) -> bool
```

## Parameters

`pid`

The process identifier of the process that triggered the error.

`tid`

The thread identifier of the thread involved in the error. Use `0` when the error is not thread-specific.

`process_name`

The display name of the process (e.g., `"game.exe"`). This is included in the composite key to distinguish errors from different processes that happen to share the same PID due to PID reuse.

`operation`

The [`Operation`](Operation.md) enum variant identifying which Windows API call failed. This distinguishes errors that occur on the same process/thread but from different operations (e.g., `SetPriorityClass` vs. `SetProcessAffinityMask`).

`error_code`

The Win32 error code or NTSTATUS code returned by the failing API. This is included in the key so that if a previously-failing operation starts producing a *different* error, the new error is still logged.

## Return value

Returns `true` if this exact error combination has not been seen before (i.e., it is new and should be logged). Returns `false` if the same combination was previously recorded (i.e., it is a duplicate and should be suppressed).

## Remarks

This function is the core of the error deduplication system. It prevents the log file from being flooded with the same error message every loop iteration — a common scenario when a process is permanently inaccessible (e.g., protected system processes that always return `ERROR_ACCESS_DENIED`).

The function operates on the [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) static, which is a two-level map: `HashMap<u32, HashMap<ApplyFailEntry, bool>>`. The flow is:

1. Lock the [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) mutex.
2. Construct an [`ApplyFailEntry`](ApplyFailEntry.md) from the provided parameters.
3. Look up the outer map by `pid`. If the PID has no entry, create a new inner map.
4. Check the inner map for the constructed `ApplyFailEntry` key.
5. If the key is **not** present, insert it and return `true` (new error — caller should log it).
6. If the key **is** present, return `false` (duplicate — caller should suppress it).

### Callers

This function is called from multiple locations throughout the codebase:

- [`log_error_if_new`](../apply.rs/log_error_if_new.md) in `apply.rs` — the primary consumer, called by all `apply_*` functions.
- [`get_process_handle`](../winapi.rs/get_process_handle.md) and [`try_open_thread`](../winapi.rs/try_open_thread.md) in `winapi.rs` — for handle acquisition errors.

### Stale entry cleanup

Entries in `PID_MAP_FAIL_ENTRY_SET` are not removed by this function. Cleanup is performed by [`purge_fail_map`](purge_fail_map.md), which is called each loop iteration to remove entries for processes that are no longer alive. This prevents unbounded memory growth.

### Error code significance

Including `error_code` in the composite key ensures that if a process starts failing with a *different* error code (e.g., transitioning from `ERROR_ACCESS_DENIED` to `ERROR_INVALID_HANDLE`), the new error is logged even though the same pid/tid/operation combination was previously recorded. This provides visibility into changing error conditions.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source lines** | L105–L149 |
| **Called by** | [`log_error_if_new`](../apply.rs/log_error_if_new.md), [`get_process_handle`](../winapi.rs/get_process_handle.md), [`try_open_thread`](../winapi.rs/try_open_thread.md) |
| **Modifies** | [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) |

## See also

- [ApplyFailEntry struct](ApplyFailEntry.md)
- [Operation enum](Operation.md)
- [PID_MAP_FAIL_ENTRY_SET static](PID_MAP_FAIL_ENTRY_SET.md)
- [purge_fail_map function](purge_fail_map.md)
- [log_error_if_new](../apply.rs/log_error_if_new.md)
- [logging.rs module overview](README.md)