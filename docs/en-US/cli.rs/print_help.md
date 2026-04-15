# print_help function (cli.rs)

Prints a concise help message to the console showing the most common command-line options and operating modes for AffinityServiceRust. This is the default help output displayed when the user passes `-help`, `--help`, `-?`, `/?`, or `?`.

## Syntax

```AffinityServiceRust/src/cli.rs#L131-L157
pub fn print_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

- **Console activation**: `print_help` unconditionally sets the global `use_console` flag to `true` before emitting output, ensuring the help text is written to the console even when the service is configured for file-based logging.

- **Output destination**: Text is emitted through the `log!` macro, which respects the `use_console` flag set at the top of the function body.

- **Content sections**: The printed help message is organized into two groups:
  - **Common Options** — covers `-help`, `-helpall`, `-console`, `-config`, `-find`, `-interval`, `-noUAC`, and `-resolution`.
  - **Modes** — covers `-validate`, `-processlogs`, `-dryrun`, `-convert`, and `-autogroup`.

- **Relationship to other help functions**: `print_help` provides a brief overview. For the full CLI reference including debug and testing options, see [print_cli_help](print_cli_help.md). For the combined CLI + configuration file reference, see [print_help_all](print_help_all.md).

- This function is invoked when `CliArgs.help_mode` is `true`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli.rs` |
| Callers | `main.rs` (when `-help` / `--help` / `-?` / `/?` / `?` is passed) |
| Callees | `log!` macro, `get_use_console!` macro |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| CliArgs struct | [CliArgs](CliArgs.md) |
| parse_args function | [parse_args](parse_args.md) |
| print_cli_help function | [print_cli_help](print_cli_help.md) |
| print_help_all function | [print_help_all](print_help_all.md) |
| config module | [config.rs overview](../config.rs/README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*