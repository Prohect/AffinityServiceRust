# ThreadStats struct (scheduler.rs)

Per-thread statistics and state used by [PrimeThreadScheduler](PrimeThreadScheduler.md) to track CPU cycle consumption, handle caching, priority boosting, and ideal processor assignment across loop iterations.

## Syntax

```rust
pub struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}
```

## Members

`last_total_time`

The thread's total execution time (kernel + user, in 100ns units) as of the most recent completed iteration. Used together with `cached_total_time` to compute the per-iteration time delta.

`cached_total_time`

The thread's total execution time sampled during the *current* iteration, before it is committed to `last_total_time` at the end of the cycle by [`update_thread_stats`](../apply.rs/update_thread_stats.md).

`last_cycles`

The thread's cumulative CPU cycle count as of the most recent completed iteration. Compared against `cached_cycles` to derive the per-iteration cycle delta that drives prime thread selection.

`cached_cycles`

The thread's cumulative CPU cycle count sampled during the *current* iteration via `QueryThreadCycleTime`. Committed to `last_cycles` at the end of the cycle.

`handle`

An optional cached [ThreadHandle](../winapi.rs/ThreadHandle.md) containing read and write HANDLEs to the thread. `None` means the handle has not been opened yet. When present, `r_limited_handle` is always valid; other handles should be checked with `is_valid_handle()` before use. The `ThreadHandle` implements `Drop` to automatically close all contained HANDLEs when the `ThreadStats` entry is removed (e.g., when a process exits).

`pinned_cpu_set_ids`

The list of CPU Set IDs currently assigned to this thread via `SetThreadSelectedCpuSets`. Used by the promote/demote logic to track which threads are currently pinned to prime cores and to restore threads to their original CPU sets upon demotion.

`active_streak`

A hysteresis counter tracking how many consecutive iterations this thread has exceeded the entry threshold. Incremented by [`update_active_streaks`](PrimeThreadScheduler.md) each cycle and capped at 254. Reset to 0 when the thread's cycle delta falls below the keep threshold. A thread must reach at least `min_active_streak` (default: 2) before it can be promoted to prime status, preventing briefly-active threads from thrashing the scheduler.

`start_address`

The thread's start address obtained via `NtQueryInformationThread(ThreadQuerySetWin32StartAddress)`. Used to resolve the thread's originating module via [`resolve_address_to_module`](../winapi.rs/resolve_address_to_module.md) for logging and for ideal processor prefix matching in [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md).

`original_priority`

The thread's [ThreadPriority](../priority.rs/ThreadPriority.md) before any prime-thread priority boost was applied. Stored on first promotion so that the thread's priority can be restored when it is demoted. `None` indicates no priority override has been applied.

`last_system_thread_info`

A cached copy of the `SYSTEM_THREAD_INFORMATION` structure from the most recent [ProcessSnapshot](../process.rs/ProcessSnapshot.md). Contains kernel time, user time, create time, wait time, context switch count, priority, base priority, thread state, and wait reason. Used for detailed logging when a process exits (see [`close_dead_process_handles`](PrimeThreadScheduler.md)).

`ideal_processor`

An [IdealProcessorState](IdealProcessorState.md) tracking the thread's current and previous ideal processor assignment. Used by [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) to detect changes and avoid redundant `SetThreadIdealProcessorEx` calls.

`process_id`

The PID of the owning process. Stored per-thread so that the `Debug` implementation can resolve `start_address` to a module name via [`resolve_address_to_module`](../winapi.rs/resolve_address_to_module.md) without requiring external context.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **new** | `pub fn new(process_id: u32) -> Self` | Creates a new `ThreadStats` with all counters zeroed, no handle, empty CPU set list, and the given `process_id`. |

## Traits

| Trait | Description |
| --- | --- |
| `Debug` | Custom implementation that formats `start_address` as a resolved module name (via `resolve_address_to_module`) instead of a raw hex value. |
| `Default` | Delegates to `Self::new(0)`. |

## Remarks

`ThreadStats` is the lowest-level tracking unit in the scheduler hierarchy. Each thread that has been observed at least once (via [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md)) gets an entry in its parent [ProcessStats](ProcessStats.md)'s `tid_to_thread_stats` map.

### Cycle delta calculation

The two-phase commit pattern (`cached_*` / `last_*`) ensures that all thread measurements within a single iteration use a consistent baseline. The flow is:

1. **Prefetch phase** — `cached_cycles` and `cached_total_time` are updated with fresh values from the OS.
2. **Selection phase** — Delta is computed as `cached_cycles - last_cycles`.
3. **Commit phase** — [`update_thread_stats`](../apply.rs/update_thread_stats.md) copies `cached_*` → `last_*`.

This design avoids self-referential timing issues where a thread's own measurement overhead could skew its delta.

### Handle lifetime

The `handle` field owns the `ThreadHandle`, and its `Drop` implementation closes all contained Windows HANDLEs. When [`close_dead_process_handles`](PrimeThreadScheduler.md) removes dead process entries from the scheduler, all associated `ThreadStats` are dropped, which automatically closes every cached thread handle. This ensures no handle leaks even when processes exit unexpectedly.

### Active streak hysteresis

The `active_streak` counter implements the "minimum activity duration" component of the scheduler's hysteresis system:

| Streak value | Meaning |
| --- | --- |
| 0 | Thread is inactive or fell below keep threshold |
| 1 | Thread just exceeded entry threshold this iteration |
| ≥ `min_active_streak` | Thread is eligible for promotion to prime status |
| 254 | Counter cap (prevents overflow) |

This works in conjunction with the entry/keep threshold split in [`select_top_threads_with_hysteresis`](PrimeThreadScheduler.md) to provide stable prime thread assignment.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/scheduler.rs |
| **Source lines** | L236–L261 |
| **Parent container** | [ProcessStats](ProcessStats.md)`.tid_to_thread_stats` |
| **Key dependencies** | [ThreadHandle](../winapi.rs/ThreadHandle.md), [ThreadPriority](../priority.rs/ThreadPriority.md), [IdealProcessorState](IdealProcessorState.md) |

## See also

- [PrimeThreadScheduler](PrimeThreadScheduler.md)
- [ProcessStats](ProcessStats.md)
- [IdealProcessorState](IdealProcessorState.md)
- [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)
- [apply_prime_threads](../apply.rs/apply_prime_threads.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)