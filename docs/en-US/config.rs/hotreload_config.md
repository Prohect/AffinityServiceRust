# hotreload_config function (config.rs)

Watches the configuration file for modifications by comparing its filesystem modification timestamp against a cached value, and hot-reloads the configuration when a change is detected. If the newly parsed configuration is valid, it atomically replaces the active configuration, updates the scheduler constants, and resets the per-PID application tracking list. If the new configuration contains errors, the previous configuration is retained and the errors are logged.

## Syntax

```AffinityServiceRust/src/config.rs#L1303-1334
pub fn hotreload_config(
    cli: &CliArgs,
    configs: &mut ConfigResult,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut List<[u32; PIDS]>,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | A reference to the CLI arguments struct. The `config_file_name` field is used to locate the configuration file on disk. |
| `configs` | `&mut ConfigResult` | A mutable reference to the active configuration result. If the reloaded configuration is valid, this is replaced with the new result; otherwise it is left unchanged. |
| `last_config_mod_time` | `&mut Option<std::time::SystemTime>` | A mutable reference to the cached modification timestamp of the config file from the last successful check. Updated to the current modification time whenever a change is detected (regardless of whether the new config is valid). On the first call, this should be `None` to force an initial load. |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | A mutable reference to the prime-thread scheduler. When the configuration is successfully reloaded, the scheduler's `constants` field is updated to the new [`ConfigConstants`](ConfigConstants.md) values from the reloaded config. |
| `process_level_applied` | `&mut List<[u32; PIDS]>` | A mutable reference to the list of process IDs that have already had process-level settings applied. Cleared on successful reload so that all processes are re-evaluated with the new rules on the next polling iteration. |

## Return value

This function does not return a value. All side effects are communicated through mutations to the `configs`, `last_config_mod_time`, `prime_core_scheduler`, and `process_level_applied` parameters.

## Remarks

### Change detection algorithm

1. The function calls `std::fs::metadata` on `cli.config_file_name` to obtain the file's metadata.
2. It extracts the modification timestamp via `metadata.modified()`.
3. If the modification time differs from `*last_config_mod_time`, a reload is triggered. If the times match (or the metadata/modified calls fail), the function returns immediately with no side effects.
4. The cached timestamp `*last_config_mod_time` is updated to `Some(mod_time)` **before** parsing begins. This prevents repeated reload attempts if parsing is slow or the file is being rapidly written.

### Reload flow

When a change is detected:

1. A log message is emitted: `"Configuration file '{name}' changed, reloading..."`.
2. The file is fully parsed via [`read_config`](read_config.md) into a new `ConfigResult`.
3. **If the new config is valid** (`new_config_result.errors.is_empty()`):
   - `*configs` is replaced with the new `ConfigResult`.
   - `configs.print_report()` is called to log the summary.
   - The scheduler's constants are updated: `prime_core_scheduler.constants = configs.constants.clone()`.
   - The total rule count is logged.
   - `process_level_applied` is cleared so all processes are re-evaluated on the next loop.
4. **If the new config has errors**:
   - The previous `*configs` is retained unchanged.
   - A log message is emitted: `"Configuration file '{name}' has errors, keeping previous configuration."`.
   - Each error is logged individually with a `"  - "` prefix.

### Atomic replacement

The replacement of `*configs` uses a simple assignment (`*configs = new_config_result`). Because the entire old `ConfigResult` is dropped and replaced in a single statement, there is no intermediate state where a partially updated configuration is visible. However, this is **not** thread-safe — the function assumes single-threaded access to the configuration, which is guaranteed by the main polling loop's sequential execution model.

### Clearing process_level_applied

After a successful reload, `process_level_applied.clear()` is called. This list tracks which PIDs have already had process-level settings (priority, affinity, CPU set, I/O priority, memory priority) applied. Clearing it ensures that the new rules are applied to all running processes on the next polling iteration, not just newly spawned ones.

### Scheduler constant propagation

The scheduler's `constants` field is updated separately from the `ConfigResult` replacement because the `PrimeThreadScheduler` maintains its own copy of the constants for performance reasons (avoiding repeated hash map lookups during per-thread scheduling decisions).

### Error resilience

The hot-reload design is intentionally conservative: a configuration file that contains **any** errors is entirely rejected. This prevents partial or inconsistent rule sets from being applied. The user must fix all errors before the reload will take effect. Warnings alone do not prevent a reload.

### Filesystem access patterns

The function calls `std::fs::metadata` on every invocation (typically once per polling loop). This is a lightweight operation on Windows (no file content is read until a change is detected) and is the standard pattern for file-change detection without filesystem watchers.

### Edge cases

| Scenario | Behavior |
|----------|----------|
| Config file deleted while running | `metadata()` fails; function returns with no side effects. Previous config is retained. |
| Config file replaced with an empty file | Parsed successfully (no errors, no rules); previous config is replaced with an empty rule set. |
| Config file saved with syntax errors | New config is rejected; previous config retained; errors are logged. |
| First invocation with `last_config_mod_time = None` | Triggers a reload unconditionally (since `Some(mod_time) != None`). |
| Rapid successive saves (editor auto-save) | Each distinct modification timestamp triggers a reload attempt. |

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | `main.rs` (main polling loop, called once per iteration) |
| Callees | [`read_config`](read_config.md), [`ConfigResult::print_report`](ConfigResult.md), [`ConfigResult::total_rules`](ConfigResult.md), `std::fs::metadata`, `log!` |
| Dependencies | [`CliArgs`](../cli.rs/CliArgs.md), [`ConfigResult`](ConfigResult.md), [`ConfigConstants`](ConfigConstants.md), `PrimeThreadScheduler` from [`scheduler.rs`](../scheduler.rs/README.md), `List` and `PIDS` from [`collections.rs`](../collections.rs/README.md) |
| I/O | Filesystem metadata read on `cli.config_file_name`; full file read (via `read_config`) only when a change is detected |
| Privileges | File system read access to the configuration file |

## See Also

| Resource | Link |
|----------|------|
| hotreload_blacklist | [hotreload_blacklist](hotreload_blacklist.md) |
| read_config | [read_config](read_config.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| ConfigConstants | [ConfigConstants](ConfigConstants.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| scheduler module | [scheduler.rs overview](../scheduler.rs/README.md) |
| config module overview | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*