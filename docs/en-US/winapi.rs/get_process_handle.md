# get_process_handle function (winapi.rs)

Opens a set of process handles at multiple access levels for the given process ID. Returns a [ProcessHandle](ProcessHandle.md) RAII wrapper that automatically closes all handles on drop. The function attempts to open four handles with progressively higher privilege requirements; the two limited-access handles are required, while the two full-access handles are optional and degrade gracefully.

## Syntax

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process. |
| `process_name` | `&str` | The process image name, used only for error logging. Passed through to [is_new_error](../logging.rs/is_new_error.md) for error deduplication. |

## Return value

`Option<ProcessHandle>` — Returns `Some(ProcessHandle)` if both required limited-access handles were successfully opened. Returns `None` if either `PROCESS_QUERY_LIMITED_INFORMATION` or `PROCESS_SET_LIMITED_INFORMATION` could not be obtained.

When `Some`, the returned [ProcessHandle](ProcessHandle.md) has the following guarantees:

| Field | Guarantee |
|-------|-----------|
| `r_limited_handle` | Always valid (`PROCESS_QUERY_LIMITED_INFORMATION`) |
| `w_limited_handle` | Always valid (`PROCESS_SET_LIMITED_INFORMATION`) |
| `r_handle` | `Some(HANDLE)` if `PROCESS_QUERY_INFORMATION` succeeded, `None` otherwise |
| `w_handle` | `Some(HANDLE)` if `PROCESS_SET_INFORMATION` succeeded, `None` otherwise |

## Remarks

### Handle acquisition order

The function opens handles in the following order, stopping and returning `None` if a required handle fails:

| Step | Access right | Required | Internal error code | On failure |
|------|-------------|----------|---------------------|------------|
| 1 | `PROCESS_QUERY_LIMITED_INFORMATION` | **Yes** | `0` | Log via [is_new_error](../logging.rs/is_new_error.md), return `None` |
| 2 | `PROCESS_SET_LIMITED_INFORMATION` | **Yes** | `1` | Close step 1 handle, log, return `None` |
| 3 | `PROCESS_QUERY_INFORMATION` | No | `2` | Set `r_handle = None`, continue |
| 4 | `PROCESS_SET_INFORMATION` | No | `3` | Set `w_handle = None`, continue |

Steps 3 and 4 require higher privilege (typically SeDebugPrivilege for protected processes). Their failure is expected for system processes and is silently absorbed — the error logging for these steps is commented out in the source code.

### Error deduplication

Failures for required handles (steps 1–2) are logged only the first time a unique `(pid, error_code)` combination is seen, via [is_new_error](../logging.rs/is_new_error.md). This prevents log flooding when a protected process is encountered repeatedly across polling iterations.

### Invalid handle checks

After each successful `OpenProcess` call, the returned handle is checked with `is_invalid()`. An invalid handle (despite a successful API return) is treated as a distinct failure case with its own `Operation::InvalidHandle` error code, ensuring it is logged separately from OS-level errors.

### Handle cleanup on partial failure

If step 1 succeeds but step 2 fails, the step 1 handle is explicitly closed before returning `None`. This prevents handle leaks on early exit paths. When the full [ProcessHandle](ProcessHandle.md) is constructed and returned, its `Drop` implementation handles cleanup.

### Callers' handle selection

Downstream functions (e.g., [get_handles](../apply.rs/get_handles.md)) prefer the full-access handles (`r_handle`, `w_handle`) when available, falling back to the limited handles. This tiered approach allows the service to function with reduced capability on protected processes rather than failing entirely.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [apply.rs](../apply.rs/README.md) (main apply loop opens handles per process) |
| **Callees** | `OpenProcess` (Win32), [is_new_error](../logging.rs/is_new_error.md), [log_to_find](../logging.rs/log_to_find.md), [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror), [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | Limited handles require no special privilege for most processes. Full handles (`PROCESS_QUERY_INFORMATION`, `PROCESS_SET_INFORMATION`) require [SeDebugPrivilege](enable_debug_privilege.md) for protected/system processes. |

## See Also

| Topic | Link |
|-------|------|
| RAII handle wrapper returned by this function | [ProcessHandle](ProcessHandle.md) |
| Thread handle equivalent | [get_thread_handle](get_thread_handle.md) |
| Handle accessor in apply module | [get_handles](../apply.rs/get_handles.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |
| Module overview | [winapi.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*