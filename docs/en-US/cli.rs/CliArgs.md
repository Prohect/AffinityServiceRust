# CliArgs type (cli.rs)

The `CliArgs` struct holds all command-line options and flags that control the runtime behavior of the AffinityServiceRust Windows service. It is populated by [`parse_args`](parse_args.md) and consumed throughout the application to determine operating mode, polling interval, file paths, privilege requests, and debug settings.

## Syntax

```AffinityServiceRust/src/cli.rs#L5-28
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
| `interval_ms` | `u32` | `5000` | Polling interval in milliseconds between process-scanning loops. Clamped to the range 16–86 400 000 by [`parse_args`](parse_args.md). |
| `help_mode` | `bool` | `false` | When `true`, the service prints the concise help message and exits. Set by `-help`, `--help`, `-?`, `/?`, or `?`. |
| `help_all_mode` | `bool` | `false` | When `true`, the service prints the detailed help (CLI + config reference) and exits. Set by `-helpall` or `--helpall`. |
| `convert_mode` | `bool` | `false` | Enables Process Lasso config conversion mode. Requires `-in` and `-out`. Set by `-convert`. |
| `autogroup_mode` | `bool` | `false` | Enables auto-grouping of rules with identical settings. Requires `-in` and `-out`. Set by `-autogroup`. |
| `find_mode` | `bool` | `false` | Enables process-discovery mode that logs processes running with default (all-core) affinity. Set by `-find`. |
| `validate_mode` | `bool` | `false` | Validates the config file for syntax errors and undefined aliases, then exits. Also forces console output. Set by `-validate`. |
| `process_logs_mode` | `bool` | `false` | Enables log-processing mode that scans find-mode logs to discover new processes. Set by `-processlogs`. |
| `dry_run` | `bool` | `false` | When `true`, the service simulates changes without applying them. Set by `-dryrun`, `-dry-run`, or `--dry-run`. |
| `config_file_name` | `String` | `"config.ini"` | Path to the configuration file. Set by `-config <file>`. |
| `blacklist_file_name` | `Option<String>` | `None` | Optional path to a blacklist file used by find mode. Set by `-blacklist <file>`. |
| `in_file_name` | `Option<String>` | `None` | Input file path for `-convert`, or logs directory for `-processlogs`. Set by `-in <file>`. |
| `out_file_name` | `Option<String>` | `None` | Output file path for `-convert`, `-autogroup`, or `-processlogs`. Set by `-out <file>`. |
| `no_uac` | `bool` | `false` | Disables the UAC elevation request on startup. Set by `-noUAC` or `-nouac`. |
| `loop_count` | `Option<u32>` | `None` | When set, limits the number of polling loops (minimum 1). `None` means infinite. Set by `-loop <count>`. |
| `time_resolution` | `u32` | `0` | Windows timer resolution in 100-nanosecond units (e.g., `5210` → 0.5210 ms). `0` means do not modify. Set by `-resolution <t>`. |
| `log_loop` | `bool` | `false` | Logs a diagnostic message at the start of each polling loop. Set by `-logloop`. |
| `skip_log_before_elevation` | `bool` | `false` | Suppresses log output before UAC elevation completes. Set by `-skip_log_before_elevation`. |
| `no_debug_priv` | `bool` | `false` | Skips requesting `SeDebugPrivilege` at startup. Set by `-noDebugPriv` or `-nodebugpriv`. |
| `no_inc_base_priority` | `bool` | `false` | Skips requesting `SeIncreaseBasePriorityPrivilege` at startup. Set by `-noIncBasePriority` or `-noincbasepriority`. |
| `no_etw` | `bool` | `false` | Disables ETW (Event Tracing for Windows) tracing. Set by `-no_etw` or `-noetw`. |
| `continuous_process_level_apply` | `bool` | `false` | Re-applies process-level settings (priority, affinity, CPU set, I/O priority, memory priority) on every polling iteration instead of only once per PID. Set by `-continuous_process_level_apply`. |

## Remarks

- `CliArgs` derives `Debug` and `Default`. The `Default` derivation zero-initializes all fields; use `CliArgs::new()` to obtain an instance with production defaults (`interval_ms = 5000`, `config_file_name = "config.ini"`).
- Operating modes (`convert_mode`, `autogroup_mode`, `find_mode`, `validate_mode`, `process_logs_mode`) are mutually exclusive by convention. Setting more than one simultaneously produces undefined behavior at the application level; the parser does not enforce exclusivity.
- When `validate_mode` is set, console output is force-enabled so validation results are visible regardless of whether `-console` was also specified.
- The `loop_count` field is useful for integration testing; combined with `-logloop` and `-interval`, it bounds the runtime of a test session.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli.rs` |
| Constructed by | `CliArgs::new()` |
| Populated by | [`parse_args`](parse_args.md) |
| Consumed by | `main.rs`, [`read_config`](../config.rs/read_config.md), [`hotreload_config`](../config.rs/hotreload_config.md), [`hotreload_blacklist`](../config.rs/hotreload_blacklist.md), [`convert`](../config.rs/convert.md), [`sort_and_group_config`](../config.rs/sort_and_group_config.md) |

## See Also

| Resource | Link |
|----------|------|
| parse_args | [parse_args](parse_args.md) |
| config module | [config.rs overview](../config.rs/README.md) |
| cli module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
