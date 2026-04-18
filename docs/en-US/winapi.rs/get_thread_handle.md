# get_thread_handle function (winapi.rs)

Opens multiple thread handles with varying access levels for a given thread ID (TID), returning a [`ThreadHandle`](ThreadHandle.md) RAII wrapper. The `r_limited_handle` (query-limited) is required; if it cannot be obtained, the function returns `None`. The remaining handles (`r_handle`, `w_limited_handle`, `w_handle`) are attempted but may be invalid if the caller lacks sufficient privileges.

## Syntax

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `tid` | `u32` | The thread identifier of the target thread. |
| `pid` | `u32` | The process identifier that owns the thread. Used for error tracking via [`is_new_error`](../logging.rs/is_new_error.md). |
| `process_name` | `&str` | The name of the owning process. Used for error tracking and diagnostic logging. |

## Return value

Returns `Some(ThreadHandle)` if the required `THREAD_QUERY_LIMITED_INFORMATION` handle was opened successfully. Returns `None` if the required handle could not be obtained.

The returned [`ThreadHandle`](ThreadHandle.md) contains:

| Field | Access Right | Required |
|-------|-------------|----------|
| `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` | **Yes** — always valid when `Some` is returned. |
| `r_handle` | `THREAD_QUERY_INFORMATION` | No — may be `HANDLE::default()` (invalid) on failure. |
| `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` | No — may be `HANDLE::default()` (invalid) on failure. |
| `w_handle` | `THREAD_SET_INFORMATION` | No — may be `HANDLE::default()` (invalid) on failure. |

All valid handles are automatically closed when the `ThreadHandle` is dropped.

## Remarks

The function opens handles incrementally using the Windows `OpenThread` API:

1. **`r_limited_handle`** — Opened with `THREAD_QUERY_LIMITED_INFORMATION`. This is the only required handle. If it fails or returns an invalid handle, the function logs the failure via [`log_to_find`](../logging.rs/log_to_find.md) (subject to [`is_new_error`](../logging.rs/is_new_error.md) deduplication) and returns `None`.

2. **`r_handle`** — Opened with `THREAD_QUERY_INFORMATION` via [`try_open_thread`](try_open_thread.md) (internal_op_code `1`). Failure is silent; an invalid handle is stored.

3. **`w_limited_handle`** — Opened with `THREAD_SET_LIMITED_INFORMATION` via [`try_open_thread`](try_open_thread.md) (internal_op_code `2`). Failure is silent; an invalid handle is stored.

4. **`w_handle`** — Opened with `THREAD_SET_INFORMATION` via [`try_open_thread`](try_open_thread.md) (internal_op_code `3`). Failure is silent; an invalid handle is stored.

### Error code mapping for `is_new_error`

| `internal_op_code` | Meaning |
|--------------------|---------|
| `0` | `THREAD_QUERY_LIMITED_INFORMATION` open failure or invalid handle |
| `1` | `THREAD_QUERY_INFORMATION` |
| `2` | `THREAD_SET_LIMITED_INFORMATION` |
| `3` | `THREAD_SET_INFORMATION` |

Non-required handle failures (codes 1–3) are currently commented out in the source for [`try_open_thread`](try_open_thread.md) and do not generate log output.

### Platform notes

- **Windows only.** Uses `OpenThread` from `windows::Win32::System::Threading`.
- Requires the caller to have appropriate privileges. Running as administrator with [`SeDebugPrivilege`](enable_debug_privilege.md) enabled maximizes the chance of obtaining all four handles.
- Protected processes and system threads may deny even `THREAD_QUERY_LIMITED_INFORMATION` access.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs`, `scheduler.rs` |
| **Callees** | [`try_open_thread`](try_open_thread.md), [`is_new_error`](../logging.rs/is_new_error.md), [`log_to_find`](../logging.rs/log_to_find.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | `OpenThread`, `GetLastError` |
| **Privileges** | `SeDebugPrivilege` recommended |

## See Also

| Topic | Link |
|-------|------|
| ThreadHandle struct | [ThreadHandle](ThreadHandle.md) |
| try_open_thread helper | [try_open_thread](try_open_thread.md) |
| get_process_handle | [get_process_handle](get_process_handle.md) |
| Operation enum | [Operation](../logging.rs/Operation.md) |
| is_new_error | [is_new_error](../logging.rs/is_new_error.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
