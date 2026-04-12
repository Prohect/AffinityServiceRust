# sort_and_group_config function (config.rs)

Auto-groups processes with identical rule settings to reduce configuration file duplication. Reads an existing config, identifies processes sharing the same rule parameters, and merges them into named group blocks using `{ process1: process2: ... }:rule` syntax.

## Syntax

```rust
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

`in_file`

An `Option<String>` specifying the path to the input configuration file to read and analyze. If `None`, an error is logged and the function returns immediately. Typically provided via the `-in` CLI argument.

`out_file`

An `Option<String>` specifying the path to the output file where the grouped configuration will be written. If `None`, an error is logged and the function returns immediately. Typically provided via the `-out` CLI argument.

## Return value

This function does not return a value. Results are written directly to the output file. Errors and progress are logged via the logging subsystem.

## Remarks

This function is invoked when the user passes the `-autogroup` CLI flag. It performs the following steps:

1. Calls [read_config](read_config.md) to parse the input configuration file into a [ConfigResult](ConfigResult.md).
2. For each grade tier, iterates through all [ProcessConfig](ProcessConfig.md) entries and computes a canonical "rule key" string from the non-name fields (priority, affinity, cpuset, prime threads, IO priority, memory priority, ideal processor rules, grade).
3. Groups processes that share identical rule keys into a single group block.
4. Preserves constants (`@MIN_ACTIVE_STREAK`, etc.) and aliases (`*name = cpu_spec`) from the original file by re-reading raw lines.
5. Outputs compact grouped config where single-member groups are written as plain rules and multi-member groups use `{ member1: member2 }:rule` syntax.
6. Processes within each group are sorted alphabetically for consistency.

The generated output is designed to be a valid configuration file that is functionally equivalent to the input but with reduced redundancy. This is useful when a configuration has grown organically and contains many individual rules that share the same settings.

CPU aliases referenced by the original rules are resolved during parsing and then re-emitted as literal CPU specifications in the output unless the original alias definitions are preserved from the raw file.

### Example

Given an input config:

```
*p = 0-7
chrome.exe:high:*p:0:0:none:none
firefox.exe:high:*p:0:0:none:none
notepad.exe:normal:0:0:0:none:none
```

The output would group chrome and firefox:

```
*p = 0-7
{ chrome.exe: firefox.exe }:high:*p:0:0:none:none
notepad.exe:normal:0:0:0:none:none
```

### Error handling

- If `in_file` or `out_file` is `None`, logs an error message and returns.
- If the input file cannot be read or parsed, errors from [read_config](read_config.md) are reported.
- If the output file cannot be created or written to, an error is logged.

### Related functions

- [read_config](read_config.md) — parses the input configuration file
- [convert](convert.md) — converts Process Lasso configs to native format
- [parse_and_insert_rules](parse_and_insert_rules.md) — parses individual rule fields
- [format_cpu_indices](format_cpu_indices.md) — formats CPU indices as compact range strings for output

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | `pub` |
| **Called by** | `src/main.rs` (when `-autogroup` CLI flag is set) |
| **Depends on** | [read_config](read_config.md), [format_cpu_indices](format_cpu_indices.md), [ConfigResult](ConfigResult.md), [ProcessConfig](ProcessConfig.md) |