# PrimeThreadScheduler struct (scheduler.rs)

The central scheduler for prime thread management. Tracks per-process, per-thread statistics and implements hysteresis-based selection to promote the highest-activity threads onto performance cores while preventing promotion/demotion thrashing.

## Syntax

```rust
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `pid_to_process_stats` | `HashMap<u32, ProcessStats>` | Maps process IDs to their per-process statistics. Each entry contains thread-level stats, tracking configuration, and liveness state. |
| `constants` | [ConfigConstants](../config.rs/ConfigConstants.md) | Hysteresis tuning parameters: `min_active_streak`, `keep_threshold`, and `entry_threshold`. Loaded from the configuration file and hot-reloaded when the config changes. |

## Methods

| Method | Description |
|--------|-------------|
| [new](#new) | Creates a new scheduler with the given constants. |
| [reset_alive](#reset_alive) | Marks all tracked processes as not alive before a new scan pass. |
| [set_alive](#set_alive) | Marks a process as alive during the current scan pass. |
| [set_tracking_info](#set_tracking_info) | Sets the tracking depth and display name for a process. |
| [get_thread_stats](#get_thread_stats) | Returns a mutable reference to a thread's stats, inserting defaults if absent. |
| [update_active_streaks](#update_active_streaks) | Updates active streak counters for hysteresis-based thread selection. |
| [select_top_threads_with_hysteresis](#select_top_threads_with_hysteresis) | Selects the top N threads for prime core promotion using two-pass hysteresis. |
| [drop_process_by_pid](#drop_process_by_pid) | Removes a process from the scheduler, closing handles and optionally logging a report. |

---

### new

Creates a new `PrimeThreadScheduler` initialized with the given hysteresis constants.

```rust
pub fn new(constants: ConfigConstants) -> Self
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `constants` | [ConfigConstants](../config.rs/ConfigConstants.md) | Hysteresis tuning parameters that control thread promotion and demotion behavior. |

#### Return value

A new `PrimeThreadScheduler` with an empty `pid_to_process_stats` map.

---

### reset_alive

Marks every tracked process as not alive. Called at the beginning of each scan loop before re-enumerating running processes.

```rust
pub fn reset_alive(&mut self)
```

#### Remarks

After `reset_alive`, the caller walks the process snapshot and calls [set_alive](#set_alive) for each matched process. Any process that remains not alive after the walk is a candidate for cleanup via [drop_process_by_pid](#drop_process_by_pid).

---

### set_alive

Marks a specific process as alive in the current scan pass. If the process is not yet tracked, a new [ProcessStats](ProcessStats.md) entry is inserted.

```rust
pub fn set_alive(&mut self, pid: u32)
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The Windows process ID. |

---

### set_tracking_info

Sets the thread-tracking depth and display name for a process. Called once per config-matched process during each scan pass.

```rust
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The Windows process ID. |
| `track_top_x_threads` | `i32` | Number of top threads to include in the exit report. A value of `0` disables reporting. Negative values are allowed (the absolute value is used for the report count). |
| `process_name` | `String` | Human-readable process image name, used in log output. |

#### Remarks

If the process is not yet tracked, a new [ProcessStats](ProcessStats.md) entry is inserted before setting the values.

---

### get_thread_stats

Returns a mutable reference to the [ThreadStats](ThreadStats.md) for a specific thread, creating both the process and thread entries if they do not exist.

```rust
#[inline]
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The Windows process ID. |
| `tid` | `u32` | The Windows thread ID. |

#### Return value

A mutable reference to the thread's [ThreadStats](ThreadStats.md). Callers use this to read or update cycle counts, handles, CPU set pins, ideal processor state, and active streak.

---

### update_active_streaks

Updates active streak counters for all threads in a process based on their delta cycle counts relative to the cycle-count leader.

```rust
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The Windows process ID. |
| `tid_with_delta_cycles` | `&[(u32, u64)]` | Slice of `(thread_id, delta_cycles)` pairs. `delta_cycles` is the difference in `QueryThreadCycleTime` values between the current and previous scan pass. |

#### Remarks

The algorithm computes the maximum delta cycle count across all threads, then derives two thresholds:

- **Entry threshold** — `max_cycles * constants.entry_threshold` (default 0.42). A thread must exceed this to *begin* accumulating an active streak.
- **Keep threshold** — `max_cycles * constants.keep_threshold` (default 0.69). A thread with an existing streak that falls below this has its streak reset to zero.

For each thread:

1. If the thread already has a streak (`> 0`):
   - If `delta < keep_min`, the streak is reset to `0`.
   - Otherwise, the streak is incremented (capped at `254`).
2. If the thread has no streak and `delta >= entry_min`, the streak is set to `1`.

This mechanism prevents briefly-active threads from being promoted to prime status. The streak must reach `constants.min_active_streak` (default `2`) before a thread is eligible for selection in [select_top_threads_with_hysteresis](#select_top_threads_with_hysteresis).

---

### select_top_threads_with_hysteresis

Selects the top threads for prime core promotion using a two-pass hysteresis algorithm that prevents promotion/demotion thrashing.

```rust
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
)
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The Windows process ID. |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | Mutable slice of `(thread_id, delta_cycles, is_prime)` tuples. On entry, `is_prime` should be `false`. On return, selected threads have `is_prime` set to `true`. The slice is sorted in descending order of `delta_cycles` as a side effect. |
| `slot_count` | `usize` | Maximum number of threads to promote (typically the number of configured prime cores). |
| `is_currently_assigned` | `fn(&ThreadStats) -> bool` | Callback that checks whether a thread is already assigned the prime resource (e.g., has a pinned CPU set or ideal processor). This enables the "keep" pass to retain currently-promoted threads. |

#### Return value

None. Results are written back into the `is_prime` field of the `tid_with_delta_cycles` slice.

#### Remarks

The algorithm operates in two passes after sorting by descending delta cycles:

**Pass 1 — Retain (keep threshold):**
Iterates all threads. If a thread is already assigned (per `is_currently_assigned`) and its delta cycles meet or exceed the keep threshold (`max_cycles * constants.keep_threshold`), it retains its prime slot. This prevents a thread from losing prime status due to minor cycle-count fluctuations.

**Pass 2 — Promote (entry threshold):**
Fills remaining slots from highest delta cycles downward. A thread qualifies only if:
- It is not already selected (`is_prime == false`) and its TID is non-zero.
- Its delta cycles meet or exceed the entry threshold (`max_cycles * constants.entry_threshold`).
- Its `active_streak` is at least `constants.min_active_streak`.

The entry threshold is intentionally lower than the keep threshold, creating a **hysteresis band**. A thread must work harder to earn promotion than to maintain it. This eliminates rapid toggling when two threads have similar cycle counts near the boundary.

**Example:**

With `keep_threshold = 0.69` and `entry_threshold = 0.42`, and a max delta of 1,000,000 cycles:
- A currently-promoted thread stays promoted as long as its delta is ≥ 690,000.
- A new thread must sustain ≥ 420,000 cycles for at least `min_active_streak` consecutive passes to earn promotion.

---

### drop_process_by_pid

Removes all scheduler state for a process, including closing cached thread handles and clearing the module address cache. Optionally logs a diagnostic report of the top threads by CPU cycles.

```rust
pub fn drop_process_by_pid(&mut self, pid: &u32)
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `&u32` | Reference to the Windows process ID to remove. |

#### Remarks

If `track_top_x_threads` is non-zero for the process, a report is logged before cleanup. The report includes, for each of the top N threads (sorted by `last_cycles` descending):

- Thread ID, total CPU cycles, and start address resolved to a module name.
- If `last_system_thread_info` is available: kernel time, user time, create time, wait time, client ID, priority, base priority, context switch count, thread state, and wait reason.

Times are formatted using [format_100ns](format_100ns.md) (durations) and [format_filetime](format_filetime.md) (timestamps). Module resolution uses `resolve_address_to_module` from `winapi.rs`.

After logging, `drop_module_cache` is called for the PID, and the process entry is removed from `pid_to_process_stats`. Thread handles are closed automatically via the `Drop` implementation on [ThreadHandle](../winapi.rs/ThreadHandle.md).

## Requirements

| | |
|---|---|
| **Module** | `scheduler.rs` |
| **Callers** | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md), [update_thread_stats](../apply.rs/update_thread_stats.md), [hotreload_config](../config.rs/hotreload_config.md), `main.rs` |
| **Callees** | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md), [drop_module_cache](../winapi.rs/drop_module_cache.md), [log_message](../logging.rs/log_message.md), [format_100ns](format_100ns.md), [format_filetime](format_filetime.md) |
| **API** | `QueryThreadCycleTime` (via callers) |
| **Privileges** | `SeDebugPrivilege` (via callers, for thread handle access) |

## See Also

| Link | Description |
|------|-------------|
| [ProcessStats](ProcessStats.md) | Per-process statistics container held by the scheduler. |
| [ThreadStats](ThreadStats.md) | Per-thread statistics including cycles, handles, and ideal processor state. |
| [IdealProcessorState](IdealProcessorState.md) | Tracks ideal processor assignment state per thread. |
| [ConfigConstants](../config.rs/ConfigConstants.md) | Hysteresis tuning parameters. |
| [apply_prime_threads](../apply.rs/apply_prime_threads.md) | Entry point that drives the prime thread promotion/demotion pipeline. |
| [ProcessSnapshot](../process.rs/ProcessSnapshot.md) | Provides the process/thread enumeration consumed by the scheduler's callers. |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd