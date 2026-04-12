# mask_from_cpusetids function (winapi.rs)

Converts a slice of CPU set IDs to an affinity bitmask by looking up each ID's logical processor index and setting the corresponding bit.

## Syntax

```rust
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## Parameters

`cpuids`

A slice of CPU set IDs to convert. Each ID is looked up in the [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) table to find its corresponding logical processor index.

## Return value

Returns a `usize` bitmask where each set bit corresponds to a logical processor whose CPU set ID was present in the input slice. Bit N is set if the CPU set ID mapping to logical processor index N was found in `cpuids`.

## Remarks

This function performs the inverse operation of [`cpusetids_from_mask`](cpusetids_from_mask.md). It locks the [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) mutex, iterates over the input CPU set IDs, and for each one finds the matching [`CpuSetData`](CpuSetData.md) entry to determine the `logical_processor_index`. The corresponding bit in the result mask is then set via bitwise OR.

CPU set IDs that do not match any entry in [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) are silently ignored and do not contribute to the result mask.

This is useful when converting from CPU Set API representations back to the legacy affinity mask representation needed by `SetProcessAffinityMask` and related APIs.

### Conversion function family

| Function | From | To |
| --- | --- | --- |
| [`cpusetids_from_indices`](cpusetids_from_indices.md) | Logical indices | CPU set IDs |
| [`cpusetids_from_mask`](cpusetids_from_mask.md) | Affinity mask | CPU set IDs |
| [`indices_from_cpusetids`](indices_from_cpusetids.md) | CPU set IDs | Logical indices |
| **mask_from_cpusetids** | **CPU set IDs** | **Affinity mask** |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L411–L426 |
| **Called by** | [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) |
| **Calls** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## See also

- [cpusetids_from_mask](cpusetids_from_mask.md)
- [indices_from_cpusetids](indices_from_cpusetids.md)
- [CpuSetData struct](CpuSetData.md)
- [winapi.rs module overview](README.md)