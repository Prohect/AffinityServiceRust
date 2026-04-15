# cpusetids_from_indices function (winapi.rs)

Converts an array of logical CPU indices (0, 1, 2, …) to their corresponding Windows CPU Set IDs. Windows CPU Sets use opaque identifiers that do not necessarily match logical processor numbers; this function performs the translation using the cached system CPU set information.

## Syntax

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | A slice of logical processor indices to convert. Each value corresponds to a zero-based logical processor number as reported by the operating system. |

## Return value

Returns a `List<[u32; CONSUMER_CPUS]>` (`SmallVec` with inline capacity of 32) containing the CPU Set IDs that correspond to the given logical indices. If `cpu_indices` is empty, an empty list is returned. Indices that do not match any entry in the system CPU set information are silently skipped.

## Remarks

- The function acquires a lock on the `CPU_SET_INFORMATION` static to iterate over the cached `CpuSetData` entries. Each entry's `logical_processor_index` is compared against the provided indices using `slice::contains`.
- Because the lookup is performed via linear scan of both the CPU set cache and the input slice, performance is O(n × m) where n is the number of logical processors on the system and m is the length of `cpu_indices`. This is acceptable for typical CPU counts (≤ 128 cores) and small input slices.
- The returned `SmallVec` uses an inline capacity of `CONSUMER_CPUS` (32). If more than 32 CPU Set IDs are produced, the vector spills to a heap allocation.
- This function does **not** validate that the indices fall within the range of available logical processors. Out-of-range indices simply produce no matching output.
- The CPU set information is queried once at process startup via `GetSystemCpuSetInformation` and cached for the lifetime of the process. Changes to CPU topology at runtime (e.g., hot-added processors) are not reflected.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` — rule application logic that converts config-specified CPU indices to CPU Set IDs before calling `SetProcessDefaultCpuSets` or `SetThreadSelectedCpuSets`. |
| **Callees** | [get_cpu_set_information](get_cpu_set_information.md) (indirectly, via `CPU_SET_INFORMATION` lock) |
| **Windows API** | None directly; depends on cached results from `GetSystemCpuSetInformation`. |
| **Privileges** | None required. |

## See Also

| Topic | Link |
|-------|------|
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CpuSetData struct | [CpuSetData](CpuSetData.md) |
| CPU_SET_INFORMATION static | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](../collections.rs/CONSUMER_CPUS.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
