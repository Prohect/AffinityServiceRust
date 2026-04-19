# CliArgs struct (cli.rs)

Holds all parsed command-line arguments that control the runtime behavior of AffinityServiceRust. A single instance is created at startup via `new()`, populated by [parse_args](parse_args.md), and then passed by reference throughout the application lifetime.

## Syntax

```rust
#[derive(Debug, Default)]
pub struct CliArgs {
    pub interval_ms: u32,
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
    pub no_etw: bool,
    pub continuous_process_level_apply: bool,
}
```

## Members

| Member | Type | Default | Description |
|--------|------|---------|-------------|
| `interval_ms` | `u32` | `5000` | Polling interval in milliseconds between apply cycles. Clamped to the range \[16, 86 400 000\] during parsing. |
| `help_mode` | `bool` | `false` | When `true`, print basic usage help via [print_help](print_help.md) and exit. Triggered by `-help`, `--help`, `-?`, `/?`, or `?`. |
| `help_all_mode` | `bool` | `false` | When `true`, print full help (CLI options + config format) via [print_help_all](print_help_all.md) and exit. Triggered by `-helpall` or `--helpall`. |
| `convert_mode` | `bool` | `false` | Activates Process Lasso configuration converter mode. Requires `-in` and `-out` file arguments. |
| `autogroup_mode` | `bool` | `false` | Activates auto-grouping of rules with identical settings into named group blocks. Requires `-in` and `-out` file arguments. |
| `find_mode` | `bool` | `false` | Finds processes whose CPU affinity matches the system default (i.e., unmanaged processes). Optionally filtered by a `-blacklist` file. |
| `validate_mode` | `bool` | `false` | Validates the config file for syntax errors and undefined aliases, then exits. Implicitly enables console output. |
| `process_logs_mode` | `bool` | `false` | Processes log files from `-find` mode to discover new processes and their executable paths. Uses `-config`, `-blacklist`, `-in` (logs directory), and `-out` (results file). |
| `dry_run` | `bool` | `false` | Simulates the apply cycle without calling any Windows APIs that modify process or thread state. Changes are logged as if they were applied. |
| `config_file_name` | `String` | `"config.ini"` | Path to the configuration file. Overridden by the `-config <file>` argument. |
| `blacklist_file_name` | `Option<String>` | `None` | Optional path to a blacklist file used by `-find` and `-processlogs` modes to exclude known processes. |
| `in_file_name` | `Option<String>` | `None` | Input file path for `-convert` mode, or input logs directory for `-processlogs` mode. |
| `out_file_name` | `Option<String>` | `None` | Output file path for `-convert`, `-autogroup`, and `-processlogs` modes. |
| `no_uac` | `bool` | `false` | Skips the automatic UAC elevation request at startup. Useful for debugging without administrator privileges. |
| `loop_count` | `Option<u32>` | `None` | When set, limits the service to a finite number of polling iterations. Minimum value is 1. Primarily used for testing. When `None`, the service runs indefinitely. |
| `time_resolution` | `u32` | `0` | Windows timer resolution in 100-nanosecond units (e.g., `5210` = 0.5210 ms). A value of `0` means do not modify the system timer resolution. |
| `log_loop` | `bool` | `false` | Logs a message at the start of each polling iteration. Useful for verifying loop timing during debugging. |
| `skip_log_before_elevation` | `bool` | `false` | Suppresses all log output before UAC elevation completes. Prevents duplicate log entries when the process re-launches itself as administrator. |
| `no_debug_priv` | `bool` | `false` | Skips requesting `SeDebugPrivilege` at startup. Without this privilege, the service cannot open handles to system processes. |
| `no_inc_base_priority` | `bool` | `false` | Skips requesting `SeIncreaseBasePriorityPrivilege` at startup. Without this privilege, setting processes to `High` or `Realtime` priority may fail. |
| `no_etw` | `bool` | `false` | Disables ETW (Event Tracing for Windows) process-start monitoring. When disabled, newly launched processes are only detected during the next polling interval rather than in real time. |
| `continuous_process_level_apply` | `bool` | `false` | When `true`, process-level settings (priority, affinity, CPU set, IO priority, memory priority) are re-applied on every polling iteration instead of only once per PID. Useful when external tools may reset process attributes. |

## Methods

### new

```rust
pub fn new() -> Self
```

Creates a new `CliArgs` with default values. Sets `interval_ms` to `5000` and `config_file_name` to `"config.ini"`; all other fields use their `Default` trait values (`false`, `None`, `0`, or empty string).

**Return value**

A new `CliArgs` instance with default configuration.

## Remarks

- The struct derives both `Debug` and `Default`. The manual `new()` constructor overrides two fields from the `Default` implementation; all other fields delegate to `..Default::default()`.
- `CliArgs` is created once in `main` and passed by shared reference (`&CliArgs`) to the polling loop, configuration hot-reload, and apply functions. It is never mutated after [parse_args](parse_args.md) completes.
- Boolean mode flags (`convert_mode`, `find_mode`, `validate_mode`, etc.) are mutually exclusive by convention but not enforced at the type level. If multiple modes are set, the first one checked in `main` takes precedence.

## Requirements

| | |
|---|---|
| **Module** | `src/cli.rs` |
| **Callers** | `main`, [parse_args](parse_args.md), configuration hot-reload, apply loop, [hotreload_config](../config.rs/hotreload_config.md), [hotreload_blacklist](../config.rs/hotreload_blacklist.md) |
| **Dependencies** | None (plain data struct) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [cli.rs](README.md) |
| Argument parser | [parse_args](parse_args.md) |
| Basic help output | [print_help](print_help.md) |
| Full help output | [print_help_all](print_help_all.md) |
| Configuration file types | [ConfigResult](../config.rs/ConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*