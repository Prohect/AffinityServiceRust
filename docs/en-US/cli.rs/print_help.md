# print_help function (cli.rs)

Prints a concise help message to the active log output covering the most common command-line options and operating modes. This is the default help shown when the user passes `-help`, `--help`, `-?`, `/?`, or `?`. For a comprehensive reference including debug options, see [print_cli_help](print_cli_help.md) or [print_help_all](print_help_all.md).

## Syntax

```rust
pub fn print_help()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

### Console forcing

The function sets the global `USE_CONSOLE` static to `true` (via `*get_use_console!() = true`) before emitting any output. This ensures the help text is written to the console (stdout) rather than to the log file, regardless of whether `-console` was explicitly passed. This is the expected behavior because help output is always intended for interactive review.

### Content

The help message is a single `log!()` macro invocation containing a raw string literal. It covers:

- **Header** — One-line description of the service and a usage synopsis.
- **Common Options** — The most frequently used flags:
  - `-help` / `--help` — Show the basic help message.
  - `-helpall` — Show the detailed reference (delegates to [print_help_all](print_help_all.md)).
  - `-console` — Route output to console instead of the log file.
  - `-config <file>` — Specify a configuration file (default: `config.ini`).
  - `-find` — Discover processes with default affinity, optionally paired with `-blacklist <file>`.
  - `-interval <ms>` — Set the polling interval (default: 5000 ms).
  - `-noUAC` — Suppress the automatic UAC elevation request.
  - `-resolution <t>` — Set the system timer resolution (e.g., `5210` → 0.5210 ms).
- **Modes** — One-line descriptions of each utility mode:
  - `-validate` — Check configuration syntax without running.
  - `-processlogs` — Analyze find-mode logs to discover new processes.
  - `-dryrun` — Simulate changes without applying.
  - `-convert` — Convert a Process Lasso configuration.
  - `-autogroup` — Auto-group rules with identical settings.

### Relationship to other help functions

| Function | Scope | Invoked by |
|----------|-------|------------|
| **print_help** (this) | Common options and modes only | `-help`, `--help`, `-?`, `/?`, `?` |
| [print_cli_help](print_cli_help.md) | Full CLI reference including debug options | Called by [print_help_all](print_help_all.md) |
| [print_config_help](print_config_help.md) | Configuration file format template | Called by [print_help_all](print_help_all.md) |
| [print_help_all](print_help_all.md) | CLI reference + config template combined | `-helpall`, `--helpall` |

### Invocation flow

In [main](../main.rs/main.md), the help mode check occurs before any configuration is loaded:

```rust
if cli.help_mode {
    print_help();
    return Ok(());
}
```

This means the function executes and the program exits without touching the config file, blacklist, privileges, or any other subsystem. It is safe to call even in environments where the configuration file does not exist or is malformed.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [main](../main.rs/main.md) (when `cli.help_mode` is `true`) |
| Callees | `get_use_console!()` macro, `log!()` macro |
| API | None |
| Privileges | N/A |

## See Also

| Topic | Link |
|-------|------|
| Detailed CLI help | [print_cli_help](print_cli_help.md) |
| Configuration file help | [print_config_help](print_config_help.md) |
| Combined help output | [print_help_all](print_help_all.md) |
| CLI arguments structure | [CliArgs](CliArgs.md) |
| Argument parser | [parse_args](parse_args.md) |
| Entry point | [main](../main.rs/main.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd