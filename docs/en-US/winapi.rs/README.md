# winapi.rs Module (winapi.rs)

The `winapi` module provides low-level Windows API wrappers for process and thread handle management, CPU set enumeration and conversion, privilege escalation, and module address resolution. It is the primary interface between the application logic and the Win32/NT API surface.

## Overview

This module encapsulates all direct Windows API calls used by the application, organized into the following functional areas:

- **Process handles** — [`ProcessHandle`](ProcessHandle.md), [`get_process_handle`](get_process_handle.md)
- **Thread handles** — [`ThreadHandle`](ThreadHandle.md), [`get_thread_handle`](get_thread_handle.md), [`try_open_thread`](try_open_thread.md)
- **CPU set information** — [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md), [`get_cpu_set_information`](get_cpu_set_information.md), [`CpuSetData`](CpuSetData.md)
- **CPU set / index / mask conversions** — [`cpusetids_from_indices`](cpusetids_from_indices.md), [`cpusetids_from_mask`](cpusetids_from_mask.md), [`indices_from_cpusetids`](indices_from_cpusetids.md), [`mask_from_cpusetids`](mask_from_cpusetids.md), [`filter_indices_by_mask`](filter_indices_by_mask.md)
- **Privilege and elevation** — [`is_running_as_admin`](is_running_as_admin.md), [`request_uac_elevation`](request_uac_elevation.md), [`enable_debug_privilege`](enable_debug_privilege.md), [`enable_inc_base_priority_privilege`](enable_inc_base_priority_privilege.md)
- **Affinity queries** — [`is_affinity_unset`](is_affinity_unset.md)
- **Thread ideal processor** — [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md), [`get_thread_ideal_processor_ex`](get_thread_ideal_processor_ex.md), [`get_thread_start_address`](get_thread_start_address.md)
- **Module resolution** — [`MODULE_CACHE`](MODULE_CACHE.md), [`resolve_address_to_module`](resolve_address_to_module.md), [`drop_module_cache`](drop_module_cache.md), [`enumerate_process_modules`](enumerate_process_modules.md)
- **Timer resolution** — [`set_timer_resolution`](set_timer_resolution.md)
- **Process cleanup** — [`terminate_child_processes`](terminate_child_processes.md)

## Items

### Structs

| Name | Description |
| --- | --- |
| [CpuSetData](CpuSetData.md) | Stores a CPU set ID and its corresponding logical processor index. |
| [ProcessHandle](ProcessHandle.md) | Holds read/write HANDLEs to a process with limited and full access variants. Implements `Drop`. |
| [ThreadHandle](ThreadHandle.md) | Holds read/write HANDLEs to a thread with limited and full access variants. |

### Statics

| Name | Description |
| --- | --- |
| [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) | Lazy-initialized, mutex-guarded vector of all system CPU sets. |
| [MODULE_CACHE](MODULE_CACHE.md) | Per-process cache of enumerated module base/end addresses and names. |

### Functions

| Name | Description |
| --- | --- |
| [get_process_handle](get_process_handle.md) | Opens a process and returns a [`ProcessHandle`](ProcessHandle.md) with available access levels. |
| [get_thread_handle](get_thread_handle.md) | Opens a thread and returns a [`ThreadHandle`](ThreadHandle.md) with available access levels. |
| [try_open_thread](try_open_thread.md) | Attempts to open a thread with a specific access right, logging errors on failure. |
| [get_cpu_set_information](get_cpu_set_information.md) | Returns a reference to the global [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) static. |
| [cpusetids_from_indices](cpusetids_from_indices.md) | Converts logical processor indices to CPU set IDs. |
| [cpusetids_from_mask](cpusetids_from_mask.md) | Converts an affinity bitmask to CPU set IDs. |
| [indices_from_cpusetids](indices_from_cpusetids.md) | Converts CPU set IDs back to logical processor indices. |
| [mask_from_cpusetids](mask_from_cpusetids.md) | Converts CPU set IDs to an affinity bitmask. |
| [filter_indices_by_mask](filter_indices_by_mask.md) | Filters a list of CPU indices to only those present in an affinity mask. |
| [is_running_as_admin](is_running_as_admin.md) | Checks whether the current process is running with administrator privileges. |
| [request_uac_elevation](request_uac_elevation.md) | Spawns a UAC-elevated instance of the application via PowerShell. |
| [enable_debug_privilege](enable_debug_privilege.md) | Enables `SeDebugPrivilege` for the current process token. |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | Enables `SeIncreaseBasePriorityPrivilege` for the current process token. |
| [is_affinity_unset](is_affinity_unset.md) | Checks whether a process has its default (all-cores) affinity mask. |
| [get_thread_start_address](get_thread_start_address.md) | Queries the start address of a thread via `NtQueryInformationThread`. |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | Sets the ideal processor for a thread by group and number. |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | Queries the current ideal processor for a thread. |
| [resolve_address_to_module](resolve_address_to_module.md) | Resolves a memory address to a `"module.dll+0xABC"` formatted string. |
| [drop_module_cache](drop_module_cache.md) | Removes a process entry from the [`MODULE_CACHE`](MODULE_CACHE.md). |
| [terminate_child_processes](terminate_child_processes.md) | Kills orphaned console host processes spawned during UAC elevation. |
| [enumerate_process_modules](enumerate_process_modules.md) | Enumerates all loaded modules for a process, returning base/end addresses and names. |
| [set_timer_resolution](set_timer_resolution.md) | Sets the system timer resolution via `NtSetTimerResolution`. |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/winapi.rs` |
| **Called by** | [`apply_config_process_level`](../main.rs/apply_config_process_level.md)/[`apply_config_thread_level`](../main.rs/apply_config_thread_level.md), [`apply.rs`](../apply.rs/README.md) functions, [`main`](../main.rs/main.md) |
| **Key dependencies** | `windows` crate, [`ProcessConfig`](../config.rs/ProcessConfig.md), [`Operation`](../logging.rs/Operation.md), [`is_new_error`](../logging.rs/is_new_error.md) |