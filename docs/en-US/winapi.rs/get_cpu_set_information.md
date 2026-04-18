# get_cpu_set_information function (winapi.rs)

Returns a reference to the lazily-initialized, mutex-protected cache of system CPU set information. On first access, the cache is populated by calling `GetSystemCpuSetInformation` to enumerate all CPU sets on the system.

## Syntax

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

## Parameters

This function takes no parameters.

## Return value

Returns a `&'static Mutex<Vec<CpuSetData>>` — a reference to the global, lazily-initialized mutex containing a vector of [CpuSetData](CpuSetData.md) entries. Each entry maps a CPU Set ID to its logical processor index.

The returned reference has `'static` lifetime because it points to the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) global static, which is initialized once via `Lazy` and lives for the duration of the program.

## Remarks

- This function is a thin accessor for the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static. It does not perform any allocation or system calls after the first access.
- The underlying `Lazy` initializer calls `GetSystemCpuSetInformation` twice: once to determine the required buffer size, and once to retrieve the data. The results are parsed into a `Vec<CpuSetData>` of `(id, logical_processor_index)` pairs.
- If the initial `GetSystemCpuSetInformation` call fails, a diagnostic message is written via `log_to_find` and the returned vector will be empty.
- Callers must lock the mutex before reading the data. Because the CPU set topology is fixed for the lifetime of the OS boot, the data inside never changes after initialization.
- This function is called internally by [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), and [mask_from_cpusetids](mask_from_cpusetids.md).

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), [mask_from_cpusetids](mask_from_cpusetids.md) |
| **Callees** | None (accessor only; initialization calls `GetSystemCpuSetInformation` from the Windows API) |
| **Win32 API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/systeminfomationapi/nf-systeminfomationapi-getsystemcpusetinformation) (during `Lazy` initialization) |
| **Privileges** | None required |

## See Also

| Topic | Link |
|-------|------|
| CpuSetData struct | [CpuSetData](CpuSetData.md) |
| CPU_SET_INFORMATION static | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |

---

*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
