# print_cli_help function (cli.rs)

Prints a detailed help message to the log output that covers every command-line argument, operating mode, and debug/testing option supported by AffinityServiceRust. This is the extended counterpart to [print_help](print_help.md) and is invoked when the user passes `-helpall`.

## Syntax

```AffinityServiceRust/src/cli.rs#L139-141
pub fn print_cli_help() {
    // ...
}
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value. Output is written to the current log target via the `log!` macro.

## Remarks

Unlike [print_help](print_help.md), this function does **not** force console mode by setting `get_use_console!()`. It is expected that the caller has already enabled console output before invoking this function (as [print_help_all](print_help_all.md) does).

The detailed help text is organized into the following sections:

| Section | Content |
|---------|---------|
| **Basic Arguments** | `-help`, `-console`, `-noUAC`, `-config`, `-find`, `-blacklist`, `-interval`, `-resolution` |
| **Operating Modes** | `-validate`, `-processlogs`, `-dryrun`, `-convert`, `-autogroup`, `-in`, `-out` |
| **Debug & Testing Options** | `-loop`, `-logloop`, `-noDebugPriv`, `-noIncBasePriority`, `-no_etw`, `-continuous_process_level_apply` |
| **Debugging** | Quick-start debug commands for both non-admin and admin scenarios, including a note about UAC and console session limitations |

Each argument entry documents:
- The flag name(s) and any accepted aliases (e.g., `-noUAC | -nouac`)
- Whether the flag requires a subsequent value argument
- The default value when not specified
- Valid ranges or constraints (e.g., interval minimum of 16 ms)

### Platform notes

The note at the end of the help text warns that when the process elevates via UAC, a new logon session is created. Console output from the elevated session cannot be displayed in the original terminal window, so log files should be used instead.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli.rs` |
| Callers | [print_help_all](print_help_all.md) |
| Callees | `log!` macro (see [logging.rs](../logging.rs/README.md)) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| print_help | [print_help](print_help.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| print_config_help | [print_config_help](print_config_help.md) |
| parse_args | [parse_args](parse_args.md) |
| CliArgs | [CliArgs](CliArgs.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*