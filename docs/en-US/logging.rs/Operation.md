# Operation enum (logging.rs)

Enumerates the Windows API operations whose failures are tracked by the [`is_new_error`](is_new_error.md) deduplication system. Each variant corresponds to a specific Win32 or NT-native API call that may fail when AffinityServiceRust attempts to manage process/thread affinity, priority, or I/O settings. The enum is used as part of the composite key in [`ApplyFailEntry`](ApplyFailEntry.md) to ensure that distinct operation failures for the same process/thread are tracked independently.

## Syntax

```rust
#[derive(PartialEq, Eq, Hash)]
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

| Variant | Description |
|---------|-------------|
| `OpenProcess2processQueryLimitedInformation` | `OpenProcess` called with `PROCESS_QUERY_LIMITED_INFORMATION` access. |
| `OpenProcess2processSetLimitedInformation` | `OpenProcess` called with `PROCESS_SET_LIMITED_INFORMATION` access. |
| `OpenProcess2processQueryInformation` | `OpenProcess` called with `PROCESS_QUERY_INFORMATION` access. |
| `OpenProcess2processSetInformation` | `OpenProcess` called with `PROCESS_SET_INFORMATION` access. |
| `OpenThread` | `OpenThread` called with any thread access right. |
| `SetPriorityClass` | `SetPriorityClass` — setting the process priority class. |
| `GetProcessAffinityMask` | `GetProcessAffinityMask` — querying the process affinity bitmask. |
| `SetProcessAffinityMask` | `SetProcessAffinityMask` — setting the process affinity bitmask. |
| `GetProcessDefaultCpuSets` | `GetProcessDefaultCpuSets` — querying the process's default CPU set IDs. |
| `SetProcessDefaultCpuSets` | `SetProcessDefaultCpuSets` — assigning default CPU set IDs to a process. |
| `QueryThreadCycleTime` | `QueryThreadCycleTime` — reading a thread's cumulative cycle count. |
| `SetThreadSelectedCpuSets` | `SetThreadSelectedCpuSets` — assigning CPU set IDs to a specific thread. |
| `SetThreadPriority` | `SetThreadPriority` — setting a thread's scheduling priority level. |
| `NtQueryInformationProcess2ProcessInformationIOPriority` | `NtQueryInformationProcess` with `ProcessIoPriority` information class. |
| `NtSetInformationProcess2ProcessInformationIOPriority` | `NtSetInformationProcess` with `ProcessIoPriority` information class. |
| `GetProcessInformation2ProcessMemoryPriority` | `GetProcessInformation` with `ProcessMemoryPriority` class. |
| `SetProcessInformation2ProcessMemoryPriority` | `SetProcessInformation` with `ProcessMemoryPriority` class. |
| `SetThreadIdealProcessorEx` | `SetThreadIdealProcessorEx` — setting a thread's ideal processor hint. |
| `GetThreadIdealProcessorEx` | `GetThreadIdealProcessorEx` — querying a thread's ideal processor. |
| `InvalidHandle` | Sentinel variant representing an operation where a handle was obtained but found to be invalid. Used by [`get_process_handle`](../winapi.rs/get_process_handle.md) and [`get_thread_handle`](../winapi.rs/get_thread_handle.md) for internal error-code differentiation. |

## Remarks

- The enum derives `PartialEq`, `Eq`, and `Hash`, making it suitable for use as a key component in `HashMap` and `HashSet` collections. Specifically, it is part of the [`ApplyFailEntry`](ApplyFailEntry.md) composite key used by [`is_new_error`](is_new_error.md).

- The variant naming convention follows the pattern `ApiName` or `ApiName2ParameterDescription`, where the `2` separator indicates a specific overload or parameter combination of the underlying Win32 API. For example, `OpenProcess2processQueryLimitedInformation` means "`OpenProcess` called with the `PROCESS_QUERY_LIMITED_INFORMATION` access flag."

- The enum is marked `#[allow(dead_code)]` in the source, indicating that not all variants are currently referenced in the codebase. Some variants are reserved for future use or are used only in commented-out error-reporting paths.

- The `InvalidHandle` variant is a special case — it does not correspond to a specific API call but rather to the scenario where an API call succeeded but returned an invalid handle value. It is used with a secondary `error_code` discriminator (0, 1, 2, 3) to distinguish which specific handle in a multi-handle open operation was invalid.

### Grouping by category

| Category | Variants |
|----------|----------|
| **Process handle acquisition** | `OpenProcess2processQueryLimitedInformation`, `OpenProcess2processSetLimitedInformation`, `OpenProcess2processQueryInformation`, `OpenProcess2processSetInformation` |
| **Thread handle acquisition** | `OpenThread` |
| **Affinity / CPU sets** | `GetProcessAffinityMask`, `SetProcessAffinityMask`, `GetProcessDefaultCpuSets`, `SetProcessDefaultCpuSets`, `SetThreadSelectedCpuSets` |
| **Priority** | `SetPriorityClass`, `SetThreadPriority` |
| **I/O priority** | `NtQueryInformationProcess2ProcessInformationIOPriority`, `NtSetInformationProcess2ProcessInformationIOPriority` |
| **Memory priority** | `GetProcessInformation2ProcessMemoryPriority`, `SetProcessInformation2ProcessMemoryPriority` |
| **Ideal processor** | `SetThreadIdealProcessorEx`, `GetThreadIdealProcessorEx` |
| **Diagnostics** | `QueryThreadCycleTime` |
| **Sentinel** | `InvalidHandle` |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Used by** | [`is_new_error`](is_new_error.md), [`ApplyFailEntry`](ApplyFailEntry.md), [`get_process_handle`](../winapi.rs/get_process_handle.md), [`get_thread_handle`](../winapi.rs/get_thread_handle.md), `apply.rs` |
| **Traits** | `PartialEq`, `Eq`, `Hash` |
| **Platform** | Enum is platform-independent; the operations it names are Windows-specific. |

## See Also

| Topic | Link |
|-------|------|
| ApplyFailEntry struct | [ApplyFailEntry](ApplyFailEntry.md) |
| is_new_error function | [is_new_error](is_new_error.md) |
| purge_fail_map function | [purge_fail_map](purge_fail_map.md) |
| get_process_handle function | [get_process_handle](../winapi.rs/get_process_handle.md) |
| get_thread_handle function | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| logging module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
