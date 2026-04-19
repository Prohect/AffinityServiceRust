# ProcessStats struct (scheduler.rs)

Per-process statistics container used by [PrimeThreadScheduler](PrimeThreadScheduler.md) to track thread-level scheduling state, liveness, and debug configuration for a single process. Each entry in the scheduler's `pid_to_process_stats` map is a `ProcessStats` instance.

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
| `alive` | `bool` | Liveness flag for the current polling iteration. Set to `false` by [PrimeThreadScheduler::reset_alive](PrimeThreadScheduler.md) at the start of each loop, then set back to `true` by [PrimeThreadScheduler::set_alive](PrimeThreadScheduler.md) if the process is still present in the snapshot. Processes that remain `false` after the snapshot scan are cleaned up by [drop_process_by_pid](PrimeThreadScheduler.md). |
| `tid_to_thread_stats` | `HashMap<u32, ThreadStats>` | Map of thread ID → [ThreadStats](ThreadStats.md) for every thread that has been observed in this process. Entries are lazily created by [PrimeThreadScheduler::get_thread_stats](PrimeThreadScheduler.md). Thread handles and scheduling state are stored per-thread within this map. |
| `track_top_x_threads` | `i32` | Number of top threads (by CPU cycles) to include in the exit report when the process terminates. A value of `0` disables the report. Set by [PrimeThreadScheduler::set_tracking_info](PrimeThreadScheduler.md) from the per-process configuration. Negative values are accepted; the absolute value is used when generating the report. |
| `process_name` | `String` | Cached display name for the process (lowercase), used in log messages and the exit thread report. Set by [PrimeThreadScheduler::set_tracking_info](PrimeThreadScheduler.md). |
| `process_id` | `u32` | The PID this stats entry belongs to. Set at construction time and currently marked `#[allow(dead_code)]`. |

## Methods

### new

```rust
pub fn new(process_id: u32) -> Self
```

Creates a new `ProcessStats` with the given PID. All fields are initialized to their default/empty values, and `alive` is set to `true`.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `process_id` | `u32` | The process identifier this stats entry tracks. |

**Return value**

`ProcessStats` — A new instance with an empty thread stats map, `track_top_x_threads` of `0`, an empty `process_name`, and `alive` set to `true`.

### Default

```rust
impl Default for ProcessStats {
    fn default() -> Self;
}
```

Delegates to `ProcessStats::new(0)`. Provides a zero-PID default primarily for HashMap entry API ergonomics.

## Remarks

### Lifecycle

A `ProcessStats` entry is created the first time a process PID is encountered in any of the scheduler methods (`set_alive`, `set_tracking_info`, `get_thread_stats`, or `update_active_streaks`), via `HashMap::entry().or_insert()`. It persists across polling iterations until the process is no longer alive and [drop_process_by_pid](PrimeThreadScheduler.md) is called.

### Thread stats growth

The `tid_to_thread_stats` map grows monotonically during a process's lifetime — threads are added as they are observed but are never individually removed. The entire map is dropped when the process exits. This matches the Windows threading model where thread IDs are not reused within a single process lifetime.

### Exit report

When `track_top_x_threads != 0` and the process exits, [drop_process_by_pid](PrimeThreadScheduler.md) generates a log report listing the top N threads by `last_cycles`. The report includes each thread's TID, cycle count, start address (resolved to a module name via [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md)), kernel time, user time, create time, priority, context switches, and other fields from `SYSTEM_THREAD_INFORMATION`.

## Requirements

| | |
|---|---|
| **Module** | `src/scheduler.rs` |
| **Callers** | [PrimeThreadScheduler](PrimeThreadScheduler.md) (all methods) |
| **Dependencies** | [ThreadStats](ThreadStats.md), `HashMap` from `crate::collections` |
| **Privileges** | None (data structure only) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [scheduler.rs](README.md) |
| Parent scheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| Per-thread state | [ThreadStats](ThreadStats.md) |
| Process snapshot entry | [ProcessEntry](../process.rs/ProcessEntry.md) |
| Module name resolution | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*