# Operation enum (logging.rs)

Enumerates every Windows API operation that AffinityServiceRust may invoke on a target process or thread. Used as a component of the [ApplyFailEntry](ApplyFailEntry.md) deduplication key so that distinct failures on the same process are tracked independently.

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
| `OpenThread` | `OpenThread` for thread-level operations. |
| `SetPriorityClass` | [SetPriorityClass](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass) — sets process priority class. |
| `GetProcessAffinityMask` | [GetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) — queries process CPU affinity. |
| `SetProcessAffinityMask` | [SetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask) — sets process CPU affinity. |
| `GetProcessDefaultCpuSets` | [GetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets) — queries process default CPU sets. |
| `SetProcessDefaultCpuSets` | [SetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets) — sets process default CPU sets. |
| `QueryThreadCycleTime` | [QueryThreadCycleTime](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) — reads thread cycle counter for prime thread selection. |
| `SetThreadSelectedCpuSets` | [SetThreadSelectedCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets) — pins a thread to specific CPU sets. |
| `SetThreadPriority` | [SetThreadPriority](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) — sets thread priority level. |
| `NtQueryInformationProcess2ProcessInformationIOPriority` | `NtQueryInformationProcess` with `ProcessIoPriority` information class — reads IO priority. |
| `NtSetInformationProcess2ProcessInformationIOPriority` | `NtSetInformationProcess` with `ProcessIoPriority` information class — sets IO priority. |
| `GetProcessInformation2ProcessMemoryPriority` | [GetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) with `ProcessMemoryPriority` class. |
| `SetProcessInformation2ProcessMemoryPriority` | [SetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) with `ProcessMemoryPriority` class. |
| `SetThreadIdealProcessorEx` | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) — sets ideal processor hint for a thread. |
| `GetThreadIdealProcessorEx` | [GetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) — queries ideal processor hint for a thread. |
| `InvalidHandle` | Sentinel value indicating that a required handle was not available. |

## Remarks

- The naming convention `Verb2context` (e.g., `OpenProcess2processQueryLimitedInformation`) encodes both the Win32 function name and the access right or information class that was requested. This allows a single enum to disambiguate calls to the same API with different parameters.
- The enum derives `PartialEq`, `Eq`, and `Hash` so it can be used as a key inside [ApplyFailEntry](ApplyFailEntry.md) and stored in `HashMap`/`HashSet` collections.
- `InvalidHandle` is used when the failure occurs before any API call — for example, when a [ProcessHandle](../winapi.rs/ProcessHandle.md) does not carry the required access level.

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | [log_error_if_new](../apply.rs/log_error_if_new.md), all `apply_*` functions in [apply.rs](../apply.rs/README.md) |
| **Dependencies** | None (fieldless enum) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Error dedup key | [ApplyFailEntry](ApplyFailEntry.md) |
| First-occurrence check | [is_new_error](is_new_error.md) |
| Apply module | [apply.rs](../apply.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*