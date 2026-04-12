# apply_prime_threads function (apply.rs)

Main orchestration function for prime thread scheduling. Identifies the most CPU-intensive threads in a process and pins them to designated "prime" CPUs via CPU Sets for improved cache locality and scheduling predictability.

## Syntax

```rust
pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process ID of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing prime thread settings including `prime_threads_cpus`, `prime_threads_prefixes`, and `track_top_x_threads`.

`dry_run`

If `true`, records what changes would be made without calling any Windows APIs.

`current_mask`

Mutable reference to the process's current CPU affinity mask. Passed through to [apply_prime_threads_promote](apply_prime_threads_promote.md) for filtering prime CPUs against the active affinity.

`process`

Mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) representing the target process. Used to enumerate live threads and their scheduling information.

`prime_core_scheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that maintains per-thread statistics, cycle counts, and pinning state across iterations.

`apply_config_result`

Mutable reference to the [ApplyConfigResult](ApplyConfigResult.md) that accumulates change and error messages.

## Return value

This function does not return a value. Results are recorded in `apply_config_result`.

## Remarks

### Algorithm

The prime thread scheduling pipeline proceeds through these stages:

1. **Guard checks** — The function returns early if no prime CPUs are configured and tracking is disabled. A negative `track_top_x_threads` value disables prime scheduling while still allowing tracking. In dry-run mode, only a summary change message is logged.

2. **Set tracking info** — If `track_top_x_threads != 0`, calls `prime_core_scheduler.set_tracking_info()` to register the process for thread-level statistics collection.

3. **Sort threads by CPU time delta** — Enumerates all threads, computes each thread's CPU time delta (`cached_total_time - last_total_time`), and sorts descending. If tracking is enabled, snapshots `last_system_thread_info` for each thread.

4. **Build candidate pool** — Selects the top N candidate threads where N = max(prime_count × 4, logical CPU count), capped at thread count. Additionally includes any previously-pinned threads that may have dropped out of the top candidates, ensuring they can be properly demoted.

5. **Compute cycle deltas** — For each candidate, computes `cached_cycles - last_cycles` to measure recent CPU activity.

6. **Select** — Calls [apply_prime_threads_select](apply_prime_threads_select.md) with hysteresis to determine which threads should be prime.

7. **Promote** — Calls [apply_prime_threads_promote](apply_prime_threads_promote.md) to pin newly selected threads to prime CPUs and boost their priority.

8. **Demote** — Calls [apply_prime_threads_demote](apply_prime_threads_demote.md) to unpin threads that no longer qualify and restore their original priority.

9. **Handle cleanup** — Removes cached thread handles for threads that no longer exist in the process snapshot. `ThreadHandle`'s `Drop` implementation automatically closes the underlying OS handles.

### Candidate Pool Sizing

The candidate pool is intentionally larger than the number of prime slots (4× or CPU count, whichever is greater) to give the hysteresis algorithm enough data to make stable selections. Previously-pinned threads are always included regardless of their current ranking so they can transition through the demotion path cleanly.

### Configuration

- `config.prime_threads_cpus` — CPU indices designated as prime cores.
- `config.prime_threads_prefixes` — Optional module prefix rules that restrict which threads are eligible for prime status, with per-prefix CPU and priority overrides.
- `config.track_top_x_threads` — Controls tracking depth. Negative values disable prime scheduling but retain tracking. Zero disables both.

### Change Logged

- Dry run: `"Prime CPUs: -> [{cpu_list}]"`

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L704–L800 |
| **Called by** | [apply_config](../main.rs/apply_config.md) in main.rs |
| **Calls** | [apply_prime_threads_select](apply_prime_threads_select.md), [apply_prime_threads_promote](apply_prime_threads_promote.md), [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| **Windows API** | (delegated to sub-functions) |

## See also

- [apply_prime_threads_select](apply_prime_threads_select.md)
- [apply_prime_threads_promote](apply_prime_threads_promote.md)
- [apply_prime_threads_demote](apply_prime_threads_demote.md)
- [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)