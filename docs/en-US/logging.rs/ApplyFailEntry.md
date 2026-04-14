# ApplyFailEntry struct (logging.rs)

Composite key used for deduplicating Windows API operation failures in the logging subsystem. Each instance uniquely identifies a specific failure scenario by combining the thread ID, process name, operation type, and error code. Instances are stored as keys in the per-PID inner map of [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) and are compared for equality when [is_new_error](is_new_error.md) checks whether a given failure has already been logged.

## Syntax

```logging.rs
#[derive(PartialEq, Eq, Hash)]
pub struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `tid` | `u32` | The Windows thread identifier associated with the failed operation. For process-level operations (e.g., `SetPriorityClass`), this is typically `0` or the primary thread ID. For thread-level operations (e.g., `SetThreadPriority`), this is the specific thread that caused the failure. |
| `process_name` | `String` | The lowercase executable name of the target process (e.g., `"chrome.exe"`). Used both for display in log messages and as part of the deduplication key. Also serves as an invariant check — all entries in a PID's inner map are expected to share the same `process_name`. |
| `operation` | [Operation](Operation.md) | The [Operation](Operation.md) enum variant identifying which Windows API call failed (e.g., `SetPriorityClass`, `OpenProcess2processSetInformation`). |
| `error_code` | `u32` | The Win32 error code or NTSTATUS value returned by the failed operation. A value of `0` is used when no specific error code is available from the API call context, or as a custom sentinel to differentiate distinct failure modes within the same operation. |

## Remarks

- The struct derives `PartialEq`, `Eq`, and `Hash`, which are required for use as a key in `HashMap<ApplyFailEntry, bool>`. Two entries are considered equal if and only if all four fields match exactly. This means the same operation failing with a different error code on the same thread is treated as a distinct failure and will be logged separately.
- The struct fields are **not** `pub` — they are module-private. Construction is done inline within [is_new_error](is_new_error.md), which is the only function that creates `ApplyFailEntry` instances.
- The `process_name` field serves a dual purpose: it is part of the deduplication key and also acts as a PID-reuse detection mechanism. When [is_new_error](is_new_error.md) encounters an existing inner map for a PID whose entries have a different `process_name` than the new entry, it clears the entire inner map before inserting. This prevents stale deduplication state from a terminated process from suppressing errors for a new process that inherited the same PID.
- The `tid` field allows thread-level operations to be deduplicated independently per thread. For example, if `SetThreadPriority` fails with `ACCESS_DENIED` on thread 1234 of process `foo.exe`, that failure is tracked separately from the same error on thread 5678 of the same process. This ensures that the first failure on each thread is logged, providing complete diagnostic coverage.
- `ApplyFailEntry` does not implement `Debug` or `Clone`. It is created, inserted into the map, and compared — no other operations are needed.

### Deduplication flow

1. The [apply module](../apply.rs/README.md) encounters a Win32 API failure.
2. It calls [is_new_error](is_new_error.md) with `pid`, `tid`, `process_name`, `operation`, and `error_code`.
3. `is_new_error` constructs an `ApplyFailEntry` from the last four parameters and looks it up in the inner map for the given `pid`.
4. If the entry is not found, it is inserted with `alive = true` and the function returns `true` — the caller logs the error.
5. If the entry already exists, the function marks it alive and returns `false` — the caller suppresses the duplicate log message.
6. Periodically, [purge_fail_map](purge_fail_map.md) removes entries for processes that are no longer running.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Trait implementations | `PartialEq`, `Eq`, `Hash` (derived) |
| Constructed by | [is_new_error](is_new_error.md) |
| Stored in | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Compared by | [is_new_error](is_new_error.md), [purge_fail_map](purge_fail_map.md) |

## See Also

| Topic | Link |
|-------|------|
| Windows API operation identifiers | [Operation](Operation.md) |
| Error deduplication logic | [is_new_error](is_new_error.md) |
| Stale entry cleanup | [purge_fail_map](purge_fail_map.md) |
| Global failure tracking map | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Win32 error code translation | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| NTSTATUS code translation | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd