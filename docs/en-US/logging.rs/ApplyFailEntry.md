# ApplyFailEntry struct (logging.rs)

Composite key structure used for error deduplication, combining all the identifying fields of a failed Windows API operation so that the same error is only logged once per session.

## Syntax

```rust
#[derive(Hash, Eq, PartialEq)]
pub struct ApplyFailEntry {
    pid: u32,
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

## Members

`pid`

The process identifier of the target process for which the operation failed.

`tid`

The thread identifier of the target thread, or `0` if the failed operation was process-level rather than thread-level.

`process_name`

The display name of the process (e.g., `"game.exe"`). Stored as an owned `String` because the entry persists in the [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) across loop iterations, outliving any borrowed references.

`operation`

The [`Operation`](Operation.md) enum variant identifying which Windows API call failed (e.g., `SetPriorityClass`, `SetProcessAffinityMask`).

`error_code`

The Win32 error code (`u32`) returned by `GetLastError` at the time of failure. This distinguishes between different failure modes for the same operation on the same process — for example, `ERROR_ACCESS_DENIED` (5) versus `ERROR_INVALID_PARAMETER` (87).

## Remarks

`ApplyFailEntry` derives `Hash` and `Eq`, enabling it to be used as a key in the `HashMap` inside [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md). Two entries are considered equal if and only if all five fields match exactly. This means that:

- The same error code for the same operation on a different thread is tracked separately.
- A different error code for the same operation on the same thread is tracked separately.
- The same error on the same PID/TID but with a different process name (e.g., after PID reuse) is tracked separately.

The primary consumer of this struct is [`is_new_error`](is_new_error.md), which constructs an `ApplyFailEntry` from its parameters and checks for its presence in the global deduplication map. If the entry is not yet present, it is inserted and the function returns `true`, signaling the caller to log the error. If already present, the function returns `false`, suppressing the duplicate.

### Why all five fields?

Using all five fields as the composite key ensures maximum precision in deduplication:

- **`pid`** — prevents errors for different processes from interfering with each other.
- **`tid`** — allows thread-level errors (e.g., `SetThreadIdealProcessorEx` failures) to be tracked independently per thread.
- **`process_name`** — guards against PID reuse: if a new process takes over a terminated process's PID, errors for the new process are treated as novel.
- **`operation`** — distinguishes between different API calls that may fail independently for the same process.
- **`error_code`** — captures the specific failure reason, so a new type of failure on the same operation is still logged.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Lines** | L97–L103 |
| **Stored in** | [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) |
| **Created by** | [`is_new_error`](is_new_error.md) |

## See also

- [Operation enum](Operation.md)
- [is_new_error function](is_new_error.md)
- [PID_MAP_FAIL_ENTRY_SET static](PID_MAP_FAIL_ENTRY_SET.md)
- [purge_fail_map function](purge_fail_map.md)
- [logging.rs module overview](README.md)