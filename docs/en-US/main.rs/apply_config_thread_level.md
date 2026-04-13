# apply_config_thread_level function (main.rs)

Applies thread-level settings to a target process every polling iteration. Includes prime thread scheduling, ideal processor assignment, and cycle time tracking.

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

`pid` — Target process ID.

`config` — Reference to [`ProcessConfig`](../config.rs/ProcessConfig.md).

`prime_core_scheduler` — Mutable reference to [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md).

`process` — Mutable reference to [`ProcessEntry`](../process.rs/ProcessEntry.md).

`dry_run` — When `true`, records changes without applying them.

`apply_config_result` — Mutable reference to [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md).

## Remarks

Only executes when the config has prime thread, ideal processor, or thread tracking settings. Steps:

1. Query current affinity mask via `GetProcessAffinityMask` for prime thread CPU filtering
2. Clear module cache via [`drop_module_cache`](../winapi.rs/drop_module_cache.md)
3. Mark process alive in scheduler
4. [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) — Collect cycle baselines
5. [`apply_prime_threads`](../apply.rs/apply_prime_threads.md) — Select, promote, and demote prime threads
6. [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) — Assign ideal processors
7. [`update_thread_stats`](../apply.rs/update_thread_stats.md) — Persist cached data

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/main.rs |
| **Source lines** | L76–L115 |
| **Called by** | [`main`](main.md) loop |

## See also

- [apply_config_process_level](apply_config_process_level.md)
- [scheduler.rs module overview](../scheduler.rs/README.md)
- [main.rs module overview](README.md)