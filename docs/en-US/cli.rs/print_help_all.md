# print_help_all function (cli.rs)

Prints the combined CLI and configuration help text to the console, providing a comprehensive reference of all command-line flags and config file syntax in a single output.

## Syntax

```rust
pub fn print_help_all()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

`print_help_all` is the handler for the `--help-all` CLI flag. It provides a one-stop reference by combining the output of [`print_cli_help`](print_cli_help.md) and [`print_config_help`](print_config_help.md) into a single consecutive output. This is useful for users who want to see all available options and syntax without running multiple help commands.

The function calls [`print_cli_help`](print_cli_help.md) first, followed by [`print_config_help`](print_config_help.md), so the output is organized with CLI flags at the top and config file syntax below. There may be a separator or blank lines between the two sections for readability.

This function is invoked from [`main`](../main.rs/main.md) when [`CliArgs.help_all_mode`](CliArgs.md) is `true`. After printing, `main` exits without entering the main loop.

### Help hierarchy

| Flag | Function | Scope |
| --- | --- | --- |
| `--help` / `-h` | [`print_help`](print_help.md) | Brief usage summary |
| `--help-cli` | [`print_cli_help`](print_cli_help.md) | Detailed CLI flags only |
| `--help-config` | [`print_config_help`](print_config_help.md) | Config file syntax only |
| **`--help-all`** | **print_help_all** | **CLI flags + config syntax combined** |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/cli.rs |
| **Source lines** | L237–L242 |
| **Called by** | [`main`](../main.rs/main.md) when `help_all_mode` is `true` |
| **Calls** | [`print_cli_help`](print_cli_help.md), [`print_config_help`](print_config_help.md) |

## See also

- [print_help](print_help.md)
- [print_cli_help](print_cli_help.md)
- [print_config_help](print_config_help.md)
- [CliArgs struct](CliArgs.md)
- [cli.rs module overview](README.md)