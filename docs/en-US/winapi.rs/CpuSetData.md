# CpuSetData struct (winapi.rs)

Stores a CPU set ID and its corresponding logical processor index, forming the mapping between Windows CPU set identifiers and zero-based processor indices used throughout the application.

## Syntax

```rust
pub struct CpuSetData {
    pub id: u32,
    pub logical_processor_index: u8,
}
```

## Members

`id`

The Windows CPU set identifier as returned by `GetSystemCpuSetInformation`. This is the opaque ID used by APIs such as `SetProcessDefaultCpuSets` and `SetThreadSelectedCpuSets`.

`logical_processor_index`

The zero-based logical processor index corresponding to this CPU set. This maps directly to the bit position in an affinity mask and to the processor numbers used by the configuration file.

## Remarks

`CpuSetData` entries are collected at startup into the global [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) static by calling `GetSystemCpuSetInformation`. The resulting vector provides the authoritative mapping between CPU set IDs (used by the CPU Sets API) and logical processor indices (used by affinity masks and user-facing configuration).

This struct is intentionally minimal — it extracts only the two fields needed for the conversion functions in this module. The full `SYSTEM_CPU_SET_INFORMATION` structure returned by the Windows API contains additional fields (group, NUMA node, cache information, etc.) that are not required by this application.

The conversion functions that consume this data include:

- [`cpusetids_from_indices`](cpusetids_from_indices.md) — looks up `id` by matching `logical_processor_index`
- [`cpusetids_from_mask`](cpusetids_from_mask.md) — looks up `id` for each set bit position in a mask
- [`indices_from_cpusetids`](indices_from_cpusetids.md) — looks up `logical_processor_index` by matching `id`
- [`mask_from_cpusetids`](mask_from_cpusetids.md) — builds a bitmask from `logical_processor_index` values

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Lines** | L63–L66 |
| **Stored in** | [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) |
| **Used by** | [`cpusetids_from_indices`](cpusetids_from_indices.md), [`cpusetids_from_mask`](cpusetids_from_mask.md), [`indices_from_cpusetids`](indices_from_cpusetids.md), [`mask_from_cpusetids`](mask_from_cpusetids.md) |

## See also

- [CPU_SET_INFORMATION static](CPU_SET_INFORMATION.md)
- [get_cpu_set_information function](get_cpu_set_information.md)
- [winapi.rs module overview](README.md)