# apply_prime_threads_demote function (apply.rs)

Demotes threads that no longer qualify for prime status by removing their CPU set pinning and restoring their original thread priority.

## Syntax

```AffinityServiceRust/src/apply.rs#L953-961
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

`pid: u32`

The process identifier of the target process whose threads may be demoted.

`config: &ThreadLevelConfig`

The thread-level configuration containing the process name for logging. See [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md).

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

Lazy accessor returning all live thread IDs and their system thread information for the process. Used to enumerate threads that may need demotion.

`tid_with_delta_cycles: &[(u32, u64, bool)]`

Candidate thread list produced by earlier pipeline stages. Each tuple is `(thread_id, delta_cycles, is_prime)`. The `is_prime` flag indicates threads that should **remain** prime; all other threads with non-empty `pinned_cpu_set_ids` are demoted.

`prime_core_scheduler: &mut PrimeThreadScheduler`

The prime thread scheduler holding per-thread state including cached handles, pinned CPU set IDs, and original thread priority. See [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md).

`apply_config_result: &mut ApplyConfigResult`

Accumulator for change and error messages. See [ApplyConfigResult](ApplyConfigResult.md).

## Return value

This function does not return a value. Results are recorded in `apply_config_result`.

## Remarks

This function is the third and final stage of the prime thread scheduling pipeline, called by [apply_prime_threads](apply_prime_threads.md) after [apply_prime_threads_select](apply_prime_threads_select.md) and [apply_prime_threads_promote](apply_prime_threads_promote.md).

### Demotion logic

1. **Identify prime set** — Builds a `HashSet` of thread IDs currently marked `is_prime == true` in `tid_with_delta_cycles`.
2. **Enumerate live threads** — Collects all live thread IDs from the `threads()` accessor.
3. **Filter candidates** — For each live thread, skips threads that are still prime or that have empty `pinned_cpu_set_ids` (never promoted).
4. **Remove CPU set pinning** — Calls `SetThreadSelectedCpuSets` with an empty slice to clear any CPU set assignment, allowing the thread to run on any processor.
5. **Clear pinned state** — Always clears `pinned_cpu_set_ids` regardless of whether the `SetThreadSelectedCpuSets` call succeeded or failed. This prevents infinite retry loops that would spam error logs.
6. **Restore thread priority** — If an `original_priority` was saved during promotion, restores the thread to its previous priority via `SetThreadPriority`.

### Error resilience

The function intentionally clears `pinned_cpu_set_ids` even when `SetThreadSelectedCpuSets` fails. This design choice prioritizes avoiding log spam over guaranteed cleanup. If the API call fails (e.g., due to a dead thread handle), the thread will naturally lose its pinning when it exits or is recreated.

Errors are deduplicated through [log_error_if_new](log_error_if_new.md) so each unique `(pid, tid, operation, error_code)` combination is reported only once.

### Thread handle selection

Write handles are preferred over limited write handles. If both are invalid, the thread is skipped with an error logged.

### Priority restoration

The function uses `thread_stats.original_priority.take()` to consume the stored priority. This ensures the priority is only restored once, preventing double-restoration in subsequent cycles.

### Example change messages

On successful demotion:
- `Thread 1234 -> (demoted, start=ntdll.dll)`

On priority restoration failure, an error is logged via `log_error_if_new`.

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Called by** | [apply_prime_threads](apply_prime_threads.md) |
| **Calls** | `SetThreadSelectedCpuSets`, `SetThreadPriority`, `resolve_address_to_module`, [log_error_if_new](log_error_if_new.md) |
| **Win32 API** | [SetThreadSelectedCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [SetThreadPriority](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| **Privileges** | `THREAD_SET_INFORMATION` or `THREAD_SET_LIMITED_INFORMATION` on target threads |

## See Also

| Topic | Description |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | Orchestrator function for the prime thread scheduling pipeline |
| [apply_prime_threads_select](apply_prime_threads_select.md) | Selects which threads qualify for prime status |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | Promotes selected threads with CPU pinning and priority boost |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Manages per-thread scheduling state and hysteresis |
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change and error reporting |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*