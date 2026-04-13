# update_thread_stats function (apply.rs)

Commits cached cycle and time counters stored in [ThreadStats](../scheduler.rs/ThreadStats.md) so that the next polling iteration can compute correct deltas. This function is the final step in the per-process thread-level apply pipeline — it must be called after [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) and [apply_prime_threads](apply_prime_threads.md) have finished, but before the next cycle begins.

## Syntax

```AffinityServiceRust/src/apply.rs#L1327-1340
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
| `pid` | `u32` | Process identifier whose thread stats should be committed. Used as the key into `prime_scheduler.pid_to_process_stats`. If no entry exists for this pid (e.g., the process was never tracked), the function returns immediately. |
| `prime_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | The scheduler state that holds per-process, per-thread statistics. This function mutates the [ThreadStats](../scheduler.rs/ThreadStats.md) entries for every tracked thread of the given process. |

## Return value

None (`()`).

## Remarks

### Double-buffer commit pattern

The prime-thread pipeline uses a double-buffer strategy for cycle counts and CPU times:

| Field | Written by | Read by | Committed by |
|-------|-----------|---------|--------------|
| `cached_cycles` | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | [apply_prime_threads](apply_prime_threads.md) (delta = `cached_cycles - last_cycles`) | `update_thread_stats` → `last_cycles` |
| `cached_total_time` | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | [apply_prime_threads](apply_prime_threads.md) (delta = `cached_total_time - last_total_time`) | `update_thread_stats` → `last_total_time` |

During a polling cycle:

1. [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) writes the latest counters into the `cached_*` fields.
2. [apply_prime_threads](apply_prime_threads.md) computes deltas by subtracting `last_*` from `cached_*`.
3. `update_thread_stats` promotes `cached_*` → `last_*` and zeroes out `cached_*`.

This separation ensures that the delta calculation in step 2 always compares the *current* snapshot against the *previous* snapshot, even if intermediate functions read or mutate the cached values.

### Zero-guard

Only non-zero `cached_*` values are committed. A thread whose cycle count or total time was not queried during the current cycle (because it fell outside the [prefetch counter limit](prefetch_all_thread_cycles.md#counter-limit) or because `QueryThreadCycleTime` failed) retains its previous `last_*` values. This prevents a missed query from resetting the baseline to zero, which would cause an artificially large delta on the next successful query and could incorrectly promote a low-activity thread.

After the commit, the `cached_*` field is set to `0` to indicate that no fresh data has been collected yet for the next cycle.

### Idempotency

Calling this function more than once within the same polling cycle is harmless — the second call sees `cached_*` values of `0` and skips every entry. However, doing so would cause the *next* cycle to compute a delta against the same `last_*` baseline twice, effectively halving the reported delta. Callers must therefore invoke `update_thread_stats` exactly once per process per cycle.

### No-op when process is untracked

If `pid` does not exist in `prime_scheduler.pid_to_process_stats`, the `if let Some(ps)` guard causes an immediate return. This is safe and expected for processes that have a [ProcessConfig](../config.rs/ProcessConfig.md) but do not use prime-thread or ideal-processor features (i.e., `track_top_x_threads == 0` and `prime_threads_cpus` is empty).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| Callees | None — pure Rust field assignments on [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Win32 API | None |
| Privileges | None |

## See Also

| Topic | Link |
|-------|------|
| Cycle-time prefetch (populates cached values) | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| Prime-thread orchestration (consumes cached values) | [apply_prime_threads](apply_prime_threads.md) |
| Per-thread stats model | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Scheduler state | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Thread-level apply orchestration | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| apply module overview | [apply](README.md) |