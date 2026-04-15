# apply_prime_threads function (apply.rs)

The `apply_prime_threads` function orchestrates the prime-thread scheduling pipeline for a single process. It identifies the most CPU-intensive threads by sorting them according to their cycle-count deltas, selects the top candidates using hysteresis, promotes winners to designated high-performance CPUs via CPU Sets with optional priority boosts, and demotes threads that no longer qualify. The function also manages thread handle lifecycle by closing handles for threads that have exited the process.

## Syntax

```AffinityServiceRust/src/apply.rs#L696-712
#[allow(clippy::too_many_arguments)]
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

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Used for scheduler lookups, error deduplication, and log messages. |
| `config` | `&ThreadLevelConfig` | The thread-level configuration containing `prime_threads_cpus` (CPU indices designated for prime threads), `prime_threads_prefixes` (module-prefix matching rules with optional per-prefix CPU overrides), `track_top_x_threads` (the tracking count; negative values disable prime scheduling), and `name` (the human-readable config rule name). |
| `dry_run` | `bool` | When `true`, the function records what *would* change in `apply_config_result` without calling any Windows APIs to modify state. Only a synthetic change message listing the prime CPU set is emitted. When `false`, the full select/promote/demote pipeline is executed. |
| `current_mask` | `&mut usize` | The current process affinity mask, as determined by a prior call to [`apply_affinity`](apply_affinity.md). Passed through to [`apply_prime_threads_promote`](apply_prime_threads_promote.md) to filter prime CPU indices against the live affinity mask. A value of `0` means no affinity mask is active and no filtering is applied. |
| `process` | `&'a ProcessEntry` | The process snapshot entry (immutable borrow tied to lifetime `'a`), used to obtain the thread count via `process.thread_count()` for sizing the candidate pool. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A lazy closure that returns a reference to the thread-information map on demand. The closure is invoked only when the function actually needs to enumerate threads, avoiding the cost of building the map when early-exit conditions are met (e.g., dry-run mode or disabled prime scheduling). The returned map keys are thread IDs and values are `SYSTEM_THREAD_INFORMATION` snapshots from the most recent system process information query. |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | The mutable prime-thread scheduler state that tracks per-thread statistics (cached cycles, last cycles, active streaks, pinned CPU set IDs, handles, etc.) across apply cycles. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during the select/promote/demote phases. |

## Return value

This function does not return a value. All outcomes are communicated through the `prime_core_scheduler` state mutations and the `apply_config_result` accumulator.

## Remarks

### Gate conditions

The function determines whether to run the prime-thread pipeline based on two flags:

- **`do_prime`**: `true` when either `prime_threads_cpus` or `prime_threads_prefixes` is non-empty, **and** `track_top_x_threads >= 0`. A negative `track_top_x_threads` value explicitly disables prime-thread scheduling.
- **`has_tracking`**: `true` when `track_top_x_threads != 0`. Tracking mode records per-thread system thread information (`last_system_thread_info`) for external consumption even when prime scheduling is disabled.

If both `do_prime` and `has_tracking` are `false`, the function returns immediately. In dry-run mode with `has_prime_cpus` true, a change message of the form `"Prime CPUs: -> [<cpu_list>]"` is recorded and the function returns without further processing.

### Algorithm

1. **Build candidate list**: For each thread in `threads()` (invoking the lazy closure), the function looks up the thread's cached cycle count in the scheduler. Only threads with `cached_cycles > 0` (i.e., threads whose cycles were successfully prefetched by [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md)) are included. If `has_tracking` is enabled, the thread's `last_system_thread_info` is also updated. The list is sorted by time delta (cached total time minus last total time) in descending order.

2. **Size the candidate pool**: The pool size is `max(prime_count * 4, cpu_count)` capped at the process thread count. This ensures enough candidates are considered to account for threads that may briefly spike in activity. Previously-pinned threads that have dropped out of the top candidates are also re-added to ensure they can be properly demoted.

3. **Compute cycle deltas**: For each candidate thread, the delta is `cached_cycles - last_cycles` (saturating subtraction). The third element of each tuple (`bool`) is initialized to `false` and will be set to `true` by the selection phase for threads that qualify as prime.

4. **Select** ([`apply_prime_threads_select`](apply_prime_threads_select.md)): Applies hysteresis-based selection to mark the top threads as prime. Threads already pinned receive a lower "keep" threshold; new candidates must exceed a higher "entry" threshold and have a sufficient active streak.

5. **Promote** ([`apply_prime_threads_promote`](apply_prime_threads_promote.md)): For each newly-selected prime thread, pins it to the designated CPUs via `SetThreadSelectedCpuSets` and optionally boosts its thread priority.

6. **Demote** ([`apply_prime_threads_demote`](apply_prime_threads_demote.md)): For each thread that was previously pinned but is no longer selected as prime, removes the CPU set pinning (by setting an empty CPU set) and restores its original thread priority.

7. **Handle cleanup**: After the promote/demote cycle, the function builds a set of live thread IDs from the candidate list. For any thread in the scheduler's state that is not in the live set, the thread handle is dropped (which closes the underlying OS handles via `ThreadHandle`'s `Drop` implementation). This prevents handle leaks for threads that have exited the process.

### Edge cases

- If `prime_threads_cpus` is empty but `prime_threads_prefixes` is non-empty, the function still runs the pipeline. Each prefix entry may carry its own `cpus` override, and if none match a particular thread, that thread will not be promoted.
- If `track_top_x_threads` is negative, prime scheduling is disabled but thread tracking may still occur if the absolute value triggers `has_tracking`.
- Threads whose cycles were not prefetched (e.g., because handle acquisition failed in `prefetch_all_thread_cycles`) have `cached_cycles == 0` and are excluded from the candidate list entirely.
- Previously-pinned threads that no longer appear in the top candidates are re-injected into the candidate list with their last known cycle delta. This ensures the demotion phase can process them even if their CPU activity has dropped to zero.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | None directly; delegates to [`apply_prime_threads_promote`](apply_prime_threads_promote.md) and [`apply_prime_threads_demote`](apply_prime_threads_demote.md) which call `SetThreadSelectedCpuSets`, `SetThreadPriority`, `GetThreadPriority`, and `GetLastError`. |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that iterates matched processes |
| Callees | [`apply_prime_threads_select`](apply_prime_threads_select.md), [`apply_prime_threads_promote`](apply_prime_threads_promote.md), [`apply_prime_threads_demote`](apply_prime_threads_demote.md), `PrimeThreadScheduler::get_thread_stats`, `PrimeThreadScheduler::set_tracking_info`, `get_cpu_set_information`, `format_cpu_indices` |
| Privileges | Requires thread handles with `THREAD_SET_INFORMATION` or `THREAD_SET_LIMITED_INFORMATION` (write) and `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` (read). These are obtained through the `PrimeThreadScheduler` cached handles. |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| apply_prime_threads_promote | [`apply_prime_threads_promote`](apply_prime_threads_promote.md) |
| apply_prime_threads_demote | [`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| apply_ideal_processors | [`apply_ideal_processors`](apply_ideal_processors.md) |
| update_thread_stats | [`update_thread_stats`](update_thread_stats.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |
| ProcessEntry | [`process.rs/ProcessEntry`](../process.rs/ProcessEntry.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*