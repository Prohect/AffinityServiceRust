# apply module (AffinityServiceRust)

The `apply` module is the core enforcement engine of AffinityServiceRust. It applies configured settings — process priority, CPU affinity, CPU sets, IO priority, memory priority, prime thread scheduling, and ideal processor hints — to running Windows processes. Each function reads the current state of a target process or thread via Windows API, compares it against the desired configuration, and makes changes only when a difference is detected. All changes and errors are accumulated into an [ApplyConfigResult](ApplyConfigResult.md) for structured logging.

## Structs

| Name | Description |
|------|-------------|
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change messages and error messages produced during a single apply pass. |

## Functions

| Name | Description |
|------|-------------|
| [get_handles](get_handles.md) | Extracts read and write `HANDLE`s from a [ProcessHandle](../winapi.rs/ProcessHandle.md), preferring full-access over limited. |
| [log_error_if_new](log_error_if_new.md) | Logs an error only the first time a unique (pid, operation, error_code) combination is seen. |
| [apply_priority](apply_priority.md) | Reads and optionally sets the process priority class. |
| [apply_affinity](apply_affinity.md) | Reads and optionally sets the process CPU affinity mask. Resets thread ideal processors on change. |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | Redistributes thread ideal processors across a set of CPUs after an affinity or CPU set change. |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | Reads and optionally sets the process default CPU set via the Windows CPU Sets API. |
| [apply_io_priority](apply_io_priority.md) | Reads and optionally sets process IO priority via `NtQueryInformationProcess`/`NtSetInformationProcess`. |
| [apply_memory_priority](apply_memory_priority.md) | Reads and optionally sets process memory priority via `GetProcessInformation`/`SetProcessInformation`. |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | Queries thread cycle times and computes deltas for prime thread selection. |
| [apply_prime_threads](apply_prime_threads.md) | Top-level orchestrator for prime thread scheduling: select, promote, and demote. |
| [apply_prime_threads_select](apply_prime_threads_select.md) | Selects which threads earn prime status using hysteresis thresholds. |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | Pins prime threads to dedicated CPUs via CPU sets and optionally boosts thread priority. |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | Unpins non-prime threads and restores their original thread priority. |
| [apply_ideal_processors](apply_ideal_processors.md) | Assigns ideal processor hints to threads based on module-prefix matching rules. |
| [update_thread_stats](update_thread_stats.md) | Caches current cycle/time measurements as "last" values for the next iteration's delta calculation. |

## See Also

| Topic | Link |
|-------|------|
| Configuration types | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md), [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Process handle management | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Process snapshot data | [ProcessEntry](../process.rs/ProcessEntry.md) |
| Error deduplication | [is_new_error](../logging.rs/is_new_error.md) |
| Priority enums | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*