# print_help_all function (cli.rs)

Prints both the detailed CLI help and the full configuration file reference to the console. This is the handler for the `-helpall` / `--helpall` command-line flag and provides users with comprehensive documentation of every available option and the configuration file syntax.

## Syntax

```AffinityServiceRust/src/cli.rs#L267-L271
pub fn print_help_all() {
    *get_use_console!() = true;
    print_cli_help();
    log!("");
    print_config_help();
}
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

`print_help_all` is a composition of [print_cli_help](print_cli_help.md) and [print_config_help](print_config_help.md), separated by a blank line. It forces console output by setting the global `use_console` flag to `true` via the `get_use_console!()` macro before printing, ensuring that the combined help text is always displayed interactively rather than written to a log file.

The output is divided into two major sections:

1. **CLI options** — produced by `print_cli_help`, covering all command-line arguments, operating modes, and debug/testing options with usage examples.
2. **Configuration file reference** — produced by `print_config_help`, covering terminology, config format, CPU specification formats, priority levels, ideal processor syntax, and process group syntax.

This function is invoked when the user passes `-helpall` or `--helpall` on the command line. The `help_all_mode` flag on [CliArgs](CliArgs.md) is set by [parse_args](parse_args.md), and the main module calls `print_help_all` in response.

### Activation

The flag is triggered by any of these command-line tokens:

| Token | Effect |
|-------|--------|
| `-helpall` | Sets `cli.help_all_mode = true` |
| `--helpall` | Sets `cli.help_all_mode = true` |

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` (`src/cli.rs`) |
| Callers | `main` (when `cli.help_all_mode` is `true`) |
| Callees | [print_cli_help](print_cli_help.md), [print_config_help](print_config_help.md), `get_use_console!()`, `log!()` |
| Platform | Windows (console output via `log!` macro) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| print_help | [print_help](print_help.md) |
| print_cli_help | [print_cli_help](print_cli_help.md) |
| print_config_help | [print_config_help](print_config_help.md) |
| get_config_help_lines | [get_config_help_lines](get_config_help_lines.md) |
| CliArgs | [CliArgs](CliArgs.md) |
| parse_args | [parse_args](parse_args.md) |
| config module | [config.rs overview](../config.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
