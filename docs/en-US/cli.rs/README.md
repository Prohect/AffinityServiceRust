# cli.rs Module (cli.rs)

The `cli` module handles command-line argument parsing and help text display for the AffinityService application. It defines the [`CliArgs`](CliArgs.md) struct that holds all runtime configuration derived from command-line flags and provides functions for parsing, validation, and help output.

## Overview

This module is the entry point for user-facing configuration. It translates command-line arguments into a structured [`CliArgs`](CliArgs.md) instance consumed by [`main`](../main.rs/main.md). The module also provides layered help output: a brief usage summary, detailed CLI flag documentation, config file syntax reference, and a combined help-all view.

Key behaviors:

- **Unknown arguments are silently ignored** — this allows forward compatibility and prevents crashes on unrecognized flags.
- **Interval validation** — the minimum polling interval is enforced at 16 ms to prevent excessive CPU usage.
- **Config help embedding** — [`get_config_help_lines`](get_config_help_lines.md) returns a template suitable for embedding directly into converted config files as comments.

## Items

### Structs

| Name | Description |
| --- | --- |
| [CliArgs](CliArgs.md) | Holds all parsed command-line arguments and runtime flags. |

### Functions

| Name | Description |
| --- | --- |
| [parse_args](parse_args.md) | Parses a string slice of arguments into a [`CliArgs`](CliArgs.md) struct. |
| [print_help](print_help.md) | Prints a brief usage summary to the console. |
| [print_cli_help](print_cli_help.md) | Prints detailed documentation for all CLI flags. |
| [get_config_help_lines](get_config_help_lines.md) | Returns config file syntax help as a vector of static string lines. |
| [print_config_help](print_config_help.md) | Prints the config file syntax help to the console. |
| [print_help_all](print_help_all.md) | Prints combined CLI and config help (equivalent to `--help-all`). |

## Parsing Flow

1. [`main`](../main.rs/main.md) collects `std::env::args()` and constructs a default `CliArgs`.
2. [`parse_args`](parse_args.md) iterates over the argument list, matching known flags and consuming their values.
3. Validation is applied (e.g., interval minimum of 16 ms).
4. The populated [`CliArgs`](CliArgs.md) is returned to `main` for dispatch.

## Help Modes

| Flag | Function called | Description |
| --- | --- | --- |
| `--help` / `-h` | [print_help](print_help.md) | Brief usage summary |
| `--help-cli` | [print_cli_help](print_cli_help.md) | Detailed CLI flag docs |
| `--help-config` | [print_config_help](print_config_help.md) | Config file syntax reference |
| `--help-all` | [print_help_all](print_help_all.md) | Combined CLI + config help |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/cli.rs` |
| **Called by** | [`main`](../main.rs/main.md) in `src/main.rs` |
| **Key dependencies** | Standard library only |