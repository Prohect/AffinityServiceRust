# apply module (AffinityServiceRust)

The `apply` module is responsible for applying process- and thread-level configuration policies to running Windows processes. It provides functions that read current process/thread attributes (priority, affinity, CPU sets, I/O priority, memory priority), compare them against a desired configuration, and issue the appropriate Windows API calls to bring the process into compliance. The module also implements a "prime thread" scheduling algorithm that identifies the most CPU-intensive threads in a process and pins them to designated high-performance cores using CPU Sets and ideal processor assignments, with hysteresis-based promotion and demotion to avoid rapid oscillation.

## Functions

| Function | Description |
|----------|-------------|
| [`get_handles`](get_handles.md) | Extracts read and write `HANDLE` values from a `ProcessHandle`, preferring full-access handles over limited ones. |
| [`log_error_if_new`](log_error_if_new.md) | Logs an error to `ApplyConfigResult` only if the same pid/operation/error-code combination has not been logged before. |
| [`apply_priority`](apply_priority.md) | Reads the current process priority class and sets it to the configured value if different. |
| [`apply_affinity`](apply_affinity.md) | Reads the current process affinity mask and sets it to the configured CPU mask, redistributing thread ideal processors on change. |
| [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) | Redistributes thread ideal processors across a set of CPUs after an affinity or CPU-set change, ordered by CPU time with a random shift. |
| [`apply_process_default_cpuset`](apply_process_default_cpuset.md) | Queries and sets the default CPU Set IDs for a process, optionally resetting thread ideal processors afterward. |
| [`apply_io_priority`](apply_io_priority.md) | Reads the current process I/O priority via `NtQueryInformationProcess` and sets it to the configured value via `NtSetInformationProcess`. |
| [`apply_memory_priority`](apply_memory_priority.md) | Reads the current process memory priority via `GetProcessInformation` and sets it to the configured value via `SetProcessInformation`. |
| [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) | Opens handles to the top CPU-consuming threads and queries their cycle counters to establish baseline measurements for prime-thread selection. |
| [`apply_prime_threads`](apply_prime_threads.md) | Orchestrates the prime-thread scheduling pipeline: sorts threads by CPU delta, selects candidates, promotes winners, and demotes losers. |
| [`apply_prime_threads_select`](apply_prime_threads_select.md) | Selects the top threads for prime status using hysteresis thresholds to prevent rapid flipping. |
| [`apply_prime_threads_promote`](apply_prime_threads_promote.md) | Pins newly-selected prime threads to designated CPUs via `SetThreadSelectedCpuSets` and optionally boosts their priority. |
| [`apply_prime_threads_demote`](apply_prime_threads_demote.md) | Removes CPU-set pinning and restores original thread priority for threads that no longer qualify as prime. |
| [`apply_ideal_processors`](apply_ideal_processors.md) | Assigns ideal processors to threads based on module-prefix matching rules, selecting top N threads by cycle count per rule. |
| [`update_thread_stats`](update_thread_stats.md) | Commits cached cycle and time measurements into `last_cycles`/`last_total_time` and resets the cached values to zero. |

## Structs

| Struct | Description |
|--------|-------------|
| [`ApplyConfigResult`](ApplyConfigResult.md) | Accumulates human-readable change descriptions and error messages produced during a single configuration-apply pass. |

## See Also

| Reference | Link |
|-----------|------|
| config module | [`config.rs`](../config.rs/README.md) |
| priority module | [`priority.rs`](../priority.rs/README.md) |
| process module | [`process.rs`](../process.rs/README.md) |
| scheduler module | [`scheduler.rs`](../scheduler.rs/README.md) |
| winapi module | [`winapi.rs`](../winapi.rs/README.md) |
| logging module | [`logging.rs`](../logging.rs/README.md) |
| error_codes module | [`error_codes.rs`](../error_codes.rs/README.md) |
| collections module | [`collections.rs`](../collections.rs/README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*