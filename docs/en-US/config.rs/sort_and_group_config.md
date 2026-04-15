# sort_and_group_config function (config.rs)

Auto-groups processes that share identical rule settings into named group blocks to reduce configuration file duplication. The function reads an existing config file, identifies processes whose rule fields (priority, affinity, cpuset, etc.) are identical, merges them into `{ }` group blocks with generated names, and writes a compact, deduplicated output file.

## Syntax

```AffinityServiceRust/src/config.rs#L1070-1277
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `in_file` | `Option<String>` | The path to the input configuration file to read and analyze. The file must be a valid AffinityServiceRust config in UTF-8 encoding. If `None`, the function logs an error and returns immediately. Corresponds to the `-in <file>` CLI argument. |
| `out_file` | `Option<String>` | The path to the output file where the grouped configuration will be written. If `None`, the function logs an error and returns immediately. Corresponds to the `-out <file>` CLI argument. |

## Return value

This function does not return a value. Output is written to the file specified by `out_file`. Diagnostic messages and summary statistics are emitted via the `log!` macro.

## Remarks

### Algorithm overview

1. **Read and partition lines** — The input file is read line by line and partitioned into two sections:
   - **Preamble**: Comment lines (`#`-prefixed), blank lines, constant definitions (`@`-prefixed), and alias definitions (`*`-prefixed). These are preserved verbatim in the output.
   - **Rules section**: Everything else — individual process rule lines and `{ }` group blocks.

2. **Extract rule keys** — For each process rule (whether standalone or inside a group block), the process name is separated from the rule string (everything after the first `:`). The rule string becomes a grouping key.

3. **Group by rule** — A `HashMap<String, Vec<String>>` maps each unique rule string to the list of process names that share it. A separate `rule_order` vector preserves the insertion order so that the output maintains a stable ordering based on first occurrence.

4. **Emit grouped output** — For each unique rule string (in insertion order):
   - If only **one** process uses the rule, it is emitted as a single-line rule: `name:rule_string`.
   - If **multiple** processes share the rule, they are emitted as a named group block. The group name is auto-generated as `grp_0`, `grp_1`, etc.

5. **Format groups** — For multi-process groups:
   - If the single-line representation (`grp_N { a: b: c }:rule`) is shorter than 128 characters, it is emitted on one line.
   - Otherwise, a multi-line format is used with 4-space indentation and line wrapping at 128 characters:
     ```/dev/null/example.ini#L1-4
     grp_0 {
         process1.exe: process2.exe: process3.exe
         process4.exe: process5.exe
     }:priority:affinity:cpuset:prime:io:mem:ideal:grade
     ```

6. **Deduplication** — Within each group, member names are sorted alphabetically and deduplicated via `sort()` and `dedup()` before output.

### Preamble preservation

All non-rule lines (comments, blanks, constants, aliases) are collected into the preamble and written to the output before any rules. Trailing blank lines at the end of the preamble are trimmed to a single separator. This ensures that alias definitions remain available for `*alias` references in the rule section.

### Group block re-parsing

When the input file already contains `{ }` group blocks, the function re-parses them using [`collect_group_block`](collect_group_block.md) and [`collect_members`](collect_members.md) to extract member names and rule suffixes. The existing group names are discarded — all groups in the output receive fresh auto-generated names (`grp_0`, `grp_1`, …).

### Line length threshold

The constant `128` is used as the maximum line length for single-line group output. Groups whose single-line representation exceeds this threshold are formatted as multi-line blocks with `const INDENT: &str = "    "` (4 spaces) for readability.

### Output statistics

On completion, the function logs a summary message:

```/dev/null/example.txt#L1-2
Auto-grouped: {total} total process rules → {single} individual + {grouped} processes merged into {groups} groups
Written to {out_path}
```

Where:
- `total` = `single_count + grouped_member_count`
- `single` = number of processes with unique rules (no grouping needed)
- `grouped` = total number of processes that were merged into groups
- `groups` = number of group blocks created

### Error handling

| Condition | Behavior |
|-----------|----------|
| `in_file` is `None` | Logs `"Error: -in <file> is required for -autogroup"` and returns. |
| `out_file` is `None` | Logs `"Error: -out <file> is required for -autogroup"` and returns. |
| Input file cannot be read | Logs `"Failed to read {path}: {error}"` and returns. |
| Output file cannot be created | Logs `"Failed to create {path}: {error}"` and returns. |
| Write failure during output | Logs `"Failed to write to {path}"` and returns. |

### CLI usage

```/dev/null/example.sh#L1
AffinityServiceRust.exe -autogroup -in config.ini -out config_grouped.ini
```

### Idempotency

Running `sort_and_group_config` on its own output produces an equivalent file (modulo group names and formatting), since the function re-parses any existing groups. However, group names will be renumbered starting from `grp_0` on each run.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | `main.rs` (when `cli.autogroup_mode` is `true`) |
| Callees | [`collect_group_block`](collect_group_block.md), [`collect_members`](collect_members.md), `std::fs::read_to_string`, `File::create`, `writeln!`, `log!` |
| Dependencies | `HashMap` from [`collections.rs`](../collections.rs/README.md); `std::fs::File`, `std::io::Write` |
| Privileges | File system read/write access to the specified paths |

## See Also

| Resource | Link |
|----------|------|
| convert | [convert](convert.md) |
| read_config | [read_config](read_config.md) |
| collect_group_block | [collect_group_block](collect_group_block.md) |
| collect_members | [collect_members](collect_members.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| config module overview | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*