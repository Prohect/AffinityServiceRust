# print_cli_help function (cli.rs)

Prints the detailed CLI help message to the console or log, covering all command-line options including basic arguments, operating modes, and debug/testing options.

## Syntax

```rust
pub fn print_cli_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

Unlike [print_help](print_help.md), which displays only the most common options, `print_cli_help` outputs the complete reference for every supported command-line flag and argument. The output is organized into three sections:

1. **Basic Arguments** — General-purpose flags such as `-help`, `-console`, `-noUAC`, `-config`, `-find`, `-blacklist`, `-interval`, and `-resolution`.
2. **Operating Modes** — Task-specific modes including `-validate`, `-processlogs`, `-dryrun`, `-convert`, `-autogroup`, and the `-in` / `-out` file arguments they consume.
3. **Debug & Testing Options** — Flags intended for development and troubleshooting: `-loop`, `-logloop`, `-noDebugPriv`, `-noIncBasePriority`, `-no_etw`, and `-continuous_process_level_apply`.

The output also includes a **Debugging** section with ready-to-paste example command lines for both non-admin (console) and admin (log-file) testing scenarios, along with a note explaining that `-console` output is lost when UAC elevation spawns a new session.

### Console side effect

This function does **not** set the console output flag itself. It is the caller's responsibility to ensure console output is enabled before calling this function. In practice, [print_help_all](print_help_all.md) sets the console flag before delegating to `print_cli_help`.

### Output mechanism

All output is written through the project's `log!` macro, which routes to either the console or the log file depending on the current `use_console` global state.

## Requirements

| | |
|---|---|
| **Module** | `src/cli.rs` |
| **Callers** | [print_help_all](print_help_all.md) |
| **Callees** | `log!` macro |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [cli.rs](README.md) |
| Basic help | [print_help](print_help.md) |
| Config format help | [print_config_help](print_config_help.md) |
| Combined help | [print_help_all](print_help_all.md) |
| Argument parser | [parse_args](parse_args.md) |
| CLI state struct | [CliArgs](CliArgs.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*