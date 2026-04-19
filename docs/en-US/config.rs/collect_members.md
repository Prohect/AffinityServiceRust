# collect_members function (config.rs)

Parses a colon-separated list of process names from a text fragment and appends them to an accumulator vector. Used internally to extract group member names from both single-line and multi-line group block definitions.

## Syntax

```rust
fn collect_members(text: &str, members: &mut Vec<String>)
```

## Parameters

`text: &str`

A string fragment containing colon-separated process names. Typically the content between `{` and `}` in a group definition, or a single line within a multi-line group block. Whitespace around each item is trimmed. Items beginning with `#` are treated as comments and skipped.

`members: &mut Vec<String>`

**\[in, out\]** Accumulator vector to which parsed process names are appended. Existing entries are preserved; new names are pushed onto the end. All names are normalized to lowercase.

## Return value

This function does not return a value. Results are communicated through the `members` out-parameter.

## Remarks

### Parsing rules

1. The input `text` is split on `:` (colon) characters.
2. Each resulting fragment is trimmed of leading and trailing whitespace.
3. The fragment is converted to lowercase for case-insensitive matching downstream.
4. Empty fragments and fragments starting with `#` are discarded.
5. All surviving fragments are pushed onto `members`.

### Separator choice

The colon `:` separator is used because the main rule syntax already uses `:` as a field delimiter. Within a group block (between `{` and `}`), process names are separated by colons — not commas or semicolons — to maintain consistency with the outer config line format.

### No deduplication

`collect_members` does **not** check for duplicate names. If the same process name appears multiple times in the input or across multiple calls, it will appear multiple times in `members`. Deduplication, if needed, is handled by the caller or by later stages such as [sort_and_group_config](sort_and_group_config.md).

### Example

Given the input text `"game.exe: helper.exe: # comment: tool.EXE"`, the function appends `["game.exe", "helper.exe", "tool.exe"]` to `members`. The comment fragment is skipped, and `tool.EXE` is lowercased.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Private (`fn`) — internal to the config module |
| **Callers** | [read_config](read_config.md) (inline group parsing), [collect_group_block](collect_group_block.md) (multi-line group parsing), [sort_and_group_config](sort_and_group_config.md) (auto-grouping tool) |
| **Callees** | None (uses only `str` standard library methods) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Multi-line group block collector | [collect_group_block](collect_group_block.md) |
| Rule parser that consumes group members | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Main config reader | [read_config](read_config.md) |
| Auto-grouping utility | [sort_and_group_config](sort_and_group_config.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*