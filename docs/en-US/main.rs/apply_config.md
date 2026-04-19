# apply_config function (main.rs)

Combined entry point that applies both process-level and thread-level configuration for a single matched process. Creates a shared thread cache so that thread enumeration is performed at most once per process per iteration, then delegates to [apply_process_level](apply_process_level.md) and [apply_thread_level](apply_thread_level.md). Tracks which PIDs have been processed at each level and logs the merged results.

## Syntax

```rust
fn apply_config(
    cli: &CliArgs,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut SmallVec<[u32; PIDS]>,
    thread_level_applied: &mut SmallVec<[u32; PENDING]>,
    grade: &u32,
    pid: &u32,
    name: &&str,
    process_level_config: &ProcessLevelConfig,
    process: &ProcessEntry,
)
```

## Parameters

`cli: &CliArgs`

The parsed [CLI arguments](../cli.rs/CliArgs.md). Used to read `cli.dry_run` which is forwarded to both apply functions.

`configs: &ConfigResult`

The full [ConfigResult](../config.rs/ConfigResult.md). Used to look up a matching [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) from `configs.thread_level_configs` by `grade` and `name`.

`prime_core_scheduler: &mut PrimeThreadScheduler`

The [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) instance, passed through to [apply_thread_level](apply_thread_level.md) for prime thread tracking and scheduling.

`process_level_applied: &mut SmallVec<[u32; PIDS]>`

Running list of PIDs that have already had process-level settings applied. The current `pid` is appended unconditionally after [apply_process_level](apply_process_level.md) completes. On subsequent iterations (unless `-continuous_process_level_apply` is set), PIDs in this list are skipped for process-level work.

`thread_level_applied: &mut SmallVec<[u32; PENDING]>`

Running list of PIDs that have had thread-level settings applied in the *current* iteration. The current `pid` is appended only when a [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) exists and [apply_thread_level](apply_thread_level.md) is invoked. This prevents the same process from being processed twice in a single loop iteration (which would break the scheduler's delta-based cycle time tracking).

`grade: &u32`

The configuration grade (polling frequency multiplier) under which the matched rule was found. Used to look up the corresponding thread-level config in `configs.thread_level_configs`.

`pid: &u32`

The process identifier of the target process.

`name: &&str`

The lowercase executable name of the target process (e.g., `"chrome.exe"`). Used as the key to look up thread-level config.

`process_level_config: &ProcessLevelConfig`

The [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) that matched this process. Forwarded to [apply_process_level](apply_process_level.md).

`process: &ProcessEntry`

The [ProcessEntry](../process.rs/ProcessEntry.md) from the current snapshot, used to enumerate threads via `process.get_threads()`.

## Return value

This function does not return a value. All results are communicated through `process_level_applied`, `thread_level_applied`, and the log output produced by [log_apply_results](log_apply_results.md).

## Remarks

### Thread cache

The function creates a `OnceCell<HashMap<u32, SYSTEM_THREAD_INFORMATION>>` that lazily evaluates `process.get_threads()`. This cell is shared by closure reference across both [apply_process_level](apply_process_level.md) and [apply_thread_level](apply_thread_level.md), ensuring that thread enumeration — which involves `NtQuerySystemInformation` — happens at most once per call regardless of how many apply functions need thread data.

### Two-level apply flow

1. A fresh [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) is created.
2. [apply_process_level](apply_process_level.md) is called unconditionally with the process-level config.
3. The function then looks up a thread-level config at `configs.thread_level_configs[grade][name]`. If one exists, [apply_thread_level](apply_thread_level.md) is called and the PID is added to `thread_level_applied`.
4. The PID is always added to `process_level_applied`.
5. [log_apply_results](log_apply_results.md) is called with the accumulated result.

### Grade invariant

The function asserts (by convention) that the grade used to look up the process-level config is the same grade used for the thread-level config lookup. This is guaranteed by the caller in the main loop, which iterates configs by grade.

## Requirements

| | |
|---|---|
| **Module** | `src/main.rs` |
| **Callers** | [main](main.md) loop — both the ETW-pending path and the full-match path |
| **Callees** | [apply_process_level](apply_process_level.md), [apply_thread_level](apply_thread_level.md), [log_apply_results](log_apply_results.md), `ProcessEntry::get_threads` |
| **Win32 API** | None directly (delegated to callees) |
| **Privileges** | None directly (delegated to callees) |

## See Also

| Topic | Link |
|-------|------|
| Process-level apply wrapper | [apply_process_level](apply_process_level.md) |
| Thread-level apply wrapper | [apply_thread_level](apply_thread_level.md) |
| Result logging | [log_apply_results](log_apply_results.md) |
| Result accumulator | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| Configuration types | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md), [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Module overview | [main.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*