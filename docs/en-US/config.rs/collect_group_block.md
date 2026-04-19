# collect_group_block function (config.rs)

Collects process names from a multi-line group block definition, reading lines until a closing `}` brace is encountered. Returns the accumulated member list, any rule suffix following the closing brace, and the line index to resume parsing from.

## Syntax

```rust
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## Parameters

`lines: &[String]`

The complete set of lines read from the configuration file. The function reads forward from `start_index` looking for the closing `}` brace.

`start_index: usize`

The zero-based line index to begin scanning from. This should be the line immediately after the one containing the opening `{` brace. The function reads `lines[start_index]`, `lines[start_index + 1]`, and so on until either a `}` is found or the end of the file is reached.

`first_line_content: &str`

Any content that appeared after the opening `{` on the same line. For example, given `group_name { proc1.exe: proc2.exe`, this parameter would be `"proc1.exe: proc2.exe"`. If the opening brace was the last non-whitespace character on its line, this will be an empty string or whitespace. This content is parsed for member names before reading subsequent lines.

## Return value

`Option<(Vec<String>, Option<String>, usize)>` — Returns `Some(...)` on success or `None` if the end of the file is reached without finding a closing `}`.

The tuple elements are:

| Index | Type | Description |
|-------|------|-------------|
| `.0` | `Vec<String>` | Collected group member names, lowercased and trimmed. Accumulated from `first_line_content` and all subsequent lines up to and including the line containing `}`. |
| `.1` | `Option<String>` | The rule suffix that follows the closing `}`. If the line containing `}` has `:rule_fields...` after the brace, the text after the first `:` is returned as `Some(rule_fields)`. If there is no `:` after `}`, returns `None`. |
| `.2` | `usize` | The line index immediately after the line containing `}`. The caller should resume parsing from this index. |

## Remarks

### Parsing algorithm

1. If `first_line_content` is non-empty and does not start with `#`, it is passed to [collect_members](collect_members.md) to extract any process names that appeared on the same line as the opening `{`.
2. The function then iterates through `lines` starting at `start_index`:
   - If a line contains `}`, the content before `}` is parsed for members, the content after `}` is examined for a rule suffix (text after a leading `:`), and the function returns successfully.
   - Otherwise, the entire line (if non-empty and not a comment) is parsed for members via [collect_members](collect_members.md).
3. If the loop exhausts all lines without finding `}`, the function returns `None`, indicating an unclosed group block.

### Rule suffix extraction

The text immediately following the closing `}` determines whether the group has an associated rule. The parser expects the format `}:priority:affinity:...`. The leading `:` is stripped, and the remaining text becomes the rule suffix returned in `.1`. This suffix is later split on `:` by [parse_and_insert_rules](parse_and_insert_rules.md) to create the actual config entries.

If no `:` follows the `}` (e.g., the closing brace is alone on a line), the caller in [read_config](read_config.md) treats this as an error — a group without a rule definition.

### Comment handling

Lines starting with `#` inside a group block are silently ignored and do not contribute members. This allows users to comment out individual process names within a group:

```
my_group {
    active_game.exe
    # disabled_game.exe
    another_game.exe
}:normal:*ecore:0:0:none:none:0:1
```

### Single-line vs. multi-line

This function is called only for multi-line groups — cases where the opening `{` line does not also contain a closing `}`. Single-line groups (e.g., `group { a: b }:rule`) are handled inline by [read_config](read_config.md) without calling this function.

### Error case

When the function returns `None`, the caller in [read_config](read_config.md) pushes an error into [ConfigResult](ConfigResult.md) in the format `"Line {N}: Unclosed group '{name}' - missing }"` and skips to the next line.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Private (`fn`) — internal to the config module |
| **Callers** | [read_config](read_config.md) (multi-line group parsing), [sort_and_group_config](sort_and_group_config.md) (auto-grouping reader) |
| **Callees** | [collect_members](collect_members.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Member name parser | [collect_members](collect_members.md) |
| Rule insertion for collected members | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Main config reader | [read_config](read_config.md) |
| Config result with error reporting | [ConfigResult](ConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*