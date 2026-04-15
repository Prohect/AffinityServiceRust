# try_open_thread function (winapi.rs)

Attempts to open a single thread handle with the specified access rights. This is an internal helper used by [get_thread_handle](get_thread_handle.md) to obtain optional thread handles (read-full, write-limited, write-full) without failing the entire operation if any individual handle cannot be acquired.

## Syntax

```rust
fn try_open_thread(
    pid: u32,
    tid: u32,
    process_name: &str,
    access: THREAD_ACCESS_RIGHTS,
    internal_op_code: u32,
) -> HANDLE
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID that owns the thread. Used for error tracking context. |
| `tid` | `u32` | The thread ID to open. |
| `process_name` | `&str` | The name of the owning process. Used for error tracking context. |
| `access` | `THREAD_ACCESS_RIGHTS` | The desired access rights for the thread handle (e.g., `THREAD_QUERY_INFORMATION`, `THREAD_SET_LIMITED_INFORMATION`, `THREAD_SET_INFORMATION`). |
| `internal_op_code` | `u32` | An internal operation code used to differentiate which handle type failed in error tracking. See the mapping table below. |

### internal_op_code mapping

| Code | Handle Type |
|------|-------------|
| `1` | `r_handle` (`THREAD_QUERY_INFORMATION`) |
| `2` | `w_limited_handle` (`THREAD_SET_LIMITED_INFORMATION`) |
| `3` | `w_handle` (`THREAD_SET_INFORMATION`) |

## Return value

Returns a `HANDLE`. On success, this is a valid open thread handle. On failure, returns `HANDLE::default()` (an invalid handle). The caller is responsible for checking handle validity before use.

## Remarks

- This function is marked `#[inline(always)]` to eliminate call overhead, as it is invoked multiple times per thread in the hot path of [get_thread_handle](get_thread_handle.md).
- Unlike the required `r_limited_handle` in `get_thread_handle`, failure from `try_open_thread` does **not** cause `get_thread_handle` to return `None`. The returned invalid handle is stored directly in the [ThreadHandle](ThreadHandle.md) struct, and callers must check validity before using it.
- Error logging through [is_new_error](../logging.rs/is_new_error.md) is currently commented out in the implementation to reduce log noise for expected failures (e.g., elevated processes that deny `THREAD_SET_INFORMATION`).
- Contains an inner helper function `error_detail` that maps `internal_op_code` values to human-readable handle-type names for diagnostic purposes.
- This function is **not** public (`fn` without `pub`), making it internal to the `winapi` module.
- Calls `OpenThread` from the Windows API via the `windows` crate.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Visibility** | Private (module-internal) |
| **Callers** | [get_thread_handle](get_thread_handle.md) |
| **Callees** | `OpenThread` (Win32 API) |
| **API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |
| **Privileges** | Requires sufficient access rights on the target thread; typically needs `SeDebugPrivilege` for system-level threads. |

## See Also

| Topic | Link |
|-------|------|
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| ThreadHandle struct | [ThreadHandle](ThreadHandle.md) |
| is_new_error | [is_new_error](../logging.rs/is_new_error.md) |
| Operation enum | [Operation](../logging.rs/Operation.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
