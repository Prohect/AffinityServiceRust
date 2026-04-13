# CliArgs struct (cli.rs)

Container for all command-line arguments accepted by AffinityServiceRust. Every runtime option — polling interval, mode flags, file paths, privilege toggles, and debug knobs — is stored as a public field on this structure. A `CliArgs` instance is created once at startup via `CliArgs::new()`, populated by [parse_args](parse_args.md), and then passed by shared reference (`&CliArgs`) throughout the service lifetime to gate behavior in the polling loop, hot-reload logic, and utility modes.

## Syntax

```rust
#[derive(Debug, Default)]
pub struct CliArgs {
    pub interval_ms: u64,
    pub help_mode: bool,
    pub help_all_mode: bool,
    pub convert_mode: bool,
    pub autogroup_mode: bool,
    pub find_mode: bool,
    pub validate_mode: bool,
    pub process_logs_mode: bool,
    pub dry_run: bool,
    pub config_file_name: String,
    pub blacklist_file_name: Option<String>,
    pub in_file_name: Option<String>,
    pub out_file_name: Option<String>,
    pub no_uac: bool,
    pub loop_count: Option<u32>,
    pub time_resolution: u32,
    pub log_loop: bool,
    pub skip_log_before_elevation: bool,
    pub no_debug_priv: bool,
    pub no_inc_base_priority: bool,
}
```

## Members

`interval_ms` (`u64`)

Polling interval in milliseconds between successive iterations of the main loop. Corresponds to the `-interval <ms>` CLI flag. Default: **5000**. Minimum enforced by [parse_args](parse_args.md): **16**.

`help_mode` (`bool`)

When `true`, the [main](../main.rs/main.md) function prints the basic help message via [print_help](print_help.md) and exits immediately. Set by `-help`, `--help`, `-?`, `/?`, or `?`. Default: **false**.

`help_all_mode` (`bool`)

When `true`, the [main](../main.rs/main.md) function prints the combined detailed CLI and configuration help via [print_help_all](print_help_all.md) and exits immediately. Set by `-helpall` or `--helpall`. Default: **false**.

`convert_mode` (`bool`)

When `true`, the service runs in convert mode, translating a Process Lasso configuration file to AffinityServiceRust format. Uses `in_file_name` as input and `out_file_name` as output. Set by `-convert`. Default: **false**.

`autogroup_mode` (`bool`)

When `true`, the service runs in autogroup mode, reading a configuration file, grouping rules with identical settings into named group blocks, and writing the result to an output file. Uses `in_file_name` as input and `out_file_name` as output. Set by `-autogroup`. Default: **false**.

`find_mode` (`bool`)

When `true`, enables unmanaged-process discovery on every polling iteration. Processes with default (system-wide) CPU affinity that are not in the configuration or blacklist are logged to `.find.log` files. Set by `-find`. Default: **false**.

`validate_mode` (`bool`)

When `true`, the service loads and validates the configuration file for syntax errors and undefined aliases, prints the report, and exits without entering the polling loop. Also forces console output. Set by `-validate`. Default: **false**.

`process_logs_mode` (`bool`)

When `true`, the service runs in log-processing mode. It scans `.find.log` files in the logs directory, filters out known/blacklisted processes, uses Everything search (`es.exe`) to resolve executable paths, and writes results to a file. Set by `-processlogs`. Default: **false**.

`dry_run` (`bool`)

When `true`, the service simulates all changes without making any Win32 API calls. The [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) records what would have been changed, and the service exits after one iteration. Set by `-dryrun`, `-dry-run`, or `--dry-run`. Default: **false**.

`config_file_name` (`String`)

Path to the configuration file. Corresponds to `-config <file>`. Default: **`"config.ini"`**.

`blacklist_file_name` (`Option<String>`)

Optional path to a blacklist file containing process names to exclude from management and find-mode discovery. Corresponds to `-blacklist <file>`. Default: **`None`**.

`in_file_name` (`Option<String>`)

Optional input file path. Semantics depend on the active mode: the source configuration for `-convert` and `-autogroup`, or the logs directory for `-processlogs`. Corresponds to `-in <file>`. Default: **`None`** (each mode applies its own default, e.g., `"logs"` for `-processlogs`).

`out_file_name` (`Option<String>`)

Optional output file path. Semantics depend on the active mode: the destination file for `-convert`, `-autogroup`, and `-processlogs`. Corresponds to `-out <file>`. Default: **`None`** (each mode applies its own default, e.g., `"new_processes_results.txt"` for `-processlogs`).

`no_uac` (`bool`)

When `true`, suppresses the automatic UAC elevation request when the service detects it is not running as administrator. The service continues with limited privileges and logs a warning. Set by `-noUAC` or `-nouac`. Default: **false**.

`loop_count` (`Option<u32>`)

Optional maximum number of polling iterations. When `Some(n)`, the service exits after completing `n` loops. Useful for testing and scripted runs. Minimum enforced: **1**. Corresponds to `-loop <count>`. Default: **`None`** (infinite loop).

`time_resolution` (`u32`)

System timer resolution in 100-nanosecond units. A value of `5210` corresponds to 0.5210 ms. When non-zero, [set_timer_resolution](../winapi.rs/set_timer_resolution.md) is called at startup. Corresponds to `-resolution <t>`. Default: **0** (do not change timer resolution).

`log_loop` (`bool`)

When `true`, a log message is emitted at the start of each polling iteration, including the loop number. Useful for debugging timing and loop behavior. Set by `-logloop`. Default: **false**.

`skip_log_before_elevation` (`bool`)

When `true`, suppresses log output during the startup phase before UAC elevation. This prevents duplicated or confusing log entries when the service re-launches itself with elevated privileges. Set by `-skip_log_before_elevation`. Default: **false**.

`no_debug_priv` (`bool`)

When `true`, the service does not request `SeDebugPrivilege` at startup. This limits the service's ability to open handles to protected processes. Set by `-noDebugPriv` or `-nodebugpriv`. Default: **false**.

`no_inc_base_priority` (`bool`)

When `true`, the service does not request `SeIncreaseBasePriorityPrivilege` at startup. This prevents setting process priority to High or Realtime for other processes. Set by `-noIncBasePriority` or `-noincbasepriority`. Default: **false**.

## Remarks

### Construction

`CliArgs::new()` returns a struct with sensible defaults:

- `interval_ms` = 5000
- `config_file_name` = `"config.ini"`
- All `bool` fields = `false`
- All `Option` fields = `None`
- All `u32` fields = 0

The remaining fields are set to their `Default` trait values via `..Default::default()`. The `#[derive(Default)]` attribute on the struct provides this behavior.

### Mode exclusivity

The mode flags (`help_mode`, `help_all_mode`, `convert_mode`, `autogroup_mode`, `validate_mode`, `process_logs_mode`, `dry_run`, `find_mode`) are not mutually exclusive at the parsing level. However, the [main](../main.rs/main.md) function checks them in a priority order and exits after handling the first active mode:

1. `help_mode`
2. `help_all_mode`
3. `convert_mode`
4. `autogroup_mode`
5. `validate_mode` (checked after config load)
6. `process_logs_mode` (checked after config and blacklist load)

The `find_mode` and `dry_run` flags are compatible with the main polling loop and do not cause early exit. If multiple mutually exclusive mode flags are set, only the highest-priority one takes effect.

### Lifetime

A single `CliArgs` instance is created on the stack in [main](../main.rs/main.md) and lives for the duration of the program. It is passed by shared reference to functions such as [hotreload_config](../config.rs/hotreload_config.md), [hotreload_blacklist](../config.rs/hotreload_blacklist.md), [process_find](../main.rs/process_find.md), and [set_timer_resolution](../winapi.rs/set_timer_resolution.md). The struct is never modified after [parse_args](parse_args.md) returns.

### Console output

Two flags cause console output to be forced (`*get_use_console!() = true`): `-console` (handled directly in [parse_args](parse_args.md)) and `-validate` (which implies `-console`). The `-console` flag does not have a corresponding field in `CliArgs` because it writes directly to the global `USE_CONSOLE` static in the [logging](../logging.rs/README.md) module.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [main](../main.rs/main.md), [hotreload_config](../config.rs/hotreload_config.md), [hotreload_blacklist](../config.rs/hotreload_blacklist.md), [process_find](../main.rs/process_find.md), [set_timer_resolution](../winapi.rs/set_timer_resolution.md) |
| Populated by | [parse_args](parse_args.md) |
| Derives | `Debug`, `Default` |
| Privileges | N/A (data structure only) |

## See Also

| Topic | Link |
|-------|------|
| Argument parser | [parse_args](parse_args.md) |
| Basic help output | [print_help](print_help.md) |
| Full help output | [print_help_all](print_help_all.md) |
| Entry point that consumes CliArgs | [main](../main.rs/main.md) |
| Configuration loading | [read_config](../config.rs/read_config.md) |
| Hot-reload of config | [hotreload_config](../config.rs/hotreload_config.md) |
| Hot-reload of blacklist | [hotreload_blacklist](../config.rs/hotreload_blacklist.md) |