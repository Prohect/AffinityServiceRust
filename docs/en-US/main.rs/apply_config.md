# apply_config function (main.rs)

Orchestrates the full application of both process-level and thread-level configuration to a single matched process. It retrieves the process's thread map once, applies process-level settings, looks up and applies any corresponding thread-level settings, records which PIDs have been applied, and delegates logging to `log_apply_results`.

## Syntax

```rust
fn apply_config(
    cli: &CliArgs,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut smallvec::SmallVec<[u32; PIDS]>,
    thread_level_applied: &mut smallvec::SmallVec<[u32; PENDING]>,
    grade: &u32,
    pid: &u32,
    name: &&str,
    process_level_config: &ProcessLevelConfig,
    process: &ProcessEntry,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | Parsed command-line arguments. The `dry_run` field controls whether Win32 API calls are actually issued. |
| `configs` | `&ConfigResult` | The full loaded configuration, including both `process_level_configs` and `thread_level_configs` keyed by grade and process name. |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | The thread scheduler instance used for prime-thread tracking when thread-level rules are present. |
| `process_level_applied` | `&mut smallvec::SmallVec<[u32; PIDS]>` | Accumulator of PIDs that have already had process-level settings applied. The current PID is pushed into this list upon return. |
| `thread_level_applied` | `&mut smallvec::SmallVec<[u32; PENDING]>` | Accumulator of PIDs that have already had thread-level settings applied in this iteration. Prevents duplicate thread-level application later in the loop. |
| `grade` | `&u32` | The configuration grade (polling-interval multiplier). Used to look up the matching thread-level config for the same grade. |
| `pid` | `&u32` | The Windows process ID of the target process. |
| `name` | `&&str` | The lowercase executable name (e.g. `"game.exe"`) used as the configuration lookup key. |
| `process_level_config` | `&ProcessLevelConfig` | The already-resolved process-level configuration entry for this process. |
| `process` | `&ProcessEntry` | A reference to the process snapshot entry, from which the thread map is obtained via `get_threads()`. |

## Return value

This function does not return a value.

## Remarks

- The function calls `process.get_threads()` once and reuses the resulting `HashMap<u32, SYSTEM_THREAD_INFORMATION>` for both `apply_process_level` and `apply_thread_level`, avoiding duplicate thread enumeration.
- An `ApplyConfigResult` is created internally to collect changes and errors from both levels. After both levels have been applied, the combined result is forwarded to `log_apply_results`.
- Thread-level configuration is looked up in `configs.thread_level_configs` by the same `grade` and `name`. If no thread-level entry exists for the process, only process-level settings are applied.
- The contract documented in the source (`assert(grade for process_level_config == grade for thread_level_config)`) means that callers must ensure the grade used to find the process-level config is the same grade used to look up the thread-level config.
- The PID is unconditionally pushed into `process_level_applied` regardless of whether any changes were actually made. This prevents re-application on subsequent loop iterations (unless `cli.continuous_process_level_apply` is set in the main loop).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main.rs` |
| Callers | [main](main.md) (main polling loop, both grade-iteration and ETW-pending paths) |
| Callees | [apply_process_level](apply_process_level.md), [apply_thread_level](apply_thread_level.md), [log_apply_results](log_apply_results.md) |
| API | `ProcessEntry::get_threads` (process module) |
| Privileges | Inherits privilege requirements from `apply_process_level` and `apply_thread_level` |

## See Also

| Resource | Link |
|----------|------|
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| main | [main](main.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
