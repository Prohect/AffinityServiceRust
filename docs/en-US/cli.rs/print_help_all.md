# print_help_all function (cli.rs)

Prints the complete help reference for AffinityServiceRust by combining the detailed CLI options (via [print_cli_help](print_cli_help.md)) and the configuration file format template (via [print_config_help](print_config_help.md)) into a single consolidated output. This is the most comprehensive help available from the command line, invoked when the user passes `-helpall` or `--helpall`.

## Syntax

```rust
pub fn print_help_all()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

### Implementation

The function performs three steps:

1. **Force console output** — Sets the global `USE_CONSOLE` static to `true` via `*get_use_console!() = true`. This ensures all subsequent `log!()` calls within this function and its callees write to stdout rather than the log file, because help output is always intended for interactive review.

2. **Print CLI help** — Calls [print_cli_help](print_cli_help.md) to emit the full command-line reference, including basic arguments, operating modes, debug/testing options, and practical debugging examples.

3. **Print separator** — Logs a blank line (`log!("")`) to visually separate the CLI reference from the configuration template.

4. **Print config help** — Calls [print_config_help](print_config_help.md) to emit the configuration file format template, which describes the colon-delimited rule syntax, per-field value options, CPU alias definitions, and group block syntax.

### Console forcing

This function is the only help function (besides [print_help](print_help.md)) that sets the `USE_CONSOLE` global directly. It does so because neither [print_cli_help](print_cli_help.md) nor [print_config_help](print_config_help.md) set the global themselves — they rely on their caller to have configured the output destination. By setting it once at the top, `print_help_all` ensures both sub-functions emit to the console.

### Output structure

The combined output rendered by this function follows this layout:

```
=== COMMAND LINE OPTIONS ===
  (basic arguments)
  (operating modes)
  (debug & testing options)
  (debugging examples)

=== CONFIGURATION FILE FORMAT ===
  ## ============================================
  ## AffinityServiceRust Configuration File
  ## ============================================
  ## Format: process:priority:affinity:cpuset:prime:io:memory:ideal:grade
  ## (field descriptions)
  ## (alias examples)
  ## (group syntax)
  ## ============================================
```

### Relationship to other help functions

| Function | Scope | Invoked by |
|----------|-------|------------|
| [print_help](print_help.md) | Common options and modes only | `-help`, `--help`, `-?`, `/?`, `?` |
| [print_cli_help](print_cli_help.md) | Full CLI reference including debug options | **print_help_all** (this) |
| [print_config_help](print_config_help.md) | Configuration file format template | **print_help_all** (this) |
| **print_help_all** (this) | CLI reference + config template combined | `-helpall`, `--helpall` |

### Invocation flow

In [main](../main.rs/main.md), the help-all mode check occurs before any configuration is loaded, immediately after the basic help check:

```rust
if cli.help_all_mode {
    print_help_all();
    return Ok(());
}
```

This means the function executes and the program exits without touching the config file, blacklist, privileges, or any other subsystem. It is safe to call even in environments where the configuration file does not exist or is malformed.

### When to use `-helpall` vs `-help`

- Use `-help` (which calls [print_help](print_help.md)) for a quick reminder of the most common options and mode names.
- Use `-helpall` (which calls this function) when you need the full flag reference, debug options, example commands, and the configuration file syntax template in one output.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [main](../main.rs/main.md) (when `cli.help_all_mode` is `true`) |
| Callees | `get_use_console!()` macro, [print_cli_help](print_cli_help.md), [print_config_help](print_config_help.md), `log!()` macro |
| API | None |
| Privileges | N/A |

## See Also

| Topic | Link |
|-------|------|
| Basic help output | [print_help](print_help.md) |
| Detailed CLI help (called internally) | [print_cli_help](print_cli_help.md) |
| Configuration file help (called internally) | [print_config_help](print_config_help.md) |
| Config help line provider | [get_config_help_lines](get_config_help_lines.md) |
| CLI arguments structure | [CliArgs](CliArgs.md) |
| Argument parser | [parse_args](parse_args.md) |
| Entry point | [main](../main.rs/main.md) |