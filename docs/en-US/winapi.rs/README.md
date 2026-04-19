# winapi module (AffinityServiceRust)

The `winapi` module provides safe Rust wrappers around the Windows API functions used throughout AffinityServiceRust for process and thread handle management, CPU set topology queries and conversions, privilege escalation, affinity inspection, ideal processor assignment, module enumeration, address resolution, timer resolution control, and child process cleanup. All handle-owning types implement RAII via `Drop` to prevent resource leaks.

## External FFI

The module links directly to `ntdll.dll` for undocumented NT APIs not exposed by the `windows` crate:

| Function | Description |
|----------|-------------|
| `NtQueryInformationProcess` | Queries per-process information classes (IO priority, memory priority, etc.). |
| `NtQueryInformationThread` | Queries per-thread information classes (start address via `ThreadQuerySetWin32StartAddress`). |
| `NtSetInformationProcess` | Sets per-process information classes (IO priority, memory priority). |
| `NtSetTimerResolution` | Sets the global Windows timer resolution to a specified interval. |

## Statics

| Name | Type | Description |
|------|------|-------------|
| `CPU_SET_INFORMATION` | `Lazy<Mutex<Vec<CpuSetData>>>` | Cached system CPU set topology obtained from `GetSystemCpuSetInformation`. Populated once on first access and reused for all CPU index ↔ CPU Set ID conversions. |
| `MODULE_CACHE` | `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` | Per-PID cache of module base address ranges and names. Populated on first [resolve_address_to_module](resolve_address_to_module.md) call per process and cleared on process exit via [drop_module_cache](drop_module_cache.md). |

## Structs

| Name | Description |
|------|-------------|
| [CpuSetData](CpuSetData.md) | Lightweight cache entry pairing a CPU Set ID with its logical processor index. |
| [ProcessHandle](ProcessHandle.md) | RAII wrapper for up to four process HANDLEs at different access levels (read/write × limited/full). |
| [ThreadHandle](ThreadHandle.md) | RAII wrapper for up to four thread HANDLEs at different access levels (read/write × limited/full). |

## Functions — Handle Management

| Name | Description |
|------|-------------|
| [get_process_handle](get_process_handle.md) | Opens a process with multiple access levels, returning a [ProcessHandle](ProcessHandle.md). |
| [get_thread_handle](get_thread_handle.md) | Opens a thread with multiple access levels, returning a [ThreadHandle](ThreadHandle.md). |
| [try_open_thread](try_open_thread.md) | Lower-level helper that opens a single thread handle with a specified access right. |

## Functions — CPU Set Conversions

| Name | Description |
|------|-------------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | Converts logical CPU indices (0, 1, 2…) to opaque Windows CPU Set IDs. |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Converts an affinity bitmask to CPU Set IDs. |
| [indices_from_cpusetids](indices_from_cpusetids.md) | Converts CPU Set IDs back to logical CPU indices. |
| [mask_from_cpusetids](mask_from_cpusetids.md) | Converts CPU Set IDs to an affinity bitmask. |
| [filter_indices_by_mask](filter_indices_by_mask.md) | Filters a list of CPU indices to only those present in an affinity mask. |

## Functions — Privilege & Elevation

| Name | Description |
|------|-------------|
| [is_running_as_admin](is_running_as_admin.md) | Checks whether the current process has an elevated administrator token. |
| [request_uac_elevation](request_uac_elevation.md) | Re-launches the current process with `runas` verb to trigger UAC elevation. |
| [enable_debug_privilege](enable_debug_privilege.md) | Enables `SeDebugPrivilege` on the current process token. |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | Enables `SeIncreaseBasePriorityPrivilege` on the current process token. |

## Functions — Process & Thread Inspection

| Name | Description |
|------|-------------|
| [is_affinity_unset](is_affinity_unset.md) | Checks if a process's affinity mask equals the full system mask (i.e., no restriction applied). |
| [get_thread_start_address](get_thread_start_address.md) | Retrieves a thread's Win32 start address via `NtQueryInformationThread`. |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | Wrapper for `SetThreadIdealProcessorEx`, setting the ideal processor hint. |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | Wrapper for `GetThreadIdealProcessorEx`, querying the current ideal processor. |

## Functions — Module Resolution

| Name | Description |
|------|-------------|
| [resolve_address_to_module](resolve_address_to_module.md) | Resolves a virtual address to a `"module.dll+0xOffset"` string using cached module data. |
| [drop_module_cache](drop_module_cache.md) | Removes cached module data for a specific PID. |
| [enumerate_process_modules](enumerate_process_modules.md) | Lists all modules in a process with base address, size, and name. |

## Functions — System Utilities

| Name | Description |
|------|-------------|
| [terminate_child_processes](terminate_child_processes.md) | Terminates all child processes of the current process (cleanup on startup). |
| [set_timer_resolution](set_timer_resolution.md) | Sets the Windows global timer resolution via `NtSetTimerResolution`. |

## See Also

| Topic | Link |
|-------|------|
| Process snapshot module | [process.rs](../process.rs/README.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Configuration application | [apply.rs](../apply.rs/README.md) |
| Priority type definitions | [priority.rs](../priority.rs/README.md) |
| CLI argument parsing | [cli.rs](../cli.rs/README.md) |
| Error code formatting | [error_codes.rs](../error_codes.rs/README.md) |
| Logging and error deduplication | [logging.rs](../logging.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*