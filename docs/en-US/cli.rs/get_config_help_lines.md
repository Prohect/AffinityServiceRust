# get_config_help_lines function (cli.rs)

Returns the configuration file syntax help text as a vector of static string lines, suitable for embedding directly into converted config files as comments or for display on the console.

## Syntax

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

## Parameters

This function takes no parameters.

## Return value

Returns a `Vec<&'static str>` containing one element per line of the configuration file syntax help text. Each element is a static string slice describing a configuration directive, its syntax, or an example. The lines are ordered logically from general concepts to specific directives.

## Remarks

This function provides the canonical reference for the configuration file format. The returned lines cover all supported directives including:

- **Constants** — `min_active_streak`, `keep_threshold`, `entry_threshold`
- **CPU aliases** — named groups of CPU indices for reuse across rules
- **Process groups** — grouping multiple process names under shared rules
- **Rule directives** — `priority`, `affinity`, `cpuset`, `io_priority`, `memory_priority`, `prime`, `ideal_processor`
- **CPU specification syntax** — individual indices, ranges (`0-3`), masks (`0xFF`), and alias references

The primary consumers of this function are:

1. **[`print_config_help`](print_config_help.md)** — prints the lines to the console when the user requests `--help-config`.
2. **[`print_help_all`](print_help_all.md)** — includes these lines as part of the combined help output for `--help-all`.
3. **Config conversion** — the [`convert`](../config.rs/ConfigResult.md) function in `config.rs` embeds these lines as comments at the top of converted configuration files, giving users an inline reference for the syntax.

By returning a `Vec` of individual lines rather than a single multi-line string, the function allows callers to flexibly format the output — for example, prefixing each line with a comment character (`#`) when embedding in a config file, or printing directly when displaying help.

### Content stability

The returned lines are compiled into the binary as static string literals. They do not change at runtime and are consistent across all invocations. The content reflects the current version's supported configuration syntax.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/cli.rs |
| **Source lines** | L199–L229 |
| **Called by** | [`print_config_help`](print_config_help.md), [`print_help_all`](print_help_all.md), [`convert`](../config.rs/ConfigResult.md) in config.rs |
| **Dependencies** | None (returns static data) |

## See also

- [print_config_help function](print_config_help.md)
- [print_help_all function](print_help_all.md)
- [CliArgs struct](CliArgs.md)
- [cli.rs module overview](README.md)