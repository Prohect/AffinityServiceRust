# ThreadStats struct (scheduler.rs)

Per-thread statistics and state used by [PrimeThreadScheduler](PrimeThreadScheduler.md) to track CPU cycle deltas, active streaks, thread handles, CPU set pins, ideal processor assignments, and original priority for prime thread scheduling with hysteresis.

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

| Member | Type | Description |
|--------|------|-------------|
| `last_total_time` | `i64` | The most recently observed total time (kernel + user) for this thread, in 100-nanosecond intervals. Updated each scheduling pass from `SYSTEM_THREAD_INFORMATION`. Used with `cached_total_time` to compute deltas. |
| `cached_total_time` | `i64` | The total time value from the previous scheduling pass. The delta (`last_total_time - cached_total_time`) represents CPU time consumed during the last interval. |
| `last_cycles` | `u64` | The most recently observed CPU cycle count for this thread, obtained via `QueryThreadCycleTime`. Used with `cached_cycles` to compute per-interval cycle deltas for ranking threads. |
| `cached_cycles` | `u64` | The CPU cycle count from the previous scheduling pass. The delta (`last_cycles - cached_cycles`) is the primary metric used by [select_top_threads_with_hysteresis](PrimeThreadScheduler.md) to rank threads for prime core promotion. |
| `handle` | `Option<`[ThreadHandle](../winapi.rs/ThreadHandle.md)`)>` | Thread handle container. `None` means the handle has not been opened yet. When `Some`, the `r_limited_handle` is always valid. Other handles (`r_handle`, `w_limited_handle`, `w_handle`) should be checked with `is_valid_handle()` before use. Handles are closed automatically when `ThreadStats` is dropped (via `ThreadHandle`'s `Drop` implementation). |
| `pinned_cpu_set_ids` | `Vec<u32>` | The CPU set IDs currently assigned to this thread via `SetThreadSelectedCpuSets`. An empty vector means the thread inherits its process-level default CPU set. When a thread is promoted to prime status, it is pinned to performance-core CPU set IDs; when demoted, this vector is cleared to restore default behavior. |
| `active_streak` | `u8` | The number of consecutive scheduling intervals in which this thread exceeded the entry threshold. Must reach [ConfigConstants](../config.rs/ConfigConstants.md)`.min_active_streak` before the thread is eligible for prime promotion. Reset to `0` when cycles drop below the keep threshold. Capped at `254`. |
| `start_address` | `usize` | The start address of the thread's entry point, obtained via `NtQueryInformationThread(ThreadQuerySetWin32StartAddress)`. Used for diagnostic logging and module name resolution via [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md). |
| `original_priority` | `Option<`[ThreadPriority](../priority.rs/ThreadPriority.md)`)>` | The thread's priority before promotion. Captured when a thread is first promoted to a prime core so it can be restored upon demotion. `None` means the thread has never been promoted or has been fully demoted. |
| `last_system_thread_info` | `Option<SYSTEM_THREAD_INFORMATION>` | Cached copy of the last `SYSTEM_THREAD_INFORMATION` from [ProcessSnapshot](../process.rs/ProcessSnapshot.md). Used by [drop_process_by_pid](PrimeThreadScheduler.md) to produce detailed exit reports including kernel time, user time, create time, context switches, priority, thread state, and wait reason. |
| `ideal_processor` | [IdealProcessorState](IdealProcessorState.md) | Tracks the current and previous ideal processor assignment for this thread. Used by `apply_ideal_processors` to detect changes and avoid redundant `SetThreadIdealProcessorEx` calls. |
| `process_id` | `u32` | The PID of the process that owns this thread. Stored for use in debug formatting (module name resolution) and passed through to `resolve_address_to_module`. |

## Remarks

### Thread lifecycle

A `ThreadStats` instance is created on first access via [PrimeThreadScheduler::get_thread_stats](PrimeThreadScheduler.md) with all numeric fields initialized to zero, empty vectors, and `None` optionals. The owning `PrimeThreadScheduler` populates fields incrementally:

1. **Cycle prefetch** — `prefetch_all_thread_cycles` in `apply.rs` opens the thread handle (populating `handle`), queries `QueryThreadCycleTime` (updating `last_cycles` / `cached_cycles`), and records `start_address` and `last_system_thread_info`.
2. **Streak update** — [update_active_streaks](PrimeThreadScheduler.md) increments or resets `active_streak` based on cycle deltas relative to thresholds.
3. **Selection** — [select_top_threads_with_hysteresis](PrimeThreadScheduler.md) reads `active_streak` and checks current assignment status to decide prime promotion.
4. **Promotion** — `apply_prime_threads_promote` pins the thread to performance cores (populating `pinned_cpu_set_ids`), saves `original_priority`, and optionally boosts thread priority.
5. **Demotion** — `apply_prime_threads_demote` clears `pinned_cpu_set_ids`, restores `original_priority`, and resets CPU set assignments.
6. **Cleanup** — When the owning process exits, [drop_process_by_pid](PrimeThreadScheduler.md) logs final statistics, drops all `ThreadHandle`s (closing OS handles), and removes the process entry.

### Cycle delta computation

The two-field pattern (`last_*` / `cached_*`) implements a simple double-buffering scheme. At the start of each scheduling pass, `cached_cycles` is set to the previous `last_cycles`, then `last_cycles` is updated with the current value from `QueryThreadCycleTime`. The delta (`last_cycles - cached_cycles`) represents cycles consumed during one scheduling interval and is the primary sort key for thread ranking.

### Debug formatting

`ThreadStats` implements a custom `fmt::Debug` that resolves `start_address` to a module name via `resolve_address_to_module(process_id, start_address)` for human-readable output. The `handle` field is excluded from debug output.

### Thread safety

`ThreadStats` is not `Send` or `Sync` by default because it contains `Option<ThreadHandle>` (which wraps raw `HANDLE` values). All access is mediated through the `PrimeThreadScheduler`, which is only accessed on the main service loop thread.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Callers | [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md), [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [update_thread_stats](../apply.rs/update_thread_stats.md) |
| Dependencies | [ThreadHandle](../winapi.rs/ThreadHandle.md), [ThreadPriority](../priority.rs/ThreadPriority.md), [IdealProcessorState](IdealProcessorState.md), `SYSTEM_THREAD_INFORMATION` (ntapi) |
| API | `QueryThreadCycleTime`, `SetThreadSelectedCpuSets`, `SetThreadIdealProcessorEx` |
| Privileges | `SeDebugPrivilege` (for opening thread handles across security boundaries) |

## See Also

| Topic | Link |
|-------|------|
| Scheduler overview | [scheduler.rs overview](README.md) |
| Ideal processor state | [IdealProcessorState](IdealProcessorState.md) |
| Process-level statistics | [ProcessStats](ProcessStats.md) |
| Prime thread scheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| Thread handle wrapper | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Thread priority enum | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Hysteresis constants | [ConfigConstants](../config.rs/ConfigConstants.md) |