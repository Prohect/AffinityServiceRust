# ApplyFailEntry struct (logging.rs)

Composite key struct representing a unique operation failure event. Each `ApplyFailEntry` identifies a specific combination of thread ID, process name, Windows API operation, and error code. It is used as a key in the per-PID failure tracking map ([`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set)) to deduplicate error log messages — ensuring that repeated failures for the same operation on the same thread are logged only once.

## Syntax

```rust
#[derive(PartialEq, Eq, Hash)]
pub struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    error_code: u32,
    operation: Operation,
}
```

## Members

| Field | Type | Visibility | Description |
|-------|------|------------|-------------|
| `tid` | `u32` | Private | The thread identifier associated with the failure. For process-level operations (where no specific thread is involved), this is typically `0`. |
| `process_name` | `String` | Private | The name of the process that the operation was attempted on. Used to detect when a PID has been reused by a different process (see Remarks). |
| `operation` | [`Operation`](Operation.md) | Private | The Windows API operation that failed (e.g., `OpenProcess2processQueryLimitedInformation`, `SetPriorityClass`, `SetThreadIdealProcessorEx`). |
| `error_code` | `u32` | Private | The Win32 error code returned by the failed operation. When no contextual error code is available, this is set to `0` or a custom discriminator value to differentiate failure modes. |

## Remarks

### Derives

The struct derives `PartialEq`, `Eq`, and `Hash`, which are required for use as a key in `HashMap` and for equality comparisons in the [`is_new_error`](is_new_error.md) deduplication logic. Two `ApplyFailEntry` instances are considered equal if **all four fields** match.

### PID reuse detection

The `process_name` field serves a dual purpose:

1. **Key identity** — It is part of the composite key used for deduplication.
2. **PID reuse guard** — In [`is_new_error`](is_new_error.md), when a failure entry set for a given PID contains entries whose `process_name` differs from the incoming entry's `process_name`, the entire set is cleared before inserting the new entry. This handles the case where the OS has recycled a PID for a new process — stale failure entries from the old process are discarded so that the new process's failures are properly logged.

### Alive flag

In the `PID_MAP_FAIL_ENTRY_SET` map, each `ApplyFailEntry` is paired with a `bool` alive flag (`HashMap<ApplyFailEntry, bool>`). This flag is used by [`purge_fail_map`](purge_fail_map.md) to implement mark-and-sweep garbage collection:

- All entries are marked as dead (`false`) at the start of each purge cycle.
- Entries matching currently running processes are re-marked as alive (`true`).
- Dead entries (processes that have exited) are removed from the map.

### Field visibility

All fields are **private** (no `pub` modifier). `ApplyFailEntry` instances are only created and inspected within the `logging` module — specifically in the [`is_new_error`](is_new_error.md) and [`purge_fail_map`](purge_fail_map.md) functions. External callers interact with the failure tracking system exclusively through those functions.

### Typical construction

```rust
let entry = ApplyFailEntry {
    tid,
    process_name: process_name.to_string(),
    operation,
    error_code,
};
```

The `process_name` is cloned into an owned `String` because the entry must outlive the borrowed `&str` passed to `is_new_error`.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Created by** | [`is_new_error`](is_new_error.md) |
| **Stored in** | [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) |
| **Dependencies** | [`Operation`](Operation.md) enum |
| **Platform** | Platform-independent struct; used in the context of Windows API error tracking |

## See Also

| Topic | Link |
|-------|------|
| Operation enum | [Operation](Operation.md) |
| is_new_error function | [is_new_error](is_new_error.md) |
| purge_fail_map function | [purge_fail_map](purge_fail_map.md) |
| PID_MAP_FAIL_ENTRY_SET static | [statics](statics.md#pid_map_fail_entry_set) |
| logging module overview | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
