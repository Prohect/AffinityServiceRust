# CPU_SET_INFORMATION static (winapi.rs)

Lazily-initialized, mutex-guarded global vector of [CpuSetData](CpuSetData.md) entries that maps every logical processor on the system to its Windows CPU Set ID. This static is the single source of truth for CPU set topology within AffinityServiceRust and is queried by all CPU set translation functions in the module.

## Syntax

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
    Mutex::new({
        let mut cpu_set_data: Vec<CpuSetData> = Vec::new();
        let mut required_size: u32 = 0;

        let current_process = unsafe { GetCurrentProcess() };

        let _ = unsafe {
            GetSystemCpuSetInformation(None, 0, &mut required_size, Some(current_process), Some(0))
        };

        let mut buffer: Vec<u8> = vec![0u8; required_size as usize];

        let success = unsafe {
            GetSystemCpuSetInformation(
                Some(buffer.as_mut_ptr() as *mut SYSTEM_CPU_SET_INFORMATION),
                required_size,
                &mut required_size,
                Some(current_process),
                Some(0),
            )
            .as_bool()
        };

        if !success {
            log_to_find("GetSystemCpuSetInformation failed");
        } else {
            let mut offset = 0;
            while offset < required_size as usize {
                let entry = unsafe {
                    let entry_ptr = buffer.as_ptr().add(offset) as *const SYSTEM_CPU_SET_INFORMATION;
                    &*entry_ptr
                };

                let data = unsafe { extract_cpu_set_data(entry) };
                cpu_set_data.push(data);
                offset += entry.Size as usize;
            }
        }
        cpu_set_data
    })
});
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| Inner value | `Vec<CpuSetData>` | A vector containing one [CpuSetData](CpuSetData.md) entry per logical processor on the system. Each entry pairs the processor's opaque Windows CPU Set ID with its zero-based logical processor index. The vector length equals the number of logical processors visible to the current process. |

## Remarks

### Initialization

`CPU_SET_INFORMATION` is wrapped in `once_cell::sync::Lazy`, so the initialization closure runs exactly once, on the first access. The initialization performs a two-pass call to `GetSystemCpuSetInformation`:

1. **Size query** — The first call passes a zero-length buffer to obtain the required byte size in `required_size`.
2. **Data retrieval** — A buffer of the required size is allocated, and the second call fills it with an array of variable-length `SYSTEM_CPU_SET_INFORMATION` structures.

The raw buffer is then walked entry-by-entry using each entry's `Size` field to advance the offset. Each entry is passed to the module-private helper `extract_cpu_set_data`, which reads the `CpuSet.Id` and `CpuSet.LogicalProcessorIndex` fields from the union and constructs a [CpuSetData](CpuSetData.md) value.

### Failure handling

If `GetSystemCpuSetInformation` fails on the second call, the error is logged to the find log and the resulting vector is empty. All downstream translation functions will return empty results, effectively disabling CPU set operations without crashing the service.

### Thread safety

The vector is protected by a `std::sync::Mutex`. All accessors lock the mutex before iterating. Because the data is read-only after initialization, contention is minimal — the lock exists primarily to satisfy Rust's `Send`/`Sync` requirements for a global mutable static.

### Topology assumptions

- The CPU set data is captured once at first access and is never refreshed. If the system topology changes at runtime (e.g., hot-add CPU), the cache becomes stale. This is acceptable because CPU hot-plug is extremely rare on desktop Windows systems where AffinityServiceRust operates.
- The `LogicalProcessorIndex` field is stored as `u8`, supporting up to 256 logical processors. Systems with more than 256 processors across multiple processor groups may require structural changes.

### Accessor

The public function [get_cpu_set_information](get_cpu_set_information.md) returns `&'static Mutex<Vec<CpuSetData>>`, providing shared access to the cache without exposing the `Lazy` wrapper.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Crate dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex` |
| **Populated by** | `GetSystemCpuSetInformation` (Win32), `extract_cpu_set_data` (module-private helper) |
| **Accessed via** | [get_cpu_set_information](get_cpu_set_information.md) |
| **Consumed by** | [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), [mask_from_cpusetids](mask_from_cpusetids.md) |
| **API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |
| **Privileges** | None — `GetSystemCpuSetInformation` does not require elevation |

## See Also

| Topic | Link |
|-------|------|
| Element type | [CpuSetData](CpuSetData.md) |
| Public accessor function | [get_cpu_set_information](get_cpu_set_information.md) |
| CPU indices → CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Affinity mask → CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set IDs → CPU indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU Set IDs → affinity mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU set application to processes | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd