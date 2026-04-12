# get_process_handle function (winapi.rs)

Opens a target process and returns a [`ProcessHandle`](ProcessHandle.md) containing read and write HANDLEs at both limited and full access levels.

## Syntax

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## Parameters

`pid`

The process identifier of the target process to open.

`process_name`

The name of the target process, used for error logging when handle acquisition fails.

## Return value

Returns `Some(ProcessHandle)` if at least the limited-access handles could be opened successfully. Returns `None` if the process could not be opened at all.

## Remarks

This function attempts to open the target process with multiple access levels, building a [`ProcessHandle`](ProcessHandle.md) with as many capabilities as the caller's privileges allow:

1. **Read limited** (`PROCESS_QUERY_LIMITED_INFORMATION`) — always required; if this fails, the function returns `None`.
2. **Read full** (`PROCESS_QUERY_INFORMATION`) — optional; set to `None` for protected processes that deny full query access.
3. **Write limited** (`PROCESS_SET_LIMITED_INFORMATION`) — always required; if this fails, the function returns `None`.
4. **Write full** (`PROCESS_SET_INFORMATION`) — optional; set to `None` for protected processes that deny full set access.

The limited handles are sufficient for most operations (priority, affinity mask). Full handles are needed for advanced operations such as CPU set assignment and I/O/memory priority via `NtSetInformationProcess`.

Errors during handle acquisition are logged via [`is_new_error`](../logging.rs/is_new_error.md) with the corresponding [`Operation`](../logging.rs/Operation.md) variant to ensure deduplication. Error codes are translated using [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md).

The returned [`ProcessHandle`](ProcessHandle.md) implements `Drop`, which automatically closes all opened handles when the value goes out of scope.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L96–L195 |
| **Called by** | [`apply_config`](../main.rs/apply_config.md) in main.rs, [`is_affinity_unset`](is_affinity_unset.md) |
| **Calls** | [`is_new_error`](../logging.rs/is_new_error.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Windows API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |

## See also

- [ProcessHandle](ProcessHandle.md)
- [get_thread_handle](get_thread_handle.md)
- [winapi.rs module overview](README.md)