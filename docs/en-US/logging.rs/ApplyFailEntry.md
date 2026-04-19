# ApplyFailEntry struct (logging.rs)

Composite key representing a unique process/thread operation failure. Used as a hash-map key inside the per-PID error deduplication map (`PID_MAP_FAIL_ENTRY_SET`) so that repeated identical failures are logged only once.

## Syntax

```rust
#[derive(PartialEq, Eq, Hash)]
pub struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `tid` | `u32` | The thread identifier associated with the failure. For process-level operations that have no specific thread, callers typically pass `0`. |
| `process_name` | `String` | The executable name of the process that failed (e.g. `"chrome.exe"`). Also used as a staleness indicator — when the PID is reused by a different process, the name mismatch triggers a clear of all prior entries for that PID. |
| `operation` | [Operation](Operation.md) | The Windows API operation that failed (e.g. `SetProcessAffinityMask`, `SetThreadPriority`). |
| `error_code` | `u32` | The Win32 or NTSTATUS error code returned by the failing call. A value of `0` is used when no OS error code is available and the caller needs a custom sentinel. |

## Remarks

- The struct derives `PartialEq`, `Eq`, and `Hash` so it can be used as a key in `HashMap<ApplyFailEntry, bool>`. The `bool` value in the map tracks liveness — `true` means the entry was seen during the current apply cycle, `false` means it is a candidate for removal during [purge_fail_map](purge_fail_map.md).
- The `process_name` field serves a dual purpose: it is part of the equality key **and** it is used by [is_new_error](is_new_error.md) to detect PID reuse. If the incoming `process_name` does not match any existing entry's `process_name` for that PID, the entire sub-map is cleared before inserting the new entry.
- All fields are private; instances are constructed exclusively inside [is_new_error](is_new_error.md).

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | [is_new_error](is_new_error.md) (sole constructor) |
| **Callees** | None |
| **Dependencies** | [Operation](Operation.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Error deduplication logic | [is_new_error](is_new_error.md) |
| Stale entry cleanup | [purge_fail_map](purge_fail_map.md) |
| Operation enum | [Operation](Operation.md) |
| Consumer of dedup results | [log_error_if_new](../apply.rs/log_error_if_new.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*