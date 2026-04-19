# apply_prime_threads function (apply.rs)

Orchestrates prime thread scheduling for a process by identifying CPU-intensive threads and pinning them to designated "prime" CPUs for improved cache locality and performance. This is the top-level entry point for the prime thread subsystem, coordinating selection, promotion, and demotion phases.

## Syntax

```AffinityServiceRust/src/apply.rs#L708-718
pub fn apply_prime_threads<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

### `pid`

The process ID of the target process.

### `config`

A reference to the [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) containing prime thread scheduling settings including `prime_threads_cpus`, `prime_threads_prefixes`, `track_top_x_threads`, and `ideal_processor_rules`.

### `dry_run`

If `true`, the function records intended changes in `apply_config_result` without making any Windows API calls. Only the initial prime CPU description is logged.

### `current_mask`

A mutable reference to the current process affinity mask. Passed through to [`apply_prime_threads_promote`](apply_prime_threads_promote.md) which uses it to filter prime CPU sets against the process affinity.

### `process`

A reference to the [`ProcessEntry`](../process.rs/ProcessEntry.md) for the target process. Used to obtain the total thread count for candidate pool sizing.

### `threads`

A closure returning a reference to the `HashMap<u32, SYSTEM_THREAD_INFORMATION>` mapping thread IDs to their system thread information. Passed to the demotion phase for enumerating live threads.

### `prime_core_scheduler`

A mutable reference to the [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) that maintains per-thread state (cycle counts, pinned CPU sets, active streaks, handles) across iterations.

### `apply_config_result`

A mutable reference to [`ApplyConfigResult`](ApplyConfigResult.md) that accumulates change and error messages.

## Return value

This function does not return a value. Results are accumulated in `apply_config_result`.

## Remarks

### Algorithm

The prime thread algorithm proceeds in four stages:

1. **Candidate building** — Threads with nonzero cached cycle counts are collected and sorted by total CPU time delta (kernel + user time) in descending order. The candidate pool is sized at `max(prime_count × 4, cpu_count)` capped at the total thread count. Previously-pinned threads that have fallen out of the top candidates are re-added to ensure they can be properly demoted.

2. **Selection** — [`apply_prime_threads_select`](apply_prime_threads_select.md) uses hysteresis-based selection via [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md) to determine which threads qualify as prime. This prevents rapid flipping between prime and non-prime status.

3. **Promotion** — [`apply_prime_threads_promote`](apply_prime_threads_promote.md) pins newly selected prime threads to designated CPUs via `SetThreadSelectedCpuSets` and optionally boosts their thread priority.

4. **Demotion** — [`apply_prime_threads_demote`](apply_prime_threads_demote.md) removes CPU set pinning from threads that no longer qualify and restores their original thread priority.

### Prerequisites

[`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) must be called before this function to populate cached cycle counts and open thread handles. The cycle data is consumed from the `PrimeThreadScheduler`'s per-thread stats.

### Tracking mode

When `track_top_x_threads` is nonzero, the function enables tracking mode which stores `SYSTEM_THREAD_INFORMATION` snapshots for each thread via `last_system_thread_info`. A negative value for `track_top_x_threads` disables the prime scheduling phase while still allowing tracking.

### Early exit conditions

The function returns immediately if:
- `prime_threads_cpus` and `prime_threads_prefixes` are both empty **and** `track_top_x_threads` is zero.
- In `dry_run` mode, only a single change message describing the prime CPUs is recorded.

### Candidate pool sizing

The candidate pool intentionally oversamples relative to the number of prime slots (`prime_count × 4`) to provide the hysteresis algorithm with sufficient context about thread activity levels. This ensures that demotion candidates are always visible.

## Requirements

| Requirement | Value |
|---|---|
| Module | `apply` |
| Config type | [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| Callers | Service main loop (per-process enforcement) |
| Callees | [`apply_prime_threads_select`](apply_prime_threads_select.md), [`apply_prime_threads_promote`](apply_prime_threads_promote.md), [`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| Prerequisite | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| Windows API | None directly (delegated to sub-functions) |
| Privileges | `THREAD_SET_INFORMATION`, `THREAD_QUERY_INFORMATION` (via sub-functions) |

## See Also

| Topic | Link |
|---|---|
| Prime thread selection | [apply_prime_threads_select](apply_prime_threads_select.md) |
| Prime thread promotion | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| Prime thread demotion | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| Cycle time prefetching | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| Hysteresis scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Thread-level config | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Result accumulator | [ApplyConfigResult](ApplyConfigResult.md) |
| Module overview | [apply.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*