# cpusetids_from_mask function (winapi.rs)

Converts a `usize` affinity bitmask into a vector of Windows CPU Set IDs by testing each bit position against the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) cache. Each set bit in the mask that corresponds to a known logical processor index produces the matching CPU Set ID in the output.

## Syntax

```rust
#[allow(dead_code)]
pub fn cpusetids_from_mask(mask: usize) -> Vec<u32>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `mask` | `usize` | A bitmask where each set bit represents a logical processor index. Bit 0 corresponds to CPU 0, bit 1 to CPU 1, and so on. Only the low 64 bits are meaningful (logical processor indices 0–63). A value of `0` produces an empty result immediately. |

## Return value

A `Vec<u32>` containing the Windows CPU Set IDs corresponding to every logical processor whose bit is set in `mask`. The order of IDs in the output follows the iteration order of the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) cache (which reflects the order returned by `GetSystemCpuSetInformation`). If `mask` is `0` or no bits correspond to known processors, an empty vector is returned.

**Example:**

| `mask` (binary) | Logical CPUs set | Output (CPU Set IDs) |
|-----------------|-----------------|----------------------|
| `0b0000` | (none) | `[]` |
| `0b0101` | 0, 2 | `[<id for CPU 0>, <id for CPU 2>]` |
| `0b1111` | 0, 1, 2, 3 | `[<id for CPU 0>, <id for CPU 1>, <id for CPU 2>, <id for CPU 3>]` |

## Remarks

This function is the bitmask-based counterpart to [cpusetids_from_indices](cpusetids_from_indices.md). While `cpusetids_from_indices` accepts an explicit list of CPU index values, `cpusetids_from_mask` works with the compact bitmask representation used by `GetProcessAffinityMask` and related Win32 APIs.

### Algorithm

1. If `mask == 0`, return an empty `Vec` immediately.
2. Lock the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) mutex.
3. Iterate over every [CpuSetData](CpuSetData.md) entry in the cache.
4. For each entry whose `logical_processor_index` is less than 64, test whether the corresponding bit `(1usize << logical_processor_index) & mask` is non-zero.
5. If set, push the entry's `id` into the result vector.

### 64-processor limit

The function explicitly checks `logical_processor_index < 64` before bit-testing. On systems with more than 64 logical processors (multiple processor groups), processors beyond index 63 cannot be represented in a single `usize` mask and are silently excluded. For such systems, the index-based functions ([cpusetids_from_indices](cpusetids_from_indices.md)) should be preferred.

### Dead-code annotation

The function is annotated `#[allow(dead_code)]` because it is not currently referenced by the main application code path. It is retained as a utility for mask-based CPU set workflows and for symmetry with [mask_from_cpusetids](mask_from_cpusetids.md).

### Thread safety

The function acquires the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) mutex for the duration of the iteration. The lock is released when the `MutexGuard` goes out of scope at the end of the function.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callees** | [get_cpu_set_information](get_cpu_set_information.md) (via `CPU_SET_INFORMATION` access) |
| **Dependencies** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md), [CpuSetData](CpuSetData.md) |
| **API** | None directly; translates masks produced by [`GetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |

## See Also

| Topic | Link |
|-------|------|
| Index-based conversion | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Reverse: CPU Set IDs → mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| Reverse: CPU Set IDs → indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| Filter indices by mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU set topology cache | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU set data element | [CpuSetData](CpuSetData.md) |