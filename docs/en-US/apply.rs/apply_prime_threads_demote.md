# apply_prime_threads_demote function (apply.rs)

Demotes threads that no longer qualify for prime status by removing their CPU set pinning and restoring their original thread priority.

## Syntax

```rust
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

`pid`

The process ID of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing the process rule configuration, used for error logging context (`config.name`).

`process`

Mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. Used to enumerate the current set of live thread IDs.

`tid_with_delta_cycles`

Slice of `(tid, delta_cycles, is_prime)` tuples produced by [apply_prime_threads_select](apply_prime_threads_select.md). The `is_prime` flag indicates which threads were selected for prime status; threads **not** in this set but still carrying pinned CPU set IDs are candidates for demotion.

`prime_core_scheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md). Thread stats are updated to clear `pinned_cpu_set_ids` and consume the saved `original_priority` for restoration.

`apply_config_result`

Mutable reference to [ApplyConfigResult](ApplyConfigResult.md) where change messages and errors are collected.

## Return value

This function does not return a value. Results are recorded in `apply_config_result`.

## Remarks

This function is the counterpart to [apply_prime_threads_promote](apply_prime_threads_promote.md) and is called as the final step of the [apply_prime_threads](apply_prime_threads.md) orchestration.

### Demotion algorithm

1. Build a `HashSet` of thread IDs that are currently marked as prime from `tid_with_delta_cycles`.
2. Collect all live thread IDs from `process.get_threads()`.
3. For each live thread that is **not** in the prime set **and** has non-empty `pinned_cpu_set_ids`:
   - Clear the thread's CPU set assignment by calling `SetThreadSelectedCpuSets` with an empty slice.
   - On success, log a change message: `"Thread {tid} -> (demoted, start={module})"`.
   - On failure, log the error via [log_error_if_new](log_error_if_new.md).
   - **Always** clear `pinned_cpu_set_ids` regardless of success or failure, to prevent infinite retry loops that would spam the logs.
4. If the thread had a saved `original_priority` (set during promotion), restore it with `SetThreadPriority`. Log errors if restoration fails.

### Defensive clearing

The `pinned_cpu_set_ids` vector is cleared unconditionally after attempting demotion — even if the `SetThreadSelectedCpuSets` call fails. This is a deliberate design choice to avoid a situation where a persistent API error causes the same thread to be retried every loop iteration, flooding the error log.

### Change messages

| Event | Format |
| --- | --- |
| CPU set cleared | `Thread {tid} -> (demoted, start={module})` |
| Priority restore failure | Error logged via [log_error_if_new](log_error_if_new.md) with operation `SetThreadPriority` |
| CPU set clear failure | Error logged via [log_error_if_new](log_error_if_new.md) with operation `SetThreadSelectedCpuSets` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/apply.rs` |
| **Lines** | L962–L1053 |
| **Called by** | [apply_prime_threads](apply_prime_threads.md) |
| **Calls** | [log_error_if_new](log_error_if_new.md), [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| **Windows API** | `SetThreadSelectedCpuSets`, `SetThreadPriority` |

## See also

- [apply_prime_threads](apply_prime_threads.md) — orchestrator that calls this function
- [apply_prime_threads_promote](apply_prime_threads_promote.md) — promotes threads to prime status
- [apply_prime_threads_select](apply_prime_threads_select.md) — selects which threads qualify for prime
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — manages per-thread tracking state