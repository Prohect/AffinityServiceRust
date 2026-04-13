# hotreload_config function (config.rs)

Checks if the configuration file has been modified since the last check and reloads it if so.

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

`cli`

Reference to [`CliArgs`](../cli.rs/CliArgs.md) containing the config file path.

`configs`

Mutable reference to the grade-keyed configuration map. Replaced in-place when a valid new config is loaded.

`last_config_mod_time`

Tracks the last known modification time of the config file.

`prime_core_scheduler`

Mutable reference to the [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md). Its constants are updated when the config reloads.

`process_level_applied`

Mutable reference to the set of PIDs that have had process-level settings applied. Cleared on reload so that all processes get re-applied with the new configuration.

## Remarks

Called each loop iteration from [`main`](../main.rs/main.md) after sleeping. If the config file modification time has changed:

1. Parses the new config via [`read_config`](read_config.md).
2. If parsing succeeds (no errors), replaces the active configuration, updates scheduler constants, clears `process_level_applied`, and logs the reload.
3. If parsing fails, keeps the previous configuration and logs the errors.

This function was previously inline in `main()` and has been extracted for clarity.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Called by** | [`main`](../main.rs/main.md) |

## See also

- [hotreload_blacklist](hotreload_blacklist.md)
- [read_config](read_config.md)
- [config.rs module overview](README.md)