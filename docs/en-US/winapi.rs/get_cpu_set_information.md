# get_cpu_set_information function (winapi.rs)

Returns a reference to the global [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) static, ensuring it is initialized on first access.

## Syntax

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

## Parameters

This function takes no parameters.

## Return value

Returns a `&'static Mutex<Vec<CpuSetData>>` — a reference to the lazily-initialized, mutex-guarded vector of [`CpuSetData`](CpuSetData.md) entries representing all CPU sets on the system.

## Remarks

This function is a thin accessor that returns a reference to the [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) static. On first call, the static is initialized by querying `GetSystemCpuSetInformation`, which populates the vector with one [`CpuSetData`](CpuSetData.md) entry per logical processor on the system.

Callers must lock the returned mutex before reading the vector contents. The lock should be held for the minimum time necessary to avoid contention.

This function is used throughout the codebase wherever CPU set ID lookups or conversions are needed, including [`cpusetids_from_indices`](cpusetids_from_indices.md), [`cpusetids_from_mask`](cpusetids_from_mask.md), [`indices_from_cpusetids`](indices_from_cpusetids.md), and [`mask_from_cpusetids`](mask_from_cpusetids.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L353–L355 |
| **Called by** | [`cpusetids_from_indices`](cpusetids_from_indices.md), [`cpusetids_from_mask`](cpusetids_from_mask.md), [`indices_from_cpusetids`](indices_from_cpusetids.md), [`mask_from_cpusetids`](mask_from_cpusetids.md) |
| **Windows API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemcpusetinformation) (via lazy initialization) |

## See also

- [CPU_SET_INFORMATION static](CPU_SET_INFORMATION.md)
- [CpuSetData struct](CpuSetData.md)
- [cpusetids_from_indices](cpusetids_from_indices.md)
- [cpusetids_from_mask](cpusetids_from_mask.md)