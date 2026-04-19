# hotreload_config function (config.rs)

Checks whether the configuration file has been modified on disk and, if so, reloads it. On a successful reload with no parse errors, the active configuration is replaced, the prime thread scheduler's constants are updated, and all process-level applied state is reset so that rules are re-evaluated against running processes.

## Syntax

```rust
pub fn hotreload_config(
    cli: &CliArgs,
    configs: &mut ConfigResult,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut List<[u32; PIDS]>,
    full_process_level_match: &mut bool,
)
```

## Parameters

`cli: &CliArgs`

Reference to the parsed command-line arguments. The `config_file_name` field provides the path to the configuration file to monitor.

`configs: &mut ConfigResult`

**\[in, out\]** The currently active configuration. Replaced in-place with the newly parsed [ConfigResult](ConfigResult.md) if the reload succeeds without errors. If errors are found, this value is left unchanged and the previous configuration remains in effect.

`last_config_mod_time: &mut Option<std::time::SystemTime>`

**\[in, out\]** Tracks the last-seen modification timestamp of the config file. On each call the function queries the file's metadata and compares the modification time against this value. If they differ (or this is `None`), a reload is triggered and the new timestamp is stored. The caller persists this value across polling iterations.

`prime_core_scheduler: &mut PrimeThreadScheduler`

**\[in, out\]** The prime thread scheduler instance. On a successful reload, the scheduler's `constants` field is replaced with the new [ConfigConstants](ConfigConstants.md) from the freshly parsed configuration so that updated hysteresis thresholds take effect immediately.

`process_level_applied: &mut List<[u32; PIDS]>`

**\[in, out\]** A list of process IDs for which process-level rules have already been applied. Cleared on successful reload so that all running processes are re-evaluated against the new rules in the next polling cycle.

`full_process_level_match: &mut bool`

**\[in, out\]** Flag indicating whether all currently running processes have been matched against process-level rules. Reset to `true` on a successful reload, signalling the main loop to perform a full match pass.

## Return value

This function does not return a value. Results are communicated through the mutable reference parameters.

## Remarks

### Reload decision logic

The function uses a two-step guard:

1. Query `std::fs::metadata` for the config file path. If the metadata call fails (e.g., file deleted, permission denied), no action is taken and the previous configuration remains active.
2. Compare `metadata.modified()` against `*last_config_mod_time`. If the timestamps are equal, the file has not changed and the function returns immediately. If they differ (including the initial case where `last_config_mod_time` is `None`), a reload is initiated.

This means the function is safe to call on every polling iteration with negligible overhead — it only performs a single `metadata()` syscall when the file has not changed.

### Reload procedure

When a modification is detected:

1. `*last_config_mod_time` is updated to the new modification time.
2. A log message is emitted: `"Configuration file '{path}' changed, reloading..."`.
3. [read_config](read_config.md) is called to parse the file from scratch.
4. The new [ConfigResult](ConfigResult.md) is checked via `errors.is_empty()`:
   - **No errors:** The active `configs` is replaced, [print_report](ConfigResult.md) is called, the scheduler's constants are updated, `process_level_applied` is cleared, `full_process_level_match` is set to `true`, and a completion message logs the total rule count.
   - **Errors present:** The previous `configs` is retained, an error message is logged, and each individual error is printed. The service continues operating with the old configuration.

### State reset on successful reload

Clearing `process_level_applied` forces the main polling loop to re-apply process-level settings (priority, affinity, CPU sets, IO priority, memory priority) to all currently running processes, even those that were already configured under the old rules. This ensures that rule changes (e.g., a process moved from P-cores to E-cores) take effect without requiring the process to be restarted.

Setting `full_process_level_match` to `true` tells the main loop to perform a full scan of all running processes rather than only newly discovered ones.

### Thread safety

This function is not thread-safe and must be called from the main polling loop only. All mutable parameters are exclusive references, enforced by Rust's borrow checker.

### Error resilience

The function follows a fail-safe pattern: a malformed config file never disrupts the running service. The old configuration continues to be applied until a valid config file is saved. This allows users to edit the config file while the service is running without risking a period of unconfigured operation.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Callers** | Main polling loop in `src/main.rs` |
| **Callees** | [read_config](read_config.md), [ConfigResult::print_report](ConfigResult.md), `std::fs::metadata` |
| **Dependencies** | [CliArgs](../cli.rs/CliArgs.md), [ConfigResult](ConfigResult.md), [ConfigConstants](ConfigConstants.md), [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), [List](../collections.rs/README.md) |
| **Privileges** | File system read access to the configuration file path |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Blacklist hot-reload counterpart | [hotreload_blacklist](hotreload_blacklist.md) |
| Config file parser | [read_config](read_config.md) |
| Parse result container | [ConfigResult](ConfigResult.md) |
| Tuning constants propagated to scheduler | [ConfigConstants](ConfigConstants.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI arguments | [CliArgs](../cli.rs/CliArgs.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*