# winapi module (AffinityServiceRust)

The `winapi` module provides safe Rust wrappers around Windows API functions used throughout AffinityServiceRust for process and thread handle management, CPU set translation, privilege elevation, and low-level system queries. All handle-owning types implement `Drop` for automatic cleanup, and all public functions translate Windows error codes into Rust-friendly return types. The module also exposes lazily-initialized global caches for CPU set topology and per-process module maps.

## Structs

| Struct | Description |
|--------|-------------|
| [CpuSetData](CpuSetData.md) | Pairs a Windows CPU Set ID with its corresponding logical processor index. |
| [ProcessHandle](ProcessHandle.md) | RAII container holding up to four process handles at different access levels. Automatically closes all valid handles on drop. |
| [ThreadHandle](ThreadHandle.md) | RAII container holding up to four thread handles at different access levels. Automatically closes all valid handles on drop. |

## Functions

| Function | Description |
|----------|-------------|
| [get_process_handle](get_process_handle.md) | Opens a process with multiple access levels and returns a [ProcessHandle](ProcessHandle.md). |
| [get_thread_handle](get_thread_handle.md) | Opens a thread with multiple access levels and returns a [ThreadHandle](ThreadHandle.md). |
| [try_open_thread](try_open_thread.md) | Attempts to open a thread with a single specific access right, returning an invalid handle on failure. |
| [get_cpu_set_information](get_cpu_set_information.md) | Returns a reference to the lazily-initialized system CPU set data. |
| [cpusetids_from_indices](cpusetids_from_indices.md) | Converts logical CPU indices to Windows CPU Set IDs. |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Converts an affinity bitmask to Windows CPU Set IDs. |
| [indices_from_cpusetids](indices_from_cpusetids.md) | Converts Windows CPU Set IDs back to logical CPU indices. |
| [mask_from_cpusetids](mask_from_cpusetids.md) | Converts Windows CPU Set IDs to an affinity bitmask. |
| [filter_indices_by_mask](filter_indices_by_mask.md) | Filters a slice of CPU indices, keeping only those present in an affinity mask. |
| [is_running_as_admin](is_running_as_admin.md) | Checks whether the current process is running with administrator (elevated) privileges. |
| [request_uac_elevation](request_uac_elevation.md) | Requests UAC elevation by re-launching the process via `Start-Process -Verb RunAs`. |
| [enable_debug_privilege](enable_debug_privilege.md) | Enables `SeDebugPrivilege` on the current process token. |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | Enables `SeIncreaseBasePriorityPrivilege` on the current process token. |
| [is_affinity_unset](is_affinity_unset.md) | Checks whether a process's affinity mask equals the system default (all CPUs). |
| [get_thread_start_address](get_thread_start_address.md) | Queries a thread's start address via `NtQueryInformationThread`. |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | Sets a thread's ideal processor with processor-group awareness. |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | Gets a thread's current ideal processor with processor-group awareness. |
| [resolve_address_to_module](resolve_address_to_module.md) | Resolves a memory address to a module name plus offset string (e.g., `"kernel32.dll+0x1A3"`). |
| [drop_module_cache](drop_module_cache.md) | Clears the cached module list for a specific process. |
| [terminate_child_processes](terminate_child_processes.md) | Terminates all child processes of the current process. |
| [enumerate_process_modules](enumerate_process_modules.md) | Enumerates loaded modules in a target process, returning base address, size, and name for each. |
| [set_timer_resolution](set_timer_resolution.md) | Sets the system timer resolution via `NtSetTimerResolution`. |

## Statics

| Static | Description |
|--------|-------------|
| [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) | Lazily-initialized, mutex-guarded vector of [CpuSetData](CpuSetData.md) entries representing the system's CPU set topology. |
| [MODULE_CACHE](MODULE_CACHE.md) | Per-process cache of enumerated module address ranges and names, used by [resolve_address_to_module](resolve_address_to_module.md). |

## See Also

| Topic | Link |
|-------|------|
| Process snapshot and enumeration | [process module](../process.rs/README.md) |
| Rule application using handles and CPU sets | [apply module](../apply.rs/README.md) |
| Prime-thread scheduler (consumes thread handles and CPU sets) | [scheduler module](../scheduler.rs/README.md) |
| Configuration parsing (CPU specs, process rules) | [config module](../config.rs/README.md) |
| Priority and IO/memory priority enums | [priority module](../priority.rs/README.md) |
| Error code formatting helpers | [error_codes module](../error_codes.rs/README.md) |
| CLI argument parsing | [cli module](../cli.rs/README.md) |
| Service main loop | [main module](../main.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd