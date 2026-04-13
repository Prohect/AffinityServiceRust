# apply_prime_threads function (apply.rs)

Top-level orchestrator for the prime-thread scheduling pipeline. This function identifies the most CPU-intensive threads in a process, selects a subset for promotion using hysteresis-based thresholds, pins promoted threads to dedicated performance-core CPUs via per-thread CPU sets, and demotes threads that no longer qualify. The actual selection, promotion, and demotion steps are delegated to [apply_prime_threads_select](apply_prime_threads_select.md), [apply_prime_threads_promote](apply_prime_threads_promote.md), and [apply_prime_threads_demote](apply_prime_threads_demote.md) respectively.

## Syntax

```AffinityServiceRust/src/apply.rs#L713-800
pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used as the key into the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) state maps and for logging. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | Parsed configuration rule for this process. The fields `prime_threads_cpus`, `prime_threads_prefixes`, and `track_top_x_threads` control whether prime-thread scheduling is active and how many threads to track. |
| `dry_run` | `bool` | When `true`, the function records the configured prime CPUs as a change message and returns without modifying any OS state. |
| `current_mask` | `&mut usize` | The process's current affinity mask, previously populated by [apply_affinity](apply_affinity.md). Passed through to [apply_prime_threads_promote](apply_prime_threads_promote.md) so that prime CPU indices can be filtered against the effective affinity. |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | Snapshot entry for the target process. Provides the thread list and per-thread kernel/user time from the most recent `NtQuerySystemInformation` snapshot. Passed through to [apply_prime_threads_demote](apply_prime_threads_demote.md) for live-thread enumeration. |
| `prime_core_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Persistent state that tracks per-thread cycle counts, active streaks, pinned CPU set IDs, original priorities, and open thread handles across polling cycles. |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages. Populated by this function and all three delegate functions. |

## Return value

None (`()`). All results are communicated through `apply_config_result` and side effects on `prime_core_scheduler`, `process`, and `current_mask`.

## Remarks

### Activation conditions

The function performs work only when at least one of the following is true:

- `config.prime_threads_cpus` is non-empty, or `config.prime_threads_prefixes` is non-empty (there are prime CPUs to pin to).
- `config.track_top_x_threads` is non-zero (thread tracking is enabled for diagnostic logging by the scheduler).

A negative `track_top_x_threads` value disables the prime-thread promotion/demotion pipeline while still allowing thread tracking. When both conditions fail, the function returns immediately.

### Algorithm overview

```/dev/null/pipeline.txt#L1-5
 ┌────────────────┐    ┌──────────────┐    ┌─────────────┐    ┌──────────────┐
 │ Collect & Sort │───>│    Select    │───>│   Promote   │───>│    Demote    │
 │  Candidates    │    │  (hysteresis)│    │ (CPU pin +  │    │ (unpin +     │
 │                │    │              │    │  prio boost) │    │  prio restore)│
 └────────────────┘    └──────────────┘    └─────────────┘    └──────────────┘
```

1. **Collect thread time deltas** — Iterates over all threads in the process snapshot. For each thread, computes the delta between the cached total time (`KernelTime + UserTime`) stored in [ThreadStats](../scheduler.rs/ThreadStats.md) and the `last_total_time` from the previous cycle. If `track_top_x_threads` is non-zero, the current `SYSTEM_THREAD_INFORMATION` is also saved into `last_system_thread_info` for diagnostic output by the scheduler.

2. **Sort and select candidates** — Threads are sorted by time delta in descending order. The candidate pool size is computed as `max(prime_count × 4, cpu_count)` capped at the total thread count. The factor of 4 ensures the candidate pool is large enough for the hysteresis algorithm to consider threads that are rising in CPU usage but have not yet reached prime status. Previously-pinned threads that fell outside the top candidates are appended to the pool so they can be properly demoted if they no longer qualify.

3. **Build cycle-delta tuples** — For each candidate thread, a `(tid, delta_cycles, is_prime)` tuple is constructed. `delta_cycles` is the difference between `cached_cycles` (set by [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)) and `last_cycles` from the previous iteration. The `is_prime` flag is initially `false` for all entries.

4. **Select** — [apply_prime_threads_select](apply_prime_threads_select.md) marks the top *N* entries as prime using the hysteresis algorithm from [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), where *N* equals the number of configured prime CPUs.

5. **Promote** — [apply_prime_threads_promote](apply_prime_threads_promote.md) pins each newly-promoted thread to its assigned prime CPU set and optionally boosts its thread priority.

6. **Demote** — [apply_prime_threads_demote](apply_prime_threads_demote.md) unpins threads that lost prime status and restores their original thread priority.

7. **Handle cleanup** — After the pipeline completes, the function iterates over [ThreadStats](../scheduler.rs/ThreadStats.md) entries and removes handles for threads that no longer appear in the live thread set. The `Drop` implementation on [ThreadHandle](../winapi.rs/ThreadHandle.md) automatically closes OS handles when they are taken out of `Option`.

### Candidate pool sizing

The candidate pool formula `max(prime_count × 4, cpu_count)` is a tuning heuristic:

- The `prime_count × 4` term ensures that at least four times as many threads as prime slots are evaluated, giving the hysteresis algorithm room to track rising threads.
- The `cpu_count` lower bound ensures that on systems with many CPUs but few prime slots, enough threads are tracked to detect meaningful CPU usage patterns.
- Capping at `thread_count` prevents out-of-bounds indexing for processes with fewer threads than the computed pool size.

### Relationship to prefetch_all_thread_cycles

This function assumes that [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) has already been called for this process in the current cycle. `prefetch_all_thread_cycles` opens thread handles, queries `QueryThreadCycleTime`, and populates `cached_cycles` and `cached_total_time` in [ThreadStats](../scheduler.rs/ThreadStats.md). `apply_prime_threads` reads those cached values to compute deltas. If `prefetch_all_thread_cycles` was not called (or if it failed for all threads), all cycle deltas will be zero and no threads will be selected as prime.

### Dry-run behaviour

In dry-run mode, the function records a single change message:

`"Prime CPUs: -> [4,5,6,7]"`

It does not call any of the delegate functions, query thread cycle times, or modify any scheduler state.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| Callees | [apply_prime_threads_select](apply_prime_threads_select.md), [apply_prime_threads_promote](apply_prime_threads_promote.md), [apply_prime_threads_demote](apply_prime_threads_demote.md), [get_cpu_set_information](../winapi.rs/get_cpu_set_information.md), [format_cpu_indices](../config.rs/format_cpu_indices.md) |
| Win32 API | None directly — all Win32 calls are made by the delegate functions |
| Privileges | Inherits privilege requirements of [apply_prime_threads_promote](apply_prime_threads_promote.md) and [apply_prime_threads_demote](apply_prime_threads_demote.md) |

## See Also

| Topic | Link |
|-------|------|
| Prime thread selection (hysteresis) | [apply_prime_threads_select](apply_prime_threads_select.md) |
| Prime thread promotion (CPU pin + priority boost) | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| Prime thread demotion (unpin + priority restore) | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| Cycle-time prefetch | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| Scheduler state and hysteresis algorithm | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Thread-level apply orchestration | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| Cycle/time commit after apply | [update_thread_stats](update_thread_stats.md) |
| Configuration model | [ProcessConfig](../config.rs/ProcessConfig.md) |
| apply module overview | [apply](README.md) |