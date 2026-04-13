# ProcessStats struct (scheduler.rs)

Per-process statistics container that tracks thread-level data for prime thread scheduling. Each `ProcessStats` instance is keyed by PID inside [`PrimeThreadScheduler`](PrimeThreadScheduler.md) and holds the collection of [`ThreadStats`](ThreadStats.md) for every thread observed in that process.

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

| Member | Type | Description |
|--------|------|-------------|
| `alive` | `bool` | Liveness flag used by the scheduler's mark-and-sweep cycle. Set to `false` by [`PrimeThreadScheduler::reset_alive`](PrimeThreadScheduler.md) at the start of each scheduling pass, then set back to `true` by [`PrimeThreadScheduler::set_alive`](PrimeThreadScheduler.md) when the process is still present in the snapshot. Processes that remain `false` after the pass are candidates for cleanup via [`drop_process_by_pid`](PrimeThreadScheduler.md). |
| `tid_to_thread_stats` | `HashMap<u32, ThreadStats>` | Map from thread ID (TID) to per-thread statistics. Entries are lazily created on first access through [`PrimeThreadScheduler::get_thread_stats`](PrimeThreadScheduler.md). Entries persist until the entire `ProcessStats` is dropped. |
| `track_top_x_threads` | `i32` | Controls diagnostic logging when the process exits. When nonzero, [`drop_process_by_pid`](PrimeThreadScheduler.md) logs the top *N* threads (by CPU cycles) for post-mortem analysis. The absolute value determines the count; the sign is reserved for future use. Set via [`PrimeThreadScheduler::set_tracking_info`](PrimeThreadScheduler.md). |
| `process_name` | `String` | Lowercase display name of the process (e.g., `"game.exe"`). Set via [`PrimeThreadScheduler::set_tracking_info`](PrimeThreadScheduler.md) and used in log messages. |
| `process_id` | `u32` | The Windows process identifier. Stored for reference; not actively used after construction. |

## Remarks

### Construction

`ProcessStats` is created through `ProcessStats::new(pid)`, which initializes the struct with `alive: true`, an empty thread stats map, zero `track_top_x_threads`, and an empty process name.

A `Default` implementation is also provided, calling `new(0)`.

### Alive-flag lifecycle

The `alive` field implements a two-phase mark-and-sweep pattern:

1. **Mark phase** — `PrimeThreadScheduler::reset_alive` sets `alive = false` on every tracked process.
2. **Sweep phase** — As processes are found in the current [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md), `PrimeThreadScheduler::set_alive` sets `alive = true`.
3. **Cleanup** — Any process still marked `alive == false` has exited and should be cleaned up with `drop_process_by_pid`, which closes thread handles and frees the module cache.

### Thread stats map growth

The `tid_to_thread_stats` map grows monotonically during a process's lifetime. Threads that exit are **not** individually removed; their entries remain until the entire process is dropped. This avoids the overhead of per-thread cleanup and is acceptable because short-lived threads accumulate negligible memory.

### Tracking info

`track_top_x_threads` and `process_name` are set together via `set_tracking_info` because they originate from the same [`ProcessConfig`](../config.rs/ProcessConfig.md) rule. They are only meaningful when the process eventually exits and the scheduler logs diagnostic output.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Constructed by | [`PrimeThreadScheduler::set_alive`](PrimeThreadScheduler.md), `ProcessStats::new` |
| Consumed by | [`PrimeThreadScheduler`](PrimeThreadScheduler.md) methods |
| Dependencies | [`ThreadStats`](ThreadStats.md) |

## See Also

| Topic | Link |
|-------|------|
| Parent scheduler | [`PrimeThreadScheduler`](PrimeThreadScheduler.md) |
| Per-thread data | [`ThreadStats`](ThreadStats.md) |
| Hysteresis constants | [`ConfigConstants`](../config.rs/ConfigConstants.md) |
| Process snapshot source | [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md) |
| Process configuration | [`ProcessConfig`](../config.rs/ProcessConfig.md) |