# cli module (AffinityServiceRust)

The `cli` module implements command-line argument parsing and help text printing for AffinityServiceRust. It defines the [CliArgs](CliArgs.md) struct that captures all runtime options — polling interval, operating modes, debug flags, file paths, and privilege controls — and provides [parse_args](parse_args.md) to populate it from process arguments. The module also exposes several help printers that document basic usage, detailed CLI options, and configuration file format.

## Structs

| Name | Description |
|------|-------------|
| [CliArgs](CliArgs.md) | Runtime configuration populated from command-line arguments. Holds polling interval, mode flags, file paths, privilege toggles, and debug options. |

## Functions

| Name | Description |
|------|-------------|
| [parse_args](parse_args.md) | Parses a string slice of command-line arguments into a [CliArgs](CliArgs.md) instance. |
| [print_help](print_help.md) | Prints a concise usage summary covering the most common options and operating modes. |
| [print_cli_help](print_cli_help.md) | Prints detailed CLI help including all basic arguments, operating modes, and debug/testing options. |
| [get_config_help_lines](get_config_help_lines.md) | Returns a `Vec<&'static str>` containing the configuration file format documentation template. |
| [print_config_help](print_config_help.md) | Prints the configuration file format help by iterating over [get_config_help_lines](get_config_help_lines.md). |
| [print_help_all](print_help_all.md) | Prints the combined full help — CLI options followed by configuration file format — via [print_cli_help](print_cli_help.md) and [print_config_help](print_config_help.md). |

## See Also

| Topic | Link |
|-------|------|
| Configuration parsing | [config.rs](../config.rs/README.md) |
| Main entry point | [main.rs](../main.rs/README.md) |
| Logging infrastructure | [logging.rs](../logging.rs/README.md) |
| Priority enumerations | [priority.rs](../priority.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*