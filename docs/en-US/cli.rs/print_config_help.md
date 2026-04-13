# print_config_help function (cli.rs)

Prints the configuration file help template to the active log output. This function iterates over the lines returned by [get_config_help_lines](get_config_help_lines.md) and logs each one individually via the `log!()` macro. It provides a quick-reference for the configuration file syntax when invoked as part of the combined help output.

## Syntax

```rust
pub fn print_config_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

### Implementation

The function is a thin wrapper around [get_config_help_lines](get_config_help_lines.md):

```rust
pub fn print_config_help() {
    for line in get_config_help_lines() {
        log!("{}", line);
    }
}
```

Each line from the template vector is passed to the `log!()` macro, which routes output to either the console (stdout) or the log file depending on the global `USE_CONSOLE` state. The function does not set `USE_CONSOLE` itself — it relies on its caller to have configured the output destination beforehand.

### Console behavior

When called from [print_help_all](print_help_all.md), console output has already been forced by that function (`*get_use_console!() = true`). If `print_config_help` were called in isolation without console mode enabled, the template lines would be written to the log file instead of stdout. In practice, this function is not called standalone — it is always invoked as the second half of [print_help_all](print_help_all.md).

### Output format

The output consists of 24 comment-prefixed lines (each beginning with `##`) that describe the colon-delimited configuration rule format, per-field value options, CPU alias syntax, and group block syntax. The lines are emitted one at a time, each as a separate log entry, which means each line receives a timestamp prefix when written to a log file. When written to the console, the lines appear sequentially without timestamps (the console logger does not prepend timestamps).

### Relationship to other help functions

| Function | Scope | Invoked by |
|----------|-------|------------|
| [print_help](print_help.md) | Common options and modes only | `-help`, `--help`, `-?`, `/?`, `?` |
| [print_cli_help](print_cli_help.md) | Full CLI reference including debug options | [print_help_all](print_help_all.md) |
| **print_config_help** (this) | Configuration file format template | [print_help_all](print_help_all.md) |
| [print_help_all](print_help_all.md) | CLI reference + config template combined | `-helpall`, `--helpall` |

### Invocation flow

This function is never called directly from [main](../main.rs/main.md). The call chain is:

1. User passes `-helpall` → [parse_args](parse_args.md) sets `cli.help_all_mode = true`.
2. [main](../main.rs/main.md) checks `cli.help_all_mode` and calls [print_help_all](print_help_all.md).
3. [print_help_all](print_help_all.md) sets `USE_CONSOLE = true`, calls [print_cli_help](print_cli_help.md), logs a blank separator line, then calls `print_config_help()`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [print_help_all](print_help_all.md) |
| Callees | [get_config_help_lines](get_config_help_lines.md), `log!()` macro |
| API | None |
| Privileges | N/A |

## See Also

| Topic | Link |
|-------|------|
| Config help line provider | [get_config_help_lines](get_config_help_lines.md) |
| Combined CLI + config help | [print_help_all](print_help_all.md) |
| Detailed CLI help | [print_cli_help](print_cli_help.md) |
| Basic help output | [print_help](print_help.md) |
| Configuration file parser | [read_config](../config.rs/read_config.md) |
| CLI arguments structure | [CliArgs](CliArgs.md) |
| Entry point | [main](../main.rs/main.md) |