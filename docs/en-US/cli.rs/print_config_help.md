# print_config_help function (cli.rs)

Prints the configuration file format documentation to the active output (console or log file). This function iterates over the lines returned by [get_config_help_lines](get_config_help_lines.md) and writes each one using the project's `log!` macro.

## Syntax

```rust
pub fn print_config_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

`print_config_help` is a thin wrapper around [get_config_help_lines](get_config_help_lines.md). It exists as a separate function so that the help text can be printed independently (e.g., from a future `--help-config` flag) or composed with other help sections as [print_help_all](print_help_all.md) does.

Unlike [print_help](print_help.md) and [print_help_all](print_help_all.md), this function does **not** set the console output flag (`get_use_console!()`) itself. When called as part of `print_help_all`, the flag is already set by the caller; when called standalone, the caller is responsible for ensuring the output destination is configured.

The configuration help text covers:

- **Terminology** — P-core / E-core naming conventions for Intel hybrid CPUs.
- **Config format** — Field-by-field breakdown of the colon-delimited rule syntax.
- **CPU specification formats** — Ranges (`0-7`), individual CPUs (`0;4;8`), hex bitmasks (`0xFF`), and alias references (`*pcore`).
- **Priority levels** — Valid values for process priority, IO priority, and memory priority.
- **Ideal processor syntax** — Module-prefix matching rules with multi-segment support.
- **Process groups** — Named and anonymous `{ }` group blocks.

## Requirements

| | |
|---|---|
| **Module** | `src/cli.rs` |
| **Callers** | [print_help_all](print_help_all.md) |
| **Callees** | [get_config_help_lines](get_config_help_lines.md), `log!` macro |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [cli.rs](README.md) |
| Help text source | [get_config_help_lines](get_config_help_lines.md) |
| Full help printer | [print_help_all](print_help_all.md) |
| CLI help printer | [print_cli_help](print_cli_help.md) |
| Config parser | [read_config](../config.rs/read_config.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*