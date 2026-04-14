# collect_group_block function (config.rs)

Collects process member names from a multi-line group block, scanning forward from a starting line index until the closing `}` brace is found. Returns the accumulated member names, any rule suffix that follows the closing brace, and the line index to resume parsing from. Returns `None` if the block is never closed (i.e., end-of-file is reached without encountering `}`).

## Syntax

```rust
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `lines` | `&[String]` | The complete line vector of the configuration file, as collected by [read_config](read_config.md). The function reads forward from `start_index` through this vector. |
| `start_index` | `usize` | The 0-based index into `lines` at which to begin scanning for group members. This is typically the line immediately after the one containing the opening `{`. |
| `first_line_content` | `&str` | Any text that appeared on the same line as the opening `{`, after the brace. For example, if the config line is `group_name { game.exe: helper.exe`, then `first_line_content` would be `"game.exe: helper.exe"`. This content is parsed for members before the function begins scanning subsequent lines. If empty or starting with `#`, it is ignored. |

## Return value

Returns `Option<(Vec<String>, Option<String>, usize)>`:

| Variant | Meaning |
|---------|---------|
| `Some((members, rule_suffix, next_index))` | The closing `}` was found. `members` is the collected list of lowercased process names. `rule_suffix` is `Some(suffix_string)` if text of the form `}:rule_text` was found after the brace, or `None` if nothing followed. `next_index` is the line index immediately after the closing-brace line (i.e., `i + 1`), which the caller should use to resume its main parse loop. |
| `None` | End-of-file was reached without finding a closing `}`. The group block is unclosed and the caller should record an error. |

## Remarks

### Algorithm

1. **First-line content** — If `first_line_content` is non-empty and does not start with `#`, it is passed to [collect_members](collect_members.md) to extract any member names from the remainder of the opening-brace line.
2. **Line scanning** — Starting at `start_index`, each line is trimmed and examined:
   - If the line contains `}`, the text before `}` is parsed for members (via [collect_members](collect_members.md)), the text after `}` is checked for a `:` prefix to extract the rule suffix, and the function returns `Some(...)`.
   - If the line is non-empty and does not start with `#`, it is passed to [collect_members](collect_members.md) to accumulate more member names.
   - Empty lines and comment lines (starting with `#`) are skipped.
3. **Unclosed block** — If the loop exhausts `lines` without finding `}`, the function returns `None`.

### Rule suffix extraction

After the closing `}`, the remaining text on the same line is trimmed. If it begins with `:`, the colon is stripped and the rest becomes the rule suffix string. This suffix is later split on `:` by the caller ([read_config](read_config.md)) and passed to [parse_and_insert_rules](parse_and_insert_rules.md).

If the text after `}` does not start with `:` (or is empty), `rule_suffix` is `None`, which causes the caller to report a "missing rule" error for the group.

### Example: multi-line group

Given the following config file lines (0-indexed):

```text
L0: my_group {
L1:     game.exe: helper.exe
L2:     launcher.exe
L3: }:high:0-7
```

The caller ([read_config](read_config.md)) detects `{` on line 0 and calls:

```text
collect_group_block(lines, 1, "")
```

The function:

1. Skips `first_line_content` (empty).
2. Processes line 1: collects `["game.exe", "helper.exe"]`.
3. Processes line 2: collects `["launcher.exe"]`.
4. Processes line 3: finds `}`, extracts the suffix `"high:0-7"`.
5. Returns `Some((["game.exe", "helper.exe", "launcher.exe"], Some("high:0-7"), 4))`.

### Example: single-line after brace

```text
L0: my_group { game.exe: helper.exe
L1: launcher.exe
L2: }:high:0-7
```

Here the caller passes `first_line_content = "game.exe: helper.exe"` and `start_index = 1`:

1. Parses `first_line_content` → `["game.exe", "helper.exe"]`.
2. Processes line 1 → `["launcher.exe"]`.
3. Processes line 2 → finds `}`, suffix = `"high:0-7"`.
4. Returns `Some((["game.exe", "helper.exe", "launcher.exe"], Some("high:0-7"), 3))`.

### Comment handling within blocks

Lines starting with `#` inside a group block are skipped entirely. Inline comments within a member line are handled by [collect_members](collect_members.md), which skips segments that start with `#` after splitting on `:`.

### Visibility

This function has **crate-private** visibility (`fn`, not `pub fn`). It is called only by [read_config](read_config.md) and [sort_and_group_config](sort_and_group_config.md) within the `config` module.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [read_config](read_config.md), [sort_and_group_config](sort_and_group_config.md) |
| **Callees** | [collect_members](collect_members.md) |
| **API** | Pure function — no I/O, no Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Member name tokenizer | [collect_members](collect_members.md) |
| Main config file reader (caller) | [read_config](read_config.md) |
| Rule field parsing (consumes member list) | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Auto-grouping utility (also uses group blocks) | [sort_and_group_config](sort_and_group_config.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd