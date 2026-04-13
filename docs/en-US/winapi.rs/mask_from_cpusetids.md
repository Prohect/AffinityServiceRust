# mask_from_cpusetids function (winapi.rs)

Converts a slice of Windows CPU Set IDs back to a `usize` affinity bitmask by looking up each ID's logical processor index in the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) cache and setting the corresponding bit.

## Syntax

```rust
#[allow(dead_code)]
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | A slice of Windows CPU Set IDs to convert. Each ID should be a value previously obtained from the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) cache or from a Windows CPU set API. |

## Return value

A `usize` bitmask where bit *N* is set if any CPU Set ID in `cpuids` maps to logical processor index *N*. Returns `0` if `cpuids` is empty or if no IDs match entries in the CPU set cache.

**Examples:**

| Input CPU Set IDs | Logical indices (looked up) | Output mask |
|-------------------|-----------------------------|-------------|
| `[]` | — | `0x0` |
| IDs mapping to CPUs 0, 1 | 0, 1 | `0x3` |
| IDs mapping to CPUs 0, 2, 4 | 0, 2, 4 | `0x15` |

## Remarks

This function is the inverse of [cpusetids_from_mask](cpusetids_from_mask.md). Together they form a round-trip conversion between affinity masks and CPU Set IDs:

```
mask → cpusetids_from_mask → cpusetids → mask_from_cpusetids → mask  (identity)
```

### Algorithm

1. If the input slice is empty, return `0` immediately.
2. Lock the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) mutex.
3. Iterate over every [CpuSetData](CpuSetData.md) entry in the cache.
4. For each entry whose `id` is present in `cpuids`, set bit `logical_processor_index` in the accumulator mask (guarded by an `idx < 64` check to prevent shift overflow).
5. Return the accumulated mask.

### 64-processor limit

Logical processor indices at or above 64 are silently skipped because a `usize` bitmask on 64-bit Windows can only represent processors 0–63. Systems with more than 64 logical processors across multiple processor groups would require the CPU Set–based APIs ([cpusetids_from_indices](cpusetids_from_indices.md), [indices_from_cpusetids](indices_from_cpusetids.md)) rather than mask-based functions.

### Thread safety

The function acquires the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) mutex for the duration of the lookup. The lock is released when the `MutexGuard` goes out of scope at the end of the function.

### Dead code allowance

The function is annotated with `#[allow(dead_code)]` because it may not be directly called in the current codebase but is retained as part of the complete set of CPU set translation utilities.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Dependencies** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md), [get_cpu_set_information](get_cpu_set_information.md) |
| **API** | None (pure lookup against cached data) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Inverse conversion (mask → CPU Set IDs) | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU indices → CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| CPU Set IDs → CPU indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| Filter indices by mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU set topology data | [CpuSetData](CpuSetData.md) |
| Global CPU set cache | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |