# apply_prime_threads_select function (apply.rs)

Selects the top *N* threads for prime status from a pool of candidates using hysteresis-based thresholds. This function is a thin wrapper around [PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm), providing the "currently pinned" predicate that the hysteresis algorithm uses to distinguish threads that are already prime (and thus subject to a more lenient *keep* threshold) from threads that must meet the stricter *entry* threshold to be newly promoted.

## Syntax

```AffinityServiceRust/src/apply.rs#L807-816
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
| `pid` | `u32` | Process identifier of the target process. Used as the key into the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) per-process stats map. |
| `prime_count` | `usize` | The number of prime slots available — equal to `config.prime_threads_cpus.len()`. At most this many entries in `tid_with_delta_cycles` will have their `is_prime` flag set to `true`. |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | Mutable slice of candidate tuples `(tid, delta_cycles, is_prime)`. On entry, `is_prime` is `false` for every element. On return, up to `prime_count` elements will have `is_prime` set to `true`. The slice is expected to be pre-sorted by `delta_cycles` descending by the caller ([apply_prime_threads](apply_prime_threads.md)) so that the hysteresis algorithm evaluates the most CPU-active threads first. |
| `prime_core_scheduler` | `&mut` [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | The persistent scheduler state. Provides access to per-thread [ThreadStats](../scheduler.rs/ThreadStats.md) (active streak counters, pinned CPU set IDs) and the [ConfigConstants](../config.rs/ConfigConstants.md) that define the hysteresis thresholds (`entry_threshold`, `keep_threshold`, `min_active_streak`). |

## Return value

None (`()`). The function communicates its results by mutating the `is_prime` flag in each element of `tid_with_delta_cycles`.

## Remarks

### Hysteresis algorithm

The function delegates entirely to `prime_core_scheduler.select_top_threads_with_hysteresis()`, passing:

- `pid` — to look up the per-process [ProcessStats](../scheduler.rs/ProcessStats.md).
- `tid_with_delta_cycles` — the candidate pool with mutable `is_prime` flags.
- `prime_count` — the maximum number of threads to select.
- A closure `|thread_stats| !thread_stats.pinned_cpu_set_ids.is_empty()` — this predicate returns `true` when a thread is *currently* prime (i.e., it has been pinned to a CPU set by [apply_prime_threads_promote](apply_prime_threads_promote.md) in a previous cycle).

The hysteresis algorithm (documented in detail at [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm)) uses two thresholds to prevent rapid promotion/demotion flapping:

| Threshold | Applied to | Condition |
|-----------|-----------|-----------|
| **keep_threshold** | Threads that are *already* prime (`pinned_cpu_set_ids` non-empty) | Thread stays prime if `delta_cycles >= keep_threshold% × max_delta_cycles` among candidates. This is a lower bar, providing stickiness so that a thread does not get demoted during brief CPU usage dips. |
| **entry_threshold** | Threads that are *not* currently prime | Thread becomes prime only if `delta_cycles >= entry_threshold% × max_delta_cycles` **and** `active_streak >= min_active_streak`. This is a higher bar, preventing ephemeral CPU bursts from triggering immediate promotion. |

The `entry_threshold` is always greater than or equal to `keep_threshold`, creating a dead zone that dampens oscillation. The `min_active_streak` requirement (from [ConfigConstants](../config.rs/ConfigConstants.md)) further stabilises selection by requiring sustained CPU activity over multiple consecutive polling cycles before a thread can enter prime status.

### Candidate ordering matters

Although the hysteresis algorithm examines cycle deltas to decide which threads qualify, the ordering of candidates in the input slice influences tie-breaking. Callers ([apply_prime_threads](apply_prime_threads.md)) sort by time delta descending before calling this function, ensuring that the most CPU-active threads are evaluated first. When multiple threads exceed the entry threshold but the number of qualifying threads exceeds `prime_count`, threads earlier in the slice (higher CPU usage) are preferred.

### Separation of concerns

`apply_prime_threads_select` performs *only* the selection step. It does not open any OS handles, call any Windows APIs, or modify any thread state. The actual pinning and priority changes are handled by [apply_prime_threads_promote](apply_prime_threads_promote.md) and [apply_prime_threads_demote](apply_prime_threads_demote.md) based on the `is_prime` flags set here. This separation allows the selection logic to be tested and reasoned about independently of OS side effects.

### Reuse for ideal processor rules

The same `select_top_threads_with_hysteresis` method is also used by [apply_ideal_processors](apply_ideal_processors.md) with a different "is currently assigned" predicate (`|ts| ts.ideal_processor.is_assigned`). The two call sites share the hysteresis algorithm but differ in what constitutes "already selected" status.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_prime_threads](apply_prime_threads.md) |
| Callees | [PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | None — pure Rust logic; no OS calls |
| Privileges | None |

## See Also

| Topic | Link |
|-------|------|
| Prime thread orchestration | [apply_prime_threads](apply_prime_threads.md) |
| Promotion after selection | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| Demotion of deselected threads | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| Hysteresis algorithm and scheduler state | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Per-thread stats (active streak, pinned IDs) | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Hysteresis constants | [ConfigConstants](../config.rs/ConfigConstants.md) |
| Ideal processor rule selection (reuses same algorithm) | [apply_ideal_processors](apply_ideal_processors.md) |
| Cycle-time prefetch (populates cached_cycles) | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| apply module overview | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd