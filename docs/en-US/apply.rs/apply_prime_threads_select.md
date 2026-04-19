# apply_prime_threads_select function (apply.rs)

Selects the top threads for prime status using hysteresis-based promotion logic. This function is the decision layer of the prime thread scheduling pipeline — it determines *which* threads qualify for prime CPU pinning based on CPU cycle deltas and active streaks, without performing any actual system calls.

## Syntax

```AffinityServiceRust/src/apply.rs#L794-800
pub fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

## Parameters

`pid: u32`

The process ID whose threads are being evaluated.

`prime_count: usize`

The maximum number of threads that can be promoted to prime status. Typically equal to the number of configured prime CPUs (`config.prime_threads_cpus.len()`).

`tid_with_delta_cycles: &mut [(u32, u64, bool)]`

A mutable slice of `(thread_id, delta_cycles, is_prime)` tuples. On entry, the `is_prime` field is `false` for all entries. On exit, threads selected for prime status have `is_prime` set to `true`. The slice should already be populated with candidate threads and their cycle deltas from the current scheduling interval.

`prime_core_scheduler: &mut PrimeThreadScheduler`

The [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) instance that owns thread statistics, hysteresis constants, and the `select_top_threads_with_hysteresis` algorithm.

## Return value

This function does not return a value. Results are written in-place to the `is_prime` field of each tuple in `tid_with_delta_cycles`.

## Remarks

This function delegates entirely to [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md), passing a predicate that considers a thread "currently assigned" if its `pinned_cpu_set_ids` is non-empty. The hysteresis algorithm applies two thresholds:

- **Keep threshold** — A thread that is already prime stays prime if its cycle delta is at or above this percentage of the maximum cycle delta among all candidates.
- **Entry threshold** — A thread that is not currently prime must exceed this (higher) percentage of the maximum *and* maintain an active streak of at least `min_active_streak` intervals before being promoted.

This two-threshold approach prevents rapid flipping (thrashing) between prime and non-prime states when multiple threads have similar CPU utilization.

The function is deliberately separated from `apply_prime_threads_promote` and `apply_prime_threads_demote` to maintain a clean **select → promote → demote** pipeline within [`apply_prime_threads`](apply_prime_threads.md).

### Predicate: `is_currently_assigned`

The closure passed to the hysteresis selector is:

```AffinityServiceRust/src/apply.rs#L801-803
|thread_stats| {
    !thread_stats.pinned_cpu_set_ids.is_empty()
}
```

A thread is considered currently assigned (and eligible for the lower keep threshold) if it has been pinned to one or more CPU set IDs by a previous promote pass.

## Requirements

| Requirement | Value |
|---|---|
| **Module** | `src/apply.rs` |
| **Called by** | [`apply_prime_threads`](apply_prime_threads.md) |
| **Calls** | [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md) |
| **Win32 API** | None |
| **Privileges** | None (no system calls) |

## See Also

| Topic | Description |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | Orchestrator that calls select → promote → demote |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | Pins selected prime threads to CPUs and boosts priority |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | Unpins and restores threads that lost prime status |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | Collects cycle data consumed by selection |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Scheduler owning hysteresis logic and thread statistics |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*