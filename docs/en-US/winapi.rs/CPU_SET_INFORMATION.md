# CPU_SET_INFORMATION static (winapi.rs)

Lazy-initialized, mutex-guarded vector containing CPU set information for all logical processors on the system. This is the authoritative source for mapping between CPU set IDs and logical processor indices.

## Syntax

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
    // Calls GetSystemCpuSetInformation to populate
    // ...
});
```

## Members

The static holds a `Vec<CpuSetData>` behind a `Mutex`. Each [`CpuSetData`](CpuSetData.md) entry contains:

- `id` — the system-assigned CPU set ID
- `logical_processor_index` — the zero-based index of the logical processor

## Remarks

The vector is populated once on first access by calling the Windows API `GetSystemCpuSetInformation`. This API enumerates all CPU sets on the system, each corresponding to a logical processor. The initialization is lazy (via `once_cell::sync::Lazy`) to avoid calling Windows APIs at static initialization time.

All access to the vector is synchronized through a `Mutex`, ensuring thread safety. In practice, the data is read-only after initialization, but the mutex is still required because `Lazy<T>` with interior mutability needs `Sync`.

This static is the backing store for [`get_cpu_set_information`](get_cpu_set_information.md) and is used indirectly by all CPU set conversion functions:

- [`cpusetids_from_indices`](cpusetids_from_indices.md)
- [`cpusetids_from_mask`](cpusetids_from_mask.md)
- [`indices_from_cpusetids`](indices_from_cpusetids.md)
- [`mask_from_cpusetids`](mask_from_cpusetids.md)

The vector is ordered by logical processor index, matching the order returned by `GetSystemCpuSetInformation`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source line** | L312 |
| **Accessed via** | [`get_cpu_set_information`](get_cpu_set_information.md) |
| **Windows API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemcpusetinformation) |

## See also

- [CpuSetData struct](CpuSetData.md)
- [get_cpu_set_information](get_cpu_set_information.md)
- [cpusetids_from_indices](cpusetids_from_indices.md)
- [cpusetids_from_mask](cpusetids_from_mask.md)