# indices_from_cpusetids function (winapi.rs)

Converts a slice of CPU set IDs back to their corresponding logical processor indices.

## Syntax

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
```

## Parameters

`cpuids`

A slice of CPU set IDs as returned by Windows CPU set APIs (e.g., `GetProcessDefaultCpuSets`). Each ID is an opaque identifier assigned by the system during CPU set enumeration.

## Return value

Returns a `Vec<u32>` containing the zero-based logical processor indices that correspond to the provided CPU set IDs. The order of the output matches the order of the input. IDs that do not match any known CPU set entry are silently omitted.

## Remarks

This function is the inverse of [`cpusetids_from_indices`](cpusetids_from_indices.md). It looks up each CPU set ID in the global [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) vector (accessed via [`get_cpu_set_information`](get_cpu_set_information.md)) and returns the `logical_processor_index` field of the matching [`CpuSetData`](CpuSetData.md) entry.

The returned indices are zero-based and correspond directly to bit positions in an affinity mask and to the processor numbers used in configuration files.

This function acquires the [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) mutex lock for the duration of the lookup. The lock is released when the function returns.

### Conversion family

| Function | From | To |
| --- | --- | --- |
| [cpusetids_from_indices](cpusetids_from_indices.md) | Logical processor indices | CPU set IDs |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Affinity bitmask | CPU set IDs |
| **indices_from_cpusetids** | **CPU set IDs** | **Logical processor indices** |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU set IDs | Affinity bitmask |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L392–L408 |
| **Called by** | [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md), [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) |
| **Calls** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## See also

- [cpusetids_from_indices](cpusetids_from_indices.md)
- [mask_from_cpusetids](mask_from_cpusetids.md)
- [CpuSetData struct](CpuSetData.md)
- [winapi.rs module overview](README.md)