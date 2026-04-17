# cli module (AffinityServiceRust)

The `cli` module provides command-line argument parsing and help text generation for the AffinityServiceRust Windows service. It defines the `CliArgs` structure that holds all runtime configuration derived from command-line arguments, and exposes functions to parse arguments, display usage information, and emit configuration file documentation.

## Functions

| Function | Description |
|----------|-------------|
| [parse_args](parse_args.md) | Parses a slice of command-line argument strings into a `CliArgs` instance. |
| [print_help](print_help.md) | Prints a concise help message showing common options and operating modes. |
| [print_cli_help](print_cli_help.md) | Prints a detailed help message including all arguments, debug options, and usage examples. |
| [get_config_help_lines](get_config_help_lines.md) | Returns a vector of static string slices containing the configuration file reference template. |
| [print_config_help](print_config_help.md) | Prints the configuration file reference to the current log output. |
| [print_help_all](print_help_all.md) | Prints both the detailed CLI help and the full configuration file reference. |

## Structs

| Struct | Description |
|--------|-------------|
| [CliArgs](CliArgs.md) | Holds all command-line options and flags that control the runtime behavior of the service. |

## See Also

| Resource | Link |
|----------|------|
| config module | [config.rs overview](../config.rs/README.md) |
| main module | [main.rs overview](../main.rs/README.md) |
| logging module | [logging.rs overview](../logging.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
