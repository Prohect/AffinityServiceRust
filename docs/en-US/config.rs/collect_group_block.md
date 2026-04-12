# collect_group_block function (config.rs)

Collects members from a multi-line group block until the closing brace is found. Handles both single-line `{ a, b }` and multi-line group definitions that span several lines in the config file.

## Syntax

```rust
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## Parameters

`lines`

A slice of all lines from the config file being parsed.

`start_index`

The line index to begin scanning from (the line immediately after the opening brace `{`).

`first_line_content`

Any content that appeared on the same line as the opening brace, after the `{` character. This content is parsed for member names before scanning subsequent lines.

## Return value

Returns `Option<(Vec<String>, Option<String>, usize)>`:

- **`Some((members, rule_suffix, next_index))`** — Successfully found the closing brace `}`.
  - `members` — A `Vec<String>` of collected process names from within the braces.
  - `rule_suffix` — An `Option<String>` containing the rule definition that follows `}:` (the part after the colon). `None` if no colon follows the closing brace.
  - `next_index` — The line index to resume parsing from (the line after the one containing `}`).

- **`None`** — The closing brace `}` was never found, indicating an unclosed group block. The caller should emit an error.

## Remarks

This function is called by [read_config](read_config.md) when it encounters a line containing `{` but no matching `}` on the same line. It scans forward through subsequent lines, collecting process names separated by colons via [collect_members](collect_members.md).

Lines that are empty or start with `#` (comments) are skipped during member collection.

When the closing `}` is found, any content before it on that line is also collected as members. The text after `}` is checked for a `:` prefix — if present, the remainder is returned as the rule suffix that will be passed to [parse_and_insert_rules](parse_and_insert_rules.md).

### Example

A multi-line group block in config:

```
browsers {
    chrome.exe
    firefox.exe
    msedge.exe
}:high:*p:0:0:none:none
```

When `read_config` encounters the `browsers {` line, it calls `collect_group_block` with:
- `lines` — all config lines
- `start_index` — index of the `chrome.exe` line
- `first_line_content` — empty string (nothing after `{` on the first line)

The function returns:
- `members`: `["chrome.exe", "firefox.exe", "msedge.exe"]`
- `rule_suffix`: `Some("high:*p:0:0:none:none")`
- `next_index`: index of the line after `}:high:*p:0:0:none:none`

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Lines** | L381–L411 |
| **Visibility** | Private (`fn`) |
| **Called by** | [read_config](read_config.md) |
| **Calls** | [collect_members](collect_members.md) |