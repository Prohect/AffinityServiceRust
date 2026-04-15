# collect_members function (config.rs)

Splits a colon-delimited string of process names into individual lowercase member entries and appends them to the provided list. This is a helper function used internally by the configuration parser to extract process names from both inline rule lines and `{ }` group blocks.

## Syntax

```AffinityServiceRust/src/config.rs#L242-249
fn collect_members(text: &str, members: &mut Vec<String>) {
    for item in text.split(':') {
        let item = item.trim().to_lowercase();
        if !item.is_empty() && !item.starts_with('#') {
            members.push(item);
        }
    }
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `text` | `&str` | A string containing one or more process names separated by colons (`:`). May also contain inline comments (tokens starting with `#`) and whitespace, both of which are filtered out. |
| `members` | `&mut Vec<String>` | A mutable reference to a vector that collected member names are appended to. The caller is responsible for initializing this vector; `collect_members` only appends and never clears it. |

## Return value

This function does not return a value. Results are accumulated in the `members` vector passed by the caller.

## Remarks

### Processing steps

1. The input `text` is split on the `:` delimiter.
2. Each resulting token is trimmed of leading and trailing whitespace and converted to lowercase.
3. Tokens that are empty after trimming or that begin with `#` (inline comments) are discarded.
4. All remaining tokens are pushed onto the `members` vector.

### Deduplication

`collect_members` does **not** perform deduplication. If the same process name appears multiple times in `text`, it will be added to `members` multiple times. Deduplication is the responsibility of downstream consumers (e.g., [`parse_and_insert_rules`](parse_and_insert_rules.md) detects and warns about redundant rules).

### Case normalization

All member names are lowercased before insertion. This ensures case-insensitive matching at runtime, since Windows process names are case-insensitive.

### Usage context

This function is called from two sites:

- **[`collect_group_block`](collect_group_block.md)**: Collects members from each line inside a multi-line `{ }` group block and from the content before the closing brace.
- **[`read_config`](read_config.md)**: Collects members from single-line group blocks where both the opening and closing braces appear on the same line.
- **[`sort_and_group_config`](sort_and_group_config.md)**: Collects members when re-parsing group blocks during the auto-grouping pass.

### Edge cases

| Input | Result |
|-------|--------|
| `""` (empty string) | Nothing appended |
| `"  "` (whitespace only) | Nothing appended |
| `"# comment"` | Nothing appended (comment filtered) |
| `"game.exe"` | `["game.exe"]` appended |
| `"Game.EXE : app.exe"` | `["game.exe", "app.exe"]` appended |
| `"a.exe: : b.exe"` | `["a.exe", "b.exe"]` appended (empty token skipped) |

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (`fn`, not `pub fn`) |
| Callers | [`collect_group_block`](collect_group_block.md), [`read_config`](read_config.md), [`sort_and_group_config`](sort_and_group_config.md) |
| Callees | `str::split`, `str::trim`, `str::to_lowercase`, `str::starts_with`, `Vec::push` |
| API | Standard library only |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| collect_group_block | [collect_group_block](collect_group_block.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| read_config | [read_config](read_config.md) |
| sort_and_group_config | [sort_and_group_config](sort_and_group_config.md) |
| config module overview | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*