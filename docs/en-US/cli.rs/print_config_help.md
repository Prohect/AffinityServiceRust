# print_config_help function (cli.rs)

Prints the configuration file syntax reference to the console, documenting all supported directives, fields, and their formats.

## Syntax

```rust
pub fn print_config_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value. Output is printed directly to stdout.

## Remarks

`print_config_help` retrieves the help text from [`get_config_help_lines`](get_config_help_lines.md) and prints each line to the console. This provides a quick reference for users who need to understand the configuration file format without consulting external documentation.

The function is invoked when the `--help-config` flag is passed on the command line. It is also called as part of [`print_help_all`](print_help_all.md), which combines CLI and config help into a single output.

The output covers all configuration directives including:

- Process matching rules and group syntax
- Priority, affinity, and CPU set fields
- I/O priority and memory priority fields
- Prime thread scheduling options
- Ideal processor prefix rules
- CPU alias definitions
- Constants for tuning the prime thread scheduler

This is the same content returned by [`get_config_help_lines`](get_config_help_lines.md), which is also used to embed help text as comments in converted configuration files.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/cli.rs |
| **Source lines** | L231–L235 |
| **Called by** | [`main`](../main.rs/main.md) (when `--help-config` is passed), [`print_help_all`](print_help_all.md) |
| **Calls** | [`get_config_help_lines`](get_config_help_lines.md) |

## See also

- [get_config_help_lines](get_config_help_lines.md)
- [print_help](print_help.md)
- [print_cli_help](print_cli_help.md)
- [print_help_all](print_help_all.md)
- [cli.rs module overview](README.md)