# get_handles function (apply.rs)

Extracts read and write handles from a [ProcessHandle](../winapi.rs/ProcessHandle.md), preferring full-access handles over limited-access handles. This is the common entry point for all per-process apply functions that need to query or modify process state.

## Syntax

```AffinityServiceRust/src/apply.rs#L63-64
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>)
```

## Parameters

`process_handle: &ProcessHandle`

A reference to a [ProcessHandle](../winapi.rs/ProcessHandle.md) containing up to four handles: full and limited variants for both read and write access.

## Return value

Returns a tuple `(Option<HANDLE>, Option<HANDLE>)` where:

| Index | Description |
|-------|-------------|
| `.0` | Read handle — `r_handle` if present (`Some`), otherwise `Some(r_limited_handle)`. |
| `.1` | Write handle — `w_handle` if present (`Some`), otherwise `Some(w_limited_handle)`. |

Both elements are always `Some` when the `ProcessHandle` was successfully opened, because `r_limited_handle` and `w_limited_handle` are unconditionally populated. The `Option` wrapper allows callers to pattern-match with `let (Some(r), Some(w)) = get_handles(...) else { return; }` to bail out when handles are invalid.

## Remarks

- This function is marked `#[inline(always)]` because it is called at the top of every process-level apply function (`apply_priority`, `apply_affinity`, `apply_io_priority`, `apply_memory_priority`, `apply_process_default_cpuset`).
- Full-access handles (`r_handle` / `w_handle`) are preferred because they are opened with broader access rights (`PROCESS_QUERY_INFORMATION` / `PROCESS_SET_INFORMATION`). Limited handles (`r_limited_handle` / `w_limited_handle`) use `PROCESS_QUERY_LIMITED_INFORMATION` / `PROCESS_SET_LIMITED_INFORMATION`, which may not suffice for all operations (e.g., `NtQueryInformationProcess` for IO priority requires full query access).
- The function does **not** validate whether the returned handles are actually valid Win32 handles; callers rely on the Win32 API calls themselves to report errors if a handle is stale or insufficient.

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Visibility** | `fn` (crate-private) |
| **Callers** | [apply_priority](apply_priority.md), [apply_affinity](apply_affinity.md), [apply_process_default_cpuset](apply_process_default_cpuset.md), [apply_io_priority](apply_io_priority.md), [apply_memory_priority](apply_memory_priority.md) |
| **Callees** | None |
| **API / OS** | None |
| **Privileges** | None (handle opening is done elsewhere) |

## See Also

| Topic | Link |
|-------|------|
| ProcessHandle struct | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| Process priority application | [apply_priority](apply_priority.md) |
| Process affinity application | [apply_affinity](apply_affinity.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*