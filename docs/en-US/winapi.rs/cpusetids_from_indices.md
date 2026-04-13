# cpusetids_from_indices function (winapi.rs)

Converts a slice of logical CPU indices (0, 1, 2, …) into the corresponding Windows CPU Set IDs by looking up each index in the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) global cache. This is the primary translation function used when applying CPU set rules from configuration, where users specify human-readable CPU indices.

## Syntax

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | A slice of zero-based logical processor indices to translate. These correspond to the CPU numbers visible in Task Manager or specified in configuration files (e.g., `0;1;4-7`). Duplicate values are permitted but will produce duplicate IDs in the output. |

## Return value

A `Vec<u32>` containing the Windows CPU Set IDs that correspond to the provided logical processor indices. The output order follows the iteration order of the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) cache, **not** the order of the input indices. If an input index does not match any entry in the cache (e.g., the index exceeds the number of logical processors), it is silently omitted from the result.

Returns an empty `Vec` immediately if `cpu_indices` is empty.

## Remarks

### Translation mechanism

The function acquires the mutex lock on [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md), then iterates over every [CpuSetData](CpuSetData.md) entry in the cache. For each entry whose `logical_processor_index` appears in the input slice, the entry's `id` field is pushed to the result vector.

This is effectively a lookup join: for each cached `(id, logical_processor_index)` pair, if `logical_processor_index ∈ cpu_indices`, emit `id`.

### Performance considerations

The current implementation performs an O(n × m) scan where n is the cache size (number of logical processors) and m is the input slice length, because it calls `cpu_indices.contains()` for each cache entry. For typical desktop and server processor counts (≤ 256 logical processors) and typical rule sizes (≤ 64 CPUs), this is negligible.

### Ordering

The output CPU Set IDs appear in the order they are stored in the cache (which matches the order returned by `GetSystemCpuSetInformation`). This is typically ascending by logical processor index, but callers should not rely on any specific ordering. APIs that consume CPU Set IDs (e.g., `SetProcessDefaultCpuSets`) treat them as unordered sets.

### Empty-input fast path

If the input slice is empty, the function returns an empty `Vec` immediately without acquiring the mutex lock, avoiding unnecessary synchronization.

### Usage example

Given a system where CPU 0 has Set ID 256, CPU 1 has Set ID 257, and CPU 2 has Set ID 258:

| Input | Output |
|-------|--------|
| `&[0, 2]` | `vec![256, 258]` |
| `&[1]` | `vec![257]` |
| `&[99]` | `vec![]` (no such CPU) |
| `&[]` | `vec![]` |

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **Callees** | [get_cpu_set_information](get_cpu_set_information.md) (acquires `Mutex<Vec<CpuSetData>>` lock) |
| **Dependencies** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md), [CpuSetData](CpuSetData.md) |

## See Also

| Topic | Link |
|-------|------|
| Reverse operation: CPU Set IDs → indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| Affinity mask → CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set IDs → affinity mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU set topology cache | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU set cache accessor | [get_cpu_set_information](get_cpu_set_information.md) |
| CPU set application to processes | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |