# cpusetids_from_mask function (winapi.rs)

Converts a CPU affinity bitmask into a list of Windows CPU Set IDs. Each set bit in the mask corresponds to a logical processor index, which is mapped to its opaque CPU Set ID via the cached system CPU set topology.

## Syntax

```rust
pub fn cpusetids_from_mask(mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `mask` | `usize` | A bitmask where bit *N* represents logical processor *N*. For example, `0x15` (binary `10101`) represents processors 0, 2, and 4. Only the lower 64 bits are meaningful; processors with index ≥ 64 are silently skipped. |

## Return value

`List<[u32; CONSUMER_CPUS]>` — A stack-allocated list of CPU Set IDs corresponding to the logical processors indicated by the set bits in `mask`. Returns an empty list if `mask` is `0`.

## Remarks

- The function acquires a lock on the [CPU_SET_INFORMATION](README.md) cache and iterates over all [CpuSetData](CpuSetData.md) entries. For each entry whose `logical_processor_index` corresponds to a set bit in `mask`, the entry's `id` is appended to the result.
- The bit test is performed as `(1usize << logical_processor_index) & mask != 0`, which limits the function to processors 0–63 within a single processor group. Entries with `logical_processor_index >= 64` are excluded by an explicit bounds check.
- This function is the mask-based counterpart to [cpusetids_from_indices](cpusetids_from_indices.md), which accepts an explicit list of processor indices instead of a bitmask.
- Currently marked `#[allow(dead_code)]` — available for use but not called in the current codebase.

### Relationship to affinity masks

Windows affinity masks (as returned by `GetProcessAffinityMask`) use the same bit-per-processor encoding that this function accepts. This makes `cpusetids_from_mask` the natural bridge between legacy affinity APIs and the newer CPU Sets API.

### Performance

The function performs a single pass over the cached CPU set data (one entry per logical processor), making it O(n) where n is the number of logical processors on the system. The `Mutex` lock is held for the duration of the iteration.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Currently unused (`#[allow(dead_code)]`) |
| **Callees** | [get_cpu_set_information](README.md) |
| **Win32 API** | None directly; consumes cached data from [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/systemcpusetinformation/nf-systemcpusetinformation-getsystemcpusetinformation) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Index-based conversion | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Reverse: CPU Set IDs → indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| Reverse: CPU Set IDs → mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| Filter indices by mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU set topology cache | [CpuSetData](CpuSetData.md) |
| Module overview | [winapi.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*