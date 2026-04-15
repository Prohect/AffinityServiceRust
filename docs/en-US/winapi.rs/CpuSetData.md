# CpuSetData struct (winapi.rs)

Holds a CPU Set ID and its corresponding logical processor index. This is an internal representation of the data extracted from Windows `SYSTEM_CPU_SET_INFORMATION` structures, used to translate between user-facing logical processor indices and the opaque CPU Set IDs required by Windows CPU set APIs.

## Syntax

```rust
#[derive(Clone, Copy)]
pub struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u32` | The opaque CPU Set ID assigned by Windows. This value is passed to APIs such as `SetProcessDefaultCpuSets` and `SetThreadSelectedCpuSets`. |
| `logical_processor_index` | `u8` | The zero-based logical processor index that corresponds to this CPU set. This is the user-facing index (0, 1, 2, …) that maps to a physical or logical core. |

## Remarks

- Both fields are **module-private** (no `pub` visibility). External code accesses `CpuSetData` only indirectly through the conversion functions [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), and [mask_from_cpusetids](mask_from_cpusetids.md).

- The struct derives `Clone` and `Copy`, making it cheap to pass by value and store in `Vec<CpuSetData>` without reference lifetime concerns.

- Instances are created by the `extract_cpu_set_data` helper function, which reads from the union fields of a raw `SYSTEM_CPU_SET_INFORMATION` structure in an `unsafe` block.

- The global [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static stores a `Vec<CpuSetData>` that is populated once at initialization time and reused for all subsequent CPU set lookups.

- The `logical_processor_index` field is stored as `u8`, supporting up to 256 logical processors. This covers the vast majority of consumer and server systems within a single processor group.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `winapi.rs` |
| Populated by | `extract_cpu_set_data` (unsafe, module-private) |
| Stored in | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static |
| Platform | Windows only |
| Underlying API | `GetSystemCpuSetInformation` via `SYSTEM_CPU_SET_INFORMATION` |

## See Also

| Topic | Link |
|-------|------|
| CPU_SET_INFORMATION static | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| collections module | [collections.rs](../collections.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
