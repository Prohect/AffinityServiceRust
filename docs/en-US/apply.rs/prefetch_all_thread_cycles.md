# prefetch_all_thread_cycles function (apply.rs)

The `prefetch_all_thread_cycles` function opens handles to the top CPU-consuming threads in a process and queries their hardware cycle counters via `QueryThreadCycleTime`. This establishes baseline measurements that are later consumed by the prime-thread selection algorithm to compute per-thread cycle deltas. The function also resolves and caches each thread's start address for later module-prefix matching during promotion. After querying cycles, it updates the active-streak counters used by the hysteresis-based selection logic.

## Syntax

```AffinityServiceRust/src/apply.rs#L584-595
pub fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Used for thread-stats lookup in the scheduler and for error deduplication in log messages. |
| `config` | `&ThreadLevelConfig` | The thread-level configuration. The `name` field is used in error log messages and passed to `get_thread_handle` for handle acquisition. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A lazy closure that returns a reference to a map of thread IDs to their `SYSTEM_THREAD_INFORMATION` snapshots from the most recent system process information query. The closure is invoked once when thread data is needed. The `KernelTime` and `UserTime` fields are summed to compute each thread's total CPU time for initial sorting and delta computation. |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | The mutable prime-thread scheduler state that stores per-thread statistics including cached cycles, last cycles, total time values, active streaks, thread handles, and start addresses. This function reads and updates multiple fields in the scheduler's per-thread stats. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for error messages. Only `QueryThreadCycleTime` failures are logged here; no change messages are produced by this function. |

## Return value

This function does not return a value. All outcomes are communicated through mutations to `prime_scheduler` thread stats and error messages appended to `apply_config_result`.

## Remarks

### Algorithm

1. **Compute time deltas**: For every thread in `threads`, the function computes `KernelTime + UserTime` and stores it in `thread_stats.cached_total_time`. It also computes the delta from the previous cycle's `last_total_time`. The results are collected into a fixed-capacity list of `(tid, delta_time)` tuples.

2. **Sort by time delta**: The list is sorted in descending order of time delta using `sort_unstable_by_key` with `Reverse`, so the most CPU-active threads appear first.

3. **Cap the candidate count**: Only the top `min(cpu_count * 2, thread_count).saturating_sub(1) + 1` threads are retained for cycle querying. The `saturating_sub(1)` prevents arithmetic underflow when the thread count is zero. This limits the number of thread handles opened to approximately twice the CPU count, which is sufficient to cover prime candidates and a margin of alternates without opening handles to every thread in a process that may have hundreds or thousands.

4. **Open handles and query cycles**: For each candidate thread:
   - If the thread does not already have a cached handle in the scheduler, `get_thread_handle` is called to open one. The handle is stored in `thread_stats.handle` for reuse across apply cycles.
   - The read handle is selected (preferring `r_handle` over `r_limited_handle`).
   - If `thread_stats.start_address` is `0` (not yet resolved), `get_thread_start_address` is called to populate it.
   - `QueryThreadCycleTime` is called with the read handle to obtain the current hardware cycle count. On success, the value is stored in `thread_stats.cached_cycles`. On failure, the Win32 error code is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::QueryThreadCycleTime`.

5. **Compute cycle deltas**: After all candidate threads have been queried, the function iterates over the scheduler's thread stats for the process and computes `cached_cycles - last_cycles` (saturating subtraction) for each thread that has a non-zero `cached_cycles`. Threads with zero `cached_cycles` (i.e., those whose handles could not be opened or whose cycle query failed) have their `active_streak` reset to `0`.

6. **Update active streaks**: The computed cycle deltas are passed to `prime_scheduler.update_active_streaks`, which increments the streak counter for threads with positive deltas and resets it for threads with zero deltas. The active streak is used by the hysteresis selection in [`apply_prime_threads_select`](apply_prime_threads_select.md) to require sustained activity before promotion.

### Side effects

- Opens and caches OS thread handles in `prime_scheduler` for threads that did not previously have handles.
- Resolves and caches thread start addresses for later module-name resolution during the promotion phase.
- Updates `cached_total_time`, `cached_cycles`, and active-streak counters in the scheduler's per-thread stats.
- This function does **not** apply any CPU sets, priorities, or other scheduling changes to threads. It is a pure measurement/prefetch phase.

### Edge cases

- If `threads` is empty or all time deltas are zero, the function returns early after the initial collection step.
- Threads for which `get_thread_handle` returns `None` (e.g., the thread has exited or access is denied) are silently skipped. The handle acquisition function logs its own errors internally.
- The candidate cap uses `(cpu_count * 2).min(thread_count).saturating_sub(1)` which safely handles the case where `thread_count` is 0 without arithmetic underflow.
- The `TIDS_CAPED` and `TIDS_FULL` constants from the `collections` module control the maximum capacity of the fixed-size lists used internally.

### When this function is called

This function is called **before** [`apply_prime_threads`](apply_prime_threads.md) in the per-process apply pipeline. The cycle values it caches are consumed by `apply_prime_threads` to compute deltas for the select/promote/demote pipeline. After the pipeline completes, [`update_thread_stats`](update_thread_stats.md) commits the cached values into `last_cycles` and `last_total_time` and clears the caches for the next cycle.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | `QueryThreadCycleTime`, `GetLastError` |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that iterates matched processes |
| Callees | [`log_error_if_new`](log_error_if_new.md), `winapi::get_thread_handle`, `winapi::get_thread_start_address`, `winapi::get_cpu_set_information`, `PrimeThreadScheduler::get_thread_stats`, `PrimeThreadScheduler::update_active_streaks`, `error_codes::error_from_code_win32` |
| Privileges | Requires thread handles with `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` for `QueryThreadCycleTime`. |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| update_thread_stats | [`update_thread_stats`](update_thread_stats.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*