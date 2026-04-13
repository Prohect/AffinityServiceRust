# Operation enum (logging.rs)

Identifies each distinct Windows API operation that can fail during rule application to a running process. `Operation` variants serve as keys in the [ApplyFailEntry](ApplyFailEntry.md) composite key, enabling the error deduplication system in [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) to distinguish between failures from different API calls on the same process. Each variant corresponds to a specific Win32 or NT native API call (or a specific access-rights variant of one), allowing the service to log the first occurrence of each unique failure while suppressing subsequent identical errors.

## Syntax

```logging.rs
#[derive(PartialEq, Eq, Hash)]
#[allow(dead_code)]
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

| Variant | Win32 / NT API | Description |
|---------|---------------|-------------|
| `OpenProcess2processQueryLimitedInformation` | `OpenProcess` with `PROCESS_QUERY_LIMITED_INFORMATION` | Failed to open a process handle for limited query access. |
| `OpenProcess2processSetLimitedInformation` | `OpenProcess` with `PROCESS_SET_LIMITED_INFORMATION` | Failed to open a process handle for limited set access. |
| `OpenProcess2processQueryInformation` | `OpenProcess` with `PROCESS_QUERY_INFORMATION` | Failed to open a process handle for full query access. |
| `OpenProcess2processSetInformation` | `OpenProcess` with `PROCESS_SET_INFORMATION` | Failed to open a process handle for full set access. |
| `OpenThread` | `OpenThread` | Failed to open a thread handle. |
| `SetPriorityClass` | `SetPriorityClass` | Failed to set the process priority class. |
| `GetProcessAffinityMask` | `GetProcessAffinityMask` | Failed to query the current process affinity mask. |
| `SetProcessAffinityMask` | `SetProcessAffinityMask` | Failed to set the process affinity mask. |
| `GetProcessDefaultCpuSets` | `GetProcessDefaultCpuSets` | Failed to query the process's default CPU set assignment. |
| `SetProcessDefaultCpuSets` | `SetProcessDefaultCpuSets` | Failed to set the process's default CPU set assignment. |
| `QueryThreadCycleTime` | `QueryThreadCycleTime` | Failed to read a thread's cycle time counter (used by the prime-thread scheduler). |
| `SetThreadSelectedCpuSets` | `SetThreadSelectedCpuSets` | Failed to assign selected CPU sets to a thread. |
| `SetThreadPriority` | `SetThreadPriority` | Failed to set a thread's scheduling priority. |
| `NtQueryInformationProcess2ProcessInformationIOPriority` | `NtQueryInformationProcess` (I/O priority class) | Failed to query the process's I/O priority via the NT native API. |
| `NtSetInformationProcess2ProcessInformationIOPriority` | `NtSetInformationProcess` (I/O priority class) | Failed to set the process's I/O priority via the NT native API. |
| `GetProcessInformation2ProcessMemoryPriority` | `GetProcessInformation` (`ProcessMemoryPriority`) | Failed to query the process's memory priority. |
| `SetProcessInformation2ProcessMemoryPriority` | `SetProcessInformation` (`ProcessMemoryPriority`) | Failed to set the process's memory priority. |
| `SetThreadIdealProcessorEx` | `SetThreadIdealProcessorEx` | Failed to set a thread's ideal processor. |
| `GetThreadIdealProcessorEx` | `GetThreadIdealProcessorEx` | Failed to query a thread's ideal processor. |
| `InvalidHandle` | *(none)* | Sentinel variant indicating that a required handle was invalid or null before the API call was attempted. |

## Remarks

- The enum derives `PartialEq`, `Eq`, and `Hash` so that it can be used as part of the [ApplyFailEntry](ApplyFailEntry.md) composite key inside `HashMap` and `HashSet` data structures. It does **not** derive `Debug` or `Clone`.
- The `#[allow(dead_code)]` attribute suppresses unused-variant warnings. Not every variant is actively used in the current codebase — some exist for future use or for completeness of the API surface coverage.
- The naming convention uses `2` as a separator to encode "with" or "for" relationships. For example, `OpenProcess2processQueryLimitedInformation` reads as "OpenProcess **for** PROCESS_QUERY_LIMITED_INFORMATION access." Similarly, `NtSetInformationProcess2ProcessInformationIOPriority` reads as "NtSetInformationProcess **with** ProcessInformation class I/O Priority." This convention avoids ambiguity when a single Win32 function is called with different access rights or information classes that can fail independently.
- The `OpenProcess` call is split into four variants because the service opens handles with different access rights for different operations (read-limited, write-limited, read-full, write-full). A failure to open with `PROCESS_SET_INFORMATION` is a distinct error from a failure to open with `PROCESS_QUERY_LIMITED_INFORMATION`, and both can occur for the same PID.
- The `InvalidHandle` variant represents a pre-call failure — the handle that would have been passed to an API was already known to be invalid (e.g., `NULL` from a prior failed `OpenProcess` call). This allows the deduplication system to suppress repeated log messages about cascading failures that all stem from the same root cause (a failed handle acquisition).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Traits | `PartialEq`, `Eq`, `Hash` |
| Used in | [ApplyFailEntry](ApplyFailEntry.md) (as a field), [is_new_error](is_new_error.md) (as a parameter) |
| Callers | [log_error_if_new](../apply.rs/log_error_if_new.md), [get_process_handle](../winapi.rs/get_process_handle.md), [get_thread_handle](../winapi.rs/get_thread_handle.md), [apply_priority](../apply.rs/apply_priority.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md), [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

## See Also

| Topic | Link |
|-------|------|
| Composite failure key using this enum | [ApplyFailEntry](ApplyFailEntry.md) |
| Error deduplication logic | [is_new_error](is_new_error.md) |
| Per-PID failure tracking map | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Stale entry cleanup | [purge_fail_map](purge_fail_map.md) |
| Win32 error code translation | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| NTSTATUS error code translation | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| logging module overview | [logging module](README.md) |