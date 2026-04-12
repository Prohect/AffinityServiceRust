# convert function (config.rs)

Converts a Process Lasso configuration file into the native AffinityServiceRust config format.

## Syntax

```rust
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

`in_file`

Path to the input Process Lasso configuration file (UTF-16 LE encoded INI format). Required; if `None`, an error is logged and the function returns early.

`out_file`

Path to the output file where the converted AffinityServiceRust configuration will be written. Required; if `None`, an error is logged and the function returns early.

## Return value

This function does not return a value. Results are written directly to the output file, and diagnostic messages are logged to the console.

## Remarks

This function reads a Process Lasso INI-style configuration file (UTF-16 LE encoded) and produces an equivalent AffinityServiceRust config file. It parses three key sections from the Process Lasso format:

- **NamedAffinities** — Comma-separated pairs of `alias,cpu_spec` that are converted into `*alias = cpu_spec` CPU alias lines.
- **DefaultPriorities** — Comma-separated pairs of `process,priority` that map to the priority field in output rules. Numeric priority codes (`1`–`6`) are translated to named priorities (`idle`, `below normal`, `normal`, `above normal`, `high`, `real time`).
- **DefaultAffinitiesEx** — Comma-separated triples of `process,mask,cpuset` where the cpuset value is used as the affinity field. If a cpuset matches a named affinity, the alias reference (e.g., `*alias`) is used instead of the raw CPU specification.

The output file includes:

1. Config help lines (from [get_config_help_lines](../cli.rs/get_config_help_lines.md)) as header comments.
2. CPU alias definitions derived from `NamedAffinities`.
3. Process rules in the format `name:priority:affinity:0:0:none:none`, sorted alphabetically by process name.

Processes that appear in either `DefaultPriorities` or `DefaultAffinitiesEx` (or both) are merged into a single output rule. Missing fields default to `none` for priority and `0` for affinity.

The input file is read using [read_utf16le_file](read_utf16le_file.md), which handles the UTF-16 LE encoding typical of Process Lasso configuration exports.

This function is invoked via the `-convert` CLI flag along with `-in` and `-out` arguments.

### Example usage

```
AffinityService.exe -convert -in "C:\ProgramData\Process Lasso\processgovernor.ini" -out config.txt
```

### Example output

```
# Converted from Process Lasso config

# CPU Aliases (from Process Lasso NamedAffinities)
*perf = 0-7
*eff = 8-15

chrome.exe:normal:*perf:0:0:none:none
game.exe:high:*perf:0:0:none:none
obs64.exe:above normal:*eff:0:0:none:none
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | `pub` |
| **Called by** | [main](../main.rs/main.md) (when `-convert` CLI flag is set) |
| **Depends on** | [read_utf16le_file](read_utf16le_file.md), [get_config_help_lines](../cli.rs/get_config_help_lines.md) |

## See also

- [sort_and_group_config](sort_and_group_config.md) — Auto-groups processes with identical rules after conversion.
- [read_config](read_config.md) — Parses the native config format that `convert` produces.
- [parse_cpu_spec](parse_cpu_spec.md) — CPU specification format used in output rules.