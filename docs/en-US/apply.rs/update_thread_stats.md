# update_thread_stats function (apply.rs)

The `update_thread_stats` function commits the cached cycle-count and total-time measurements collected during the current apply cycle into the persistent `last_cycles` and `last_total_time` fields of each thread's statistics, then resets the cached values to zero. This establishes the baseline for computing deltas in the next apply cycle. It is intended to be called at the end of the per-process apply pipeline, after all prime-thread selection, promotion, and demotion logic has consumed the cached values.

## Syntax

```AffinityServiceRust/src/apply.rs#L1311-1324
pub fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) {
    if let Some(ps) = prime_scheduler.pid_to_process_stats.get_mut(&pid) {
        for ts in ps.tid_to_thread_stats.values_mut() {
            if ts.cached_cycles > 0 {
                ts.last_cycles = ts.cached_cycles;
                ts.cached_cycles = 0;
            }
            if ts.cached_total_time > 0 {
                ts.last_total_time = ts.cached_total_time;
                ts.cached_total_time = 0;
            }
        }
    }
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID whose thread statistics should be committed. Used to look up the corresponding `ProcessStats` entry in the scheduler's `pid_to_process_stats` map. If no entry exists for this PID, the function is a no-op. |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | The mutable prime-thread scheduler state containing per-process, per-thread statistics. The function iterates over all thread stats for the given PID and updates their `last_*` / `cached_*` fields. |

## Return value

This function does not return a value.

## Remarks

### Commit semantics

The function uses a guard pattern: only cached values that are strictly greater than zero are committed. This ensures that threads whose cycles or total time could not be measured during the current cycle (e.g., because handle acquisition failed in [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md)) retain their previous `last_*` baseline rather than being reset to zero.

After committing, the `cached_cycles` and `cached_total_time` fields are set to `0`. This ensures that if a thread is not measured in the *next* cycle, its delta will be computed correctly (the delta calculation uses `cached_cycles.saturating_sub(last_cycles)`, so a cached value of `0` yields a delta of `0`).

### Call order

This function must be called **after** all of the following have consumed the cached measurements:

1. [`apply_prime_threads`](apply_prime_threads.md) — uses `cached_cycles` and `cached_total_time` to build candidate lists and compute deltas.
2. [`apply_prime_threads_select`](apply_prime_threads_select.md) — reads cycle deltas computed from the cached values.
3. [`apply_prime_threads_promote`](apply_prime_threads_promote.md) — logs `delta_cycles` in change messages.
4. [`apply_prime_threads_demote`](apply_prime_threads_demote.md) — operates on the same selection results.
5. [`apply_ideal_processors`](apply_ideal_processors.md) — uses `cached_cycles - last_cycles` for thread ranking.

Calling `update_thread_stats` before these functions would cause all deltas to be zero, effectively disabling the prime-thread algorithm.

### Edge cases

- If `pid` is not present in `pid_to_process_stats` (e.g., the process was never registered with the scheduler or has already been cleaned up), the `if let Some` guard causes the function to return immediately without error.
- If a thread's `cached_cycles` is `0` (because `QueryThreadCycleTime` failed or was never called for that thread), `last_cycles` retains its previous value. This means the next cycle's delta for that thread will still be based on the most recent successful measurement.
- The function does not remove stale thread entries. Thread cleanup is handled separately by [`apply_prime_threads`](apply_prime_threads.md), which drops handles for threads no longer present in the live thread snapshot.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | None (pure data manipulation; no OS calls) |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that runs the per-process apply pipeline |
| Callees | None (reads and writes fields of `PrimeThreadScheduler` directly) |
| Privileges | None |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_ideal_processors | [`apply_ideal_processors`](apply_ideal_processors.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*