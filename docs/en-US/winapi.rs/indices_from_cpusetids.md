# indices_from_cpusetids function (winapi.rs)

Converts an array of CPU Set IDs back to their corresponding logical processor indices. This is the inverse of [cpusetids_from_indices](cpusetids_from_indices.md), used when reading back CPU Set assignments to display or compare against user-configured CPU index lists.

## Syntax

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | Slice of CPU Set IDs to convert. These are opaque system-assigned identifiers obtained from Windows CPU Set APIs or from a prior call to [cpusetids_from_indices](cpusetids_from_indices.md) / [cpusetids_from_mask](cpusetids_from_mask.md). |

## Return value

`List<[u32; CONSUMER_CPUS]>` — A stack-allocated list of logical processor indices corresponding to the input CPU Set IDs, sorted in ascending order. Returns an empty list if `cpuids` is empty.

## Remarks

The function iterates over the cached [CPU_SET_INFORMATION](README.md) topology data (populated at startup via `GetSystemCpuSetInformation`). For each cached entry whose `id` field matches a value in the input slice, the entry's `logical_processor_index` is appended to the result list.

### Sort order

The returned list is explicitly sorted in ascending order via `indices.sort()` before being returned. This provides a stable, deterministic ordering regardless of the order in which CPU sets appear in the system topology cache or the input slice.

### Relationship to other conversion functions

| Function | Direction |
|----------|-----------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | Indices → CPU Set IDs |
| **indices_from_cpusetids** | CPU Set IDs → Indices |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Affinity mask → CPU Set IDs |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU Set IDs → Affinity mask |

### Lock acquisition

This function acquires the `Mutex` lock on the `CPU_SET_INFORMATION` static for the duration of the iteration. The lock is held for O(N × M) time where N is the number of cached CPU set entries and M is the length of `cpuids`. On consumer systems (≤64 CPUs), this is negligible.

### Unmatched IDs

CPU Set IDs in `cpuids` that do not match any entry in the cached topology data are silently ignored. No error is reported. This can happen if the topology cache is stale or if an invalid ID is passed.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md), diagnostic logging |
| **Callees** | [get_cpu_set_information](README.md) |
| **Win32 API** | None directly; depends on cached data from [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Forward conversion (indices → IDs) | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Mask-based conversion | [cpusetids_from_mask](cpusetids_from_mask.md), [mask_from_cpusetids](mask_from_cpusetids.md) |
| Index filtering by mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU set topology cache | [CpuSetData](CpuSetData.md) |
| Module overview | [winapi.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*