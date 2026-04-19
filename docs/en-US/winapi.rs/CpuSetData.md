# CpuSetData struct (winapi.rs)

Lightweight cache record holding the essential fields from a `SYSTEM_CPU_SET_INFORMATION` entry. The service enumerates CPU set topology once at startup via `GetSystemCpuSetInformation` and stores the results as a `Vec<CpuSetData>` in the static [CPU_SET_INFORMATION](README.md) cache. All subsequent CPU-index ↔ CPU-Set-ID conversions operate on this cached data rather than re-querying the OS.

## Syntax

```rust
#[derive(Clone, Copy)]
pub struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `id` | `u32` | The opaque CPU Set ID assigned by Windows (from `SYSTEM_CPU_SET_INFORMATION.CpuSet.Id`). This value is passed to APIs such as `SetProcessDefaultCpuSets` and `SetThreadSelectedCpuSets`. CPU Set IDs are **not** sequential and do not correspond to logical processor indices. |
| `logical_processor_index` | `u8` | The zero-based logical processor index for this CPU set entry (from `SYSTEM_CPU_SET_INFORMATION.CpuSet.LogicalProcessorIndex`). This is the same index used in affinity masks — bit *N* in an affinity mask corresponds to `logical_processor_index == N`. Stored as `u8`, supporting up to 256 logical processors per group. |

## Remarks

- `CpuSetData` derives `Clone` and `Copy` because it is a small (5-byte), stack-only value type with no heap allocations or resource ownership.
- The struct fields are **module-private** (no `pub` visibility). All access goes through the conversion functions that iterate the cached `Vec<CpuSetData>`:
  - [cpusetids_from_indices](cpusetids_from_indices.md) — logical indices → CPU Set IDs
  - [cpusetids_from_mask](cpusetids_from_mask.md) — affinity mask → CPU Set IDs
  - [indices_from_cpusetids](indices_from_cpusetids.md) — CPU Set IDs → logical indices
  - [mask_from_cpusetids](mask_from_cpusetids.md) — CPU Set IDs → affinity mask
  - [filter_indices_by_mask](filter_indices_by_mask.md) — filter indices by affinity mask
- The `SYSTEM_CPU_SET_INFORMATION` union contains many additional fields (e.g., `Group`, `NumaNodeIndex`, `LastLevelCacheIndex`, `CoreIndex`, `EfficiencyClass`) that are not captured in `CpuSetData`. Only `Id` and `LogicalProcessorIndex` are needed for the service's CPU pinning and affinity operations.

### Topology caching

The static `CPU_SET_INFORMATION` is a `Lazy<Mutex<Vec<CpuSetData>>>` initialized once on first access. The initialization calls `GetSystemCpuSetInformation` twice — first to determine the required buffer size, then to fill it — and walks the variable-length entries using each entry's `Size` field as the stride. The resulting `Vec<CpuSetData>` is locked behind a `Mutex` and never modified after initialization.

### CPU Set ID vs. Logical Index

| Concept | Example | Used by |
|---------|---------|---------|
| Logical processor index | `0`, `1`, `2`, … | Affinity masks, configuration `affinity_cpus` lists |
| CPU Set ID | `256`, `257`, `258`, … (opaque) | `SetProcessDefaultCpuSets`, `SetThreadSelectedCpuSets` |

Configuration files use human-friendly logical indices. The Windows CPU Sets API requires opaque IDs. `CpuSetData` bridges this gap.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), [mask_from_cpusetids](mask_from_cpusetids.md), [get_cpu_set_information](README.md) |
| **Dependencies** | `SYSTEM_CPU_SET_INFORMATION` (windows crate) |
| **Win32 API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) (at cache initialization time) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Index → CPU Set ID conversion | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Mask → CPU Set ID conversion | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set ID → index conversion | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU Set ID → mask conversion | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU set application to processes | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*