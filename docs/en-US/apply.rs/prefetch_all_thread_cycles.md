# prefetch_all_thread_cycles function (apply.rs)

Prefetches thread cycle counts for the top CPU-consuming threads of a process, establishing baseline measurements for the hysteresis-based prime thread promotion/demotion algorithm.

This function opens handles to threads sorted by kernel+user time, queries their cycle counters via `QueryThreadCycleTime`, computes delta cycles from cached values, and updates the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) with active streak information. It also resolves thread start addresses for later module matching.

## Syntax

```AffinityServiceRust/src/apply.rs#L585-591
pub fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid: u32`

The process ID whose threads are being measured.

`config: &ThreadLevelConfig`

The thread-level configuration for the process. The `name` field is used for error logging.

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

Lazy accessor returning the map of thread IDs to their `SYSTEM_THREAD_INFORMATION` structures (from `NtQuerySystemInformation`).

`prime_scheduler: &mut PrimeThreadScheduler`

Mutable reference to the scheduler that stores per-thread cycle/time caches, active streaks, and thread handles. Updated in-place with new measurements.

`apply_config_result: &mut ApplyConfigResult`

Accumulator for errors encountered during cycle time queries.

## Return value

This function does not return a value. Results are stored in `prime_scheduler` as side effects:

- `cached_cycles` — current raw cycle count per thread.
- `cached_total_time` — current kernel+user time per thread.
- Active streaks are updated via [`PrimeThreadScheduler::update_active_streaks`](../scheduler.rs/PrimeThreadScheduler.md).

## Remarks

### Algorithm

1. **Compute time deltas** — For every thread in the process, computes `cached_total_time` (kernel + user) and the delta from `last_total_time`. Stores the result in a list sorted descending by delta time.

2. **Prune dead threads** — Removes entries from `pid_to_process_stats` for threads that no longer exist. Drops cached thread handles for dead threads to avoid handle leaks.

3. **Cap candidate count** — Only the top threads are processed, capped at `min(cpu_count * 2, thread_count)`. This limits overhead on processes with hundreds or thousands of threads.

4. **Open thread handles** — For each candidate thread, opens a thread handle (if not already cached in `thread_stats.handle`) via [`get_thread_handle`](../winapi.rs/get_thread_handle.md). Handles are cached for reuse across iterations.

5. **Resolve start address** — If `thread_stats.start_address` is zero, queries the thread start address via [`get_thread_start_address`](../winapi.rs/get_thread_start_address.md). This is later used for module-prefix matching during prime thread promotion.

6. **Query cycle time** — Calls `QueryThreadCycleTime` to read the thread's CPU cycle counter. The value is stored in `cached_cycles`.

7. **Compute cycle deltas** — After all queries, computes `cached_cycles - last_cycles` for each thread with nonzero cached cycles. Threads with zero cached cycles have their `active_streak` reset to 0.

8. **Update active streaks** — Calls `PrimeThreadScheduler::update_active_streaks` with the cycle delta list. Threads whose cycles exceed the keep threshold have their streak incremented; others are reset.

### Thread handle caching

Thread handles are opened once and stored in `thread_stats.handle`. This avoids repeatedly calling `OpenThread` every polling interval. Handles are automatically dropped when threads are pruned from the stats map.

### Candidate pool sizing

The candidate pool is sized at `cpu_count * 2` (based on CPU set information), ensuring enough threads are tracked to handle churn without excessive overhead. The pool always includes at least the thread count minus one.

### Platform notes

- `QueryThreadCycleTime` returns CPU cycles (not wall-clock time), providing a high-resolution, scheduling-independent measure of thread activity.
- The function prefers full-access read handles (`r_handle`) over limited handles (`r_limited_handle`).

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Called by** | [`apply_prime_threads`](apply_prime_threads.md), main polling loop |
| **Calls** | [`get_thread_handle`](../winapi.rs/get_thread_handle.md), [`get_thread_start_address`](../winapi.rs/get_thread_start_address.md), [`PrimeThreadScheduler::update_active_streaks`](../scheduler.rs/PrimeThreadScheduler.md), [`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | [`QueryThreadCycleTime`](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) |
| **Privileges** | `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` on target threads |

## See Also

| Topic | Description |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | Orchestrator that calls this function before prime selection |
| [apply_prime_threads_select](apply_prime_threads_select.md) | Uses cycle deltas computed here for hysteresis-based selection |
| [update_thread_stats](update_thread_stats.md) | Copies cached values to `last_*` fields after the apply cycle completes |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Scheduler struct that stores all per-thread statistics |
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for changes and errors |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*