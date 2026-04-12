# parse_args function (cli.rs)

Parses a slice of command-line argument strings into a [`CliArgs`](CliArgs.md) struct, validating values and applying defaults where appropriate.

## Syntax

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## Parameters

`args`

A slice of command-line argument strings, typically obtained from `std::env::args().collect()`. The first element is expected to be the executable path and is skipped during parsing.

`cli`

A mutable reference to a [`CliArgs`](CliArgs.md) struct that will be populated with the parsed values. The struct should be initialized with defaults before being passed to this function.

## Return value

Returns `Ok(())` on successful parsing. Returns `Err` if a critical validation failure occurs (e.g., a required value is missing for a flag that takes an argument).

## Remarks

The function iterates over the argument slice, matching each argument against known flags and consuming associated values. The parsing follows these conventions:

- **Boolean flags** (e.g., `--dry-run`, `--no-uac`, `--find`) set their corresponding field to `true` when present.
- **Value flags** (e.g., `--interval`, `--config`, `--loop-count`) consume the next argument as their value.
- **Unknown arguments are silently ignored** — this is a deliberate design choice for forward compatibility, allowing newer flags to be passed without crashing older versions of the application.

### Validation

After parsing, the function performs validation:

- **Interval minimum** — if `interval_ms` is set to a value below 16 ms, it is clamped to 16 ms. This prevents excessively tight polling loops that would consume unnecessary CPU time.
- **File path defaults** — if `config_file_name` is not specified, a default path is used.

### Supported flags

| Flag | Field | Type | Description |
| --- | --- | --- | --- |
| `--interval` / `-i` | `interval_ms` | `u64` | Polling interval in milliseconds |
| `--help` / `-h` | `help_mode` | `bool` | Show brief help |
| `--help-all` | `help_all_mode` | `bool` | Show combined CLI + config help |
| `--convert` | `convert_mode` | `bool` | Convert legacy config format |
| `--autogroup` | `autogroup_mode` | `bool` | Auto-group config entries |
| `--find` | `find_mode` | `bool` | Find mode — discover running processes |
| `--validate` | `validate_mode` | `bool` | Validate config without applying |
| `--process-logs` | `process_logs_mode` | `bool` | Process .find.log files |
| `--dry-run` | `dry_run` | `bool` | Simulate changes without applying |
| `--config` / `-c` | `config_file_name` | `String` | Config file path |
| `--blacklist` | `blacklist_file_name` | `Option<String>` | Blacklist file path |
| `--in` | `in_file_name` | `Option<String>` | Input file for convert mode |
| `--out` | `out_file_name` | `Option<String>` | Output file for convert mode |
| `--no-uac` | `no_uac` | `bool` | Skip UAC elevation |
| `--loop-count` | `loop_count` | `Option<u32>` | Limit iterations then exit |
| `--time-resolution` | `time_resolution` | `u32` | Windows timer resolution |
| `--log-loop` | `log_loop` | `bool` | Log every loop iteration |
| `--skip-log-before-elevation` | `skip_log_before_elevation` | `bool` | Suppress logs before UAC |
| `--no-debug-priv` | `no_debug_priv` | `bool` | Skip SeDebugPrivilege |
| `--no-inc-base-priority` | `no_inc_base_priority` | `bool` | Skip SeIncreaseBasePriorityPrivilege |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/cli.rs |
| **Source lines** | L38–L119 |
| **Called by** | [`main`](../main.rs/main.md) |
| **Produces** | [`CliArgs`](CliArgs.md) |

## See also

- [CliArgs struct](CliArgs.md)
- [print_help](print_help.md)
- [print_help_all](print_help_all.md)
- [cli.rs module overview](README.md)