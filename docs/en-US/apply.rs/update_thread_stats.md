# update_thread_stats function (apply.rs)

Updates cached thread statistics at the end of each loop iteration, persisting cycle and time measurements for delta calculation in the next iteration.

## Syntax

```rust
pub fn update_thread_stats(
    pid: u32,
    prime_scheduler: &mut PrimeThreadScheduler,
)
```

## Parameters

`pid`

The process ID whose thread statistics should be updated.

`prime_scheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that holds per-process, per-thread statistics. The scheduler's `pid_to_process_stats` map is accessed to locate the [ProcessStats](../scheduler.rs/ProcessStats.md) entry for the given `pid`.

## Return value

This function does not return a value.

## Remarks

This function is the final step in each apply-config loop iteration. It copies the `cached_*` fields into the `last_*` fields of every [ThreadStats](../scheduler.rs/ThreadStats.md) entry for the process, then zeroes the cached values. This establishes the baseline for computing deltas on the next iteration.

For each thread in the process's `tid_to_thread_stats` map:

- If `cached_cycles > 0`: sets `last_cycles = cached_cycles`, then clears `cached_cycles` to `0`.
- If `cached_total_time > 0`: sets `last_total_time = cached_total_time`, then clears `cached_total_time` to `0`.

The guard `> 0` ensures that threads which were not measured in the current iteration (e.g., because they were outside the candidate pool in [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)) retain their previous baseline values rather than being reset to zero.

This function makes no Windows API calls and performs no I/O. It is a pure in-memory bookkeeping operation.

### Call sequence

In `main.rs`, the typical per-process apply sequence ends with:

1. [apply_priority](apply_priority.md) / [apply_affinity](apply_affinity.md) / [apply_process_default_cpuset](apply_process_default_cpuset.md) / [apply_io_priority](apply_io_priority.md) / [apply_memory_priority](apply_memory_priority.md)
2. [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)
3. [apply_prime_threads](apply_prime_threads.md) / [apply_ideal_processors](apply_ideal_processors.md)
4. **`update_thread_stats`** ← finalizes the iteration

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/apply.rs` |
| **Line** | L1327–L1340 |
| **Called by** | `apply_config()` in [main.rs](../main.rs/README.md) |
| **Depends on** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), [ThreadStats](../scheduler.rs/ThreadStats.md) |
| **Windows API** | None |