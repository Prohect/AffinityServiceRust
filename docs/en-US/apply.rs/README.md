# apply module (AffinityServiceRust)

The `apply` module contains all functions that directly modify process and thread attributes via Windows API calls. It is the enforcement layer of AffinityServiceRust: once the configuration has been parsed and processes enumerated, every change to priority class, CPU affinity mask, CPU set, I/O priority, memory priority, ideal processor assignment, and prime-thread scheduling flows through functions defined here.

Each public function follows a common pattern: accept a process identifier, a reference to the parsed [ProcessConfig](../config.rs/ProcessConfig.md), a `dry_run` flag, any required OS handles, and an [ApplyConfigResult](#applyconfigresult) accumulator. The function reads the current value from the OS, compares it with the desired value, and—when they differ and `dry_run` is `false`—calls the appropriate Windows API to apply the change. Successes are recorded as *changes*; failures are recorded as *errors* (deduplicated by the [log_error_if_new](log_error_if_new.md) helper so that repeated failures for the same operation do not flood the log).

## Structs

| Name | Description |
|------|-------------|
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulates human-readable change descriptions and error messages produced during a single apply pass. |

## Functions

| Name | Description |
|------|-------------|
| [get_handles](get_handles.md) | Extracts the best available read and write `HANDLE`s from a [ProcessHandle](../winapi.rs/ProcessHandle.md), preferring full-access over limited. |
| [log_error_if_new](log_error_if_new.md) | Logs an error message only if the same pid / operation / error-code combination has not been logged before. |
| [apply_priority](apply_priority.md) | Sets the process priority class (Idle through Realtime) via `SetPriorityClass`. |
| [apply_affinity](apply_affinity.md) | Sets the hard CPU affinity mask on a process via `SetProcessAffinityMask`. |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | Redistributes thread ideal processors across a set of CPUs after an affinity or CPU-set change. |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | Applies a soft CPU set preference to a process via `SetProcessDefaultCpuSets`. |
| [apply_io_priority](apply_io_priority.md) | Sets I/O priority on a process via `NtSetInformationProcess`. |
| [apply_memory_priority](apply_memory_priority.md) | Sets memory priority on a process via `SetProcessInformation(ProcessMemoryPriority)`. |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | Queries thread cycle times for the top CPU-consuming threads to establish baselines for prime-thread selection. |
| [apply_prime_threads](apply_prime_threads.md) | Top-level orchestrator for the prime-thread scheduling pipeline (select → promote → demote). |
| [apply_prime_threads_select](apply_prime_threads_select.md) | Selects the top *N* threads by CPU cycles using hysteresis thresholds. |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | Pins newly-selected prime threads to performance-core CPU sets and optionally boosts their priority. |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | Unpins threads that lost prime status and restores their original priority. |
| [apply_ideal_processors](apply_ideal_processors.md) | Assigns ideal processors to threads whose start module matches configurable prefix rules. |
| [update_thread_stats](update_thread_stats.md) | Commits cached cycle and time counters so the next iteration computes correct deltas. |

## See Also

| Topic | Link |
|-------|------|
| Process-level apply orchestration | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Thread-level apply orchestration | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| Configuration model | [ProcessConfig](../config.rs/ProcessConfig.md) |
| Prime-thread scheduler state | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| OS handle wrappers | [ProcessHandle](../winapi.rs/ProcessHandle.md), [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Priority enumerations | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Error formatting helpers | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md), [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd