# PrimeThreadScheduler type (scheduler.rs)

The `PrimeThreadScheduler` is the top-level scheduling engine that owns per-process statistics and orchestrates hysteresis-based "prime thread" selection. It maintains a map from process IDs to [`ProcessStats`](ProcessStats.md) entries, tracks thread liveness across polling iterations, computes active-streak counters for stable thread promotion, and performs two-pass thread selection that resists promotion/demotion thrashing. It also handles process cleanup on exit, including thread-handle closure, module-cache purging, and optional diagnostic reporting of top threads by CPU cycles.

## Syntax

```AffinityServiceRust/src/scheduler.rs#L15-20
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `pid_to_process_stats` | `HashMap<u32, ProcessStats>` | Map from Windows process IDs (PIDs) to their corresponding [`ProcessStats`](ProcessStats.md) structs. Entries are created lazily via `set_alive` or `get_thread_stats` and removed by `drop_process_by_pid` when a process exits. |
| `constants` | `ConfigConstants` | Scheduler tuning constants loaded from the configuration file, including `entry_threshold`, `keep_threshold`, and `min_active_streak`. These control the hysteresis behavior of prime-thread selection. |

## Methods

### `new`

```AffinityServiceRust/src/scheduler.rs#L22-27
pub fn new(constants: ConfigConstants) -> Self
```

Constructs a new `PrimeThreadScheduler` with an empty process map and the given configuration constants.

### `reset_alive`

```AffinityServiceRust/src/scheduler.rs#L29-31
pub fn reset_alive(&mut self)
```

Sets the `alive` flag to `false` on every [`ProcessStats`](ProcessStats.md) entry. Called at the start of each polling iteration; processes that are subsequently seen in the snapshot will be re-marked alive by `set_alive`.

### `set_alive`

```AffinityServiceRust/src/scheduler.rs#L33-35
pub fn set_alive(&mut self, pid: u32)
```

Marks the process with the given PID as alive. If no `ProcessStats` entry exists for the PID, a new default entry is created and inserted.

### `set_tracking_info`

```AffinityServiceRust/src/scheduler.rs#L37-41
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

Sets diagnostic tracking metadata on the process's `ProcessStats` entry. `track_top_x_threads` controls how many threads are logged on process exit (0 = disabled, positive = top N, negative = top N with full `SYSTEM_THREAD_INFORMATION` dump). `process_name` is stored for use in log output.

### `get_thread_stats`

```AffinityServiceRust/src/scheduler.rs#L44-50
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

Returns a mutable reference to the [`ThreadStats`](ThreadStats.md) for the given `(pid, tid)` pair, creating the `ProcessStats` and/or `ThreadStats` entries if they do not already exist. Marked `#[inline]` for performance in tight loops.

### `update_active_streaks`

```AffinityServiceRust/src/scheduler.rs#L57-79
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

Updates hysteresis streak counters for all threads provided in `tid_with_delta_cycles`. The algorithm works as follows:

1. Determines the **max delta cycles** across all threads in the slice.
2. **Early exit on zero activity.** If `max_cycles` is 0 (i.e., no thread reported any delta cycles), the method resets `active_streak` to 0 for **every** thread in **every** tracked process and returns immediately. This prevents stale streaks from persisting during idle periods when no meaningful CPU work is occurring.
3. Computes `entry_min = max_cycles × entry_threshold` and `keep_min = max_cycles × keep_threshold`.
4. For each thread:
   - If the thread already has a nonzero streak and its delta drops below `keep_min`, the streak is **reset to 0**.
   - If the thread has a nonzero streak and meets `keep_min`, the streak is **incremented** (capped at 254).
   - If the thread has zero streak and its delta meets or exceeds `entry_min`, the streak is **set to 1**.

This asymmetric entry/keep threshold prevents threads from flickering between prime and non-prime status when their CPU usage is near the boundary. The global streak reset at zero activity ensures that when the system goes idle, all threads lose their accumulated streaks and must re-qualify from scratch when activity resumes.

### `select_top_threads_with_hysteresis`

```AffinityServiceRust/src/scheduler.rs#L88-126
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
)
```

Performs two-pass prime-thread selection:

- **First pass (retain):** Iterates over threads sorted by descending delta cycles. Threads that are already assigned (as determined by the `is_currently_assigned` callback) and whose delta meets or exceeds the `keep_min` threshold retain their prime status. This prevents demotion due to minor cycle fluctuations.
- **Second pass (promote):** Fills any remaining slots with threads that meet the `entry_min` threshold **and** have accumulated an `active_streak` at or above `constants.min_active_streak`. This prevents briefly-active threads from being promoted prematurely.

The `is_prime` (third element) boolean in each tuple is set to `true` for selected threads. Threads with TID 0 are always skipped. The `slot_count` parameter limits the total number of threads that can be selected.

### `drop_process_by_pid`

```AffinityServiceRust/src/scheduler.rs#L130-172
pub fn drop_process_by_pid(&mut self, pid: &u32)
```

Cleans up all state for the given process. If `track_top_x_threads` is nonzero, the method first builds a sorted report of the top N threads by `last_cycles`, including module-name resolution for each thread's start address and optional `SYSTEM_THREAD_INFORMATION` details (kernel time, user time, create time, wait time, context switches, thread state, wait reason, priority). The report is emitted via `log_message`.

After logging, the method calls `drop_module_cache` to release the per-process module-resolution cache and then calls `self.pid_to_process_stats.remove(pid)` to remove the `ProcessStats` entry from the map. It then explicitly iterates over the removed process's `tid_to_thread_stats` to drop each thread handle individually, ensuring that Win32 handles are closed promptly and deterministically rather than relying on implicit `Drop` during map removal.

## Remarks

- The scheduler is instantiated once in `main` and lives for the entire duration of the service.
- The `HashMap` type used is the project's custom `collections::HashMap`, which may differ from `std::collections::HashMap` (e.g., using a different hasher or inline storage).
- Thread handles stored inside `ThreadStats` are explicitly dropped by `drop_process_by_pid` after removing the process entry, ensuring deterministic handle closure.
- The `constants` field is cloned from the parsed configuration. If configuration is hot-reloaded, a new scheduler may be constructed with updated constants.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Callers | [main](../main.rs/main.md), [apply_thread_level](../main.rs/apply_thread_level.md), apply module |
| Callees | `winapi::resolve_address_to_module`, `winapi::drop_module_cache`, `logging::log_message` |
| Dependencies | `ConfigConstants` (config module), [ProcessStats](ProcessStats.md), [ThreadStats](ThreadStats.md), [IdealProcessorState](IdealProcessorState.md) |
| Platform | Windows (handles Win32 thread handles internally) |

## See Also

| Reference | Link |
|-----------|------|
| ProcessStats | [ProcessStats](ProcessStats.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| IdealProcessorState | [IdealProcessorState](IdealProcessorState.md) |
| format_100ns | [format_100ns](format_100ns.md) |
| format_filetime | [format_filetime](format_filetime.md) |
| main module | [main.rs README](../main.rs/README.md) |
| apply_thread_level | [apply_thread_level](../main.rs/apply_thread_level.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
