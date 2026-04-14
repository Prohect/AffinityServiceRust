# apply_config_thread_level function (main.rs)

Applies thread-level settings for a managed process on every polling iteration. Unlike the one-shot [apply_config_process_level](apply_config_process_level.md), this function is invoked repeatedly to re-evaluate prime thread scheduling, ideal processor assignment, and per-thread cycle-time tracking. It is the core of the service's dynamic thread management capability.

## Syntax

```rust
fn apply_config_thread_level(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process identifier (PID) of the target process.

`config`

A reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing the thread-level rules for this process. The relevant fields are `prime_threads_cpus`, `prime_threads_prefixes`, `ideal_processor_rules`, `affinity_cpus`, and `track_top_x_threads`.

`prime_core_scheduler`

A mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that maintains per-process, per-thread statistics (cycle counts, active streaks, promotion/demotion state). This scheduler is shared across all managed processes and accumulates data across polling iterations.

`process`

A mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. Thread enumeration data from the last snapshot is read from this entry, and cached state (such as pinned CPU set IDs and ideal processor assignments) is updated.

`dry_run`

When `true`, the function simulates changes and records intended actions in `apply_config_result` without making Win32 API calls. When `false`, thread-level settings are applied to the live process.

`apply_config_result`

A mutable reference to an [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) accumulator. Sub-functions append their changes and errors to this structure for the caller to log.

## Return value

This function does not return a value.

## Remarks

The function is a no-op if none of the thread-level configuration fields are active. Specifically, it returns immediately unless at least one of the following conditions is true:

- `config.prime_threads_cpus` is non-empty (prime thread CPU pinning is configured).
- `config.prime_threads_prefixes` is non-empty (module-prefix-based prime thread rules exist).
- `config.ideal_processor_rules` is non-empty (ideal processor assignment rules are defined).
- `config.track_top_x_threads` is non-zero (top-N thread tracking is enabled).

When the function proceeds, it performs the following steps in order:

1. **Query current affinity mask** — If prime thread CPUs or affinity CPUs are configured, the function opens a process handle and calls `GetProcessAffinityMask` to obtain the current mask. This mask is used to filter which CPUs are valid targets for prime thread pinning.

2. **Drop module cache** — Calls [drop_module_cache](../winapi.rs/drop_module_cache.md) for the PID, forcing a refresh of the module-to-address mapping used for prefix-based thread identification.

3. **Mark process alive** — Calls `prime_core_scheduler.set_alive(pid)` to signal that this process was seen in the current polling iteration. Processes not marked alive are candidates for cleanup.

4. **Prefetch thread cycle times** — Calls [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) to query `QueryThreadCycleTime` for every thread in the process and store the results in the scheduler. Delta cycle counts between iterations are used to identify the most active (prime) threads.

5. **Apply prime threads** — Calls [apply_prime_threads](../apply.rs/apply_prime_threads.md) to promote the highest-activity threads to preferred CPU cores (via `SetThreadSelectedCpuSets`) and optionally set their thread priority. Threads that fall below the activity threshold are demoted back to default scheduling.

6. **Apply ideal processors** — Calls [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) to assign threads to specific logical processors based on prefix-matching rules (e.g., threads whose start address resolves to `render.dll` get pinned to specific cores via `SetThreadIdealProcessorEx`).

7. **Update thread stats** — Calls [update_thread_stats](../apply.rs/update_thread_stats.md) to commit the current iteration's cycle counts as the baseline for the next iteration's delta calculation.

### Interaction with process-level settings

Thread-level settings build on top of process-level settings. For example, if `apply_config_process_level` set a CPU affinity mask restricting the process to cores 0–7, then prime thread scheduling will only consider cores within that mask. The `current_mask` variable bridges this relationship.

### Grade-based scheduling

This function respects the grade-based scheduling system in the main loop. A process rule with `grade=5` only has its thread-level settings evaluated every 5th polling iteration, reducing overhead for processes that do not require frequent thread re-balancing.

### ETW integration

When the ETW process monitor is active, dead processes are cleaned up reactively (on process-exit events) rather than at the end of each polling loop. This means the `prime_core_scheduler` state for exited processes is removed promptly, freeing resources.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main` |
| Callers | [main](main.md) (polling loop) |
| Callees | [get_process_handle](../winapi.rs/get_process_handle.md), [drop_module_cache](../winapi.rs/drop_module_cache.md), [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md), [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [update_thread_stats](../apply.rs/update_thread_stats.md) |
| API | `GetProcessAffinityMask`, `QueryThreadCycleTime`, `SetThreadSelectedCpuSets`, `SetThreadIdealProcessorEx`, `SetThreadPriority` |
| Privileges | `SeDebugPrivilege` (recommended for opening thread handles in protected processes) |

## See Also

| Topic | Link |
|-------|------|
| Process-level settings (one-shot) | [apply_config_process_level](apply_config_process_level.md) |
| Prime thread scheduler and hysteresis | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Process configuration structure | [ProcessConfig](../config.rs/ProcessConfig.md) |
| Main entry point and polling loop | [main](main.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd