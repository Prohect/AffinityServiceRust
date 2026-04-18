# CPU_SET_INFORMATION static (winapi.rs)

Lazily-initialized, mutex-protected cache of system CPU set data. On first access, this static queries the Windows `GetSystemCpuSetInformation` API to enumerate all CPU sets on the system and stores the results as a `Vec<CpuSetData>`. Subsequent accesses return the cached data without re-querying the OS.

## Syntax

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| { ... });
```

## Type

`once_cell::sync::Lazy<std::sync::Mutex<Vec<CpuSetData>>>`

## Remarks

### Initialization

The `Lazy` initializer performs the following steps:

1. Calls `GetSystemCpuSetInformation` with a zero-length buffer to determine the required buffer size (`required_size`).
2. Allocates a `Vec<u8>` of the required size.
3. Calls `GetSystemCpuSetInformation` again with the allocated buffer to retrieve all CPU set entries.
4. Iterates through the buffer, parsing each `SYSTEM_CPU_SET_INFORMATION` structure via the unsafe helper `extract_cpu_set_data`, which reads the `CpuSet.Id` and `CpuSet.LogicalProcessorIndex` union fields.
5. Pushes each extracted [`CpuSetData`](CpuSetData.md) entry into the result vector.

If the second call to `GetSystemCpuSetInformation` fails, a diagnostic message (`"GetSystemCpuSetInformation failed"`) is written via `log_to_find` and the vector is returned empty.

### Thread safety

The static is wrapped in a `Mutex` to allow safe concurrent access from multiple threads. In practice, the data inside never changes after the initial population — CPU set topology is fixed for the lifetime of the OS boot — but the `Mutex` is required because `Lazy` initialization must be synchronized and callers may access it from different threads.

### Lifetime

The static has `'static` lifetime and is never deallocated. The data it contains remains valid for the entire duration of the process. Changes to CPU topology at runtime (e.g., hot-added processors) are **not** reflected.

### Access pattern

This static should not be accessed directly outside the `winapi` module. Use the [`get_cpu_set_information`](get_cpu_set_information.md) accessor function, which returns a `&'static Mutex<Vec<CpuSetData>>` reference. Internally, the following functions lock and read this static:

- [`cpusetids_from_indices`](cpusetids_from_indices.md)
- [`cpusetids_from_mask`](cpusetids_from_mask.md)
- [`indices_from_cpusetids`](indices_from_cpusetids.md)
- [`mask_from_cpusetids`](mask_from_cpusetids.md)

### Buffer parsing

The raw buffer returned by `GetSystemCpuSetInformation` contains variable-sized entries. The initializer walks the buffer using each entry's `Size` field to advance the offset, ensuring correct parsing regardless of OS version or future structure extensions.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Visibility** | Private (module-internal) |
| **Accessor** | [`get_cpu_set_information`](get_cpu_set_information.md) |
| **Win32 API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminfomationapi/nf-systeminfomationapi-getsystemcpusetinformation) |
| **Dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex`, [`CpuSetData`](CpuSetData.md) |
| **Platform** | Windows only |
| **Privileges** | None required |

## See Also

| Topic | Link |
|-------|------|
| CpuSetData struct | [CpuSetData](CpuSetData.md) |
| get_cpu_set_information accessor | [get_cpu_set_information](get_cpu_set_information.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| MODULE_CACHE static | [MODULE_CACHE](MODULE_CACHE.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
