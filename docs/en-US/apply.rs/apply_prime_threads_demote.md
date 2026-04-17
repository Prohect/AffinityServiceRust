# apply_prime_threads_demote function (apply.rs)

The `apply_prime_threads_demote` function removes CPU-set pinning and restores original thread priority for threads that no longer qualify as prime. It iterates over all live threads that were previously pinned (i.e., have non-empty `pinned_cpu_set_ids`) but are not in the current prime selection set, clears their CPU set assignment by calling `SetThreadSelectedCpuSets` with an empty slice, and restores the thread's priority to the value saved in `original_priority` during promotion. This is the demotion phase of the prime-thread scheduling algorithm.

## Syntax

```AffinityServiceRust/src/apply.rs#L952-965
pub fn apply_prime_threads_demote<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Used for thread-stats lookup, error deduplication, and log messages. |
| `config` | `&ThreadLevelConfig` | The thread-level configuration. The `name` field is used in error log messages passed to `log_error_if_new` and `get_thread_handle`. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A lazy closure that returns a reference to a map of thread IDs to their `SYSTEM_THREAD_INFORMATION` snapshots from the most recent system process information query. The closure is invoked once to obtain the thread map, whose keys define the set of live threads to iterate over. |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | A slice of tuples containing the thread ID, the delta cycle count, and a boolean indicating whether the thread was selected as prime (`true`) or not (`false`). The function builds a `HashSet` of prime thread IDs from entries where `is_prime == true` and uses it to determine which threads should **not** be demoted. |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | The mutable prime-thread scheduler state. The function reads and updates per-thread stats including `handle`, `pinned_cpu_set_ids`, and `original_priority`. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during execution. |

## Return value

This function does not return a value. All outcomes are communicated through mutations to `prime_core_scheduler` and appended entries in `apply_config_result`.

## Remarks

### Algorithm

1. **Build prime set**: A `HashSet<u32>` of thread IDs is constructed from `tid_with_delta_cycles` entries where `is_prime == true`. These threads are currently selected as prime and should not be demoted.

2. **Collect live thread IDs**: The closure `threads()` is invoked to obtain the thread map, and its keys are collected into a `List<[u32; TIDS_CAPED]>` representing all live threads in the process.

3. **Iterate live threads**: For each live thread ID, the function retrieves its `thread_stats` from the scheduler. A thread is skipped (not demoted) if:
   - It is in the `prime_set` (still selected as prime), or
   - Its `pinned_cpu_set_ids` is empty (it was never promoted).

4. **Handle resolution**: The function retrieves the thread's write handle from `thread_stats.handle` using the `.as_ref()` access pattern (rather than the previous `ref` binding style). It prefers `w_handle` over `w_limited_handle`. If both are invalid, an error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::OpenThread` and the thread is skipped. If no handle exists at all, the thread is silently skipped.

5. **Remove CPU set pinning**: `SetThreadSelectedCpuSets` is called with an empty slice (`&[]`) to clear the thread's CPU set assignment, returning it to the process's default scheduling behavior. On success, a change message is recorded:
   `"Thread <tid> -> (demoted, start=<module>)"`
   where `<module>` is resolved from the thread's start address via `resolve_address_to_module`. On failure, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::SetThreadSelectedCpuSets`.

6. **Clear pinned state unconditionally**: Regardless of whether `SetThreadSelectedCpuSets` succeeded or failed, `thread_stats.pinned_cpu_set_ids` is cleared. This is a deliberate design decision to prevent infinite retry loops that would spam the error log on every apply cycle for a thread whose CPU set cannot be cleared (e.g., due to insufficient access rights or the thread having already exited).

7. **Restore original priority**: If `thread_stats.original_priority` contains a saved priority value (set during promotion by [`apply_prime_threads_promote`](apply_prime_threads_promote.md)), `SetThreadPriority` is called to restore it. The `original_priority` field is consumed via `.take()` so it is only restored once. On failure, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::SetThreadPriority`. The error message references `"RESTORE_SET_THREAD_PRIORITY"` to distinguish it from the priority-set errors during promotion.

### Edge cases

- If `SetThreadSelectedCpuSets` fails (e.g., the thread has exited between the snapshot and the API call), the `pinned_cpu_set_ids` are still cleared to avoid retrying the failed call on every subsequent apply cycle. The priority restore is still attempted.
- If `original_priority` is `None` (e.g., `GetThreadPriority` failed during promotion and no priority was saved), no priority restoration is attempted and the thread simply has its CPU set cleared.
- Threads that exist in the scheduler's state but are not present in the map returned by `threads()` (i.e., threads that have exited) are not iterated by this function because the iteration is over the map's keys. Stale entries are cleaned up separately by the handle-cleanup logic in [`apply_prime_threads`](apply_prime_threads.md).
- A thread that was demoted but whose `SetThreadPriority` call fails will have its `original_priority` consumed (set to `None` by `.take()`), so the restore will not be re-attempted on the next cycle.

### Interaction with promotion

This function undoes the work of [`apply_prime_threads_promote`](apply_prime_threads_promote.md). The `pinned_cpu_set_ids` field serves as the flag: non-empty means promoted, empty means not promoted. The `original_priority` field bridges the two phases: set during promotion, consumed during demotion.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | `SetThreadSelectedCpuSets`, `SetThreadPriority`, `GetLastError` |
| Callers | [`apply_prime_threads`](apply_prime_threads.md) |
| Callees | [`log_error_if_new`](log_error_if_new.md), `winapi::resolve_address_to_module`, `error_codes::error_from_code_win32`, `ThreadPriority::to_thread_priority_struct` |
| Privileges | Requires thread handles with `THREAD_SET_INFORMATION` or `THREAD_SET_LIMITED_INFORMATION` (write). |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| apply_prime_threads_promote | [`apply_prime_threads_promote`](apply_prime_threads_promote.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |
| ThreadPriority | [`priority.rs/ThreadPriority`](../priority.rs/ThreadPriority.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*