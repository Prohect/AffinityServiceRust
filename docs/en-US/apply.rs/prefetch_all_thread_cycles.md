# prefetch_all_thread_cycles function (apply.rs)

Queries thread cycle times for the top CPU-consuming threads in a process, establishing baseline measurements that drive the hysteresis-based prime-thread promotion and demotion algorithm. This function opens thread handles, reads cycle counters via `QueryThreadCycleTime`, and updates the cached cycle/time values in the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) so that downstream functions can compute per-thread CPU deltas.

## Syntax

```AffinityServiceRust/src/apply.rs#L602-612
pub fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used as the key into the scheduler's per-process stats map and for thread handle acquisition. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | The parsed configuration for this process. The `name` field is used in error messages when thread handle opening or cycle-time queries fail. |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | Snapshot entry for the target process. Provides the thread list and per-thread kernel/user time from the most recent `NtQuerySystemInformation` snapshot. |
| `prime_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | The scheduler state that holds per-thread [ThreadStats](../scheduler.rs/ThreadStats.md). This function writes `cached_total_time`, `cached_cycles`, `start_address`, and thread handles into the stats entries. It also calls `update_active_streaks` at the end to maintain streak counters used by the hysteresis algorithm. |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for error messages. This function does not record changes — it only records errors when `QueryThreadCycleTime` or handle acquisition fails. |

## Return value

None (`()`). Results are written into `prime_scheduler` (cached cycle/time values, thread handles, active streaks) and errors are accumulated in `apply_config_result`.

## Remarks

### Algorithm

The function operates in three phases:

**Phase 1 — Collect and sort by kernel+user time delta.**
All threads from `process.get_threads()` are collected into a `Vec<(tid, delta_time)>`, where `delta_time` is the difference between the current total CPU time (`KernelTime + UserTime`) and the `last_total_time` stored in the thread's [ThreadStats](../scheduler.rs/ThreadStats.md). The current total is also written to `cached_total_time` for later commit by [update_thread_stats](update_thread_stats.md). The vector is sorted in descending order of delta time so the most CPU-active threads are processed first.

**Phase 2 — Open handles and query cycle counters.**
The function iterates over the sorted thread list up to a limit of `min(cpu_count * 2, thread_count) - 1`, where `cpu_count` is the number of entries in the system CPU set information. For each thread:

1. If the thread does not already have an open handle in its [ThreadStats](../scheduler.rs/ThreadStats.md), a [ThreadHandle](../winapi.rs/ThreadHandle.md) is obtained via [get_thread_handle](../winapi.rs/get_thread_handle.md). The handle is stored persistently in `thread_stats.handle` for reuse across polling cycles.
2. The best available read handle (`r_handle`, falling back to `r_limited_handle`) is selected.
3. If the thread's `start_address` is `0`, it is resolved via [get_thread_start_address](../winapi.rs/get_thread_start_address.md). The start address is later used by [apply_prime_threads_promote](apply_prime_threads_promote.md) and [apply_ideal_processors](apply_ideal_processors.md) to resolve the thread's start module name for prefix matching.
4. `QueryThreadCycleTime` is called to read the thread's cumulative cycle count. On success, the value is written to `cached_cycles`. On failure, the error is routed through [log_error_if_new](log_error_if_new.md) with `Operation::QueryThreadCycleTime`.

**Phase 3 — Compute cycle deltas and update active streaks.**
After all handles have been queried, the function builds a `Vec<(tid, delta_cycles)>` from every thread in the scheduler's stats for this pid that has a non-zero `cached_cycles`, computing `delta_cycles = cached_cycles - last_cycles`. Threads with zero `cached_cycles` (those that were not queried or whose query failed) have their `active_streak` reset to `0`. The delta vector is passed to `prime_scheduler.update_active_streaks(pid, &tid_with_delta_cycles)`, which increments or resets the per-thread streak counter used by the [hysteresis selection algorithm](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm).

### Thread handle caching

Unlike [reset_thread_ideal_processors](reset_thread_ideal_processors.md), which opens and closes thread handles within a single call, `prefetch_all_thread_cycles` stores opened handles in `thread_stats.handle` for reuse across polling cycles. This avoids the overhead of repeatedly opening and closing handles for threads that remain active. Stale handles for threads that no longer exist are cleaned up by [apply_prime_threads](apply_prime_threads.md) after the promote/demote pipeline completes.

### Counter limit

The iteration limit `min(cpu_count * 2, thread_count) - 1` ensures the function queries a reasonable number of threads proportional to the system's core count. On a 16-core system, this means up to 31 threads are queried (the most CPU-active ones). This bounds the cost of cycle-time queries for processes with hundreds or thousands of threads while still covering more candidates than can be promoted to prime status.

### Relationship to apply_prime_threads

`prefetch_all_thread_cycles` must be called *before* [apply_prime_threads](apply_prime_threads.md) in each polling cycle. The prefetch populates `cached_cycles` and `cached_total_time` in the scheduler; `apply_prime_threads` reads these cached values to compute deltas and drive the selection algorithm. After both complete, [update_thread_stats](update_thread_stats.md) commits the cached values to `last_cycles` and `last_total_time` for the next iteration.

### Active streak tracking

The active streak counter in [ThreadStats](../scheduler.rs/ThreadStats.md) tracks how many consecutive polling cycles a thread has shown non-zero cycle activity. The [hysteresis selection algorithm](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm) uses this to prevent ephemeral CPU bursts from immediately promoting a thread to prime status — a thread must sustain activity for at least `min_active_streak` consecutive cycles (configured in [ConfigConstants](../config.rs/ConfigConstants.md)) before it becomes eligible for promotion.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| Callees | [get_thread_handle](../winapi.rs/get_thread_handle.md), [get_thread_start_address](../winapi.rs/get_thread_start_address.md), [get_cpu_set_information](../winapi.rs/get_cpu_set_information.md), [log_error_if_new](log_error_if_new.md), [PrimeThreadScheduler::get_thread_stats](../scheduler.rs/PrimeThreadScheduler.md), [PrimeThreadScheduler::update_active_streaks](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | [`QueryThreadCycleTime`](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) |
| Privileges | `THREAD_QUERY_LIMITED_INFORMATION` (for `QueryThreadCycleTime` and `NtQueryInformationThread`). `SeDebugPrivilege` is used for cross-process thread access. |

## See Also

| Topic | Link |
|-------|------|
| Prime thread orchestration (consumes prefetched data) | [apply_prime_threads](apply_prime_threads.md) |
| Commit cached counters after a cycle | [update_thread_stats](update_thread_stats.md) |
| Hysteresis-based thread selection | [apply_prime_threads_select](apply_prime_threads_select.md) |
| Scheduler state and streak tracking | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Per-thread stats model | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Thread handle wrapper | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| apply module overview | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd