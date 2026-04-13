# is_new_error function (logging.rs)

Checks whether a specific Windows API operation failure has already been recorded for a given process and, if not, registers it for future deduplication. Returns `true` when the error combination is seen for the first time, signaling the caller that a log message should be emitted. Returns `false` on subsequent encounters of the same failure, suppressing duplicate log output.

## Syntax

```logging.rs
pub fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process that experienced the failure. Used as the outer key in [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md). |
| `tid` | `u32` | The thread identifier associated with the failure. Included in the [ApplyFailEntry](ApplyFailEntry.md) key so that the same operation failing on different threads of the same process can be independently tracked. |
| `process_name` | `&str` | The executable name of the target process (e.g., `"notepad.exe"`). Used both as part of the dedup key and for a PID-reuse consistency check. |
| `operation` | `Operation` | The [Operation](Operation.md) variant identifying which Windows API call failed (e.g., `Operation::SetPriorityClass`, `Operation::OpenProcess2processSetLimitedInformation`). |
| `error_code` | `u32` | The Win32 or NTSTATUS error code returned by the failed API call. If no contextual error code is available, pass `0` or a custom discriminant value. |

## Return value

| Value | Meaning |
|-------|---------|
| `true` | This is the **first** time this exact `(pid, tid, process_name, operation, error_code)` combination has been seen. The caller should log the error. |
| `false` | An identical failure entry already exists for this PID. The caller should suppress the duplicate log message. |

## Remarks

### Deduplication algorithm

1. The function constructs an [ApplyFailEntry](ApplyFailEntry.md) from the supplied parameters.
2. It locks the [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) global via the [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) macro.
3. **If an inner map exists for `pid`:**
   - Scans the inner `HashMap<ApplyFailEntry, bool>` for a matching entry.
   - If found, marks the entry as alive (`true`) and returns `false` (duplicate).
   - If not found, performs a **PID-reuse check** (see below) and inserts the new entry with alive = `true`, then returns `true` (new error).
4. **If no inner map exists for `pid`:**
   - Creates a new inner map containing the single entry and inserts it into the outer map, then returns `true`.

### PID-reuse safety (invariant A)

All entries in a PID's inner map are expected to share the same `process_name`. When inserting a new entry, the function checks whether the existing entries belong to a different process name. If a mismatch is detected — indicating that the operating system has reused the PID for a new process — the inner map is **cleared** before the new entry is inserted. This prevents stale deduplication state from suppressing legitimate first-occurrence errors for the new process.

### Alive flag

Each entry's `bool` value tracks liveness for the [purge_fail_map](purge_fail_map.md) garbage collection cycle:

- When `is_new_error` encounters an existing entry, it sets the `bool` to `true`, indicating the process was still active during this polling iteration.
- [purge_fail_map](purge_fail_map.md) periodically resets all flags to `false`, then re-marks entries for currently running processes. Entries that remain `false` after this pass are removed.

### Error code semantics

When no contextual Win32 error code is available (e.g., an API returned a boolean failure with no `GetLastError` call), the caller should pass `0` for `error_code`. If the caller needs to distinguish between multiple logical failure modes of the same operation, a custom non-zero sentinel value can be used as `error_code` to maintain separate dedup entries.

### Thread safety

The function acquires the [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) mutex for the entire duration of the lookup-and-insert operation, ensuring atomicity. In practice, this function is called exclusively from the single-threaded service loop, so contention does not occur.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Callers | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| Callees | [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) (macro) |
| Data structures | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md), [ApplyFailEntry](ApplyFailEntry.md), [Operation](Operation.md) |

## See Also

| Topic | Link |
|-------|------|
| Stale entry cleanup | [purge_fail_map](purge_fail_map.md) |
| Failure entry composite key | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation identifiers enum | [Operation](Operation.md) |
| Global failure tracking map | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Error formatting for log messages | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| Apply-module error logging helper | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| logging module overview | [logging module](README.md) |