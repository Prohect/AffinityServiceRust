# apply_prime_threads_promote function (apply.rs)

Promotes newly-selected prime threads by pinning them to dedicated performance-core CPU sets via `SetThreadSelectedCpuSets` and optionally boosting their thread priority. For each thread marked as prime by [apply_prime_threads_select](apply_prime_threads_select.md), this function resolves the thread's start module, matches it against configured prefixes to determine which CPUs and priority to use, applies the CPU set pinning, and then adjusts the thread's priority — either to an explicitly configured level or by boosting one level above the current value.

## Syntax

```AffinityServiceRust/src/apply.rs#L824-833
pub fn apply_prime_threads_promote(
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used as a key into the scheduler's per-process stats map, for module resolution via [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md), and in formatted error/change messages. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | Parsed configuration rule for this process. The `prime_threads_cpus` field provides the default set of CPU indices for pinning; the `prime_threads_prefixes` field provides per-module overrides (each [PrimePrefix](../config.rs/PrimePrefix.md) can specify alternate CPUs and a thread priority). The `name` field is used in log messages. |
| `current_mask` | `&mut usize` | The process's current affinity mask, populated by [apply_affinity](apply_affinity.md). When non-zero, prime CPU indices are filtered through [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md) so that only CPUs within the process's hard affinity are used for pinning. This prevents assigning a thread to a CPU that the process is not allowed to run on. |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | Slice of `(thread_id, delta_cycles, is_prime)` tuples produced by [apply_prime_threads_select](apply_prime_threads_select.md). Only entries where `is_prime` is `true` are processed by this function. The `delta_cycles` value is included in the change message for observability but does not influence promotion logic. |
| `prime_core_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Persistent scheduler state. This function reads thread handles and start addresses from [ThreadStats](../scheduler.rs/ThreadStats.md), and writes `pinned_cpu_set_ids` and `original_priority` after successful promotion. |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages produced during promotion. |

## Return value

None (`()`). Results are communicated through mutations to `prime_core_scheduler` (updated `pinned_cpu_set_ids`, `original_priority`) and entries appended to `apply_config_result`.

## Remarks

### Algorithm

For each entry in `tid_with_delta_cycles` where `is_prime` is `true`:

1. **Skip already-pinned threads.** If `thread_stats.pinned_cpu_set_ids` is non-empty, the thread was promoted in a previous cycle and is still prime — nothing more to do.

2. **Resolve write handle.** The best available write handle is selected from the thread's cached [ThreadHandle](../winapi.rs/ThreadHandle.md) (`w_handle`, falling back to `w_limited_handle`). If both are invalid, an error is recorded via [log_error_if_new](log_error_if_new.md) with `Operation::OpenThread` and the thread is skipped.

3. **Resolve start module.** The thread's start address (populated earlier by [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)) is resolved to a module name via [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md). The module name is used for prefix matching and is included in the change message.

4. **Prefix matching.** If `config.prime_threads_prefixes` is non-empty, each [PrimePrefix](../config.rs/PrimePrefix.md) is tested against the lowercased start module. The first matching prefix determines:
   - The CPU set to use (`prefix.cpus` if present, otherwise `config.prime_threads_cpus`).
   - The thread priority to set (`prefix.thread_priority` if not `None`).

   If no prefix matches and the prefix list is non-empty, the thread is **skipped** — it does not qualify for promotion under any configured module prefix.

5. **Filter CPUs by affinity mask.** If `*current_mask` is non-zero, the resolved CPU indices are filtered through [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md) to remove CPUs outside the process's hard affinity. This prevents `SetThreadSelectedCpuSets` from specifying a CPU that the process is restricted from using.

6. **Convert to CPU set IDs.** The filtered CPU indices are translated to Windows CPU set IDs via [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md). If the result is empty (no valid CPU set IDs resolved), the thread is skipped.

7. **Pin thread.** `SetThreadSelectedCpuSets` is called with the resolved CPU set IDs. On success, the IDs are stored in `thread_stats.pinned_cpu_set_ids` (so the thread is recognised as pinned in future cycles) and a change message is recorded. On failure, the Win32 error code is routed through [log_error_if_new](log_error_if_new.md) with `Operation::SetThreadSelectedCpuSets`.

8. **Read current priority.** `GetThreadPriority` is called to read the thread's current priority level. If the call returns `THREAD_PRIORITY_ERROR_RETURN` (`0x7FFFFFFF`), the priority boost step is skipped entirely.

9. **Save original priority.** The current priority (as a [ThreadPriority](../priority.rs/ThreadPriority.md) enum) is stored in `thread_stats.original_priority`. This value is later used by [apply_prime_threads_demote](apply_prime_threads_demote.md) to restore the thread's priority when it is demoted.

10. **Boost priority.** The new priority is determined by one of two paths:
    - If a matching prefix specifies a non-`None` `thread_priority`, that explicit value is used (*priority set*).
    - Otherwise, the current priority is boosted by one level via `ThreadPriority::boost_one()` (*priority boosted*). For example, `Normal` becomes `AboveNormal`, `AboveNormal` becomes `Highest`.

    If the new priority equals the current priority (e.g. the thread is already at `TimeCritical` and cannot be boosted further), no set call is made. Otherwise, `SetThreadPriority` is called. On success, a change message is recorded; on failure, the error is logged via [log_error_if_new](log_error_if_new.md) with `Operation::SetThreadPriority`.

### Change message format

Two change messages are emitted per successfully promoted thread:

**CPU set pinning:**
```/dev/null/example.txt#L1
Thread 1234 -> (promoted, [4,5], cycles=98000, start=ntdll.dll!RtlUserThreadStart)
```

**Priority adjustment:**
```/dev/null/example.txt#L1-2
Thread 1234 -> (priority boosted: Normal -> AboveNormal)
Thread 1234 -> (priority set: Normal -> Highest)
```

The word "boosted" is used when the priority was auto-incremented by one level; "set" is used when an explicit priority was configured via a prefix rule.

### Prefix matching details

Prefix matching is case-insensitive (`to_lowercase()` on both the module name and the prefix). The first matching prefix wins — order in the configuration file matters. If a prefix specifies `cpus: None`, the default `config.prime_threads_cpus` is used. If a prefix specifies `thread_priority: None`, the auto-boost-by-one logic applies.

When `config.prime_threads_prefixes` is empty (no prefix rules), all prime-selected threads are promoted using `config.prime_threads_cpus` and the auto-boost priority logic.

### Affinity mask interaction

The `current_mask` filtering step is critical for correctness. If a process has a hard affinity mask set by [apply_affinity](apply_affinity.md) that excludes some of the configured prime CPUs, those CPUs must not appear in the `SetThreadSelectedCpuSets` call. Without this filter, the API would succeed but the thread would not actually run on the specified CPUs, potentially causing confusing behaviour. When `*current_mask` is `0` (no affinity was queried or the process has no affinity constraint), the filter is bypassed and all configured prime CPUs are used.

### Idempotency

The function is idempotent within a single cycle: threads that are already pinned (`pinned_cpu_set_ids` is non-empty) are skipped. Across cycles, a thread that remains prime is not re-promoted because its `pinned_cpu_set_ids` persists in [ThreadStats](../scheduler.rs/ThreadStats.md) until cleared by [apply_prime_threads_demote](apply_prime_threads_demote.md).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_prime_threads](apply_prime_threads.md) |
| Callees | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md), [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md), [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md), [indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md), [log_error_if_new](log_error_if_new.md) |
| Win32 API | [`SetThreadSelectedCpuSets`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [`GetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority), [`SetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| Privileges | `THREAD_SET_LIMITED_INFORMATION` (for `SetThreadSelectedCpuSets`), `THREAD_SET_INFORMATION` (for `SetThreadPriority`), `THREAD_QUERY_LIMITED_INFORMATION` (for `GetThreadPriority`). The service typically holds `SeDebugPrivilege` which grants all of these. |

## See Also

| Topic | Link |
|-------|------|
| Prime thread orchestration | [apply_prime_threads](apply_prime_threads.md) |
| Hysteresis-based selection | [apply_prime_threads_select](apply_prime_threads_select.md) |
| Demotion (inverse operation) | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| Cycle-time prefetch (populates start addresses and handles) | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| Prefix configuration model | [PrimePrefix](../config.rs/PrimePrefix.md) |
| Thread priority enum and boost logic | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| CPU set ID translation | [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md) |
| Affinity mask filtering | [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md) |
| Module name resolution | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| Scheduler state model | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), [ThreadStats](../scheduler.rs/ThreadStats.md) |
| apply module overview | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd