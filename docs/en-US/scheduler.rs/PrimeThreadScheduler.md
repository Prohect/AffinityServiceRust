# PrimeThreadScheduler struct (scheduler.rs)

The prime thread scheduling engine. Maintains per-process, per-thread statistics and implements hysteresis-based thread selection to identify and promote the most CPU-active threads ("prime threads") to dedicated processor cores. The hysteresis mechanism prevents thrashing by requiring threads to sustain activity above an entry threshold before promotion, and allowing them to remain promoted as long as they stay above a lower keep threshold.

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
| `pid_to_process_stats` | `HashMap<u32, ProcessStats>` | Map from process ID to [ProcessStats](ProcessStats.md), tracking per-process thread statistics, liveness state, and debug configuration. Entries are created on first access and removed when a process exits via [drop_process_by_pid](#drop_process_by_pid). |
| `constants` | `ConfigConstants` | Hysteresis tuning constants sourced from the service configuration: `entry_threshold`, `keep_threshold`, and `min_active_streak`. See [ConfigConstants](../config.rs/ConfigConstants.md). |

## Methods

### new

```rust
pub fn new(constants: ConfigConstants) -> Self
```

Creates a new `PrimeThreadScheduler` with the given hysteresis constants and an empty process stats map.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `constants` | `ConfigConstants` | Hysteresis tuning parameters that control thread promotion/demotion behavior. |

**Return value**

`PrimeThreadScheduler` — A new scheduler instance ready for use.

---

### reset_alive

```rust
pub fn reset_alive(&mut self)
```

Marks all tracked processes as dead by setting `alive = false` on every [ProcessStats](ProcessStats.md) entry. Called at the start of each polling loop iteration. Processes that are still running will be marked alive again via [set_alive](#set_alive) during snapshot processing. Processes that remain dead after the iteration are candidates for cleanup via [drop_process_by_pid](#drop_process_by_pid).

---

### set_alive

```rust
pub fn set_alive(&mut self, pid: u32)
```

Marks a process as alive for the current iteration. If no [ProcessStats](ProcessStats.md) entry exists for the given PID, one is created with default values.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier to mark as alive. |

---

### set_tracking_info

```rust
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

Sets debug tracking information for a process. When the process exits, if `track_top_x_threads` is non-zero, the top N threads by CPU cycles are logged in a diagnostic report.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Target process identifier. |
| `track_top_x_threads` | `i32` | Number of top threads to report on process exit. `0` disables tracking. The absolute value is used as the count. |
| `process_name` | `String` | Display name for the process in log output. |

---

### get_thread_stats

```rust
#[inline]
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

Returns a mutable reference to the [ThreadStats](ThreadStats.md) for the given process and thread. If either the process or thread entry does not exist, it is created with default values.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier. |
| `tid` | `u32` | Thread identifier. |

**Return value**

`&mut ThreadStats` — Mutable reference to the thread's statistics entry.

---

### update_active_streaks

```rust
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

Updates the `active_streak` counter on each thread based on its delta CPU cycles relative to the maximum across all threads.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier owning the threads. |
| `tid_with_delta_cycles` | `&[(u32, u64)]` | Slice of `(thread_id, delta_cycles)` pairs representing CPU cycle deltas since the last measurement. |

**Remarks**

The streak update algorithm works as follows:

1. Compute `max_cycles` — the highest delta across all threads in the slice.
2. If `max_cycles == 0`, reset all streaks to zero and return.
3. For each thread:
   - **If the thread already has a streak > 0:**
     - If `delta < keep_threshold × max_cycles`, reset streak to 0 (thread fell below keep threshold).
     - Otherwise, increment streak by 1, capped at 254.
   - **If the thread has no streak:**
     - If `delta >= entry_threshold × max_cycles`, set streak to 1 (thread entered active zone).

The `entry_threshold` is intentionally higher than `keep_threshold`, creating a hysteresis band that prevents rapid promotion/demotion cycling. The `active_streak` counter must reach `min_active_streak` before a thread qualifies for promotion in [select_top_threads_with_hysteresis](#select_top_threads_with_hysteresis).

---

### select_top_threads_with_hysteresis

```rust
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
)
```

Selects which threads should receive prime status using a two-pass hysteresis algorithm.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier owning the threads. |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | **\[in/out\]** Mutable slice of `(thread_id, delta_cycles, is_prime)` tuples. The `is_prime` field is set to `true` for threads selected as prime. Input values of `is_prime` are ignored (overwritten). |
| `slot_count` | `usize` | Maximum number of threads that can be promoted to prime status (typically the number of dedicated CPU cores). |
| `is_currently_assigned` | `fn(&ThreadStats) -> bool` | Callback that returns `true` if the thread is currently assigned to a prime resource (e.g., has a non-empty `pinned_cpu_set_ids`). Used by the first pass to identify incumbents. |

**Return value**

This function does not return a value. Results are communicated through the `is_prime` field of each tuple in `tid_with_delta_cycles`.

**Remarks**

The selection proceeds in two passes over the input, which is first sorted in descending order by `delta_cycles`:

| Pass | Purpose | Criteria |
|------|---------|----------|
| **First (retain)** | Keep currently-assigned threads that still qualify | `is_currently_assigned(stats) == true` AND `delta >= keep_threshold × max_cycles` |
| **Second (promote)** | Fill remaining slots with new candidates | `delta >= entry_threshold × max_cycles` AND `active_streak >= min_active_streak` AND thread not already selected |

This two-pass design ensures that incumbent prime threads are not demoted due to minor cycle fluctuations, while new threads must demonstrate sustained high activity before being promoted. The gap between `entry_threshold` and `keep_threshold` is the hysteresis band.

---

### drop_process_by_pid

```rust
pub fn drop_process_by_pid(&mut self, pid: &u32)
```

Removes a process and all of its thread statistics from the scheduler. Closes all cached thread handles, drops the module address cache for the process, and optionally emits a diagnostic report of the top threads.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `&u32` | Reference to the process identifier to remove. |

**Remarks**

- If `track_top_x_threads != 0` on the [ProcessStats](ProcessStats.md) entry, a report of the top N threads (by `last_cycles`) is logged. The report includes per-thread details: TID, cycle count, start address resolved to module name, kernel time, user time, create time, wait time, priority, base priority, context switches, thread state, and wait reason.
- All [ThreadHandle](../winapi.rs/ThreadHandle.md) instances stored in [ThreadStats](ThreadStats.md) are dropped, which closes their underlying Windows HANDLEs.
- Calls [drop_module_cache](../winapi.rs/drop_module_cache.md) to release the cached module enumeration for this process.
- If the PID is not found in the map, the function returns immediately with no effect.

## Remarks

### Hysteresis model

The scheduler uses a classic hysteresis (Schmitt trigger) pattern to prevent thrashing between prime and non-prime states:

```
                    ┌─────────────────────────────────┐
   entry_threshold  │  ← thread must exceed this to   │
                    │    BEGIN earning streak           │
                    ├─────────────────────────────────┤
   keep_threshold   │  ← thread must stay above this  │
                    │    to KEEP prime status           │
                    └─────────────────────────────────┘
```

The `entry_threshold` is higher than `keep_threshold`. A thread must:
1. Exceed `entry_threshold × max_cycles` to start accumulating an active streak.
2. Maintain its streak for `min_active_streak` iterations to qualify for promotion.
3. Once promoted, it only loses prime status if its cycles drop below `keep_threshold × max_cycles`.

### Lifecycle

1. **reset_alive** — called at the start of each loop iteration.
2. **set_alive** / **set_tracking_info** — called as each process is found in the snapshot.
3. **update_active_streaks** — called after cycle deltas are computed.
4. **select_top_threads_with_hysteresis** — called to determine prime thread assignments.
5. **drop_process_by_pid** — called for processes that were not marked alive (exited).

## Requirements

| | |
|---|---|
| **Module** | `src/scheduler.rs` |
| **Callers** | Main polling loop in `src/main.rs`, [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) |
| **Callees** | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md), [drop_module_cache](../winapi.rs/drop_module_cache.md), [log_message](../logging.rs/log_message.md) |
| **Dependencies** | [ConfigConstants](../config.rs/ConfigConstants.md), [ProcessStats](ProcessStats.md), [ThreadStats](ThreadStats.md), [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| **Privileges** | None directly; thread handles used by this scheduler require privileges obtained earlier |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [scheduler.rs](README.md) |
| Per-process statistics | [ProcessStats](ProcessStats.md) |
| Per-thread statistics | [ThreadStats](ThreadStats.md) |
| Ideal processor state | [IdealProcessorState](IdealProcessorState.md) |
| Thread handle RAII wrapper | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Prime thread application | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| Configuration constants | [ConfigConstants](../config.rs/ConfigConstants.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*