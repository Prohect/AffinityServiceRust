# print_help function (cli.rs)

Prints a brief usage summary to the console, showing the application name, basic invocation syntax, and pointers to more detailed help commands.

## Syntax

```rust
pub fn print_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

`print_help` is the handler for the `--help` / `-h` CLI flag. It outputs a concise overview of the application's purpose and the most common invocation patterns, along with references to the more detailed help commands (`--help-cli`, `--help-config`, `--help-all`).

The output is written directly to stdout via `println!` and is intended for interactive console use. The function does not interact with the logging system ([`log_message`](../logging.rs/log_message.md)) since help output is user-facing and should not be written to log files.

This is the shortest of the help functions — it provides just enough information for a user to understand what the application does and how to get more detailed help. For comprehensive documentation, the user should use [`print_help_all`](print_help_all.md) (triggered by `--help-all`).

### Help hierarchy

| Flag | Function | Scope |
| --- | --- | --- |
| `--help` / `-h` | **print_help** | Brief usage summary |
| `--help-cli` | [`print_cli_help`](print_cli_help.md) | Detailed CLI flag documentation |
| `--help-config` | [`print_config_help`](print_config_help.md) | Config file syntax reference |
| `--help-all` | [`print_help_all`](print_help_all.md) | Combined CLI + config help |

After printing, the application exits without entering the main loop. The exit is handled by the caller in [`main`](../main.rs/main.md) based on the `help_mode` flag in [`CliArgs`](CliArgs.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/cli.rs |
| **Source lines** | L121–L147 |
| **Called by** | [`main`](../main.rs/main.md) when `CliArgs.help_mode` is `true` |

## See also

- [print_cli_help](print_cli_help.md)
- [print_config_help](print_config_help.md)
- [print_help_all](print_help_all.md)
- [CliArgs struct](CliArgs.md)
- [cli.rs module overview](README.md)