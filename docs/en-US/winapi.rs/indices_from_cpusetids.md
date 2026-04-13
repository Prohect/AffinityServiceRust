# indices_from_cpusetids function (winapi.rs)

Converts a slice of Windows CPU Set IDs back to their corresponding logical processor indices. This is the inverse of [cpusetids_from_indices](cpusetids_from_indices.md) and is used when reading back CPU set assignments from the system into the human-friendly index representation used by configuration files.

## Syntax

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | A slice of opaque Windows CPU Set IDs to translate. These are the values returned by APIs such as `GetProcessDefaultCpuSets` or `GetThreadSelectedCpuSets`. |

## Return value

A sorted `Vec<u32>` containing the logical processor indices that correspond to the input CPU Set IDs. If `cpuids` is empty, returns an empty vector. Any CPU Set ID that does not match an entry in the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) cache is silently skipped.

## Remarks

### Algorithm

1. If the input slice is empty, returns an empty `Vec` immediately (fast path).
2. Acquires the mutex lock on [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md).
3. Iterates over every [CpuSetData](CpuSetData.md) entry in the cache.
4. For each entry whose `id` is contained in `cpuids`, pushes its `logical_processor_index` (cast to `u32`) into the result vector.
5. Sorts the result vector before returning.

### Sorting

Unlike [cpusetids_from_indices](cpusetids_from_indices.md), which returns IDs in cache-iteration order, `indices_from_cpusetids` explicitly sorts its output. This ensures a stable, ascending order of CPU indices regardless of the order in which CPU Set entries appear in the system cache or the order of IDs in the input slice.

### Symmetry with other translation functions

| From | To | Function |
|------|----|----------|
| CPU indices | CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Affinity mask | CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| **CPU Set IDs** | **CPU indices** | **indices_from_cpusetids** (this function) |
| CPU Set IDs | Affinity mask | [mask_from_cpusetids](mask_from_cpusetids.md) |

### Lock contention

The function holds the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) mutex for the duration of the iteration. Since the cache is read-only after initialization, contention is typically negligible, but callers performing bulk translations should be aware that each call acquires and releases the lock independently.

### Unmatched IDs

If a CPU Set ID in the input does not exist in the system cache (e.g., because it was obtained from a different machine or the topology changed after a hot-add), it is silently excluded from the output. No error or warning is logged.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | Public (`pub fn`) |
| **Callers** | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **Callees** | [get_cpu_set_information](get_cpu_set_information.md) (via [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)) |
| **Dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex` |

## See Also

| Topic | Link |
|-------|------|
| Inverse operation (indices → IDs) | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Mask to CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set IDs to mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU set topology cache | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| Cache element type | [CpuSetData](CpuSetData.md) |
| CPU set application logic | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetProcessDefaultCpuSets (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets) |