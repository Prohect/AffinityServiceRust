# try_open_thread function (winapi.rs)

Attempts to open a thread handle with a single specific access right. Returns an invalid `HANDLE` on failure instead of `None`, allowing the caller to store the result directly in a [ThreadHandle](ThreadHandle.md) struct where non-required handles may be invalid.

## Syntax

```rust
#[inline(always)]
#[allow(unused_variables)]
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
| `pid` | `u32` | The process ID that owns the thread. Used for error logging context. |
| `tid` | `u32` | The thread ID to open. Passed to `OpenThread`. |
| `process_name` | `&str` | Display name of the owning process. Used for error logging context. |
| `access` | `THREAD_ACCESS_RIGHTS` | The desired access right to request. Typical values are `THREAD_QUERY_INFORMATION`, `THREAD_SET_LIMITED_INFORMATION`, or `THREAD_SET_INFORMATION`. |
| `internal_op_code` | `u32` | Numeric code identifying which handle slot is being opened, used by the internal `error_detail` helper and the `is_new_error` deduplication system. Codes: `1` = `r_handle`, `2` = `w_limited_handle`, `3` = `w_handle`. |

## Return value

| Value | Meaning |
|-------|---------|
| Valid `HANDLE` | The thread was successfully opened with the requested access right. |
| `HANDLE::default()` | The open attempt failed (either `OpenThread` returned an error, or the returned handle was invalid). This is an invalid handle sentinel. |

## Remarks

This is a module-private helper called by [get_thread_handle](get_thread_handle.md) for the three non-required handle slots (`r_handle`, `w_limited_handle`, `w_handle`). The required `r_limited_handle` is opened directly in `get_thread_handle` because its failure aborts the entire operation.

### Design rationale

Unlike [get_process_handle](get_process_handle.md), which uses `Option<HANDLE>` for optional handles, [ThreadHandle](ThreadHandle.md) stores all four handles as bare `HANDLE` values. The `try_open_thread` function returns `HANDLE::default()` (an invalid handle) on failure, which the `ThreadHandle::Drop` implementation checks before calling `CloseHandle`. This avoids wrapping every thread handle in `Option` while still providing safe cleanup.

### Error logging

The function contains commented-out calls to `is_new_error` and `log_to_find`. These are retained in the source for diagnostic purposes but are disabled in production to reduce log noise from the many non-critical handle failures that occur when threads exit between enumeration and handle opening.

The inner `error_detail` helper function maps `internal_op_code` values to human-readable handle names:

| `internal_op_code` | Handle name |
|--------------------|-------------|
| `1` | `r_handle` |
| `2` | `w_limited_handle` |
| `3` | `w_handle` |

### Failure behavior

When `OpenThread` fails or returns an invalid handle, the function silently returns `HANDLE::default()`. The caller ([get_thread_handle](get_thread_handle.md)) still succeeds — it stores the invalid handle in the [ThreadHandle](ThreadHandle.md), and subsequent code must check `is_invalid()` before using that particular handle.

### Inlining

The function is marked `#[inline(always)]` because it is a thin wrapper around a single Windows API call invoked in a hot path during thread enumeration.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | Module-private (`fn`, no `pub`) |
| **Callers** | [get_thread_handle](get_thread_handle.md) |
| **API** | `OpenThread` (`kernel32.dll` / `windows` crate `Win32::System::Threading`) |
| **Privileges** | `SeDebugPrivilege` recommended for cross-process thread access |

## See Also

| Topic | Link |
|-------|------|
| Thread handle container | [ThreadHandle](ThreadHandle.md) |
| Full thread handle acquisition | [get_thread_handle](get_thread_handle.md) |
| Process handle acquisition (analogous pattern) | [get_process_handle](get_process_handle.md) |
| Error deduplication | [is_new_error](../logging.rs/README.md) |
| OpenThread (Microsoft Learn) | [OpenThread function](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd