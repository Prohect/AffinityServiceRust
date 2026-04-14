# get_cpu_set_information function (winapi.rs)

Returns a reference to the lazily-initialized, mutex-guarded global vector of [CpuSetData](CpuSetData.md) entries that describes the system's CPU set topology. On the first call, the backing [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static is initialized by querying `GetSystemCpuSetInformation`; subsequent calls return the same cached reference.

## Syntax

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

## Parameters

None.

## Return value

A `&'static Mutex<Vec<CpuSetData>>` reference to the global [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) static. The caller must `.lock().unwrap()` the mutex to access the inner vector. The returned reference is valid for the lifetime of the process.

## Remarks

This function is a thin accessor that dereferences the `Lazy<Mutex<Vec<CpuSetData>>>` static. It exists to encapsulate the global and provide a clean public API — callers never reference `CPU_SET_INFORMATION` directly.

### Thread safety

The `Mutex` guarantees exclusive access to the inner `Vec<CpuSetData>`. Because the data is populated once at initialization time and never mutated afterward, contention is limited to the brief lock/unlock sequence. All CPU set translation functions ([cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), [mask_from_cpusetids](mask_from_cpusetids.md)) lock the mutex for the duration of their scan through the vector.

### Initialization trigger

The first call to `get_cpu_set_information` (or the first dereference of `CPU_SET_INFORMATION`) triggers the `Lazy` initializer, which:

1. Calls `GetSystemCpuSetInformation` with a zero-length buffer to determine the required size.
2. Allocates a byte buffer of the required size.
3. Calls `GetSystemCpuSetInformation` again to fill the buffer.
4. Walks the variable-length `SYSTEM_CPU_SET_INFORMATION` entries and extracts each `(id, logical_processor_index)` pair into a [CpuSetData](CpuSetData.md).

If `GetSystemCpuSetInformation` fails, the vector is empty and a message is logged via `log_to_find`.

### Typical usage

```rust
let guard = get_cpu_set_information().lock().unwrap();
for entry in guard.iter() {
    // access entry.id and entry.logical_processor_index
}
```

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), [mask_from_cpusetids](mask_from_cpusetids.md) |
| **Backing static** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| **API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) (during initialization only) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Global CPU set cache | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU set entry type | [CpuSetData](CpuSetData.md) |
| CPU indices → CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Affinity mask → CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set IDs → CPU indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU Set IDs → affinity mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd