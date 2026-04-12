# apply_prime_threads_select function (apply.rs)

Selects top threads for prime status using hysteresis-based thresholds. This function wraps the scheduler's `select_top_threads_with_hysteresis` method, using CPU set pinning state as the "currently prime" predicate.

## Syntax

```rust
fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

## Parameters

`pid`

The process ID of the target process.

`prime_count`

The number of prime thread slots available (typically equal to the number of prime CPUs configured).

`tid_with_delta_cycles`

A mutable slice of tuples `(tid, delta_cycles, is_prime)`. On input, `is_prime` is `false` for all entries. On output, threads selected for prime status have `is_prime` set to `true`. The `delta_cycles` value represents the change in CPU cycles since the last measurement.

`prime_core_scheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that owns the hysteresis algorithm and per-thread statistics.

## Return value

This function does not return a value. Results are written in-place by setting the `is_prime` flag (third element) in `tid_with_delta_cycles`.

## Remarks

Hysteresis prevents threads from rapidly flipping between prime and non-prime status each iteration. The selection uses two thresholds from [ConfigConstants](../config.rs/ConfigConstants.md):

- **keep_threshold** — A thread that is *already* prime stays prime as long as its cycle delta is at least `keep_threshold`% of the maximum cycle delta in the candidate set.
- **entry_threshold** — A thread that is *not yet* prime must exceed `entry_threshold`% of the maximum cycle delta *and* have an `active_streak` of at least `min_active_streak` consecutive iterations to be promoted.

The "currently prime" predicate passed to `select_top_threads_with_hysteresis` is:

```rust
|thread_stats| !thread_stats.pinned_cpu_set_ids.is_empty()
```

This means a thread is considered currently prime if it has any CPU set IDs pinned — i.e., it was previously promoted by [apply_prime_threads_promote](apply_prime_threads_promote.md).

This function is called by [apply_prime_threads](apply_prime_threads.md) after candidates are gathered and before the promote/demote phases.

### Selection flow

1. [apply_prime_threads](apply_prime_threads.md) builds the `tid_with_delta_cycles` array from the candidate pool.
2. `apply_prime_threads_select` marks the top `prime_count` threads as prime using hysteresis.
3. [apply_prime_threads_promote](apply_prime_threads_promote.md) pins newly-selected threads to prime CPUs.
4. [apply_prime_threads_demote](apply_prime_threads_demote.md) unpins threads that lost prime status.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Line** | L802–L816 |
| **Called by** | [apply_prime_threads](apply_prime_threads.md) |
| **Calls** | [PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md) |
| **Windows API** | None |

## See also

- [apply_prime_threads](apply_prime_threads.md)
- [apply_prime_threads_promote](apply_prime_threads_promote.md)
- [apply_prime_threads_demote](apply_prime_threads_demote.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)
- [ConfigConstants](../config.rs/ConfigConstants.md)