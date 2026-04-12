# cpusetids_from_indices function (winapi.rs)

Converts a slice of logical processor indices to their corresponding Windows CPU set IDs.

## Syntax

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
```

## Parameters

`cpu_indices`

A slice of zero-based logical processor indices to convert. These correspond to bit positions in an affinity mask and to the processor numbers used in the configuration file.

## Return value

Returns a `Vec<u32>` containing the Windows CPU set IDs corresponding to the given processor indices. Indices that do not match any known CPU set entry are silently skipped.

## Remarks

This function performs a lookup against the global [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) static (accessed via [`get_cpu_set_information`](get_cpu_set_information.md)). For each index in `cpu_indices`, it searches the [`CpuSetData`](CpuSetData.md) vector for an entry whose `logical_processor_index` matches, and collects the corresponding `id` value.

This conversion is necessary because Windows CPU Set APIs (`SetProcessDefaultCpuSets`, `SetThreadSelectedCpuSets`) require CPU set IDs rather than processor indices. The configuration file specifies processors as zero-based indices for user convenience, so this function bridges the gap.

The inverse operation is provided by [`indices_from_cpusetids`](indices_from_cpusetids.md).

### Related conversions

| Function | From | To |
| --- | --- | --- |
| **cpusetids_from_indices** | processor indices | CPU set IDs |
| [cpusetids_from_mask](cpusetids_from_mask.md) | affinity mask | CPU set IDs |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU set IDs | processor indices |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU set IDs | affinity mask |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L357–L374 |
| **Called by** | [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md), [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) |
| **Calls** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## See also

- [cpusetids_from_mask](cpusetids_from_mask.md)
- [indices_from_cpusetids](indices_from_cpusetids.md)
- [CpuSetData struct](CpuSetData.md)
- [winapi.rs module overview](README.md)