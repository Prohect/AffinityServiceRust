# apply_config_process_level function (main.rs)

Applies process-level settings to a target process. These settings are one-shot per process — applied once when the process is first detected, and not re-applied on subsequent iterations unless the configuration is reloaded.

## Syntax

```rust
fn apply_config_process_level(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid` — Target process ID.

`config` — Reference to [`ProcessConfig`](../config.rs/ProcessConfig.md) with all desired settings.

`process` — Mutable reference to [`ProcessEntry`](../process.rs/ProcessEntry.md) from the current snapshot.

`dry_run` — When `true`, records changes without applying them.

`apply_config_result` — Mutable reference to [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) to collect changes and errors.

## Remarks

Applies settings in this order:
1. [`apply_priority`](../apply.rs/apply_priority.md) — Process priority class
2. [`apply_affinity`](../apply.rs/apply_affinity.md) — Hard CPU affinity mask (with thread ideal processor reset)
3. [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md) — Soft CPU set preference
4. [`apply_io_priority`](../apply.rs/apply_io_priority.md) — I/O priority
5. [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) — Memory page priority

The main loop tracks which PIDs have had process-level settings applied via a `process_level_applied: HashSet<u32>`. ETW process start events add PIDs to `process_level_pending`, triggering immediate application regardless of grade scheduling.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/main.rs |
| **Source lines** | L48–L74 |
| **Called by** | [`main`](main.md) loop |

## See also

- [apply_config_thread_level](apply_config_thread_level.md)
- [apply.rs module overview](../apply.rs/README.md)
- [main.rs module overview](README.md)