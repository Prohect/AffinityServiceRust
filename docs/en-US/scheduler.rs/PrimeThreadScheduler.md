# PrimeThreadScheduler struct (scheduler.rs)

Central scheduler that manages dynamic thread-to-CPU assignment for "prime thread" scheduling. It tracks per-process, per-thread statistics across loop iterations and uses cycle-based hysteresis to decide which threads deserve promotion to fast (prime) CPU cores.

## Syntax

```rust
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## Members

`pid_to_process_stats`

A map from process ID (`u32`) to [ProcessStats](ProcessStats.md) containing per-process thread tracking data. Entries are created lazily when a process is first encountered and removed when the process exits (via [`close_dead_process_handles`](#methods)).

`constants`

A [ConfigConstants](../config.rs/ConfigConstants.md) struct holding the tunable hysteresis parameters: `entry_threshold`, `keep_threshold`, and `min_active_streak`. These are loaded once from the configuration file and remain constant for the lifetime of the scheduler.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **new** | `pub fn new(constants: ConfigConstants) -> Self` | Constructs a new scheduler with the given hysteresis constants and an empty process map. |
| **reset_alive** | `pub fn reset_alive(&mut self)` | Marks all tracked processes as not alive. Called at the start of each loop iteration before scanning. |
| **set_alive** | `pub fn set_alive(&mut self, pid: u32)` | Marks a process as alive for the current iteration. Creates a new [ProcessStats](ProcessStats.md) entry if the PID is not yet tracked. |
| **set_tracking_info** | `pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)` | Sets the top-N thread tracking count and process display name for a tracked process. |
| **get_thread_stats** | `pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats` | Returns a mutable reference to the [ThreadStats](ThreadStats.md) for a specific thread, creating process and thread entries as needed. |
| **update_active_streaks** | `pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])` | Updates active streak counters for all threads of a process based on their delta cycle counts relative to the hysteresis thresholds. |
| **select_top_threads_with_hysteresis** | `pub fn select_top_threads_with_hysteresis(&mut self, pid: u32, tid_with_delta_cycles: &mut [(u32, u64, bool)], slot_count: usize, is_currently_assigned: fn(&ThreadStats) -> bool)` | Selects the top N threads for prime status using a two-pass hysteresis algorithm. |
| **close_dead_process_handles** | `pub fn close_dead_process_handles(&mut self)` | Removes dead processes, closes thread handles via `Drop`, clears module caches, and optionally logs top-N thread reports. |

## Remarks

### Lifecycle

The scheduler is created once in `main()` and persists across all loop iterations. Each iteration follows this sequence:

1. **`reset_alive()`** — marks all processes as dead (pessimistic assumption).
2. **`set_alive(pid)`** — called for each process matched by configuration rules to mark it as still running.
3. **`set_tracking_info()`** — updates tracking metadata from the current configuration.
4. **`get_thread_stats()` / `update_active_streaks()`** — called by [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) to update cycle deltas and streak counters.
5. **`select_top_threads_with_hysteresis()`** — called by [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) to determine which threads get prime status.
6. **`close_dead_process_handles()`** — at end of iteration, removes processes that were not marked alive and cleans up their resources.

### Hysteresis Algorithm

The two-pass selection algorithm in `select_top_threads_with_hysteresis` prevents thrashing (rapid promote/demote oscillation) by using asymmetric thresholds:

| Parameter | Default | Purpose |
| --- | --- | --- |
| `entry_threshold` | 0.42 | Fraction of max cycles a thread must exceed to be **promoted** to prime status. |
| `keep_threshold` | 0.69 | Fraction of max cycles a currently-assigned thread must exceed to **keep** prime status. |
| `min_active_streak` | 2 | Minimum consecutive iterations a thread must be above `entry_threshold` before promotion. |

**Pass 1 — Keep:** Threads that are already assigned to prime CPUs stay assigned if their delta cycles ≥ `keep_threshold × max_cycles`. The higher keep threshold prevents demotion during minor fluctuations.

**Pass 2 — Promote:** Remaining slots are filled by unassigned threads whose delta cycles ≥ `entry_threshold × max_cycles` and whose `active_streak` ≥ `min_active_streak`. The lower entry barrier combined with the streak requirement prevents briefly-active threads from being promoted.

### Active Streak Counting

`update_active_streaks` maintains a per-thread counter:

- If a thread has a nonzero streak and its delta cycles drop below `keep_threshold × max`, the streak resets to 0.
- If a thread has a nonzero streak and still qualifies, the streak increments (capped at 254).
- If a thread has zero streak and delta cycles ≥ `entry_threshold × max`, the streak is set to 1.

### Resource Cleanup

`close_dead_process_handles` uses `HashMap::retain` to remove dead processes. The [ThreadStats](ThreadStats.md) entries contain `Option<ThreadHandle>` fields whose `Drop` implementation automatically closes the underlying Windows handles. Before removal, if `track_top_x_threads` is nonzero, a detailed report of the top N threads by CPU cycles is logged, including kernel/user time, context switches, thread state, and resolved module names.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/scheduler.rs |
| **Source lines** | L13–L176 |
| **Created by** | `main()` in src/main.rs |
| **Used by** | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md), [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Key dependencies** | [ConfigConstants](../config.rs/ConfigConstants.md), [ProcessStats](ProcessStats.md), [ThreadStats](ThreadStats.md), [ThreadHandle](../winapi.rs/ThreadHandle.md) |

## See also

- [scheduler.rs module overview](README.md)
- [ProcessStats](ProcessStats.md)
- [ThreadStats](ThreadStats.md)
- [IdealProcessorState](IdealProcessorState.md)
- [apply_prime_threads](../apply.rs/apply_prime_threads.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)