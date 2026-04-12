# apply_prime_threads_promote function (apply.rs)

Promotes selected threads to prime status by pinning them to dedicated CPUs via CPU Sets and optionally boosting their thread priority.

## Syntax

```rust
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

`pid`

The process ID of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing `prime_threads_cpus`, `prime_threads_prefixes`, and related settings.

`current_mask`

Mutable reference to the process's current CPU affinity mask. Used to filter prime CPU indices so that only CPUs within the process's affinity are assigned.

`tid_with_delta_cycles`

Slice of `(thread_id, delta_cycles, is_prime)` tuples produced by [apply_prime_threads_select](apply_prime_threads_select.md). Only entries where `is_prime` is `true` are processed.

`prime_core_scheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that holds per-thread state including handles, `pinned_cpu_set_ids`, `start_address`, and `original_priority`.

`apply_config_result`

Mutable reference to an [ApplyConfigResult](ApplyConfigResult.md) that accumulates change and error messages.

## Return value

This function does not return a value. Results are recorded in `apply_config_result`.

## Remarks

### Algorithm

For each thread marked as prime (`is_prime == true`) that is not already pinned (i.e., `pinned_cpu_set_ids` is empty):

1. **Resolve start module** — The thread's start address is resolved to a module name via `resolve_address_to_module()`.

2. **Match against prefix rules** — If `config.prime_threads_prefixes` is non-empty, the start module is compared (case-insensitive) against each prefix entry. On match:
   - The prefix's dedicated CPU list overrides `config.prime_threads_cpus`.
   - The prefix's explicit `thread_priority` is used instead of auto-boost.
   - If no prefix matches, the thread is **skipped** (not promoted).

3. **Filter CPUs by affinity mask** — If `current_mask` is non-zero, the prime CPU indices are filtered through `filter_indices_by_mask()` so only CPUs within the process's affinity are assigned.

4. **Assign CPU Set** — Calls `SetThreadSelectedCpuSets` with the CPU Set IDs derived from the filtered CPU indices. On success, stores the assigned CPU Set IDs in `thread_stats.pinned_cpu_set_ids`.

5. **Boost thread priority** — After CPU pinning:
   - Reads the current thread priority via `GetThreadPriority`.
   - Saves it as `thread_stats.original_priority` for later restoration by [apply_prime_threads_demote](apply_prime_threads_demote.md).
   - If a prefix rule specified an explicit `thread_priority`, that value is set directly ("priority set").
   - Otherwise, the priority is boosted by one level via `ThreadPriority::boost_one()` ("priority boosted").
   - The priority is only changed if the new value differs from the current value.

### Change messages logged

- `"Thread {tid} -> (promoted, [{cpus}], cycles={delta}, start={module})"` — on successful CPU Set assignment.
- `"Thread {tid} -> (priority set: {old} -> {new})"` — when an explicit prefix priority is applied.
- `"Thread {tid} -> (priority boosted: {old} -> {new})"` — when auto-boost is applied.

### Error handling

- Invalid thread handles are logged via [log_error_if_new](log_error_if_new.md) with `Operation::OpenThread`.
- `SetThreadSelectedCpuSets` failures are logged with `Operation::SetThreadSelectedCpuSets`.
- `SetThreadPriority` failures are logged with `Operation::SetThreadPriority`.

### Interaction with other functions

This function is called by [apply_prime_threads](apply_prime_threads.md) after [apply_prime_threads_select](apply_prime_threads_select.md) has marked which threads are prime. Threads promoted here will later be evaluated by [apply_prime_threads_demote](apply_prime_threads_demote.md), which clears pinning and restores the original priority when a thread no longer qualifies.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L818–L960 |
| **Called by** | [apply_prime_threads](apply_prime_threads.md) |
| **Calls** | [log_error_if_new](log_error_if_new.md), `resolve_address_to_module`, `filter_indices_by_mask`, `cpusetids_from_indices`, `indices_from_cpusetids` |
| **Windows API** | `SetThreadSelectedCpuSets`, `GetThreadPriority`, `SetThreadPriority` |

## See also

- [apply_prime_threads](apply_prime_threads.md)
- [apply_prime_threads_select](apply_prime_threads_select.md)
- [apply_prime_threads_demote](apply_prime_threads_demote.md)
- [ApplyConfigResult](ApplyConfigResult.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)