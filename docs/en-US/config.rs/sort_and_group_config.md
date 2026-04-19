# sort_and_group_config function (config.rs)

Auto-groups processes that share identical rule settings into named group blocks, producing a compact configuration file with reduced duplication. This is a command-line utility function invoked via the `-autogroup` flag.

## Syntax

```rust
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

`in_file: Option<String>`

Path to the input configuration file. Must be `Some`; if `None`, the function logs an error and returns immediately.

`out_file: Option<String>`

Path to the output file where the grouped configuration will be written. Must be `Some`; if `None`, the function logs an error and returns immediately.

## Return value

This function does not return a value. Results are written to the output file and diagnostic messages are logged.

## Remarks

### Algorithm

The function performs a multi-pass transformation:

1. **Preamble extraction:** Lines at the top of the file that are comments (`#`), blank lines, constants (`@`), or aliases (`*`) are collected into a preamble section that is preserved verbatim in the output.

2. **Rule collection:** Each rule line and group block is decomposed into its member process names and rule string (everything after the first colon for single-line rules, or after the closing `}:` for groups). The rule string serves as the grouping key.

3. **Merging:** Rules with identical rule strings have their member lists concatenated. This merges both individual rules and existing group blocks that happen to share the same settings.

4. **Deduplication and sorting:** Within each merged group, member names are sorted alphabetically and deduplicated.

5. **Output generation:** For each unique rule string:
   - If only one process has that rule, it is emitted as a single-line rule: `process.exe:rule_string`
   - If multiple processes share the rule, they are emitted as a named group block. Groups are named sequentially as `grp_0`, `grp_1`, etc.

### Group formatting

Groups are formatted in one of two styles depending on length:

- **Inline style** (when the full line is under 128 characters):
  ```
  grp_0 { proc1.exe: proc2.exe: proc3.exe }:normal:*ecore:0:0:low:none:0:1
  ```

- **Multi-line style** (when the inline representation exceeds 128 characters):
  ```
  grp_1 {
      proc1.exe: proc2.exe: proc3.exe
      proc4.exe: proc5.exe
  }:normal:*pcore:0:0:none:none:0:1
  ```

  In multi-line mode, members are packed into lines up to 128 characters wide with 4-space indentation (`const INDENT: &str = "    "`). Members within a line are colon-separated.

### Preamble preservation

Constants (`@MIN_ACTIVE_STREAK = 3`), aliases (`*pcore = 0-7`), and leading comments are preserved in their original order and form. Only rule lines and group blocks are reorganized. Trailing blank lines in the preamble are trimmed to a single separator line.

### Rule order stability

Unique rule strings are emitted in the order they are first encountered in the input file. This preserves the general organization of the original config while consolidating duplicates.

### Typical usage

```
AffinityServiceRust.exe -autogroup -in config.txt -out config_grouped.txt
```

This is a one-shot transformation tool — the output file is not automatically used by the service. The user should review the output and replace the original config file manually.

### Logging

Upon completion, the function logs a summary:

```
Auto-grouped: 42 total process rules → 10 individual + 32 processes merged into 8 groups
Written to config_grouped.txt
```

### Error handling

- If either `-in` or `-out` is missing, an error message is logged and the function returns.
- If the input file cannot be read, the error is logged and the function returns.
- If the output file cannot be created or written, the error is logged and the function returns.
- Unclosed group blocks in the input are silently skipped.

### Interaction with config parser

The function reuses [collect_members](collect_members.md) and [collect_group_block](collect_group_block.md) to parse the input, ensuring consistent treatment of group syntax and member names. It does **not** call [parse_and_insert_rules](parse_and_insert_rules.md) or validate rule fields — the output preserves rule strings exactly as they appear in the input.

## Requirements

| | |
|---|---|
| **Module** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **Callers** | CLI dispatch in `main.rs` (invoked by `-autogroup` flag) |
| **Callees** | [collect_members](collect_members.md), [collect_group_block](collect_group_block.md), `std::fs::read_to_string`, `std::fs::File::create`, `std::io::Write` |
| **Dependencies** | [HashMap](../collections.rs/README.md) |
| **Privileges** | File system read/write access to input and output paths |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Process Lasso converter (related utility) | [convert](convert.md) |
| Group block parser | [collect_group_block](collect_group_block.md) |
| Member collector | [collect_members](collect_members.md) |
| Config file reader | [read_config](read_config.md) |
| CLI arguments | [CliArgs](../cli.rs/CliArgs.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*