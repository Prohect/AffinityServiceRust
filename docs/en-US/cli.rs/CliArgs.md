# CliArgs struct (cli.rs)

Holds all parsed command-line arguments and runtime flags that control the behavior of AffinityService. This struct is populated by [`parse_args`](parse_args.md) and consumed by [`main`](../main.rs/main.md) to determine which mode to run and how to configure the main loop.

## Syntax

```rust
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

`interval_ms`

The polling interval in milliseconds between main loop iterations. Defaults to a reasonable value and is validated by [`parse_args`](parse_args.md) to be no less than 16 ms to prevent excessive CPU usage.

`help_mode`

When `true`, the application prints a brief usage summary via [`print_help`](print_help.md) and exits. Triggered by `--help` or `-h`.

`help_all_mode`

When `true`, the application prints combined CLI and config help via [`print_help_all`](print_help_all.md) and exits. Triggered by `--help-all`.

`convert_mode`

When `true`, the application enters config conversion mode, translating an old-format configuration file to the current format and exiting. Uses `in_file_name` and `out_file_name` for input/output paths.

`autogroup_mode`

When `true`, the application runs the autogroup sorting/grouping utility on a configuration file. Uses `in_file_name` and `out_file_name` for input/output paths.

`find_mode`

When `true`, the application enters find mode, which logs all discovered process names to the `.find.log` file without applying any configuration changes.

`validate_mode`

When `true`, the application parses and validates the configuration file, reporting any errors or warnings, then exits without entering the main loop.

`process_logs_mode`

When `true`, the application processes `.find.log` files using `es.exe` (Everything search) to discover full executable paths for found process names. See [`process_logs`](../main.rs/process_logs.md).

`dry_run`

When `true`, the application runs the main loop but does not actually apply any configuration changes via Windows APIs. Changes that *would* be made are logged with a dry-run indicator, enabling safe testing of new configurations.

`config_file_name`

The path to the configuration file. Defaults to a standard name adjacent to the executable.

`blacklist_file_name`

Optional path to a blacklist file containing process names that should be excluded from configuration application.

`in_file_name`

Optional input file path used by convert mode and autogroup mode.

`out_file_name`

Optional output file path used by convert mode, autogroup mode, and process-logs mode.

`no_uac`

When `true`, the application does not attempt UAC elevation even if it detects that it is not running with administrator privileges. Useful for environments where UAC prompts are undesirable or when running under a task scheduler that already provides elevation.

`loop_count`

Optional limit on the number of main loop iterations. When `Some(n)`, the application exits after completing `n` iterations. When `None`, the application runs indefinitely until manually stopped. Useful for testing and scripted runs.

`time_resolution`

The Windows multimedia timer resolution in milliseconds, set via `timeBeginPeriod`. Controls the granularity of `Sleep` calls in the main loop. Lower values provide more precise timing but increase system overhead.

`log_loop`

When `true`, the application logs a marker message at the start of each loop iteration, making it easier to correlate log entries with specific iterations during debugging.

`skip_log_before_elevation`

When `true`, enables [`DUST_BIN_MODE`](../logging.rs/DUST_BIN_MODE.md) before UAC elevation, suppressing all log output from the non-elevated instance to avoid creating an abandoned log file.

`no_debug_priv`

When `true`, the application skips the call to [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md) during startup. This limits the application's ability to open handles to processes owned by other users but may be appropriate in restricted environments.

`no_inc_base_priority`

When `true`, the application skips the call to [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) during startup. This prevents the application from setting process priority classes above `Normal` for other users' processes.

## Remarks

`CliArgs` is constructed with default values and then mutated by [`parse_args`](parse_args.md), which iterates over the command-line argument list and sets each field according to the flags encountered. Unknown arguments are silently ignored for forward compatibility — this allows newer flags to be passed without breaking older versions of the application.

The struct is consumed by [`main`](../main.rs/main.md), which inspects the mode flags to determine the execution path:

1. If any help mode is set, the corresponding help function is called and the application exits.
2. If `convert_mode` or `autogroup_mode` is set, the corresponding utility runs and the application exits.
3. If `validate_mode` is set, the configuration file is parsed and validated, then the application exits.
4. If `process_logs_mode` is set, the log processing utility runs and the application exits.
5. Otherwise, the main loop begins with the configured interval, applying configurations each iteration.

### Default values

Most boolean flags default to `false`. The `interval_ms` has a sensible default (typically 1000 ms or similar), and `config_file_name` defaults to a conventional name next to the executable. See [`parse_args`](parse_args.md) for details on the defaults and validation rules.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/cli.rs |
| **Lines** | L5–L26 |
| **Created by** | [`parse_args`](parse_args.md) |
| **Consumed by** | [`main`](../main.rs/main.md) |

## See also

- [parse_args function](parse_args.md)
- [print_help function](print_help.md)
- [print_help_all function](print_help_all.md)
- [cli.rs module overview](README.md)
- [DUST_BIN_MODE](../logging.rs/DUST_BIN_MODE.md) (controlled by `skip_log_before_elevation`)
- [enable_debug_privilege](../winapi.rs/enable_debug_privilege.md) (controlled by `no_debug_priv`)
- [enable_inc_base_priority_privilege](../winapi.rs/enable_inc_base_priority_privilege.md) (controlled by `no_inc_base_priority`)