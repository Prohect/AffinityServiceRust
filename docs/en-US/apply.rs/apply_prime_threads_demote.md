# apply_prime_threads_demote function (apply.rs)

Demotes threads that no longer qualify for prime status by removing their per-thread CPU set pinning and restoring their original thread priority. This is the final stage of the prime-thread scheduling pipeline, invoked by [apply_prime_threads](apply_prime_threads.md) after [apply_prime_threads_select](apply_prime_threads_select.md) and [apply_prime_threads_promote](apply_prime_threads_promote.md) have completed.

## Syntax

```AffinityServiceRust/src/apply.rs#L966-977
pub fn apply_prime_threads_demote(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used as a key into the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) state maps and for error logging. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | Parsed configuration rule for this process. The `name` field is used in error messages. |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | Snapshot entry for the target process. Provides the live thread list — only threads that still exist in the snapshot are candidates for demotion. |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | Slice of `(thread_id, delta_cycles, is_prime)` tuples produced by [apply_prime_threads_select](apply_prime_threads_select.md). The `is_prime` flag indicates whether the thread was selected for prime status in this cycle. Threads with `is_prime == true` are skipped; threads with `is_prime == false` that still have a non-empty `pinned_cpu_set_ids` in their [ThreadStats](../scheduler.rs/ThreadStats.md) are demoted. |
| `prime_core_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Persistent scheduler state. The function reads and mutates per-thread `pinned_cpu_set_ids`, `original_priority`, and `handle` fields in [ThreadStats](../scheduler.rs/ThreadStats.md). |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages produced during demotion. |

## Return value

None (`()`). Results are communicated through `apply_config_result` and side effects on `prime_core_scheduler`.

## Remarks

### Algorithm

1. **Build prime set** — A `HashSet<u32>` of thread IDs that were selected as prime (`is_prime == true`) is constructed from the `tid_with_delta_cycles` slice for O(1) lookup.

2. **Enumerate live threads** — The live thread IDs are collected from `process.get_threads()`. Only threads that exist in the current snapshot are iterated, preventing the function from operating on stale thread IDs.

3. **Identify demotion candidates** — For each live thread, the function checks two conditions:
   - The thread is **not** in the prime set (it was not selected this cycle).
   - The thread's `pinned_cpu_set_ids` in [ThreadStats](../scheduler.rs/ThreadStats.md) is **non-empty** (it was previously promoted and pinned).

   If both conditions are met, the thread is a demotion candidate.

4. **Unpin CPU set** — `SetThreadSelectedCpuSets` is called with an empty slice (`&[]`) to clear the per-thread CPU set assignment, returning the thread to the process-default scheduling behaviour. A valid write handle (`w_handle`, falling back to `w_limited_handle`) is required.

5. **Clear pinned state** — `thread_stats.pinned_cpu_set_ids.clear()` is called **regardless of whether the unpin succeeded or failed**. This is a deliberate design choice to prevent infinite retry loops: if the unpin call fails (e.g. because the thread exited between enumeration and the API call), the thread would otherwise be retried on every subsequent cycle, generating repeated error log entries. Clearing the state ensures the function moves on.

6. **Restore original priority** — If the thread's `original_priority` field in [ThreadStats](../scheduler.rs/ThreadStats.md) is `Some`, the original [ThreadPriority](../priority.rs/ThreadPriority.md) is restored via `SetThreadPriority`. The field is consumed by `Option::take()` to prevent double-restore on subsequent cycles. If the priority restoration fails, the error is logged but the thread is still considered demoted.

7. **Log** — On successful unpin, a change message is recorded:

   `"Thread 5678 -> (demoted, start=ntdll.dll)"`

   The start module name is resolved from the thread's `start_address` via [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md).

### Error handling

Both the CPU set clearing and priority restoration calls route failures through [log_error_if_new](log_error_if_new.md):

| Operation | `Operation` variant | Common failure |
|-----------|---------------------|----------------|
| `SetThreadSelectedCpuSets` (clear) | `Operation::SetThreadSelectedCpuSets` | Thread exited or handle invalid |
| `SetThreadPriority` (restore) | `Operation::SetThreadPriority` | Thread exited or access denied |

The error for a failed priority restore is logged under the function name `apply_prime_threads_promote` with the operation tag `[RESTORE_SET_THREAD_PRIORITY]`. Despite the function name in the message, this code path resides in `apply_prime_threads_demote`. This is intentional — the promote function originally owned priority, so the "restore" counterpart retains the naming for traceability in logs.

### Relationship to apply_prime_threads_promote

Promotion and demotion are inverses:

| Action | [apply_prime_threads_promote](apply_prime_threads_promote.md) | apply_prime_threads_demote |
|--------|---------------------------------------------------------------|----------------------------|
| CPU set | Pins thread to prime CPU set IDs | Clears thread CPU set (empty slice) |
| Priority | Boosts thread priority (configured or auto +1 level) | Restores `original_priority` saved during promotion |
| State tracking | Sets `pinned_cpu_set_ids`, stores `original_priority` | Clears `pinned_cpu_set_ids`, takes `original_priority` |

### Thread handle selection

The function traverses handle tiers in the same order as other apply functions:

1. Prefer `w_handle` (full `THREAD_SET_INFORMATION` access).
2. Fall back to `w_limited_handle` (`THREAD_SET_LIMITED_INFORMATION`).
3. If both are invalid, log an error via [log_error_if_new](log_error_if_new.md) and skip the thread.
4. If no handle exists at all (`thread_stats.handle` is `None`), the thread is silently skipped.

### Edge cases

- **Thread exits between select and demote** — The function only iterates live threads from the snapshot, so a thread that exited mid-cycle is naturally excluded. However, if the snapshot is stale and the thread exited between snapshot time and the `SetThreadSelectedCpuSets` call, the API call may fail; this is handled by the unconditional `pinned_cpu_set_ids.clear()`.
- **No original priority recorded** — If `original_priority` is `None` (e.g. the thread was promoted before the priority-tracking feature was added, or `GetThreadPriority` failed during promotion), no priority restoration is attempted.
- **Thread already demoted** — A thread with empty `pinned_cpu_set_ids` is skipped immediately, so calling this function multiple times for the same thread is harmless.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_prime_threads](apply_prime_threads.md) |
| Callees | [log_error_if_new](log_error_if_new.md), [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| Win32 API | [`SetThreadSelectedCpuSets`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [`SetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| Privileges | `THREAD_SET_LIMITED_INFORMATION` (minimum for `SetThreadSelectedCpuSets`), `THREAD_SET_INFORMATION` (for `SetThreadPriority`). The service typically holds `SeDebugPrivilege` which grants both. |

## See Also

| Topic | Link |
|-------|------|
| Prime thread orchestrator | [apply_prime_threads](apply_prime_threads.md) |
| Prime thread selection (hysteresis) | [apply_prime_threads_select](apply_prime_threads_select.md) |
| Prime thread promotion (inverse operation) | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| Cycle-time prefetch | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| Scheduler state model | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Per-thread stats | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Thread priority enum | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Error deduplication | [log_error_if_new](log_error_if_new.md) |
| apply module overview | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd