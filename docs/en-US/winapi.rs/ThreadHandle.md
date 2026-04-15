# ThreadHandle struct (winapi.rs)

RAII wrapper for a set of thread handles with varying access levels. The `r_limited_handle` field is always valid (required when constructing this struct). Other handles (`r_handle`, `w_limited_handle`, `w_handle`) are attempted but may hold invalid `HANDLE` values if the corresponding `OpenThread` call failed. All valid handles are automatically closed when the struct is dropped.

## Syntax

```rust
#[derive(Debug)]
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `r_limited_handle` | `HANDLE` | Thread handle opened with `THREAD_QUERY_LIMITED_INFORMATION` access. Always valid. |
| `r_handle` | `HANDLE` | Thread handle opened with `THREAD_QUERY_INFORMATION` access. May be an invalid handle if the open call failed. |
| `w_limited_handle` | `HANDLE` | Thread handle opened with `THREAD_SET_LIMITED_INFORMATION` access. May be an invalid handle if the open call failed. |
| `w_handle` | `HANDLE` | Thread handle opened with `THREAD_SET_INFORMATION` access. May be an invalid handle if the open call failed. |

## Remarks

Unlike [ProcessHandle](ProcessHandle.md), which uses `Option<HANDLE>` for its optional handles, `ThreadHandle` stores all handles as raw `HANDLE` values. Callers must check `HANDLE::is_invalid()` before using `r_handle`, `w_limited_handle`, or `w_handle`.

The `Drop` implementation unconditionally closes `r_limited_handle` (since it is guaranteed valid) and conditionally closes each of the other three handles only if they are not invalid. This ensures no double-close or invalid-handle-close occurs.

`ThreadHandle` derives `Debug` for diagnostic output.

### Handle access rights mapping

| Field | Win32 Access Right |
|-------|--------------------|
| `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` |
| `r_handle` | `THREAD_QUERY_INFORMATION` |
| `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` |
| `w_handle` | `THREAD_SET_INFORMATION` |

### Drop behavior

```text
Drop order:
  1. CloseHandle(r_limited_handle)           — always closed
  2. CloseHandle(r_handle)                   — closed only if not invalid
  3. CloseHandle(w_limited_handle)           — closed only if not invalid
  4. CloseHandle(w_handle)                   — closed only if not invalid
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `winapi.rs` |
| Created by | [get_thread_handle](get_thread_handle.md) |
| Used by | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md), [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md), [get_thread_start_address](get_thread_start_address.md) |
| Platform | Windows |
| Privileges | Requires `SeDebugPrivilege` for threads in other user sessions |

## See Also

| Topic | Link |
|-------|------|
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| ProcessHandle | [ProcessHandle](ProcessHandle.md) |
| try_open_thread | [try_open_thread](try_open_thread.md) |
| winapi module overview | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
