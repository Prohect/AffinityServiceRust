# mask_from_cpusetids function (winapi.rs)

Converts a slice of Windows CPU Set IDs back to an affinity bitmask, where each set bit corresponds to the logical processor index associated with that CPU Set ID.

## Syntax

```rust
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | A slice of opaque Windows CPU Set IDs to convert back to a bitmask representation. |

## Return value

Returns a `usize` bitmask where each set bit represents a logical processor index that corresponds to one of the provided CPU Set IDs. Returns `0` if `cpuids` is empty or if no matching entries are found in the system CPU set information cache.

## Remarks

- This function is the inverse of [cpusetids_from_mask](cpusetids_from_mask.md). Given CPU Set IDs previously obtained from [cpusetids_from_indices](cpusetids_from_indices.md) or [cpusetids_from_mask](cpusetids_from_mask.md), it reconstructs the corresponding affinity bitmask.
- The function acquires a mutex lock on the global [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static and iterates through all cached `CpuSetData` entries, checking each entry's `id` against the provided slice.
- Only logical processor indices less than 64 are represented in the output mask, since `usize` on 64-bit Windows is 64 bits wide. Entries with a `logical_processor_index` of 64 or higher are silently skipped.
- This function is marked `#[allow(dead_code)]`, indicating it may be reserved for future use or used conditionally.

### Algorithm

1. If `cpuids` is empty, return `0` immediately.
2. Initialize `mask` to `0`.
3. Lock the `CPU_SET_INFORMATION` cache.
4. For each `CpuSetData` entry in the cache:
   - If `cpuids` contains the entry's `id` **and** the entry's `logical_processor_index` is less than 64, set the corresponding bit in `mask` via `mask |= 1 << idx`.
5. Return the accumulated `mask`.

### Relationship to other conversion functions

| Direction | Function |
|-----------|----------|
| Indices → CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Mask → CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set IDs → Indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU Set IDs → Mask | **mask_from_cpusetids** (this function) |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Dependencies** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md), [CpuSetData](CpuSetData.md) |
| **Platform** | Windows (uses cached `GetSystemCpuSetInformation` data) |
| **Privileges** | None required |

## See Also

| Topic | Link |
|-------|------|
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU_SET_INFORMATION static | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CpuSetData struct | [CpuSetData](CpuSetData.md) |
| collections module | [collections.rs](../collections.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
