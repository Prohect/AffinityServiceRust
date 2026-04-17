# cpusetids_from_mask function (winapi.rs)

Converts an affinity bitmask to a list of Windows CPU Set IDs. Each set bit in the mask corresponds to a logical processor index; the function looks up the matching CPU Set ID from the system CPU set information cache.

## Syntax

```rust
pub fn cpusetids_from_mask(mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `mask` | `usize` | A bitmask where each set bit represents a logical processor index (bit 0 = CPU 0, bit 1 = CPU 1, etc.). Supports up to 64 logical processors. |

## Return value

Returns a `List<[u32; CONSUMER_CPUS]>` (`SmallVec`) containing the CPU Set IDs corresponding to the set bits in the mask. Returns an empty list if `mask` is `0`.

## Remarks

- This function is the bitmask-based counterpart to [cpusetids_from_indices](cpusetids_from_indices.md), which accepts an explicit slice of indices.
- Windows CPU Set IDs are opaque identifiers that do not correspond directly to logical processor indices. This function bridges the gap by consulting the system CPU set information cache ([CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)).
- The function acquires a mutex lock on the global `CPU_SET_INFORMATION` static and iterates through all CPU set entries, checking each entry's `logical_processor_index` against the bitmask.
- Only logical processor indices less than 64 are considered, since `usize` on 64-bit Windows is 64 bits wide.
- This function is marked `#[allow(dead_code)]`, indicating it may be reserved for future use or used conditionally.

### Algorithm

1. If `mask` is `0`, return an empty list immediately.
2. Lock the `CPU_SET_INFORMATION` cache.
3. For each `CpuSetData` entry in the cache:
   - If the entry's `logical_processor_index` is less than 64 **and** the corresponding bit is set in `mask`, push the entry's `id` into the result list.
4. Return the collected CPU Set IDs.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Dependencies** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md), [CpuSetData](CpuSetData.md) |
| **Types** | `List<[u32; CONSUMER_CPUS]>` from [collections](../collections.rs/README.md) |
| **Platform** | Windows (uses `GetSystemCpuSetInformation` data) |

## See Also

| Topic | Link |
|-------|------|
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU_SET_INFORMATION static | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CpuSetData struct | [CpuSetData](CpuSetData.md) |
| collections module | [collections.rs](../collections.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
