# apply_thread_level function (main.rs)

Applies thread-level settings to a single process on every polling iteration. This includes prefetching thread cycle times for delta computation, running the prime thread scheduling algorithm, and assigning ideal processor hints. The function only executes if the process's [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) contains at least one thread-level setting (prime thread CPUs, prime thread prefixes, ideal processor rules, or top-X thread tracking).

## Syntax

```rust
fn apply_thread_level<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## Parameters

`pid: u32`

The process identifier of the target process.

`config: &ThreadLevelConfig`

The [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) for the matched process. Contains prime thread CPU assignments, module-prefix matching rules, ideal processor rules, and the top-X thread tracking count. If all of these fields are empty/zero, the function returns immediately.

`prime_core_scheduler: &mut PrimeThreadScheduler`

The [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) that tracks per-thread cycle time deltas, active streaks, and prime/non-prime status across iterations. The scheduler is marked alive for this PID and updated with current cycle data.

`process: &'a ProcessEntry`

The [`ProcessEntry`](../process.rs/ProcessEntry.md) for the target process, used to enumerate threads when the thread cache has not yet been populated.

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

A lazy-evaluated closure that returns the thread map for the process. Backed by a `OnceCell` so that thread enumeration happens at most once per apply cycle, shared with the process-level apply pass when called from [`apply_config`](apply_config.md).

`dry_run: bool`

When **true**, all sub-functions record what *would* change without calling Windows APIs. When **false**, changes are applied to live threads.

`apply_configs: &mut ApplyConfigResult`

Accumulator for change descriptions and error messages. See [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md).

## Return value

This function does not return a value. Results are communicated through `apply_configs`.

## Remarks

The function performs the following steps in order:

1. **Guard check** — Returns immediately if `prime_threads_cpus`, `prime_threads_prefixes`, `ideal_processor_rules` are all empty and `track_top_x_threads` is zero.
2. **Query affinity mask** — If `prime_threads_cpus` is non-empty, opens a process handle and calls `GetProcessAffinityMask` to determine which CPUs the process is allowed to use. This mask constrains which cores the prime thread scheduler can assign.
3. **Drop module cache** — Calls [`drop_module_cache`](../winapi.rs/drop_module_cache.md) for the PID so that thread-to-module lookups are refreshed.
4. **Mark alive** — Calls `prime_core_scheduler.set_alive(pid)` so the scheduler knows this process is still running (dead processes are cleaned up later in the main loop).
5. **Prefetch cycle times** — Calls [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) to query current thread cycle counts and compute deltas from the previous iteration, feeding data into the scheduler's ranking algorithm.
6. **Apply prime threads** — Calls [`apply_prime_threads`](../apply.rs/apply_prime_threads.md) to select, promote, and demote threads based on cycle time rankings and hysteresis thresholds.
7. **Apply ideal processors** — Calls [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) to assign ideal processor hints to threads matching module-prefix rules.
8. **Update stats** — Calls [`update_thread_stats`](../apply.rs/update_thread_stats.md) to cache current cycle/time measurements as baseline values for the next iteration.

### Difference from apply_process_level

[`apply_process_level`](apply_process_level.md) runs once per process (or once per config reload when `continuous_process_level_apply` is not set) and sets process-wide attributes. `apply_thread_level` runs on **every** polling iteration because thread cycle rankings change continuously and prime thread selection must be re-evaluated.

### Thread cache sharing

When called from [`apply_config`](apply_config.md), the `threads` closure shares the same `OnceCell` as the process-level pass, avoiding a redundant `NtQuerySystemInformation` call for thread enumeration.

## Requirements

| | |
|---|---|
| **Module** | `src/main.rs` |
| **Callers** | [`apply_config`](apply_config.md), main loop thread-level pass |
| **Callees** | [`get_process_handle`](../winapi.rs/get_process_handle.md), [`drop_module_cache`](../winapi.rs/drop_module_cache.md), [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md), [`apply_prime_threads`](../apply.rs/apply_prime_threads.md), [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md), [`update_thread_stats`](../apply.rs/update_thread_stats.md) |
| **Win32 API** | [`GetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |
| **Privileges** | `PROCESS_QUERY_LIMITED_INFORMATION` (affinity query), thread-level access rights delegated to callees |

## See Also

| Topic | Link |
|-------|------|
| Process-level counterpart | [apply_process_level](apply_process_level.md) |
| Combined orchestrator | [apply_config](apply_config.md) |
| Thread-level config type | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Apply engine overview | [apply.rs](../apply.rs/README.md) |
| Result accumulator | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*