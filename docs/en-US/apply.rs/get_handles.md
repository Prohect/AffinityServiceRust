# get_handles function (apply.rs)

Extracts the best available read and write `HANDLE`s from a [ProcessHandle](../winapi.rs/ProcessHandle.md), preferring full-access handles over limited-access handles. This is a small inline helper used at the top of every `apply_*` function that needs to interact with the Windows process API.

## Syntax

```AffinityServiceRust/src/apply.rs#L61-65
#[inline(always)]
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) {
    let r = process_handle.r_handle.or(Some(process_handle.r_limited_handle));
    let w = process_handle.w_handle.or(Some(process_handle.w_limited_handle));
    (r, w)
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `process_handle` | `&ProcessHandle` | A reference to the [ProcessHandle](../winapi.rs/ProcessHandle.md) that was opened for the target process. Contains up to four handles: read-limited, read-full, write-limited, and write-full. |

## Return value

Returns `(Option<HANDLE>, Option<HANDLE>)` — a tuple of (read handle, write handle).

- The **first** element is the read handle, resolved as `r_handle` if it is `Some`, otherwise `r_limited_handle`.
- The **second** element is the write handle, resolved as `w_handle` if it is `Some`, otherwise `w_limited_handle`.

Because `r_limited_handle` and `w_limited_handle` are always populated when a [ProcessHandle](../winapi.rs/ProcessHandle.md) exists, the returned `Option`s are effectively always `Some`. Callers nonetheless pattern-match with `let (Some(r), Some(w)) = get_handles(...) else { return; }` as a defensive guard.

## Remarks

[ProcessHandle](../winapi.rs/ProcessHandle.md) stores two tiers of access for both reading and writing:

| Field | Access rights | Availability |
|-------|--------------|--------------|
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | Always present |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | Present only when the service has sufficient privilege |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | Always present |
| `w_handle` | `PROCESS_SET_INFORMATION` | Present only when the service has sufficient privilege |

The full-access handles (`r_handle`, `w_handle`) are `Option<HANDLE>`. When the service runs with `SeDebugPrivilege` and elevated rights, these are typically available for all processes. When privilege is limited (e.g. `--no-debug-priv`), only the limited handles are opened. `get_handles` abstracts this tier selection so that each `apply_*` function does not need to repeat the fallback logic.

The function is marked `#[inline(always)]` because it performs no allocation and compiles down to two conditional moves. It is called on every apply cycle for every matched process, so eliminating call overhead is worthwhile.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `fn` (crate-private) |
| Callers | [apply_priority](apply_priority.md), [apply_affinity](apply_affinity.md), [apply_process_default_cpuset](apply_process_default_cpuset.md), [apply_io_priority](apply_io_priority.md), [apply_memory_priority](apply_memory_priority.md) |
| Callees | None |
| API | None — pure Rust logic over existing handles |
| Privileges | None |

## See Also

| Topic | Link |
|-------|------|
| Handle wrapper struct | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| Handle acquisition | [get_process_handle](../winapi.rs/get_process_handle.md) |
| apply module overview | [apply](README.md) |