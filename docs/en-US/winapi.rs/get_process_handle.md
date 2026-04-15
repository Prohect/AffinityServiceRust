# get_process_handle function (winapi.rs)

Opens multiple Windows process handles with varying access levels for a given process ID. Returns a [`ProcessHandle`](ProcessHandle.md) RAII wrapper that automatically closes all valid handles when dropped.

## Syntax

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process. |
| `process_name` | `&str` | The name of the target process, used for error-tracking and log messages. |

## Return value

Returns `Some(ProcessHandle)` if at least the limited-access handles (`r_limited_handle` and `w_limited_handle`) were opened successfully. Returns `None` if either limited handle could not be obtained.

The returned [`ProcessHandle`](ProcessHandle.md) contains:

| Field | Access Right | Required |
|-------|-------------|----------|
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | Yes — failure returns `None`. |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | Yes — failure returns `None`. |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | No — `None` on failure. |
| `w_handle` | `PROCESS_SET_INFORMATION` | No — `None` on failure. |

## Remarks

The function attempts to open four separate handles with progressively higher privilege requirements. The two limited handles (`PROCESS_QUERY_LIMITED_INFORMATION` and `PROCESS_SET_LIMITED_INFORMATION`) are **required** — if either fails, the function logs the error and returns `None`. The two full-access handles (`PROCESS_QUERY_INFORMATION` and `PROCESS_SET_INFORMATION`) are **optional** — failures are silently tolerated, and the corresponding fields are set to `None`.

Error deduplication is performed via [`is_new_error`](../logging.rs/is_new_error.md) so that repeated failures for the same PID/process/operation/error-code combination are logged only once. The internal error-code mapping for `is_new_error` is:

| Code | Handle |
|------|--------|
| `0` | `PROCESS_QUERY_LIMITED_INFORMATION` |
| `1` | `PROCESS_SET_LIMITED_INFORMATION` |
| `2` | `PROCESS_QUERY_INFORMATION` |
| `3` | `PROCESS_SET_INFORMATION` |

If a handle is obtained but reports as invalid via `HANDLE::is_invalid()`, it is also treated as a failure with the `Operation::InvalidHandle` variant.

All successfully opened handles are owned by the returned [`ProcessHandle`](ProcessHandle.md) and are automatically closed via its `Drop` implementation. If the function returns `None`, any partially-opened handles are closed before returning.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` rule application logic |
| **Callees** | `OpenProcess` (Win32), [`is_new_error`](../logging.rs/is_new_error.md), [`log_to_find`](../logging.rs/log_to_find.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **API** | Win32 `OpenProcess`, `GetLastError`, `CloseHandle` |
| **Privileges** | `SeDebugPrivilege` recommended for opening protected/elevated processes. |

## See Also

| Topic | Link |
|-------|------|
| ProcessHandle struct | [ProcessHandle](ProcessHandle.md) |
| get_thread_handle function | [get_thread_handle](get_thread_handle.md) |
| is_new_error function | [is_new_error](../logging.rs/is_new_error.md) |
| Operation enum | [Operation](../logging.rs/Operation.md) |
| error_from_code_win32 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
