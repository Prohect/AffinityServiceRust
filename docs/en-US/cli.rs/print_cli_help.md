# print_cli_help function (cli.rs)

Prints detailed documentation for all supported command-line flags and their usage to the console.

## Syntax

```rust
pub fn print_cli_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

`print_cli_help` outputs comprehensive documentation for every CLI flag recognized by the application, including descriptions, default values, and usage examples. It provides more detail than the brief summary produced by [`print_help`](print_help.md), covering each flag's full semantics.

The output includes documentation for flags such as:

- `--interval` / `-i` — polling interval in milliseconds
- `--config` / `-c` — configuration file path
- `--blacklist` / `-b` — blacklist file path
- `--dry-run` — simulate changes without applying
- `--find` — process discovery mode
- `--validate` — config validation mode
- `--convert` — config format conversion mode
- `--autogroup` — autogroup management mode
- `--process-logs` — find log processing mode
- `--no-uac` — suppress UAC elevation
- `--no-debug-priv` — skip `SeDebugPrivilege` acquisition
- `--no-inc-base-priority` — skip `SeIncreaseBasePriorityPrivilege` acquisition
- `--loop-count` — limit the number of main loop iterations
- `--time-resolution` — system timer resolution override
- `--log-loop` — enable per-loop diagnostic logging
- `--skip-log-before-elevation` — suppress logging before UAC re-launch

This function is invoked when the user passes `--help-cli` on the command line, as detected during [`parse_args`](parse_args.md). It is also called as part of [`print_help_all`](print_help_all.md), which combines CLI and config help into a single output.

The output is written directly to stdout via `println!`. It does not use the [`log_message`](../logging.rs/log_message.md) logging system, since help output is intended for immediate console display rather than log file recording.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/cli.rs |
| **Source lines** | L149–L197 |
| **Called by** | [`main`](../main.rs/main.md) (when `help_mode` or `help_all_mode` is set), [`print_help_all`](print_help_all.md) |

## See also

- [print_help](print_help.md)
- [print_config_help](print_config_help.md)
- [print_help_all](print_help_all.md)
- [CliArgs struct](CliArgs.md)
- [cli.rs module overview](README.md)