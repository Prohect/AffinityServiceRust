# ProcessStats type (scheduler.rs)

Holds per-process bookkeeping state used by the [`PrimeThreadScheduler`](PrimeThreadScheduler.md) to manage thread-level scheduling decisions. Each `ProcessStats` instance tracks whether the process is still alive, maintains a map of per-thread statistics, stores the process name and ID, and records how many top threads should be logged on process exit for diagnostic purposes.

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

| Field | Type | Description |
|-------|------|-------------|
| `alive` | `bool` | Liveness flag. Set to `true` at the start of each polling iteration when the process is observed in the snapshot. Reset to `false` by [`PrimeThreadScheduler::reset_alive`](PrimeThreadScheduler.md) before each scan. Processes that remain `false` after the scan are considered dead and eligible for cleanup via [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md). |
| `tid_to_thread_stats` | `HashMap<u32, ThreadStats>` | A map from Windows thread ID (`u32`) to the corresponding [`ThreadStats`](ThreadStats.md) instance. Entries are lazily created by `PrimeThreadScheduler::get_thread_stats` when a thread is first observed. Entries are dropped when the owning process is cleaned up. |
| `track_top_x_threads` | `i32` | Controls diagnostic logging on process exit. When nonzero, [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) prints the top `abs(track_top_x_threads)` threads sorted by last-observed CPU cycle count, including kernel/user time, start address, module name, and `SYSTEM_THREAD_INFORMATION` details. A value of `0` disables exit logging. Set by `PrimeThreadScheduler::set_tracking_info`. |
| `process_name` | `String` | The lowercase executable name of the process (e.g. `"game.exe"`). Populated by `PrimeThreadScheduler::set_tracking_info` and used in log output. Defaults to an empty string on construction. |
| `process_id` | `u32` | The Windows process ID (PID). Stored for identification in log messages. Marked `#[allow(dead_code)]` in the source because it is currently used only for construction and debugging. |

## Remarks

- `ProcessStats` instances are **not created directly** by callers. They are created on demand by `PrimeThreadScheduler` methods such as `set_alive`, `set_tracking_info`, and `get_thread_stats`, all of which use `HashMap::entry().or_insert(ProcessStats::new(pid))`.
- The `new(process_id)` constructor initializes `alive` to `true`, `track_top_x_threads` to `0`, `process_name` to an empty string, and `tid_to_thread_stats` to an empty map.
- A `Default` implementation is provided that delegates to `Self::new(0)`, producing a stats entry with PID 0. This is primarily for trait compliance and should not be used in production paths.
- The `HashMap` type used for `tid_to_thread_stats` is the project's custom stack-backed `HashMap` from the `collections` module, not `std::collections::HashMap`.
- When `PrimeThreadScheduler::drop_process_by_pid` runs, it first uses the thread stats to produce an optional diagnostic report, then removes the entire `ProcessStats` entry. The `ThreadHandle` instances inside each `ThreadStats` are closed via their `Drop` implementation.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Callers | [`PrimeThreadScheduler`](PrimeThreadScheduler.md) (all methods that access `pid_to_process_stats`) |
| Callees | [`ThreadStats::new`](ThreadStats.md) (via map insertion) |
| API | None directly; thread handle management is delegated to [`ThreadStats`](ThreadStats.md) |
| Privileges | None |

## See Also

| Reference | Link |
|-----------|------|
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| IdealProcessorState | [IdealProcessorState](IdealProcessorState.md) |
| scheduler module overview | [README](README.md) |
| main module | [main.rs README](../main.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
