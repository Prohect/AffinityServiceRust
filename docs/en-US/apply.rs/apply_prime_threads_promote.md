# apply_prime_threads_promote function (apply.rs)

The `apply_prime_threads_promote` function pins newly-selected prime threads to designated high-performance CPUs via `SetThreadSelectedCpuSets` and optionally boosts their thread priority. For each thread marked as prime in the selection results, the function resolves the thread's start address to a module name, matches it against configured prefix rules to determine which CPU set and priority to apply, and then issues the appropriate Windows API calls. This is the promotion phase of the prime-thread scheduling algorithm.

## Syntax

```AffinityServiceRust/src/apply.rs#L810-822
pub fn apply_prime_threads_promote(
    pid: u32,
    config: &ThreadLevelConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Used for thread-stats lookup, error deduplication, and log messages. |
| `config` | `&ThreadLevelConfig` | The thread-level configuration containing `prime_threads_cpus` (the default set of CPU indices for prime threads), `prime_threads_prefixes` (a list of prefix-matching rules that can override the CPU set and thread priority per module), and `name` (the config rule name used in log messages). |
| `current_mask` | `&mut usize` | The current process affinity mask. When non-zero, prime CPU indices are filtered through this mask via `filter_indices_by_mask` so that only CPUs within the process's affinity are used. This prevents assigning threads to CPUs the process cannot execute on. |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | A slice of tuples containing the thread ID, the delta cycle count since the last measurement, and a boolean indicating whether the thread was selected as prime (`true`) or not (`false`). Only entries where `is_prime == true` are processed by this function. |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | The mutable prime-thread scheduler state. The function reads and updates per-thread stats including `handle`, `pinned_cpu_set_ids`, `start_address`, and `original_priority`. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during execution. |

## Return value

This function does not return a value. All outcomes are communicated through mutations to `prime_core_scheduler` and appended entries in `apply_config_result`.

## Remarks

### Algorithm

For each `(tid, delta_cycles, is_prime)` tuple in `tid_with_delta_cycles` where `is_prime` is `true`:

1. **Skip already-pinned threads**: If `thread_stats.pinned_cpu_set_ids` is non-empty, the thread is already promoted and is skipped. This prevents re-applying the CPU set and priority boost on every apply cycle.

2. **Handle resolution**: The function retrieves the thread's write handle from `thread_stats.handle` using the `.as_ref()` access pattern. It prefers `w_handle` over `w_limited_handle`. If both are invalid, an error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::OpenThread` and the thread is skipped.

3. **Module prefix matching**: The thread's start address is resolved to a module name via `resolve_address_to_module`. If `config.prime_threads_prefixes` is non-empty, the module name is compared (case-insensitive) against each prefix rule's `prefix` field. The first matching prefix determines:
   - An alternative CPU set (`prefix.cpus`) to use instead of `config.prime_threads_cpus`.
   - A specific thread priority (`prefix.thread_priority`) to set instead of the default one-level boost.
   If no prefix matches and `prime_threads_prefixes` is non-empty, the thread is skipped entirely (only threads matching a prefix rule are promoted).

4. **Affinity filtering**: If `*current_mask` is non-zero, the selected prime CPU indices are filtered through the process affinity mask via `filter_indices_by_mask`. This ensures only CPUs the process is allowed to use are included in the CPU Set.

5. **CPU Set application**: The filtered CPU indices are converted to CPU Set IDs via `cpusetids_from_indices`. If the resulting list is non-empty, `SetThreadSelectedCpuSets` is called. On success, `thread_stats.pinned_cpu_set_ids` is updated and a change message is recorded:
   `"Thread <tid> -> (promoted, [<cpus>], cycles=<delta>, start=<module>)"`
   On failure, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::SetThreadSelectedCpuSets`.

6. **Priority boost**: After CPU set pinning (regardless of its success), the function reads the current thread priority via `GetThreadPriority`. If the read succeeds (return value is not `0x7FFFFFFF`):
   - The current priority is saved in `thread_stats.original_priority` so it can be restored on demotion.
   - The new priority is determined by: (a) the prefix rule's `thread_priority` if explicitly set and not `ThreadPriority::None`, or (b) `current_priority.boost_one()` which increments the priority by one level.
   - If the new priority differs from the current, `SetThreadPriority` is called. On success, a change message is recorded:
     `"Thread <tid> -> (priority set: <old> -> <new>)"` or `"Thread <tid> -> (priority boosted: <old> -> <new>)"`.
   - On failure, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::SetThreadPriority`.

### Edge cases

- If `prime_threads_prefixes` is empty, all prime-selected threads are promoted using `config.prime_threads_cpus` as the CPU set and a one-level priority boost.
- If `prime_threads_prefixes` is non-empty and no prefix matches a given thread's start module, that thread is **not** promoted even if it was selected as prime. This allows fine-grained control over which modules receive prime treatment.
- If `current_mask` is `0` (e.g., affinity was never queried), no filtering is applied and the full `prime_threads_cpus` list is used.
- If `cpusetids_from_indices` returns an empty list (e.g., all prime CPUs were filtered out by the affinity mask), no CPU set is applied and the priority boost is also skipped for that thread.
- A `GetThreadPriority` return value of `0x7FFFFFFF` (`THREAD_PRIORITY_ERROR_RETURN`) indicates failure; the priority boost is silently skipped without logging an error.

### Interaction with demotion

Threads promoted by this function are later subject to demotion by [`apply_prime_threads_demote`](apply_prime_threads_demote.md) if they fall out of the prime selection in a subsequent apply cycle. The `pinned_cpu_set_ids` field acts as the promotion marker: non-empty means promoted. The `original_priority` field is used during demotion to restore the thread's priority to its pre-promotion level.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | `SetThreadSelectedCpuSets`, `GetThreadPriority`, `SetThreadPriority`, `GetLastError` |
| Callers | [`apply_prime_threads`](apply_prime_threads.md) |
| Callees | [`log_error_if_new`](log_error_if_new.md), `winapi::resolve_address_to_module`, `winapi::filter_indices_by_mask`, `winapi::cpusetids_from_indices`, `winapi::indices_from_cpusetids`, `config::format_cpu_indices`, `error_codes::error_from_code_win32`, `ThreadPriority::from_win_const`, `ThreadPriority::boost_one`, `ThreadPriority::to_thread_priority_struct` |
| Privileges | Requires thread handles with `THREAD_SET_INFORMATION` or `THREAD_SET_LIMITED_INFORMATION` (write) and `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` (read for `GetThreadPriority`). |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| apply_prime_threads_demote | [`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |
| ThreadPriority | [`priority.rs/ThreadPriority`](../priority.rs/ThreadPriority.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*