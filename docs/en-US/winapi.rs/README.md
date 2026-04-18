# winapi module (AffinityServiceRust)

The `winapi` module provides low-level Windows API wrappers for process and thread handle management, CPU set manipulation, privilege elevation, thread inspection, module resolution, and timer configuration. It serves as the primary interface between the AffinityServiceRust service logic and the Windows operating system, encapsulating unsafe FFI calls behind safe Rust abstractions.

## Functions

| Function | Description |
|----------|-------------|
| [get_process_handle](get_process_handle.md) | Opens multiple process handles (read/write, limited/full) for a given PID. |
| [get_thread_handle](get_thread_handle.md) | Opens multiple thread handles (read/write, limited/full) for a given TID. |
| [try_open_thread](try_open_thread.md) | Attempts to open a single thread handle with the specified access rights. |
| [get_cpu_set_information](get_cpu_set_information.md) | Returns a reference to the lazily-initialized system CPU set information cache. |
| [cpusetids_from_indices](cpusetids_from_indices.md) | Converts logical CPU indices to Windows CPU Set IDs. |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Converts an affinity bitmask to Windows CPU Set IDs. |
| [indices_from_cpusetids](indices_from_cpusetids.md) | Converts Windows CPU Set IDs back to logical CPU indices. |
| [mask_from_cpusetids](mask_from_cpusetids.md) | Converts Windows CPU Set IDs to an affinity bitmask. |
| [filter_indices_by_mask](filter_indices_by_mask.md) | Filters CPU indices to only those allowed by an affinity mask. |
| [is_running_as_admin](is_running_as_admin.md) | Checks whether the current process is running with administrator privileges. |
| [request_uac_elevation](request_uac_elevation.md) | Restarts the process with elevated privileges via a UAC prompt. |
| [enable_debug_privilege](enable_debug_privilege.md) | Enables `SeDebugPrivilege` on the current process token. |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | Enables `SeIncreaseBasePriorityPrivilege` on the current process token. |
| [is_affinity_unset](is_affinity_unset.md) | Checks whether a process has its default (all-CPU) affinity mask. |
| [get_thread_start_address](get_thread_start_address.md) | Retrieves the start address of a thread via `NtQueryInformationThread`. |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | Sets the ideal processor for a thread using processor group and number. |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | Gets the ideal processor for a thread. |
| [resolve_address_to_module](resolve_address_to_module.md) | Resolves a memory address to a module name with offset (e.g., `kernel32.dll+0x345`). |
| [drop_module_cache](drop_module_cache.md) | Removes the cached module list for a given PID. |
| [terminate_child_processes](terminate_child_processes.md) | Terminates any child processes spawned by the current process. |
| [enumerate_process_modules](enumerate_process_modules.md) | Enumerates all loaded modules for a process, returning base address, size, and name. |
| [set_timer_resolution](set_timer_resolution.md) | Sets the system timer resolution via `NtSetTimerResolution`. |

## Structs

| Struct | Description |
|--------|-------------|
| [CpuSetData](CpuSetData.md) | Holds a CPU Set ID and its corresponding logical processor index. |
| [ProcessHandle](ProcessHandle.md) | RAII wrapper for a set of process handles with varying access levels. |
| [ThreadHandle](ThreadHandle.md) | RAII wrapper for a set of thread handles with varying access levels. |

## Statics

| Static | Description |
|--------|-------------|
| [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) | Lazily-initialized cache of system CPU set data queried from `GetSystemCpuSetInformation`. |
| [MODULE_CACHE](MODULE_CACHE.md) | Per-PID cache of loaded module information (base, size, name) for address resolution. |

## See Also

| Reference | Link |
|-----------|------|
| process module | [process.rs](../process.rs/README.md) |
| event_trace module | [event_trace.rs](../event_trace.rs/README.md) |
| logging module | [logging.rs](../logging.rs/README.md) |
| collections module | [collections.rs](../collections.rs/README.md) |
| error_codes module | [error_codes.rs](../error_codes.rs/README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
