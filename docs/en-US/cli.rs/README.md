# cli module (AffinityServiceRust)

The `cli` module handles command-line argument parsing and help text output for AffinityServiceRust. It defines the [CliArgs](CliArgs.md) structure that carries all runtime options — polling interval, mode flags, file paths, privilege toggles, and debug knobs — and provides a [parse_args](parse_args.md) function that populates it from the raw argument vector. Several help-printing functions offer progressively more detail, from a basic usage summary to a full reference including configuration file syntax.

All mode flags in `CliArgs` default to `false` and all file paths default to sensible values (`config.ini`, `logs`, `new_processes_results.txt`), so the service can be launched with zero arguments for the common case. The parser is a simple linear scan with no external crate dependency — each flag is matched case-sensitively (with a small number of case-variant aliases such as `-noUAC`/`-nouac`) and value-bearing flags consume the next argument.

## Structs

| Struct | Description |
|--------|-------------|
| [CliArgs](CliArgs.md) | Container for all command-line arguments with default values. Passed by reference throughout the service lifetime. |

## Functions

| Function | Description |
|----------|-------------|
| [parse_args](parse_args.md) | Parses a raw argument slice into a [CliArgs](CliArgs.md) structure. Unknown flags are silently ignored. |
| [print_help](print_help.md) | Prints a concise help message covering common options and operating modes. |
| [print_cli_help](print_cli_help.md) | Prints a detailed CLI reference including debug and testing options. |
| [get_config_help_lines](get_config_help_lines.md) | Returns a `Vec<&'static str>` of configuration file help template lines suitable for embedding in converted configs. |
| [print_config_help](print_config_help.md) | Prints the configuration help template lines to the active log output. |
| [print_help_all](print_help_all.md) | Prints both the detailed CLI help and the configuration help template in one combined output. |

## See Also

| Topic | Link |
|-------|------|
| Entry point that consumes CliArgs | [main](../main.rs/main.md) |
| Configuration file parsing | [config.rs](../config.rs/README.md) |
| Process priority enumerations | [priority.rs](../priority.rs/README.md) |
| Logging infrastructure (console vs. file) | [logging.rs](../logging.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd