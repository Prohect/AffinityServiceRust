# hotreload_config function (config.rs)

Checks the configuration file's last-modified timestamp and, if it has changed since the previous check, re-parses the file via [read_config](read_config.md). When parsing succeeds (no errors), the live rule map, tuning constants, and related scheduler state are replaced atomically. When parsing fails, the previous configuration is preserved and the errors are logged.

## Syntax

```rust
pub fn hotreload_config(
    cli: &CliArgs,
    configs: &mut HashMap<u32, HashMap<String, ProcessConfig>>,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut HashSet<u32>,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | Reference to the parsed command-line arguments. The `config_file_name` field provides the path to the configuration file whose modification time is monitored. |
| `configs` | `&mut HashMap<u32, HashMap<String, ProcessConfig>>` | Mutable reference to the live rule map (keyed by grade, then by lowercased process name). On a successful reload, this map is replaced entirely with the newly parsed rules from [ConfigResult](ConfigResult.md)`.configs`. |
| `last_config_mod_time` | `&mut Option<std::time::SystemTime>` | Mutable reference to the cached modification timestamp from the previous check. Updated to the file's current `modified()` time after each successful metadata read. When `None`, any readable timestamp triggers a reload. |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | Mutable reference to the live prime-thread scheduler. On a successful reload, the scheduler's `constants` field is replaced with the newly parsed [ConfigConstants](ConfigConstants.md), allowing threshold changes to take effect on the next scheduling cycle without restarting the service. |
| `process_level_applied` | `&mut HashSet<u32>` | Mutable reference to the set of process IDs that have already had process-level settings applied (priority, affinity, CPU set, I/O priority, memory priority). Cleared on a successful reload so that the new rules are re-applied to all running processes on the next service loop iteration. |

## Return value

This function does not return a value. All outcomes are communicated through mutations to the parameters and log output.

## Remarks

### Reload algorithm

1. **Metadata check** — `std::fs::metadata` is called on `cli.config_file_name`. If the call fails (e.g., file deleted or inaccessible), the function returns without action, preserving the current configuration.
2. **Timestamp comparison** — The file's `modified()` time is compared against `*last_config_mod_time`. If the timestamps are equal (or both are `Some` with the same value), no reload is needed and the function returns immediately.
3. **Timestamp update** — `*last_config_mod_time` is set to `Some(mod_time)` unconditionally once a new modification time is observed. This prevents repeated reload attempts on each loop iteration for the same file version.
4. **Re-parse** — [read_config](read_config.md) is called with `cli.config_file_name` to produce a fresh [ConfigResult](ConfigResult.md).
5. **Validation gate** — If `new_config_result.errors` is non-empty, the reload is aborted:
   - A message is logged indicating the file has errors and the previous configuration is being kept.
   - Each error is logged individually.
   - `configs`, `prime_core_scheduler`, and `process_level_applied` are left unchanged.
6. **Swap on success** — If there are no errors:
   - `new_config_result.print_report()` is called to log group/rule statistics and any warnings.
   - The total rule count is captured via `new_config_result.total_rules()`.
   - `*configs` is replaced with `new_config_result.configs`.
   - `prime_core_scheduler.constants` is replaced with `new_config_result.constants`.
   - A completion message with the rule count is logged.
   - `process_level_applied.clear()` is called to force re-application of process-level settings.

### Clearing process_level_applied

The `process_level_applied` set tracks which PIDs have already had process-level rules applied. Clearing it after a successful reload ensures that changed priority, affinity, CPU set, I/O priority, and memory priority settings are re-applied to all currently running processes on the next service loop iteration. Without this reset, processes that had already been configured under the old rules would not receive updated settings until they were restarted.

### Error resilience

The function uses a **fail-safe** strategy: the live configuration is never replaced with an invalid parse result. If the new file contains syntax errors, undefined aliases, or other fatal issues, the previous working configuration continues to be used. This is critical for a long-running Windows service where a transient config-file edit (e.g., user is still typing) should not disrupt active process management.

### Interaction with hotreload_blacklist

[hotreload_blacklist](hotreload_blacklist.md) monitors the blacklist file independently and on a separate timestamp. Both functions are called on each iteration of the service main loop, so config and blacklist changes are detected and applied within one loop interval (controlled by the `-interval` CLI argument, defaulting to a few seconds).

### Let-chain pattern matching

The function uses Rust's `let`-chain syntax (`if let Ok(...) = ... && let Ok(...) = ... && ...`) to combine the metadata read, modification-time extraction, and timestamp comparison into a single conditional block. If any step in the chain fails, the entire block is skipped and the function returns silently — no log output is produced for transient file-system errors during the check phase.

### First invocation behavior

On the first call, `*last_config_mod_time` is `None`. Because `Some(mod_time) != None` is always true, the first successful metadata read always triggers a reload. This is intentional — it allows the service to detect config changes that occurred between startup (when the config was first loaded by [main](../main.rs/main.md)) and the first hot-reload check.

### Thread safety

This function is not designed for concurrent access. It is called from the single-threaded service main loop. The caller is responsible for ensuring exclusive access to the `configs`, `prime_core_scheduler`, and `process_level_applied` parameters.

### Logging

| Condition | Log output |
|-----------|------------|
| File modification detected | `"Configuration file '{path}' changed, reloading..."` |
| Successful reload | Parse report (via `print_report()`), then `"Configuration reload complete: {N} rules loaded."` |
| Failed reload (errors) | `"Configuration file '{path}' has errors, keeping previous configuration."`, followed by each error prefixed with `"  - "`. |
| File inaccessible / unchanged | *(no output)* |

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [main](../main.rs/main.md) (service loop) |
| **Callees** | [read_config](read_config.md), [ConfigResult::print_report](ConfigResult.md), [ConfigResult::total_rules](ConfigResult.md) |
| **API** | `std::fs::metadata`, `std::fs::Metadata::modified` |
| **Privileges** | Read access to the configuration file path |

## See Also

| Topic | Link |
|-------|------|
| Main config parser | [read_config](read_config.md) |
| Parsed config aggregate | [ConfigResult](ConfigResult.md) |
| Hysteresis tuning constants | [ConfigConstants](ConfigConstants.md) |
| Per-process rule struct | [ProcessConfig](ProcessConfig.md) |
| Blacklist hot-reload | [hotreload_blacklist](hotreload_blacklist.md) |
| Prime-thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI arguments struct | [CliArgs](../cli.rs/CliArgs.md) |
| Config module overview | [README](README.md) |