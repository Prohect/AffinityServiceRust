# try_open_thread function (winapi.rs)

Attempts to open a thread handle with a specific access right, performing deduplicated error logging on failure.

## Syntax

```rust
pub fn try_open_thread(
    pid: u32,
    tid: u32,
    process_name: &str,
    access: THREAD_ACCESS_RIGHTS,
    internal_op_code: u32,
) -> HANDLE
```

## Parameters

`pid`

The process identifier that owns the thread. Used for error logging context only.

`tid`

The thread identifier to open.

`process_name`

The name of the process that owns the thread. Used for error logging context only.

`access`

The desired access rights for the thread handle, specified as a `THREAD_ACCESS_RIGHTS` flags value (e.g., `THREAD_QUERY_LIMITED_INFORMATION`, `THREAD_SET_INFORMATION`).

`internal_op_code`

An internal operation code that maps to an [`Operation`](../logging.rs/Operation.md) variant for error deduplication. This allows the caller to distinguish between different logical operations that open threads with the same access rights.

## Return value

Returns a `HANDLE` to the opened thread on success. Returns an invalid handle on failure (after logging the error).

## Remarks

This is a lower-level helper used by [`get_thread_handle`](get_thread_handle.md) to open individual thread handles at different access levels. It wraps the Windows `OpenThread` API call with integrated error handling.

On failure, the function calls [`is_new_error`](../logging.rs/is_new_error.md) with the pid, tid, process name, mapped operation, and Win32 error code. The error is only logged if it has not been previously recorded for the same combination, preventing log spam when the same thread repeatedly fails to open (e.g., due to access restrictions on protected processes).

The `internal_op_code` parameter is translated to an [`Operation`](../logging.rs/Operation.md) enum variant to identify the specific logical operation being attempted. This allows the deduplication system to distinguish between, for example, opening a thread for query access versus opening the same thread for set access.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L273–L301 |
| **Called by** | [get_thread_handle](get_thread_handle.md) |
| **Calls** | [`is_new_error`](../logging.rs/is_new_error.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Windows API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread), [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |

## See also

- [get_thread_handle](get_thread_handle.md)
- [ThreadHandle](ThreadHandle.md)
- [Operation enum](../logging.rs/Operation.md)