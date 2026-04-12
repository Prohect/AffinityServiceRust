# ProcessStats struct (scheduler.rs)

Tracks per-process state for the prime thread scheduler, including liveness, thread statistics, and diagnostic metadata.

## Syntax

```rust
#[derive(Debug)]
pub struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
```

## Members

`alive`

A boolean flag indicating whether this process was observed in the most recent snapshot. Set to `false` by [`PrimeThreadScheduler::reset_alive`](PrimeThreadScheduler.md) at the start of each loop iteration and back to `true` by [`PrimeThreadScheduler::set_alive`](PrimeThreadScheduler.md) when the process is found in the current [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md). Processes that remain `false` after the scan are cleaned up by [`close_dead_process_handles`](PrimeThreadScheduler.md).

`tid_to_thread_stats`

A `HashMap<u32, ThreadStats>` mapping thread IDs to their corresponding [ThreadStats](ThreadStats.md) structs. Entries are lazily created via [`PrimeThreadScheduler::get_thread_stats`](PrimeThreadScheduler.md) the first time a thread is encountered. Thread handles within are automatically closed via `Drop` when the process is removed from the scheduler.

`track_top_x_threads`

Controls the diagnostic logging behavior on process exit. When nonzero, [`close_dead_process_handles`](PrimeThreadScheduler.md) logs the top N threads (by accumulated CPU cycles) along with detailed timing and scheduling stats. The absolute value of this integer determines N. Set from [`ProcessConfig`](../config.rs/ProcessConfig.md) via [`set_tracking_info`](PrimeThreadScheduler.md). A value of `0` disables exit logging for this process.

`process_name`

The lowercase process image name (e.g. `"game.exe"`), set via [`set_tracking_info`](PrimeThreadScheduler.md). Used in log messages during process exit reporting.

`process_id`

The Windows process identifier (PID). Stored for reference but currently unused at runtime (marked `#[allow(dead_code)]`). Set during construction via `ProcessStats::new(pid)`.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **new** | `pub fn new(process_id: u32) -> Self` | Creates a new `ProcessStats` with `alive` set to `true`, empty thread map, `track_top_x_threads` of `0`, and empty `process_name`. |

## Trait Implementations

| Trait | Description |
| --- | --- |
| `Debug` | Derived — prints all fields. |
| `Default` | Delegates to `Self::new(0)`. |

## Remarks

`ProcessStats` is the per-process record inside [`PrimeThreadScheduler::pid_to_process_stats`](PrimeThreadScheduler.md). It has a two-phase lifecycle each loop iteration:

1. **Reset phase** — `reset_alive()` marks all entries as `alive = false`.
2. **Scan phase** — For each process matched by a config rule, `set_alive()` marks the entry as alive and `set_tracking_info()` updates the tracking parameters.

At the end of the iteration, `close_dead_process_handles()` removes entries still marked `alive = false`. Before removal, if `track_top_x_threads != 0`, it generates a detailed report of the top threads sorted by `last_cycles`, including kernel/user time, create time, context switches, thread state, wait reason, and the resolved module name of each thread's start address.

The `tid_to_thread_stats` map grows over the process lifetime as new threads are encountered but never shrinks — thread entries persist even after threads exit, allowing their accumulated cycle data to contribute to exit-time diagnostics. Stale thread handles are cleaned up only when the entire process entry is removed.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/scheduler.rs |
| **Source lines** | L178–L205 |
| **Owned by** | [`PrimeThreadScheduler`](PrimeThreadScheduler.md) via `pid_to_process_stats` |
| **Contains** | [`ThreadStats`](ThreadStats.md) via `tid_to_thread_stats` |

## See also

- [scheduler.rs module overview](README.md)
- [PrimeThreadScheduler](PrimeThreadScheduler.md)
- [ThreadStats](ThreadStats.md)
- [IdealProcessorState](IdealProcessorState.md)
- [ProcessSnapshot](../process.rs/ProcessSnapshot.md)