# collect_group_block function (config.rs)

Collects process-name members from a multi-line `{ … }` group block in the configuration file, scanning forward from a starting line index until the closing brace `}` is found. Returns the collected member names, any rule suffix that appears after the closing brace, and the next line index to resume parsing.

## Syntax

```AffinityServiceRust/src/config.rs#L390-418
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `lines` | `&[String]` | The full list of lines read from the configuration file. The function reads forward from `start_index` looking for the closing `}`. |
| `start_index` | `usize` | The 0-based index into `lines` at which to begin scanning for group members and the closing brace. This is typically the line immediately after the one containing the opening `{`. |
| `first_line_content` | `&str` | Any content that appeared on the same line as the opening `{`, after the brace. If non-empty and not a comment, it is parsed for member names before scanning subsequent lines. |

## Return value

Type: `Option<(Vec<String>, Option<String>, usize)>`

Returns `Some((members, rule_suffix, next_line_index))` when a closing `}` is found, or `None` if the end of the file is reached without encountering a closing brace (unclosed group).

| Tuple element | Type | Description |
|---------------|------|-------------|
| `members` | `Vec<String>` | Lowercase process names collected from within the braces, extracted via [`collect_members`](collect_members.md). |
| `rule_suffix` | `Option<String>` | The rule string that follows `}:` on the closing-brace line (i.e., everything after the first `:` that follows `}`). `None` if there is no `:` after the closing brace. |
| `next_line_index` | `usize` | The line index immediately after the line containing the closing brace; the caller should resume parsing from this index. |

## Remarks

### Scanning algorithm

1. If `first_line_content` is non-empty and does not start with `#`, it is passed to [`collect_members`](collect_members.md) to extract any process names on the same line as the opening brace.
2. The function then iterates through `lines` starting at `start_index`:
   - If the line contains a `}`, everything before the brace is collected as members (if non-empty and not a comment), the rule suffix after `}` is extracted, and the function returns.
   - Otherwise, non-empty, non-comment lines are passed to [`collect_members`](collect_members.md) to accumulate additional member names.
3. If the loop reaches the end of `lines` without finding `}`, the function returns `None`, indicating an unclosed group block.

### Rule suffix extraction

On the closing-brace line, the text after `}` is trimmed and checked for a leading `:`. If found, the colon is stripped and the remainder is returned as `Some(rule_string)`. If no colon is present (e.g., `}` is followed only by whitespace or nothing), `None` is returned for the suffix, which is treated as an error by the caller in [`read_config`](read_config.md).

### Comment handling

Lines that start with `#` (after trimming) are treated as comments and skipped entirely. Additionally, within a line, any token produced by [`collect_members`](collect_members.md) that starts with `#` is filtered out — this provides inline comment support.

### Single-line groups

Single-line groups where both `{` and `}` appear on the same line (e.g., `{ a.exe: b.exe }:normal:0-7`) are **not** handled by this function. They are detected and parsed inline within [`read_config`](read_config.md) before `collect_group_block` is called. This function is only invoked when the opening `{` line does not also contain a closing `}`.

### Edge cases

| Scenario | Behavior |
|----------|----------|
| Opening brace immediately followed by `}` on the next line | Returns members collected from `first_line_content` only. |
| Empty group block (`{ }` spanning multiple lines with only comments/blanks) | Returns an empty `members` vector; the caller emits a warning. |
| Nested braces | Not supported. A `}` inside the block terminates collection immediately regardless of context. |
| EOF reached without `}` | Returns `None`. The caller pushes an error about an unclosed group. |

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (`fn`, not `pub fn`) |
| Callers | [`read_config`](read_config.md), [`sort_and_group_config`](sort_and_group_config.md) |
| Callees | [`collect_members`](collect_members.md), `str::find`, `str::trim`, `str::starts_with`, `str::strip_prefix` |
| API | Standard library only |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| collect_members | [collect_members](collect_members.md) |
| read_config | [read_config](read_config.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| sort_and_group_config | [sort_and_group_config](sort_and_group_config.md) |
| config module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*