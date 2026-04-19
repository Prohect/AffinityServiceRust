# print_help_all function (cli.rs)

Prints the complete help output for AffinityServiceRust, combining both command-line usage and configuration file format documentation into a single reference. This is the handler for the `-helpall` / `--helpall` command-line flag.

## Syntax

```rust
pub fn print_help_all()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

`print_help_all` is the entry point for the `-helpall` / `--helpall` command-line flag. It performs three actions in sequence:

1. **Forces console output** — Sets the global `use_console` flag to `true` via `get_use_console!()`, ensuring all subsequent output goes to the interactive console rather than a log file.
2. **Prints CLI help** — Calls [print_cli_help](print_cli_help.md) to display the complete command-line argument reference, including basic arguments, operating modes, debug/testing options, and example debug commands.
3. **Prints config help** — Emits a blank separator line via `log!("")`, then calls [print_config_help](print_config_help.md) to display the configuration file format documentation covering terminology, rule syntax, CPU specification formats, priority levels, ideal processor syntax, and process group syntax.

### Console flag side effect

Because `print_help_all` forces `use_console = true`, any subsequent logging in the same process invocation will also go to the console. This is intentional — the user is running interactively and expects all output on screen.

### Relationship to other help functions

| Function | Scope | Sets console flag? |
|----------|-------|-------------------|
| [print_help](print_help.md) | Concise usage summary | Yes |
| [print_cli_help](print_cli_help.md) | Full CLI reference only | No |
| [print_config_help](print_config_help.md) | Config format reference only | No |
| **print_help_all** | Full CLI + config format | Yes |

`print_help_all` is the only function that composes `print_cli_help` and `print_config_help` together. It is also one of only two help functions (along with `print_help`) that sets the console output flag, because the other two are designed to be called as building blocks by composing functions.

## Requirements

| | |
|---|---|
| **Module** | `src/cli.rs` |
| **Callers** | `main` (when [CliArgs](CliArgs.md)`.help_all_mode` is `true`) |
| **Callees** | `get_use_console!` macro, `log!` macro, [print_cli_help](print_cli_help.md), [print_config_help](print_config_help.md) |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [cli.rs](README.md) |
| Basic help output | [print_help](print_help.md) |
| CLI help (called internally) | [print_cli_help](print_cli_help.md) |
| Config help (called internally) | [print_config_help](print_config_help.md) |
| Config help line source | [get_config_help_lines](get_config_help_lines.md) |
| CLI arguments struct | [CliArgs](CliArgs.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*