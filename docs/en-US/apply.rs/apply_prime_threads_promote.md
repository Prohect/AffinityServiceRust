# apply_prime_threads_promote function (apply.rs)

Promotes threads selected as prime to dedicated high-performance CPUs via CPU Set pinning and optionally boosts their thread priority. This function is the "reward" phase of the prime thread scheduling algorithm — threads that have demonstrated sustained high CPU usage are given preferential access to specific processor cores for improved cache locality and reduced contention.

## Syntax

```AffinityServiceRust/src/apply.rs#L811-819
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

`pid: u32`

The process identifier of the target process.

`config: &ThreadLevelConfig`

Thread-level configuration containing:
- `prime_threads_cpus` — Default set of CPU indices to pin prime threads to.
- `prime_threads_prefixes` — A list of [`PrimePrefix`](../config.rs/PrimePrefix.md) rules that override the default CPU set and thread priority based on the thread's start address module name.
- `name` — Configuration rule name for logging.

`current_mask: &mut usize`

The process's current affinity mask, used to filter prime CPU indices. If the mask is non-zero, only CPUs present in the mask are used for pinning (via `filter_indices_by_mask`). This prevents assigning threads to CPUs outside the process's allowed affinity.

`tid_with_delta_cycles: &[(u32, u64, bool)]`

Slice of `(thread_id, delta_cycles, is_prime)` tuples produced by [`apply_prime_threads_select`](apply_prime_threads_select.md). Only entries where `is_prime == true` are processed by this function.

`prime_core_scheduler: &mut PrimeThreadScheduler`

The [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) instance that tracks per-thread state including cached handles, start addresses, pinned CPU set IDs, and original thread priorities.

`apply_config_result: &mut ApplyConfigResult`

Accumulator for change messages and errors. See [`ApplyConfigResult`](ApplyConfigResult.md).

## Return value

This function does not return a value. Results are accumulated in `apply_config_result`.

## Remarks

### Promotion flow per thread

For each thread marked as prime (`is_prime == true`):

1. **Skip if already pinned** — If `thread_stats.pinned_cpu_set_ids` is non-empty, the thread is already promoted and no action is taken.

2. **Resolve write handle** — Obtains a writable thread handle from the cached `ThreadHandle`, preferring the full-access handle over the limited handle. If no valid handle is available, the thread is skipped with an error logged via [`log_error_if_new`](log_error_if_new.md).

3. **Resolve start module** — Calls `resolve_address_to_module` with the thread's cached start address to determine which module (DLL/EXE) the thread entry point belongs to.

4. **Match against prefix rules** — Iterates `config.prime_threads_prefixes` and performs a case-insensitive prefix match on the start module name. The first matching [`PrimePrefix`](../config.rs/PrimePrefix.md) rule can override:
   - The set of CPUs to pin to (via `prefix.cpus`).
   - The thread priority to set (via `prefix.thread_priority`).
   
   If prefixes are configured but none match, the thread is **skipped entirely** (not promoted). If no prefixes are configured, the default `config.prime_threads_cpus` is used.

5. **Filter CPUs by affinity mask** — If `current_mask` is non-zero, the target CPU indices are filtered to only include CPUs allowed by the process affinity mask.

6. **Pin via `SetThreadSelectedCpuSets`** — Converts target CPU indices to CPU Set IDs and calls the Windows API to pin the thread. On success, records the pinned CPU set IDs in `thread_stats.pinned_cpu_set_ids` and logs a change message including the thread ID, promoted CPUs, cycle count, and start module.

7. **Boost thread priority** — After pinning, reads the current thread priority via `GetThreadPriority` and saves it in `thread_stats.original_priority` for later restoration during demotion. The new priority is determined as follows:
   - If the matched prefix specifies a `thread_priority`, that value is used directly (logged as "priority set").
   - Otherwise, the current priority is boosted by one level via `ThreadPriority::boost_one()` (logged as "priority boosted").
   
   Priority is only changed if the new value differs from the current value. The priority boost applies regardless of whether CPU pinning succeeded.

### Prefix matching details

- Matching is **case-insensitive** — both the module name and prefix are lowercased before comparison.
- Only the **first** matching prefix is used; subsequent prefixes are not evaluated.
- When prefixes are configured but the thread's module doesn't match any prefix, the thread is **excluded from promotion**. This allows targeted promotion of specific subsystems (e.g., render threads, audio threads).

### Error handling

All Windows API errors are reported through [`log_error_if_new`](log_error_if_new.md) with deduplication, preventing log spam for persistent errors. The operations tracked include:
- `Operation::OpenThread` — Invalid thread handle.
- `Operation::SetThreadSelectedCpuSets` — CPU Set pinning failure.
- `Operation::SetThreadPriority` — Priority boost failure.

### Change messages

On successful promotion, two change messages may be logged:
- `"Thread {tid} -> (promoted, [{cpus}], cycles={delta}, start={module})"` — CPU Set pinning.
- `"Thread {tid} -> (priority boosted: {old} -> {new})"` or `"Thread {tid} -> (priority set: {old} -> {new})"` — Priority change.

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Called by** | [`apply_prime_threads`](apply_prime_threads.md) |
| **Calls** | [`log_error_if_new`](log_error_if_new.md), `resolve_address_to_module`, `filter_indices_by_mask`, `cpusetids_from_indices`, `indices_from_cpusetids`, `format_cpu_indices` |
| **Win32 API** | [`SetThreadSelectedCpuSets`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [`GetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority), [`SetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority), [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| **Privileges** | Requires `THREAD_SET_INFORMATION` and `THREAD_QUERY_INFORMATION` (or limited variants) on target threads. |

## See Also

| | |
|---|---|
| [`apply_prime_threads`](apply_prime_threads.md) | Orchestrator that calls select → promote → demote. |
| [`apply_prime_threads_select`](apply_prime_threads_select.md) | Selects which threads to promote based on hysteresis. |
| [`apply_prime_threads_demote`](apply_prime_threads_demote.md) | Reverses promotion: unpins CPUs and restores priority. |
| [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) | Manages per-thread scheduling state and hysteresis logic. |
| [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) | Configuration for thread-level settings including prime prefixes. |
| [`PrimePrefix`](../config.rs/PrimePrefix.md) | Per-module prefix rule with optional CPU set and thread priority override. |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*