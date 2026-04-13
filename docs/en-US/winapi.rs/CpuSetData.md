# CpuSetData struct (winapi.rs)

Represents a single entry in the system's CPU set topology, pairing a Windows-assigned opaque CPU Set ID with the logical processor index it corresponds to. This struct is the element type stored in the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) global cache and is used by all CPU set translation functions in the module.

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
| `id` | `u32` | The opaque CPU Set ID assigned by Windows. This value is obtained from `SYSTEM_CPU_SET_INFORMATION::Anonymous.CpuSet.Id` and is the identifier required by APIs such as `SetProcessDefaultCpuSets` and `SetThreadSelectedCpuSets`. CPU Set IDs are **not** sequential and do **not** correspond to logical processor numbers. |
| `logical_processor_index` | `u8` | The zero-based logical processor index that this CPU set entry maps to. Obtained from `SYSTEM_CPU_SET_INFORMATION::Anonymous.CpuSet.LogicalProcessorIndex`. This is the human-friendly CPU number (0, 1, 2, …) used in configuration files and affinity masks. The `u8` type limits support to 256 logical processors. |

## Remarks

Windows CPU Set APIs operate on opaque 32-bit IDs rather than logical processor indices. AffinityServiceRust configuration files use human-readable CPU indices (e.g., `0;1;4-7`), so a translation layer is needed. `CpuSetData` provides this mapping by caching the `(id, logical_processor_index)` pair for every CPU set entry reported by `GetSystemCpuSetInformation` at process startup.

### Initialization

`CpuSetData` entries are constructed by the `extract_cpu_set_data` helper during lazy initialization of the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static. The raw `SYSTEM_CPU_SET_INFORMATION` buffer returned by `GetSystemCpuSetInformation` is walked entry-by-entry, and each entry is reduced to this compact two-field struct.

### Visibility

Both fields are module-private (no `pub` modifier). External code accesses the data indirectly through the translation functions:

- [cpusetids_from_indices](cpusetids_from_indices.md) — looks up `id` by matching `logical_processor_index`
- [cpusetids_from_mask](cpusetids_from_mask.md) — looks up `id` by testing `logical_processor_index` against a bitmask
- [indices_from_cpusetids](indices_from_cpusetids.md) — looks up `logical_processor_index` by matching `id`
- [mask_from_cpusetids](mask_from_cpusetids.md) — looks up `logical_processor_index` by matching `id` and sets the corresponding bit

### Derive traits

`CpuSetData` derives `Clone` and `Copy`, making it cheap to pass by value and store in vectors without indirection.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Constructed by** | `extract_cpu_set_data` (module-private helper) |
| **Stored in** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| **Consumed by** | [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), [mask_from_cpusetids](mask_from_cpusetids.md) |
| **API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |

## See Also

| Topic | Link |
|-------|------|
| Global CPU set cache | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU indices → CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| CPU Set IDs → CPU indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| Affinity mask → CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set IDs → affinity mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU set application to processes | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |