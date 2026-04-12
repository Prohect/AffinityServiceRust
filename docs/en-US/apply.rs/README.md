# apply.rs Module (apply.rs)

The `apply` module implements the core logic for applying process configuration settings to target processes. It is the primary execution engine invoked each loop iteration by [`apply_config`](../main.rs/apply_config.md) in `main.rs`.

## Overview

This module provides functions to configure the following process and thread attributes via Windows API:

- **Process priority class** — [`apply_priority`](apply_priority.md)
- **CPU affinity mask** (hard binding) — [`apply_affinity`](apply_affinity.md)
- **CPU Sets** (soft preference) — [`apply_process_default_cpuset`](apply_process_default_cpuset.md)
- **I/O priority** — [`apply_io_priority`](apply_io_priority.md)
- **Memory priority** — [`apply_memory_priority`](apply_memory_priority.md)
- **Prime thread scheduling** (pin top threads to fast cores) — [`apply_prime_threads`](apply_prime_threads.md)
- **Ideal processor assignment** (per-thread, module-based) — [`apply_ideal_processors`](apply_ideal_processors.md)

All functions collect their results into an [`ApplyConfigResult`](ApplyConfigResult.md) struct, which accumulates human-readable change descriptions and error messages for the caller to log.

## Items

### Structs

| Name | Description |
| --- | --- |
| [ApplyConfigResult](ApplyConfigResult.md) | Collects changes and errors during configuration application. |

### Functions

| Name | Description |
| --- | --- |
| [get_handles](get_handles.md) | Extracts read and write `HANDLE`s from a [`ProcessHandle`](../winapi.rs/ProcessHandle.md). |
| [log_error_if_new](log_error_if_new.md) | Logs an error only if it has not been logged before for the same pid/tid/operation. |
| [apply_priority](apply_priority.md) | Sets the process priority class. |
| [apply_affinity](apply_affinity.md) | Sets the hard CPU affinity mask for a process. |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | Redistributes thread ideal processors across specified CPUs after an affinity or CPU set change. |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | Sets the default CPU Set (soft CPU preference) for a process. |
| [apply_io_priority](apply_io_priority.md) | Sets the I/O priority for a process. |
| [apply_memory_priority](apply_memory_priority.md) | Sets the memory page priority for a process. |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | Prefetches thread cycle counts for prime thread selection. |
| [apply_prime_threads](apply_prime_threads.md) | Main orchestration for prime thread scheduling. |
| [apply_prime_threads_select](apply_prime_threads_select.md) | Selects top threads for prime status using hysteresis. |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | Promotes selected threads to prime CPUs with priority boost. |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | Demotes threads that no longer qualify for prime status. |
| [apply_ideal_processors](apply_ideal_processors.md) | Assigns ideal processors to threads based on start module prefix rules. |
| [update_thread_stats](update_thread_stats.md) | Persists cached cycle and time data for delta calculation in the next iteration. |

## Execution Flow

The typical call order within a single loop iteration (driven by [`apply_config`](../main.rs/apply_config.md)):

1. `apply_priority` — set process priority class
2. `apply_affinity` — set hard affinity mask (may trigger `reset_thread_ideal_processors`)
3. `apply_process_default_cpuset` — set soft CPU set preference (may trigger `reset_thread_ideal_processors` if `cpu_set_reset_ideal` is enabled)
4. `apply_io_priority` — set I/O priority
5. `apply_memory_priority` — set memory priority
6. `prefetch_all_thread_cycles` — gather thread cycle baselines
7. `apply_prime_threads` → `apply_prime_threads_select` → `apply_prime_threads_promote` → `apply_prime_threads_demote`
8. `apply_ideal_processors` — module-based ideal processor assignment
9. `update_thread_stats` — persist cached data for next iteration

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/apply.rs` |
| **Called by** | [`apply_config`](../main.rs/apply_config.md) in `src/main.rs` |
| **Key dependencies** | [`ProcessConfig`](../config.rs/ProcessConfig.md), [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md), [`ProcessEntry`](../process.rs/ProcessEntry.md), [`ProcessHandle`](../winapi.rs/ProcessHandle.md) |