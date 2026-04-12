# ProcessHandle struct (winapi.rs)

Holds read and write Windows `HANDLE`s to a process, providing both limited and full access variants. Implements `Drop` to automatically close all held handles when the struct goes out of scope.

## Syntax

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
```

## Members

`r_limited_handle`

A read-only handle opened with `PROCESS_QUERY_LIMITED_INFORMATION` access. This handle is always present and valid, as limited query access is available even for protected processes.

`r_handle`

A read-only handle opened with `PROCESS_QUERY_INFORMATION` access. This is `None` for protected processes where full query access is denied.

`w_limited_handle`

A write handle opened with `PROCESS_SET_LIMITED_INFORMATION` access. This handle is always present and is sufficient for operations like `SetPriorityClass`.

`w_handle`

A write handle opened with `PROCESS_SET_INFORMATION` access. This is `None` for protected processes where full set access is denied. Required for operations like `SetProcessAffinityMask`, `SetProcessDefaultCpuSets`, and `SetProcessInformation`.

## Remarks

`ProcessHandle` is created by [`get_process_handle`](get_process_handle.md), which attempts to open the process with all four access levels. The limited handles (`r_limited_handle`, `w_limited_handle`) always succeed for accessible processes, while the full handles (`r_handle`, `w_handle`) may fail for protected or system processes, in which case they are set to `None`.

The struct implements `Drop`, which calls `CloseHandle` on all held handles when the `ProcessHandle` goes out of scope. This ensures that kernel handle resources are not leaked even when errors occur during configuration application.

Callers typically use [`get_handles`](../apply.rs/get_handles.md) in the apply module to extract the appropriate read and write `HANDLE` values, falling back from full handles to limited handles as needed.

### Access level usage

| Handle | Access right | Used for |
| --- | --- | --- |
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | `GetPriorityClass`, `GetProcessAffinityMask` (fallback) |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | `GetProcessAffinityMask`, `GetProcessDefaultCpuSets`, `NtQueryInformationProcess` |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | `SetPriorityClass` |
| `w_handle` | `PROCESS_SET_INFORMATION` | `SetProcessAffinityMask`, `SetProcessDefaultCpuSets`, `NtSetInformationProcess`, `SetProcessInformation` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Lines** | L68–L75 |
| **Created by** | [`get_process_handle`](get_process_handle.md) |
| **Consumed by** | [`apply_config`](../main.rs/apply_config.md), [`get_handles`](../apply.rs/get_handles.md), [`is_affinity_unset`](is_affinity_unset.md) |

## See also

- [get_process_handle](get_process_handle.md)
- [ThreadHandle](ThreadHandle.md)
- [winapi.rs module overview](README.md)