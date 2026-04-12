# get_handles function (apply.rs)

Extracts read and write handles from a `ProcessHandle`, preferring full-access handles over limited-access handles.

## Syntax

```rust
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>)
```

## Parameters

`process_handle`

A reference to a [`ProcessHandle`](../winapi.rs/ProcessHandle.md) containing up to four handles (read/write × full/limited access).

## Return value

A tuple of `(read_handle, write_handle)`, each wrapped in `Option<HANDLE>`.

- The **read handle** is `r_handle` if available, otherwise falls back to `r_limited_handle`.
- The **write handle** is `w_handle` if available, otherwise falls back to `w_limited_handle`.

Both values are always `Some(...)` because the limited handles are always present in a valid `ProcessHandle`.

## Remarks

This is an inline helper used at the top of nearly every `apply_*` function. The caller destructures the result with a `let-else` guard:

```rust
let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
    return;
};
```

Full-access handles (`r_handle`, `w_handle`) are preferred because they carry broader access rights (`PROCESS_QUERY_INFORMATION`, `PROCESS_SET_INFORMATION`). Limited handles (`r_limited_handle`, `w_limited_handle`) carry only `PROCESS_QUERY_LIMITED_INFORMATION` and `PROCESS_SET_LIMITED_INFORMATION` respectively, which is sufficient for some operations but not all.

The function is marked `#[inline(always)]` to eliminate call overhead.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/apply.rs` (L58–L65) |
| **Access** | Private (`fn`, not `pub fn`) |
| **Called by** | [apply_priority](apply_priority.md), [apply_affinity](apply_affinity.md), [apply_process_default_cpuset](apply_process_default_cpuset.md), [apply_io_priority](apply_io_priority.md), [apply_memory_priority](apply_memory_priority.md) |
| **Dependencies** | [`ProcessHandle`](../winapi.rs/ProcessHandle.md), `HANDLE` (Windows crate) |