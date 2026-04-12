# collect_members function (config.rs)

Collects process name members from a colon-delimited text string into a vector, filtering out empty entries and comments.

## Syntax

```rust
fn collect_members(text: &str, members: &mut Vec<String>)
```

## Parameters

`text`

A string slice containing colon-separated process names. Each segment is trimmed and lowercased before insertion. Segments that are empty or begin with `#` (comments) are skipped.

`members`

A mutable reference to a `Vec<String>` that receives the parsed member names. New entries are appended to any existing contents.

## Return value

This function does not return a value. Results are accumulated in the `members` output parameter.

## Remarks

This function is used internally by the config parser to extract process names from both single-line and multi-line group definitions. It splits the input on `:` delimiters, which is the standard separator for process names within group blocks.

Each extracted name is:
1. Trimmed of leading and trailing whitespace.
2. Converted to lowercase for case-insensitive matching.
3. Discarded if empty or if it starts with `#` (inline comment).

### Example

Given the input text `"chrome.exe: Firefox.exe: # comment: edge.exe"`, the function appends `["chrome.exe", "firefox.exe", "edge.exe"]` to the `members` vector.

This function is called by [read_config](read_config.md) when parsing group blocks (both inline `{ a: b }` and multi-line forms) and by [collect_group_block](collect_group_block.md) when accumulating members across continuation lines.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | Private (`fn`) |
| **Called by** | [read_config](read_config.md), [collect_group_block](collect_group_block.md) |
| **Depends on** | None |