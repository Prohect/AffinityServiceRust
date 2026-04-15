# get_handles function (apply.rs)

Extracts read and write `HANDLE` values from a `ProcessHandle`, preferring full-access handles over their limited counterparts. This is a private helper used by the process-level apply functions to obtain the appropriate handle pair before calling Windows APIs that query or modify process attributes.

## Syntax

```AffinityServiceRust/src/apply.rs#L63-67
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) {
    let r = process_handle.r_handle.or(Some(process_handle.r_limited_handle));
    let w = process_handle.w_handle.or(Some(process_handle.w_limited_handle));
    (r, w)
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `process_handle` | `&ProcessHandle` | A reference to the `ProcessHandle` struct that holds up to four OS handles for the target process: full-access read, limited read, full-access write, and limited write. |

## Return value

Returns a tuple `(Option<HANDLE>, Option<HANDLE>)` where:

| Position | Meaning |
|----------|---------|
| `.0` | The read handle. `process_handle.r_handle` if it is `Some`; otherwise falls back to `Some(process_handle.r_limited_handle)`. |
| `.1` | The write handle. `process_handle.w_handle` if it is `Some`; otherwise falls back to `Some(process_handle.w_limited_handle)`. |

The result is `(None, _)` or `(_, None)` only when the preferred `Option` field is `None` **and** the limited handle also happens to be a `None`-equivalent, which in practice does not occur because `ProcessHandle` always populates the limited fields.

## Remarks

- The function is marked `#[inline(always)]` to eliminate call overhead, since it is invoked at the top of every process-level apply function.
- Callers destructure the return value with a `let … else` pattern:
  ```AffinityServiceRust/src/apply.rs#L92-94
  let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
      return;
  };
  ```
  If either handle is `None`, the calling apply function returns early without attempting any Windows API calls.
- The "full-access" vs "limited" distinction exists because the service may have been granted different access rights depending on the target process's protection level. Full-access handles (e.g., `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION`) are preferred because they allow both querying and setting attributes. Limited handles may only support a subset of operations.
- This function does **not** check whether the underlying `HANDLE` value is invalid (e.g., `INVALID_HANDLE_VALUE`); that responsibility lies with the caller or with the Windows API itself, which will return an error code.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Visibility | Private (`fn`, not `pub fn`) |
| Callers | [`apply_priority`](apply_priority.md), [`apply_affinity`](apply_affinity.md), [`apply_process_default_cpuset`](apply_process_default_cpuset.md), [`apply_io_priority`](apply_io_priority.md), [`apply_memory_priority`](apply_memory_priority.md) |
| Callees | None (reads struct fields only) |
| API | None |
| Privileges | None |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ProcessHandle | [`winapi.rs/ProcessHandle`](../winapi.rs/ProcessHandle.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_process_default_cpuset | [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*