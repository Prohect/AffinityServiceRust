# print_cli_help function (cli.rs)

Prints a detailed CLI reference message covering all command-line options, including basic arguments, operating modes, debug/testing options, and practical debugging examples. This function provides the comprehensive reference that supplements the concise [print_help](print_help.md) output. It is not invoked directly by a CLI flag — instead, it is called internally by [print_help_all](print_help_all.md) as the first half of the combined help output.

## Syntax

```rust
pub fn print_cli_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

### Console behavior

Unlike [print_help](print_help.md), this function does **not** set the global `USE_CONSOLE` static itself. It relies on its caller ([print_help_all](print_help_all.md)) to have already set `USE_CONSOLE` to `true` before invocation. If called in isolation without console mode enabled, the output would be written to the log file instead of stdout.

### Content

The help message is a single `log!()` macro invocation containing a raw string literal organized into four sections:

- **Basic Arguments** — All common flags with descriptions:
  - `-help` / `--help` / `-?` / `/?` / `?` — Print the basic help message.
  - `-helpall` / `--helpall` — Print the detailed help (this output).
  - `-console` — Route output to console instead of the log file.
  - `-noUAC` / `-nouac` — Disable UAC elevation request.
  - `-config <file>` — Specify the configuration file (default: `config.ini`).
  - `-find` — Discover processes with default (unmanaged) affinity.
  - `-blacklist <file>` — Specify the blacklist file for `-find` mode.
  - `-interval <ms>` — Set the polling interval (default: 5000, minimum: 16).
  - `-resolution <t>` — Set the system timer resolution (e.g., `5210` → 0.5210 ms; 0 means do not set).

- **Operating Modes** — Detailed descriptions of each utility mode:
  - `-validate` — Validate config file syntax and undefined aliases, then exit.
  - `-processlogs` — Process find-mode logs to discover new processes and resolve paths via Everything search (`-config`, `-blacklist`, `-in`, `-out`).
  - `-dryrun` — Simulate changes without applying (shows what would happen).
  - `-convert` — Convert Process Lasso config from `-in <file>` to `-out <file>`.
  - `-autogroup` — Auto-group rules with identical settings into named group blocks (`-in <file>` `-out <file>`).
  - `-in <file>` — Input file/directory for `-convert` / `-processlogs` (default: `logs` for `-processlogs`).
  - `-out <file>` — Output file for `-convert` / `-processlogs` (default: `new_processes_results.txt`).

- **Debug & Testing Options** — Flags intended for development and troubleshooting:
  - `-loop <count>` — Run a fixed number of polling iterations then exit (default: infinite).
  - `-logloop` — Log a timestamped message at the start of each loop iteration.
  - `-noDebugPriv` — Do not request `SeDebugPrivilege` at startup.
  - `-noIncBasePriority` — Do not request `SeIncreaseBasePriorityPrivilege` at startup.

- **Debugging Examples** — Two practical command-line examples:
  - A non-admin quick debug command using `-console -noUAC -logloop -loop 3 -interval 2000 -config test.ini`.
  - An admin debug workflow (without `-console`) that writes to `logs/YYYYMMDD.log`, with a note explaining that UAC elevation starts a new session where console output is not visible.

### Relationship to other help functions

| Function | Scope | Invoked by |
|----------|-------|------------|
| [print_help](print_help.md) | Common options and modes only | `-help`, `--help`, `-?`, `/?`, `?` |
| **print_cli_help** (this) | Full CLI reference including debug options | [print_help_all](print_help_all.md) |
| [print_config_help](print_config_help.md) | Configuration file format template | [print_help_all](print_help_all.md) |
| [print_help_all](print_help_all.md) | CLI reference + config template combined | `-helpall`, `--helpall` |

### Invocation flow

This function is never called directly from [main](../main.rs/main.md). The call chain is:

1. User passes `-helpall` → [parse_args](parse_args.md) sets `cli.help_all_mode = true`.
2. [main](../main.rs/main.md) checks `cli.help_all_mode` and calls [print_help_all](print_help_all.md).
3. [print_help_all](print_help_all.md) sets `USE_CONSOLE = true`, calls `print_cli_help()`, then calls [print_config_help](print_config_help.md).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [print_help_all](print_help_all.md) |
| Callees | `log!()` macro |
| API | None |
| Privileges | N/A |

## See Also

| Topic | Link |
|-------|------|
| Basic help output | [print_help](print_help.md) |
| Configuration file help | [print_config_help](print_config_help.md) |
| Combined help output | [print_help_all](print_help_all.md) |
| CLI arguments structure | [CliArgs](CliArgs.md) |
| Argument parser | [parse_args](parse_args.md) |
| Entry point | [main](../main.rs/main.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd