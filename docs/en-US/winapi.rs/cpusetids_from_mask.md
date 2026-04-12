# cpusetids_from_mask function (winapi.rs)

Converts an affinity bitmask to a vector of CPU set IDs by mapping each set bit position to its corresponding CPU set identifier.

## Syntax

```rust
pub fn cpusetids_from_mask(mask: usize) -> Vec<u32>
```

## Parameters

`mask`

An affinity bitmask where each set bit represents a logical processor. Bit 0 corresponds to logical processor 0, bit 1 to logical processor 1, and so on.

## Return value

Returns a `Vec<u32>` containing the CPU set IDs corresponding to each set bit in the mask. The vector is ordered from the lowest bit position to the highest. If a set bit does not correspond to any known CPU set entry, it is silently skipped.

## Remarks

This function iterates over each set bit in the provided mask, determines the logical processor index from the bit position, and looks up the corresponding CPU set ID in the global [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) table via [`get_cpu_set_information`](get_cpu_set_information.md).

This is useful when converting from the legacy affinity mask representation (used by `SetProcessAffinityMask`) to the newer CPU Sets representation (used by `SetProcessDefaultCpuSets` and `SetThreadSelectedCpuSets`).

The function acquires a lock on [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) to perform the lookups. Each [`CpuSetData`](CpuSetData.md) entry's `logical_processor_index` is compared against the bit positions that are set in the mask.

### Relationship to other conversion functions

| Function | From | To |
| --- | --- | --- |
| [cpusetids_from_indices](cpusetids_from_indices.md) | `&[u32]` indices | `Vec<u32>` CPU set IDs |
| **cpusetids_from_mask** | `usize` mask | `Vec<u32>` CPU set IDs |
| [indices_from_cpusetids](indices_from_cpusetids.md) | `&[u32]` CPU set IDs | `Vec<u32>` indices |
| [mask_from_cpusetids](mask_from_cpusetids.md) | `&[u32]` CPU set IDs | `usize` mask |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L377–L390 |
| **Called by** | [`apply_affinity`](../apply.rs/apply_affinity.md) |
| **Calls** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## See also

- [cpusetids_from_indices](cpusetids_from_indices.md)
- [mask_from_cpusetids](mask_from_cpusetids.md)
- [CpuSetData struct](CpuSetData.md)
- [CPU_SET_INFORMATION static](CPU_SET_INFORMATION.md)