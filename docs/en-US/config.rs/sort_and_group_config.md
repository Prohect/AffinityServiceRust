# sort_and_group_config function (config.rs)

Reads an existing configuration file, identifies processes that share identical rule settings, and writes a new configuration file where those processes are merged into named `{ ... }` group blocks. This reduces duplication and improves maintainability of large configuration files that contain many processes with the same priority, affinity, and scheduling settings.

## Syntax

```rust
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `in_file` | `Option<String>` | Path to the input configuration file to read and analyze. If `None`, an error is logged and the function returns immediately. |
| `out_file` | `Option<String>` | Path to the output configuration file to write. If `None`, an error is logged and the function returns immediately. The output file is created or overwritten. |

## Return value

This function does not return a value. Results are written to the output file and summary statistics are logged.

## Remarks

### Purpose

Over time, configuration files accumulate many individual process rules that share the same settings. For example, ten background utilities may all share `below normal:8-15:0:0:none:none`. `sort_and_group_config` detects these duplicates and rewrites the file with compact group notation, reducing visual clutter and making bulk changes easier.

### Algorithm

1. **Read and classify lines** — The input file is read into memory. Lines are classified into two categories:
   - **Preamble lines** — Comments (`#`), blank lines, constants (`@`), and aliases (`*`) are collected as-is and preserved verbatim in the output.
   - **Rule lines** — Individual process rules (`name:rule`) and group blocks (`{ ... }:rule`) are decomposed into their member names and rule strings. The rule string is everything after the first `:` for individual rules, or the suffix after `}:` for groups.

2. **Group block parsing** — When a `{` is encountered, the parser collects member names using [collect_group_block](collect_group_block.md) (for multi-line blocks) or inline parsing (for single-line blocks), then associates all members with the rule suffix string.

3. **Build rule-to-members map** — A `HashMap<String, Vec<String>>` maps each unique rule string to the list of process names that share it. A separate `Vec<String>` (`rule_order`) preserves the order in which distinct rules were first encountered, ensuring stable output ordering.

4. **Sort and deduplicate** — For each unique rule string, the member list is sorted alphabetically and deduplicated.

5. **Generate output** — The preamble lines are written first (up to the last non-empty preamble line). Then, for each unique rule:
   - **Single member** — Written as a plain individual rule: `name:rule`.
   - **Multiple members** — Written as a named group. The group name is auto-generated as `grp_0`, `grp_1`, etc. The output format depends on line length:
     - **Short groups** (< 128 characters total) — Written as a single-line group: `grp_N { member1: member2: member3 }:rule`.
     - **Long groups** (≥ 128 characters) — Written as a multi-line group with 4-space indentation, wrapping member lists at 128 characters per line:

       ```text
       grp_N {
           member1: member2: member3
           member4: member5
       }:rule
       ```

6. **Write and log** — The output is written to `out_file`. A summary is logged: `"Auto-grouped: {total} total process rules → {singles} individual + {grouped} processes merged into {groups} groups"`.

### Preamble preservation

All content before the first rule line (comments, blank lines, constant definitions, alias definitions) is preserved in its original form and written to the output file before any rules. This ensures that `@CONSTANT` and `*alias` definitions remain intact and available for the grouped rules that follow.

### Line length heuristic

The 128-character threshold for single-line vs. multi-line group formatting is defined by the `INDENT` constant (`"    "`, 4 spaces). When the single-line representation of a group exceeds 128 characters, the function switches to multi-line format. Within the multi-line format, member names on each indented line are also wrapped at 128 characters.

### Rule string identity

Two processes are considered to have "identical rules" if their complete rule strings (everything after the first `:` in the original line) are identical after trimming. This is a string comparison, not a semantic comparison — `high:0-7` and `high:0-7:0` are treated as distinct rules even though they may produce equivalent [ProcessConfig](ProcessConfig.md) entries.

### Group naming

Auto-generated group names follow the pattern `grp_0`, `grp_1`, `grp_2`, etc., incremented for each group with two or more members. These names are for documentation/readability only — the config parser in [read_config](read_config.md) does not use group names for matching. Single-member rules are not assigned group names.

### Error handling

- If `in_file` is `None`, logs `"Error: -in <file> is required for -autogroup"` and returns.
- If `out_file` is `None`, logs `"Error: -out <file> is required for -autogroup"` and returns.
- If the input file cannot be read, logs the error and returns.
- If the output file cannot be created or written to, logs the error and returns.

No `Result` or error type is returned — all errors are logged and the function exits gracefully.

### CLI integration

This function is invoked when the user passes the `-autogroup` CLI flag along with `-in <input_config>` and `-out <output_config>`. It is a standalone offline utility that does not require the service to be running.

### Example

**Input (`config.txt`):**

```text
# CPU aliases
*perf = 0-7

game1.exe:high:*perf
game2.exe:high:*perf
game3.exe:high:*perf
helper.exe:normal:0
updater.exe:below normal:8-15
telemetry.exe:below normal:8-15
```

**Output (`config_grouped.txt`):**

```text
# CPU aliases
*perf = 0-7

grp_0 { game1.exe: game2.exe: game3.exe }:high:*perf

helper.exe:normal:0

grp_1 { telemetry.exe: updater.exe }:below normal:8-15
```

**Log output:**

```text
Auto-grouped: 6 total process rules → 1 individual + 5 processes merged into 2 groups
Written to config_grouped.txt
```

### Limitations

- The function does not re-parse rule semantics — it groups purely on rule-string equality. Semantically equivalent rules with different textual representations are not merged.
- Alias definitions in the preamble are preserved but not expanded. If two rules reference the same alias but one uses the alias and the other uses the resolved value, they are treated as different rules.
- The output file always uses group notation even if the input used a different formatting style. Comments that were interleaved between rule lines in the input are not preserved in the rule section of the output (only preamble comments are kept).

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [main](../main.rs/main.md) (via `-autogroup` CLI flag) |
| **Callees** | [collect_members](collect_members.md), [collect_group_block](collect_group_block.md), `std::fs::read_to_string`, `std::fs::File::create`, `std::io::Write::writeln` |
| **API** | Standard library file I/O only — no Windows API calls |
| **Privileges** | Read access to input file, write access to output file |

## See Also

| Topic | Link |
|-------|------|
| Process Lasso config converter | [convert](convert.md) |
| Main config file reader | [read_config](read_config.md) |
| Member name tokenizer | [collect_members](collect_members.md) |
| Multi-line group block collector | [collect_group_block](collect_group_block.md) |
| Per-process configuration record | [ProcessConfig](ProcessConfig.md) |
| CLI argument parsing | [cli module](../cli.rs/README.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd