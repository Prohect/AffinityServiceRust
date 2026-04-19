# update_thread_stats function (apply.rs)

Caches current cycle times and total times as "last" values for the next iteration's delta calculation. This function must be called at the end of each apply cycle to ensure that the next cycle computes accurate deltas for thread CPU usage.

## Syntax

```AffinityServiceRust/src/apply.rs#L1312-1315
pub fn update_thread_stats(
    pid: u32,
    prime_scheduler: &mut PrimeThreadScheduler,
)
```

## Parameters

`pid: u32`

The process ID whose thread statistics should be updated.

`prime_scheduler: &mut PrimeThreadScheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that owns the per-thread statistics cache. The scheduler's `pid_to_process_stats` map is queried for the given `pid`.

## Return value

This function does not return a value.

## Remarks

`update_thread_stats` performs two transfers for every thread tracked under the given process:

1. **Cycle snapshot** — `cached_cycles` is copied to `last_cycles`, then `cached_cycles` is zeroed. This prepares the baseline for the next call to [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md), which computes `delta_cycles = cached_cycles - last_cycles`.

2. **Total time snapshot** — `cached_total_time` is copied to `last_total_time`, then `cached_total_time` is zeroed. Total time (kernel + user) is used for sorting threads by CPU consumption and for computing time-based deltas.

Only entries where the cached value is greater than zero are updated. This avoids overwriting valid `last_*` values for threads that were not measured during the current cycle (e.g., threads that fell outside the candidate pool in [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)).

If the `pid` is not present in `pid_to_process_stats`, the function silently returns without effect.

### Call sequence

In a typical apply cycle, the call order is:

1. [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) — queries current cycle/time values and stores them in `cached_*` fields.
2. [apply_prime_threads](apply_prime_threads.md) / [apply_ideal_processors](apply_ideal_processors.md) — consumes the cached deltas for thread selection and scheduling decisions.
3. **`update_thread_stats`** — snapshots cached values to `last_*` fields and clears the cache.

Omitting this call would cause deltas to accumulate across multiple cycles, resulting in incorrect thread ranking in the prime thread and ideal processor algorithms.

## Requirements

| | |
|---|---|
| **Module** | [apply.rs](README.md) |
| **Callers** | Service main loop (after all per-process apply functions complete) |
| **Callees** | None (pure data bookkeeping) |
| **API** | None |
| **Privileges** | None |

## See Also

| Topic | Description |
|---|---|
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | Populates `cached_cycles` and `cached_total_time` that this function snapshots |
| [apply_prime_threads](apply_prime_threads.md) | Consumes cycle deltas for prime thread selection |
| [apply_ideal_processors](apply_ideal_processors.md) | Consumes cycle deltas for ideal processor assignment |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Owns the per-process, per-thread statistics maps |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*