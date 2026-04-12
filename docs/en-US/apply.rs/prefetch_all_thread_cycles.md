# prefetch_all_thread_cycles function (apply.rs)

Prefetches thread cycle counts for prime thread selection. Opens handles to the top CPU-consuming threads (by kernel+user time delta) and queries their cycle counters via `QueryThreadCycleTime`. This establishes baseline measurements for the hysteresis-based prime thread promotion/demotion algorithm.

## Syntax

```rust
pub fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process ID of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing the process rule configuration. Used for the process name when logging errors and opening thread handles.

`process`

Mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. Provides thread enumeration with kernel and user time data from the system snapshot.

`prime_scheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that holds per-thread cached cycle counts, total time stats, start addresses, handles, and active streak counters.

`apply_config_result`

Mutable reference to [ApplyConfigResult](ApplyConfigResult.md) for collecting error messages.

## Return value

This function does not return a value. It updates state in `prime_scheduler` as a side effect.

## Remarks

### Algorithm

1. **Collect time deltas** — Iterates all threads in `process`, computes each thread's total CPU time (kernel + user), caches it in the scheduler's `cached_total_time`, and calculates the delta from `last_total_time`.
2. **Sort by time delta** — Sorts threads descending by time delta so the most active threads are processed first.
3. **Limit candidate count** — Only the top N threads are processed, where N = `logical CPU count × 2` (from the system CPU set information). This avoids opening handles to idle threads.
4. **Open handles** — For each candidate thread that does not already have an open handle, calls [get_thread_handle](../winapi.rs/get_thread_handle.md) to obtain a `ThreadHandle`. If the thread's `start_address` is not yet resolved, queries it via `get_thread_start_address`.
5. **Query cycle counts** — Calls `QueryThreadCycleTime` on each candidate's read handle, storing the result in `cached_cycles`. On failure, the error is logged via [log_error_if_new](log_error_if_new.md).
6. **Compute cycle deltas** — After querying, builds a list of `(tid, delta_cycles)` tuples by subtracting `last_cycles` from `cached_cycles`. Threads with zero cached cycles have their `active_streak` reset to 0.
7. **Update active streaks** — Calls `prime_scheduler.update_active_streaks()` with the cycle delta list. Active streaks are used later by the hysteresis selection algorithm in [apply_prime_threads_select](apply_prime_threads_select.md).

### Handle reuse

Thread handles opened during prefetch are stored in `ThreadStats.handle` and reused across iterations. They are only closed when a thread exits (detected in [apply_prime_threads](apply_prime_threads.md)) or when `ThreadHandle` is dropped.

### Relationship to prime thread scheduling

This function is a prerequisite for [apply_prime_threads](apply_prime_threads.md). It must be called first to populate `cached_cycles` and `cached_total_time` so that the prime thread selection and promotion/demotion logic has accurate, up-to-date cycle data. The cycle deltas computed here form the basis of the hysteresis comparison that determines which threads get promoted to prime CPUs.

### Error handling

- If `get_thread_handle` fails, the thread is silently skipped (the error is already logged internally by the winapi module).
- If `QueryThreadCycleTime` fails, the error is logged via [log_error_if_new](log_error_if_new.md) with `Operation::QueryThreadCycleTime` and the thread's cycle data is not updated.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L597–L702 |
| **Called by** | [apply_config](../main.rs/apply_config.md) in main.rs |
| **Calls** | [get_thread_handle](../winapi.rs/get_thread_handle.md), `get_thread_start_address`, `QueryThreadCycleTime`, [log_error_if_new](log_error_if_new.md), `PrimeThreadScheduler::update_active_streaks` |
| **Windows API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) (via get_thread_handle), [QueryThreadCycleTime](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) |

## See also

- [apply_prime_threads](apply_prime_threads.md)
- [apply_prime_threads_select](apply_prime_threads_select.md)
- [update_thread_stats](update_thread_stats.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)