# apply_prime_threads_select function (apply.rs)

The `apply_prime_threads_select` function selects the top threads for prime status using a hysteresis-based algorithm. It delegates to `PrimeThreadScheduler::select_top_threads_with_hysteresis`, which applies differentiated entry and keep thresholds to prevent threads from rapidly flipping between prime and non-prime status across successive apply cycles. A thread that is already pinned to a CPU set (i.e., currently prime) is evaluated against a more lenient keep threshold, while a non-prime thread must exceed a stricter entry threshold and meet a minimum active-streak requirement to be promoted.

## Syntax

```AffinityServiceRust/src/apply.rs#L793-802
pub fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Passed through to the scheduler to look up per-process thread statistics. |
| `prime_count` | `usize` | The maximum number of threads that can be selected as prime. This equals the number of CPUs in `config.prime_threads_cpus`, i.e., the number of dedicated high-performance cores available for pinning. |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | A mutable slice of tuples, one per candidate thread. Each tuple contains: the thread ID (`u32`), the delta cycle count since the last measurement (`u64`), and a boolean selection flag (`bool`). On entry, the boolean is `false` for all elements. On exit, the boolean is set to `true` for threads selected as prime. The slice is expected to be pre-sorted by delta cycles in descending order by the caller ([`apply_prime_threads`](apply_prime_threads.md)). |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | The prime thread scheduler that maintains per-process, per-thread statistics (active streaks, pinned CPU set IDs, cycle history). The function calls `select_top_threads_with_hysteresis` on this scheduler, which reads and updates the thread stats. |

## Return value

This function does not return a value. The selection results are communicated by mutating the `is_prime` boolean (third element) in each tuple of the `tid_with_delta_cycles` slice.

## Remarks

### Hysteresis algorithm

The function delegates entirely to `PrimeThreadScheduler::select_top_threads_with_hysteresis`, passing a closure `|thread_stats| !thread_stats.pinned_cpu_set_ids.is_empty()` as the "is currently prime" predicate. This closure returns `true` when the thread already has CPU set IDs pinned (i.e., it was promoted in a previous cycle), causing the scheduler to apply the more lenient **keep threshold**. Threads not currently pinned must exceed the stricter **entry threshold** and have an `active_streak` count at or above the configured minimum to be selected.

The hysteresis mechanism ensures that:
- A thread that is already running on a prime core stays there even if its cycle count temporarily dips below the entry threshold. This avoids unnecessary demotion-then-re-promotion churn.
- A thread must demonstrate sustained high CPU activity (measured by active streak) before being promoted, preventing brief spikes from triggering a promotion that would immediately be reversed.

### Selection limit

At most `prime_count` threads are marked as prime. If more threads qualify than there are prime slots, only the top `prime_count` by delta cycles are selected.

### Interaction with promote/demote

This function is the first of three stages in the prime-thread pipeline orchestrated by [`apply_prime_threads`](apply_prime_threads.md):

1. **Select** (`apply_prime_threads_select`) — marks threads as prime or non-prime in the `tid_with_delta_cycles` slice.
2. **Promote** ([`apply_prime_threads_promote`](apply_prime_threads_promote.md)) — pins newly-selected prime threads to CPUs and boosts their priority.
3. **Demote** ([`apply_prime_threads_demote`](apply_prime_threads_demote.md)) — unpins and restores priority for threads that lost prime status.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | None (pure logic; no OS calls) |
| Callers | [`apply_prime_threads`](apply_prime_threads.md) |
| Callees | `PrimeThreadScheduler::select_top_threads_with_hysteresis` |
| Privileges | None |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_promote | [`apply_prime_threads_promote`](apply_prime_threads_promote.md) |
| apply_prime_threads_demote | [`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*