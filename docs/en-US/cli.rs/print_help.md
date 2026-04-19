# print_help function (cli.rs)

Prints the basic usage help message for AffinityServiceRust to the console. This is the default help output shown when the user passes `-help`, `--help`, `-?`, `/?`, or `?` on the command line.

## Syntax

```rust
pub fn print_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

The function unconditionally sets the global `use_console` flag to `true` before printing, ensuring that help output is directed to the console rather than a log file. This is necessary because the service normally logs to files, but help text must be visible to the interactive user.

The help text is emitted via the `log!` macro as a single multi-line raw string literal.

### Sections displayed

| Section | Content |
|---------|---------|
| **Header** | One-line description of the service's purpose |
| **Common Options** | `-help`, `-helpall`, `-console`, `-config`, `-find`, `-interval`, `-noUAC`, `-resolution` |
| **Modes** | `-validate`, `-processlogs`, `-dryrun`, `-convert`, `-autogroup` |

### Relationship to other help functions

- `print_help` shows a concise subset of options suitable for quick reference.
- [print_cli_help](print_cli_help.md) shows the full CLI reference, including debug and testing options.
- [print_config_help](print_config_help.md) shows configuration file format documentation.
- [print_help_all](print_help_all.md) combines `print_cli_help` and `print_config_help` into one output.

### Console flag side effect

Because `print_help` forces `use_console = true`, any subsequent logging in the same process invocation will also go to the console. This is intentional — when the user asks for help, they are running interactively and do not expect log file output.

## Requirements

| | |
|---|---|
| **Module** | `src/cli.rs` |
| **Callers** | `main` (when [CliArgs](CliArgs.md)`.help_mode` is `true`) |
| **Callees** | `log!` macro, `get_use_console!` macro |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| CLI argument parser | [parse_args](parse_args.md) |
| Detailed CLI help | [print_cli_help](print_cli_help.md) |
| Config format help | [print_config_help](print_config_help.md) |
| Combined help | [print_help_all](print_help_all.md) |
| CLI arguments struct | [CliArgs](CliArgs.md) |
| Module overview | [cli.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*