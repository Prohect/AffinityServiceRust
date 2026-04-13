# get_config_help_lines function (cli.rs)

Returns a vector of static string slices containing a configuration file help template. These lines describe the format and syntax of the AffinityServiceRust configuration file in a comment block suitable for embedding at the top of converted or auto-grouped configuration files. The template provides a quick-reference for the colon-delimited rule format, field meanings, CPU alias syntax, and group syntax.

## Syntax

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

## Parameters

This function takes no parameters.

## Return value

Returns a `Vec<&'static str>` containing the help template lines. Each element is a single line of text, prefixed with `##` to form a comment block when written to a configuration file. The vector contains 24 lines covering:

- A header banner (`## ===...===`).
- A title line (`## AffinityServiceRust Configuration File`).
- A documentation pointer (`## Full documentation: docs/cli.md and docs/config.md`).
- The colon-delimited rule format synopsis (`## Format: process:priority:affinity:cpuset:prime:io:memory:ideal:grade`).
- Per-field descriptions (process, priority, affinity, cpuset, prime, io, memory, ideal, grade) with example values.
- CPU alias examples (`*a`, `*p`, `*e`).
- Group syntax synopsis (`{ proc1: proc2 }:priority:affinity...`).
- A closing banner.

## Remarks

### Purpose

This function exists to provide a reusable help template that can be:

1. **Printed to the console** — via [print_config_help](print_config_help.md), which iterates over the returned lines and logs each one.
2. **Embedded in generated files** — the [convert](../config.rs/convert.md) and [sort_and_group_config](../config.rs/sort_and_group_config.md) functions can prepend these lines to their output files so that users have an in-file reference for the configuration syntax.

By returning a `Vec<&'static str>` rather than printing directly, the function gives callers full control over how and where the lines are rendered.

### Static lifetime

All strings in the returned vector have `'static` lifetime because they are string literals embedded in the binary. This means the vector can be stored, iterated multiple times, or passed across function boundaries without lifetime concerns.

### Comment prefix convention

Each line begins with `##` (double hash). In the AffinityServiceRust configuration file format, lines starting with `#` or `##` are treated as comments. The double-hash convention distinguishes auto-generated help comments from user-written single-hash comments, making it easy to strip or update the template without disturbing user annotations.

### Content summary

The template documents the nine colon-delimited fields of a process rule:

| Field | Position | Example values |
|-------|----------|----------------|
| process | 1 | `game.exe` |
| priority | 2 | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` |
| affinity | 3 | `0-7`, `0;4;8`, `0xFF`, `*alias` |
| cpuset | 4 | `*p`, `*e`, `*alias` |
| prime | 5 | `?10*pN01`, `*p@module.dll` |
| io | 6 | `none`, `very low`, `low`, `normal`, `high` |
| memory | 7 | `none`, `very low`, `low`, `medium`, `below normal`, `normal` |
| ideal | 8 | `*alias[@prefix]`, `0` |
| grade | 9 | `1` (every loop), `5` (every 5th loop) |

### Typical usage

```rust
// Print to console
for line in get_config_help_lines() {
    log!("{}", line);
}

// Embed in output file
let mut output = String::new();
for line in get_config_help_lines() {
    output.push_str(line);
    output.push('\n');
}
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [print_config_help](print_config_help.md), [print_help_all](print_help_all.md) (indirectly via `print_config_help`), [convert](../config.rs/convert.md), [sort_and_group_config](../config.rs/sort_and_group_config.md) |
| Callees | None (pure function) |
| API | None |
| Privileges | N/A |

## See Also

| Topic | Link |
|-------|------|
| Prints config help lines to log output | [print_config_help](print_config_help.md) |
| Combined CLI + config help | [print_help_all](print_help_all.md) |
| Configuration file parser | [read_config](../config.rs/read_config.md) |
| Process Lasso config converter | [convert](../config.rs/convert.md) |
| Auto-grouping utility | [sort_and_group_config](../config.rs/sort_and_group_config.md) |
| Process rule structure | [ProcessConfig](../config.rs/ProcessConfig.md) |
| CLI arguments structure | [CliArgs](CliArgs.md) |