# is_affinity_unset function (winapi.rs)

Checks whether a process has its default (all-cores) affinity mask, indicating that no custom affinity has been applied.

## Syntax

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## Parameters

`pid`

The process identifier of the target process to query.

`process_name`

The name of the target process. Used for error logging context and for opening the process handle via [`get_process_handle`](get_process_handle.md).

## Return value

Returns `true` if the process's current affinity mask matches the system affinity mask (i.e., all logical processors are enabled). Returns `false` if the process has a custom affinity mask set, or if the query fails.

## Remarks

This function opens a temporary [`ProcessHandle`](ProcessHandle.md) via [`get_process_handle`](get_process_handle.md), then calls `GetProcessAffinityMask` to retrieve both the process affinity mask and the system affinity mask. If the two masks are equal, the process has not had its affinity restricted and the function returns `true`.

The comparison is straightforward: an "unset" affinity means the process can run on all processors that the system allows. Any process whose mask differs from the system mask has been explicitly constrained, either by this application or by another tool.

This check is used during configuration application to determine whether an affinity change is needed. If the process already has a custom affinity that differs from the configured value, it may indicate that another tool or the process itself has set an affinity. The function allows the apply logic to distinguish between "never modified" and "already modified" states.

If `get_process_handle` returns `None` (e.g., the process exited or access was denied), the function returns `false` as a conservative default, which will cause the caller to attempt the affinity application and handle any resulting errors through the normal path.

If `GetProcessAffinityMask` fails, the error is logged via [`is_new_error`](../logging.rs/is_new_error.md) with deduplicated error handling, and the function returns `false`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L583–L635 |
| **Called by** | [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_config`](../main.rs/apply_config.md) |
| **Calls** | [`get_process_handle`](get_process_handle.md), [`is_new_error`](../logging.rs/is_new_error.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Windows API** | [GetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |

## See also

- [ProcessHandle](ProcessHandle.md)
- [apply_affinity](../apply.rs/apply_affinity.md)
- [filter_indices_by_mask](filter_indices_by_mask.md)
- [winapi.rs module overview](README.md)