# indices_from_cpusetids function (winapi.rs)

Converts an array of Windows CPU Set IDs back to their corresponding logical processor indices. This is the inverse of [cpusetids_from_indices](cpusetids_from_indices.md), used when reading back CPU Set assignments from the operating system and translating them into user-facing processor numbers.

## Syntax

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | A slice of Windows CPU Set IDs to convert back to logical processor indices. These are the opaque identifiers returned by APIs such as `GetProcessDefaultCpuSets` or `GetThreadSelectedCpuSets`. |

## Return value

Returns a `List<[u32; CONSUMER_CPUS]>` (`SmallVec` with inline capacity of 32) containing the logical processor indices that correspond to the given CPU Set IDs. The returned list is **sorted in ascending order**. If `cpuids` is empty, an empty list is returned. CPU Set IDs that do not match any entry in the system CPU set information cache are silently skipped.

## Remarks

- The function acquires a lock on the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static and performs a linear scan of all cached [CpuSetData](CpuSetData.md) entries. For each entry whose `id` is found in the input `cpuids` slice, the corresponding `logical_processor_index` (cast to `u32`) is pushed into the result list.
- After collecting all matching indices, the result is sorted via `indices.sort()` to ensure a deterministic, ascending order regardless of how the CPU set cache or input slice is ordered.
- The lookup complexity is O(n × m) where n is the number of system CPU sets and m is the length of `cpuids`. This is acceptable for typical CPU counts and small input slices.
- The returned `SmallVec` uses an inline capacity of `CONSUMER_CPUS` (32). If more than 32 indices are produced, the vector spills to a heap allocation.
- The CPU set information is queried once at process startup via `GetSystemCpuSetInformation` and cached for the lifetime of the process. Runtime topology changes are not reflected.

### Algorithm

1. If `cpuids` is empty, return an empty list immediately.
2. Lock the `CPU_SET_INFORMATION` cache.
3. For each `CpuSetData` entry in the cache:
   - If the entry's `id` is contained in `cpuids`, push the entry's `logical_processor_index` (as `u32`) into the result list.
4. Sort the result list in ascending order.
5. Return the sorted list.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` — reads back current CPU set assignments for comparison and logging. |
| **Callees** | [get_cpu_set_information](get_cpu_set_information.md) (indirectly, via `CPU_SET_INFORMATION` lock) |
| **Windows API** | None directly; depends on cached results from `GetSystemCpuSetInformation`. |
| **Privileges** | None required. |

## See Also

| Topic | Link |
|-------|------|
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CpuSetData struct | [CpuSetData](CpuSetData.md) |
| CPU_SET_INFORMATION static | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](../collections.rs/CONSUMER_CPUS.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
