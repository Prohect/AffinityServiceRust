# cpusetids_from_indices function (winapi.rs)

Converts an array of logical CPU indices (0-based processor numbers) into their corresponding Windows CPU Set IDs. This translation is necessary because Windows CPU Set APIs operate on opaque system-assigned IDs rather than the user-friendly logical processor indices.

## Syntax

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | Slice of zero-based logical processor indices to convert. Each value represents a logical processor number as seen in Task Manager or `PROCESSOR_NUMBER::Number`. |

## Return value

`List<[u32; CONSUMER_CPUS]>` — A stack-allocated list of CPU Set IDs corresponding to the input indices. The list may be shorter than the input if some indices do not match any entry in the cached CPU set topology. Returns an empty list if `cpu_indices` is empty.

## Remarks

- The function acquires a lock on the [CPU_SET_INFORMATION](README.md) static cache and iterates through all cached [CpuSetData](CpuSetData.md) entries. For each entry whose `logical_processor_index` appears in the input slice, the entry's `id` is appended to the result list.
- The CPU set topology is queried once at process startup via `GetSystemCpuSetInformation` and cached for the lifetime of the service. This function does not call any Windows API directly — it only reads the cache.
- The output order follows the iteration order of the cached CPU set data, which matches the system enumeration order (typically ascending by logical processor index).
- The result uses `List<[u32; CONSUMER_CPUS]>`, a stack-allocated fixed-capacity list from `crate::collections`, avoiding heap allocation for systems with up to `CONSUMER_CPUS` logical processors.

### CPU Set ID vs. Logical Index

| Concept | Example | Used by |
|---------|---------|---------|
| Logical processor index | `0`, `1`, `2`, ... | Configuration files, affinity masks, `PROCESSOR_NUMBER::Number` |
| CPU Set ID | `0x100`, `0x101`, ... | `SetProcessDefaultCpuSets`, `SetThreadSelectedCpuSets`, and other CPU Set APIs |

The mapping between indices and IDs is system-specific and determined at boot time. The same physical core may have different CPU Set IDs across reboots.

### Relationship to other conversion functions

| Function | Direction |
|----------|-----------|
| **cpusetids_from_indices** | Logical index → CPU Set ID |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Affinity mask → CPU Set ID |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU Set ID → Logical index |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU Set ID → Affinity mask |
| [filter_indices_by_mask](filter_indices_by_mask.md) | Logical indices filtered by affinity mask |

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **Callees** | [get_cpu_set_information](README.md) (reads cached CPU set topology) |
| **Win32 API** | None directly; relies on cached data from [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| CPU set topology cache | [CpuSetData](CpuSetData.md) |
| Reverse conversion | [indices_from_cpusetids](indices_from_cpusetids.md) |
| Mask-based conversion | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU set application | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*