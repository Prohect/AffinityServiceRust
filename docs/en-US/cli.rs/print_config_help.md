# print_config_help function (cli.rs)

Prints the full configuration file reference template to the current log output. This function iterates over the lines returned by [get_config_help_lines](get_config_help_lines.md) and writes each one using the `log!` macro.

## Syntax

```AffinityServiceRust/src/cli.rs#L262-266
pub fn print_config_help() {
    for line in get_config_help_lines() {
        log!("{}", line);
    }
}
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

`print_config_help` is a thin wrapper around [get_config_help_lines](get_config_help_lines.md) that sends each returned line to the active logging sink via the `log!` macro. Unlike [print_help](print_help.md) and [print_help_all](print_help_all.md), this function does **not** set the console output flag — the caller is responsible for ensuring that output is routed to the desired destination before calling this function.

The printed content covers the complete configuration file syntax reference, including:

- Terminology for P-cores, E-cores, and HyperThreading notation
- Configuration line format and field descriptions
- All supported CPU specification formats (ranges, hex masks, individual indices, aliases)
- Priority level enumerations for process priority, I/O priority, and memory priority
- Ideal processor syntax with module-prefix filtering
- Process group syntax using `{ }` blocks

This function is called by [print_help_all](print_help_all.md) as the second part of the combined help output, and is also used independently when only the config-file reference is needed.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [print_help_all](print_help_all.md), external entry points |
| Callees | [get_config_help_lines](get_config_help_lines.md), `log!` macro |
| API | Internal |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| get_config_help_lines | [get_config_help_lines](get_config_help_lines.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| print_help | [print_help](print_help.md) |
| print_cli_help | [print_cli_help](print_cli_help.md) |
| cli module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*