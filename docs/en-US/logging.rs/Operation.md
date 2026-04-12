# Operation enum (logging.rs)

Enumerates all Windows API operations that can produce errors during configuration application. Each variant represents a specific API call or logical operation, enabling precise error deduplication and human-readable error reporting.

## Syntax

```rust
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operation {
    OpenProcess2processQueryLimitedInformation,
    OpenProcess2processSetLimitedInformation,
    OpenProcess2processQueryInformation,
    OpenProcess2processSetInformation,
    OpenThread,
    SetPriorityClass,
    GetProcessAffinityMask,
    SetProcessAffinityMask,
    GetProcessDefaultCpuSets,
    SetProcessDefaultCpuSets,
    QueryThreadCycleTime,
    SetThreadSelectedCpuSets,
    SetThreadPriority,
    NtQueryInformationProcess2ProcessInformationIOPriority,
    NtSetInformationProcess2ProcessInformationIOPriority,
    GetProcessInformation2ProcessMemoryPriority,
    SetProcessInformation2ProcessMemoryPriority,
    SetThreadIdealProcessorEx,
    GetThreadIdealProcessorEx,
    InvalidHandle,
}
```

## Members

`OpenProcess2processQueryLimitedInformation`

Failed to open a process with `PROCESS_QUERY_LIMITED_INFORMATION` access.

`OpenProcess2processSetLimitedInformation`

Failed to open a process with `PROCESS_SET_LIMITED_INFORMATION` access.

`OpenProcess2processQueryInformation`

Failed to open a process with `PROCESS_QUERY_INFORMATION` access.

`OpenProcess2processSetInformation`

Failed to open a process with `PROCESS_SET_INFORMATION` access.

`OpenThread`

Failed to open a thread handle via `OpenThread`.

`SetPriorityClass`

Failed to set the process priority class via `SetPriorityClass`.

`GetProcessAffinityMask`

Failed to query the process affinity mask via `GetProcessAffinityMask`.

`SetProcessAffinityMask`

Failed to set the process affinity mask via `SetProcessAffinityMask`.

`GetProcessDefaultCpuSets`

Failed to query the process default CPU sets via `GetProcessDefaultCpuSets`.

`SetProcessDefaultCpuSets`

Failed to set the process default CPU sets via `SetProcessDefaultCpuSets`.

`QueryThreadCycleTime`

Failed to query thread cycle time via `QueryThreadCycleTime`.

`SetThreadSelectedCpuSets`

Failed to set the selected CPU sets for a thread via `SetThreadSelectedCpuSets`.

`SetThreadPriority`

Failed to set the thread priority via `SetThreadPriority`.

`NtQueryInformationProcess2ProcessInformationIOPriority`

Failed to query the I/O priority of a process via `NtQueryInformationProcess` with `ProcessIoPriority` information class.

`NtSetInformationProcess2ProcessInformationIOPriority`

Failed to set the I/O priority of a process via `NtSetInformationProcess` with `ProcessIoPriority` information class.

`GetProcessInformation2ProcessMemoryPriority`

Failed to query the memory priority of a process via `GetProcessInformation` with `ProcessMemoryPriority` information class.

`SetProcessInformation2ProcessMemoryPriority`

Failed to set the memory priority of a process via `SetProcessInformation` with `ProcessMemoryPriority` information class.

`SetThreadIdealProcessorEx`

Failed to set the ideal processor for a thread via `SetThreadIdealProcessorEx`.

`GetThreadIdealProcessorEx`

Failed to query the ideal processor for a thread via `GetThreadIdealProcessorEx`.

`InvalidHandle`

A sentinel variant indicating that the operation involved an invalid or null handle. Used when an error occurs due to a handle that was expected to be valid but was not (e.g., a protected process whose full-access handle is `None`).

## Remarks

`Operation` serves as a component of the composite key used by the error deduplication system. Each error is recorded as an [`ApplyFailEntry`](ApplyFailEntry.md) containing the `Operation` variant along with the PID, TID, process name, and error code. The [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) stores these entries, and [`is_new_error`](is_new_error.md) checks membership to determine whether a given error has already been logged.

The enum derives `Hash` and `Eq` so it can be used as part of `HashMap` and `HashSet` keys. It also derives `Clone` and `Debug` for convenience in logging and diagnostics.

### Naming convention

The variant names follow a pattern that encodes both the Windows API function and the specific access right or information class involved:

- `OpenProcess2process...` — `OpenProcess` with the specified access right suffix.
- `NtQueryInformationProcess2ProcessInformation...` — `NtQueryInformationProcess` with the specified information class.
- `Get/SetProcessInformation2ProcessMemoryPriority` — `GetProcessInformation`/`SetProcessInformation` with `ProcessMemoryPriority`.

This naming convention makes error log entries self-descriptive without needing to look up operation codes.

### Usage in apply functions

Each `apply_*` function in [`apply.rs`](../apply.rs/README.md) uses the appropriate `Operation` variant when calling [`log_error_if_new`](../apply.rs/log_error_if_new.md), which in turn calls [`is_new_error`](is_new_error.md). Similarly, [`get_process_handle`](../winapi.rs/get_process_handle.md) and [`try_open_thread`](../winapi.rs/try_open_thread.md) use the `OpenProcess*` and `OpenThread` variants respectively.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Lines** | L74–L95 |
| **Used by** | [`ApplyFailEntry`](ApplyFailEntry.md), [`is_new_error`](is_new_error.md), [`log_error_if_new`](../apply.rs/log_error_if_new.md), [`get_process_handle`](../winapi.rs/get_process_handle.md), [`try_open_thread`](../winapi.rs/try_open_thread.md) |

## See also

- [ApplyFailEntry struct](ApplyFailEntry.md)
- [is_new_error function](is_new_error.md)
- [PID_MAP_FAIL_ENTRY_SET static](PID_MAP_FAIL_ENTRY_SET.md)
- [logging.rs module overview](README.md)