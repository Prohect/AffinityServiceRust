# mask_from_cpusetids function (winapi.rs)

Converts a slice of CPU Set IDs back to a processor affinity bitmask. This is the inverse of [cpusetids_from_mask](cpusetids_from_mask.md), mapping opaque Windows CPU Set identifiers to positional bits in a `usize` mask.

## Syntax

```rust
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | Slice of CPU Set IDs to convert. Each ID must correspond to an entry in the system's CPU set topology obtained from [get_cpu_set_information](../winapi.rs/README.md). |

## Return value

`usize` — A bitmask where bit *N* is set if any of the provided CPU Set IDs maps to logical processor index *N*. Returns `0` if the input slice is empty.

### Examples

| Input CPU Set IDs | Logical processor indices | Output mask |
|-------------------|---------------------------|-------------|
| `[]` | *(none)* | `0x0` |
| `[256]` | `[0]` | `0x1` |
| `[256, 258, 260]` | `[0, 2, 4]` | `0x15` |

*(Actual CPU Set ID values are system-specific; the above are illustrative.)*

## Remarks

- The function acquires the `CPU_SET_INFORMATION` mutex lock and iterates over all cached [CpuSetData](CpuSetData.md) entries. For each entry whose `id` is found in the input slice, the corresponding bit (`1 << logical_processor_index`) is set in the result mask.
- Logical processor indices ≥ 64 are silently skipped, as they cannot be represented in a single `usize` bitmask on 64-bit systems. This is consistent with the Windows affinity mask limitation of 64 processors per processor group.
- The function performs an O(C × N) lookup where C is the number of CPU set entries and N is the length of the input slice, using `slice::contains` for each cached entry. This is efficient for the small input sizes typical of consumer systems (≤ 64 CPUs).
- CPU Set IDs in the input that do not match any entry in the cached topology are silently ignored — no bits are set for unrecognized IDs.
- This function is currently marked `#[allow(dead_code)]` but is available for use by any component that needs to convert CPU Set IDs back to an affinity mask representation.

### Relationship to other conversion functions

| Function | Direction |
|----------|-----------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | CPU indices → CPU Set IDs |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Affinity mask → CPU Set IDs |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU Set IDs → CPU indices |
| **mask_from_cpusetids** | **CPU Set IDs → Affinity mask** |
| [filter_indices_by_mask](filter_indices_by_mask.md) | CPU indices × mask → filtered indices |

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Available for general use; currently `#[allow(dead_code)]` |
| **Callees** | [get_cpu_set_information](../winapi.rs/README.md) (acquires cached CPU set topology) |
| **Win32 API** | None directly; relies on cached data from [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Inverse: mask to CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| Indices to CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| CPU Set IDs to indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU set topology cache | [CpuSetData](CpuSetData.md) |
| Affinity check utility | [is_affinity_unset](is_affinity_unset.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*