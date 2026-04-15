# is_new_error function (logging.rs)

Tracks operation failures to avoid spamming logs with repeated error messages. Returns `true` only on the first occurrence of a given PID/TID/process-name/operation/error-code combination, allowing callers to conditionally log or handle the error only once per unique failure.

## Syntax

```rust
pub fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier associated with the failure. Used as the top-level key in the failure tracking map. |
| `tid` | `u32` | The thread identifier associated with the failure. Combined with `process_name`, `operation`, and `error_code` to form a unique failure key. Use `0` for process-level operations that are not thread-specific. |
| `process_name` | `&str` | The name of the process associated with the failure. Used both as part of the failure key and for stale-entry detection (see Remarks). |
| `operation` | [`Operation`](Operation.md) | The Windows API operation that failed. Each variant of the [`Operation`](Operation.md) enum represents a distinct API call or handle-acquisition step. |
| `error_code` | `u32` | The Win32 error code or custom discriminator returned by the failed operation. Use `0` when there is no contextual error code, or a custom value if you need to differentiate failures that share the same operation but have distinct causes. |

## Return value

Returns `true` if this is the **first** time this specific `(pid, tid, process_name, operation, error_code)` combination has been recorded. Returns `false` if the same combination was already present in the failure tracking map.

## Remarks

### Data structure

The function uses the global [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) static, which has the type:

```text
Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>
```

- The **outer key** is the `pid`.
- The **inner key** is an [`ApplyFailEntry`](ApplyFailEntry.md) struct containing `(tid, process_name, operation, error_code)`.
- The **inner value** is a `bool` "alive" flag used by [`purge_fail_map`](purge_fail_map.md) to detect and remove stale entries.

### Algorithm

1. Construct an [`ApplyFailEntry`](ApplyFailEntry.md) from the provided parameters.
2. Lock the `PID_MAP_FAIL_ENTRY_SET` mutex.
3. Look up the `pid` in the outer map:
   - **If found** — Search the inner `HashMap` for a matching entry:
     - **If a matching entry exists** — Mark it as alive (`true`) and return `false` (not a new error).
     - **If no matching entry exists** — Check whether the existing entries have the same `process_name` as the new entry. If not (indicating PID reuse by a different process), **clear the entire inner map** before inserting the new entry. Insert the new entry with `alive = true` and return `true`.
   - **If not found** — Create a new inner map containing only the new entry with `alive = true`, insert it under the `pid` key, and return `true`.

### PID reuse detection

When a PID is reused by a different process (e.g., the original process exited and a new process was assigned the same PID), the existing failure entries become stale. The function detects this by comparing the `process_name` of the first existing entry against the new entry's `process_name`. If they differ, the inner map is cleared before inserting the new entry. This prevents false negatives (where a genuinely new error is suppressed because a previous, unrelated process with the same PID had the same operation failure).

### Alive flag

Each entry stores an `alive` flag (`bool` value in the inner `HashMap`). When `is_new_error` finds a matching entry, it sets `alive = true`. The companion function [`purge_fail_map`](purge_fail_map.md) uses this flag to implement a mark-and-sweep garbage collection scheme: it first marks all entries as dead (`false`), then re-marks entries for currently running processes as alive, and finally removes any entries that remain dead.

### Thread safety

The function acquires the `PID_MAP_FAIL_ENTRY_SET` mutex via the `get_pid_map_fail_entry_set!()` macro. The lock is held for the duration of the lookup-and-insert operation.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Callers** | [`get_process_handle`](../winapi.rs/get_process_handle.md), [`get_thread_handle`](../winapi.rs/get_thread_handle.md), `apply.rs` rule application logic |
| **Callees** | `get_pid_map_fail_entry_set!()` macro (locks [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set)) |
| **Statics** | [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) |
| **Dependencies** | [`ApplyFailEntry`](ApplyFailEntry.md), [`Operation`](Operation.md) |
| **Platform** | Platform-independent logic (data is platform-specific) |

## See Also

| Topic | Link |
|-------|------|
| purge_fail_map | [purge_fail_map](purge_fail_map.md) |
| ApplyFailEntry struct | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation enum | [Operation](Operation.md) |
| PID_MAP_FAIL_ENTRY_SET | [statics](statics.md#pid_map_fail_entry_set) |
| get_process_handle | [get_process_handle](../winapi.rs/get_process_handle.md) |
| get_thread_handle | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| logging module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
