# ThreadStats type (scheduler.rs)

Per-thread statistics and state container used by the `PrimeThreadScheduler` to track CPU cycle activity, manage thread handles, record ideal-processor assignments, and support hysteresis-based prime-thread selection across polling iterations. Each `ThreadStats` instance corresponds to a single OS thread identified by its TID within a parent process.

## Syntax

```rust
pub struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: List<[u32; CONSUMER_CPUS]>,
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
| `last_total_time` | `i64` | The most recently observed total (kernel + user) execution time for this thread, in 100-nanosecond intervals. Updated each polling iteration. |
| `cached_total_time` | `i64` | The `last_total_time` value from the previous polling iteration. Used to compute per-iteration delta time. |
| `last_cycles` | `u64` | The most recently observed CPU cycle count for this thread, as returned by `QueryThreadCycleTime` or equivalent. |
| `cached_cycles` | `u64` | The `last_cycles` value from the previous polling iteration. The difference `last_cycles - cached_cycles` gives the per-iteration delta used for prime-thread ranking. |
| `handle` | `Option<ThreadHandle>` | Optional thread handle container. `None` means the handle has not been opened yet. When `Some`, the `r_limited_handle` within is always valid; callers should check `is_valid_handle()` before using elevated handles. The `ThreadHandle`'s `Drop` implementation closes the OS handles automatically. |
| `pinned_cpu_set_ids` | `List<[u32; CONSUMER_CPUS]>` | Stack-allocated list of CPU set IDs that this thread has been pinned to by the prime-thread engine. Empty when the thread is not currently assigned to any prime CPU set. The capacity is bounded by the `CONSUMER_CPUS` constant. |
| `active_streak` | `u8` | Counter for consecutive polling iterations during which this thread's cycle delta exceeded the entry threshold. Used by `PrimeThreadScheduler::update_active_streaks` to implement hysteresis—threads must sustain activity for `min_active_streak` iterations before being promoted to prime status. Capped at 254 to prevent overflow. Reset to 0 when cycles drop below the keep threshold. |
| `start_address` | `usize` | The thread's start address (entry point), used to resolve the originating module name via `resolve_address_to_module`. Useful for prefix-based thread filtering (e.g., only promote threads starting in `"game.dll"`). |
| `original_priority` | `Option<ThreadPriority>` | Snapshot of the thread's priority level at the time it was first observed. Stored so that the service can restore the original priority when a thread loses prime status or the process exits. `None` if not yet captured. |
| `last_system_thread_info` | `Option<SYSTEM_THREAD_INFORMATION>` | The most recent `SYSTEM_THREAD_INFORMATION` snapshot from `NtQuerySystemInformation` for this thread. Used for diagnostic reporting when a process exits and `track_top_x_threads` is nonzero. Contains kernel time, user time, create time, wait reason, context switches, and scheduling priority. |
| `ideal_processor` | `IdealProcessorState` | Tracks the current and previous ideal-processor assignment for this thread. See [IdealProcessorState](IdealProcessorState.md). |
| `process_id` | `u32` | The Windows PID of the parent process. Stored here so that the custom `Debug` implementation can call `resolve_address_to_module` without requiring external context. |

## Remarks

- `ThreadStats` implements a custom `fmt::Debug` trait that resolves `start_address` to a module name using `resolve_address_to_module(process_id, start_address)`, making debug output human-readable. The `handle` and `last_system_thread_info` fields are intentionally omitted from the debug output for brevity.
- The `new(process_id)` constructor initializes all numeric fields to zero, all `Option` fields to `None`, `pinned_cpu_set_ids` to an empty list, and `ideal_processor` to a default `IdealProcessorState`.
- The `Default` implementation calls `Self::new(0)`, which is suitable for placeholder contexts but means the `process_id` must be explicitly set or the entry must be created through `PrimeThreadScheduler::get_thread_stats`.
- The delta between `last_cycles` and `cached_cycles` is the primary metric used by `PrimeThreadScheduler::select_top_threads_with_hysteresis` to rank threads and decide which ones receive prime CPU assignments.
- Thread handles are opened lazily. The `handle` field remains `None` until the apply engine first needs to call a Win32 API on the thread (e.g., `SetThreadAffinityMask`, `SetThreadIdealProcessorEx`, `SetThreadPriority`). Once opened, the handle is reused for the lifetime of the `ThreadStats` entry and closed automatically when the entry is dropped.
- The `CONSUMER_CPUS` constant bounds the maximum number of CPU set IDs that can be stored per thread without heap allocation, reflecting typical consumer hardware core counts.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Callers | [PrimeThreadScheduler](PrimeThreadScheduler.md), `apply` module (thread-level apply functions) |
| Dependencies | [ThreadPriority](../priority.rs/ThreadPriority.md), [IdealProcessorState](IdealProcessorState.md), `winapi::ThreadHandle`, `winapi::resolve_address_to_module`, `collections::List`, `collections::CONSUMER_CPUS`, `ntapi::ntexapi::SYSTEM_THREAD_INFORMATION` |
| Platform | Windows (depends on NT thread information structures and Win32 thread handles) |

## See Also

| Reference | Link |
|-----------|------|
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ProcessStats | [ProcessStats](ProcessStats.md) |
| IdealProcessorState | [IdealProcessorState](IdealProcessorState.md) |
| ThreadPriority | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| format_100ns | [format_100ns](format_100ns.md) |
| format_filetime | [format_filetime](format_filetime.md) |
| scheduler module | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
