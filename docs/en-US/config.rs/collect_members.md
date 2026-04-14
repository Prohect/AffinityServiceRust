# collect_members function (config.rs)

Splits a colon-delimited text string into individual member names, trimming whitespace and lowercasing each entry, then appends the results to an existing vector. This function is the shared tokenizer used by both inline rule lines and multi-line group blocks to extract process names from configuration text.

## Syntax

```rust
fn collect_members(text: &str, members: &mut Vec<String>)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `text` | `&str` | A colon-separated string of member names (e.g., `"game.exe: helper.exe: launcher.exe"`). Each segment is trimmed and lowercased before being added. Segments that are empty after trimming or that begin with `#` (comments) are skipped. |
| `members` | `&mut Vec<String>` | Output vector to which parsed member names are appended. Existing entries in the vector are preserved; new names are pushed to the end. The caller is responsible for deduplication if needed. |

## Return value

This function has no return value. Results are accumulated into the `members` vector passed by mutable reference.

## Remarks

### Parsing rules

1. The input `text` is split on the `:` character.
2. Each resulting segment is trimmed of leading and trailing whitespace, then lowercased.
3. A segment is **skipped** if:
   - It is empty after trimming.
   - It starts with `#` (treated as an inline comment).
4. All surviving segments are pushed onto `members` as `String` values.

### Case normalization

All member names are converted to lowercase via `to_lowercase()`. This ensures case-insensitive process name matching throughout the service, since Windows process names are case-insensitive.

### Comment handling

The `#` check allows inline comments within group blocks. For example, in a multi-line group block:

```
my_group {
    game.exe: helper.exe
    # this line is a comment and is skipped by the caller
    launcher.exe: updater.exe
}:high:0-7
```

The `#`-prefixed segments within a single line are also filtered. For example, `"game.exe: # not a process"` yields only `["game.exe"]`.

### No deduplication

`collect_members` does not check for duplicate names. If the same process name appears multiple times across calls (e.g., on different lines of a group block), it will appear multiple times in `members`. Deduplication, if needed, is handled downstream — for example, [sort_and_group_config](sort_and_group_config.md) calls `dedup()` on the member list after sorting.

### Visibility

This function has **crate-private** visibility (`fn`, not `pub fn`). It is called only within the `config` module by [read_config](read_config.md) (for inline group parsing) and [collect_group_block](collect_group_block.md) (for multi-line group blocks).

### Usage context

`collect_members` is typically called in one of two scenarios:

1. **Inline group** — When [read_config](read_config.md) encounters a single-line group like `{ a: b: c }:rule`, it extracts the text between `{` and `}` and passes it to `collect_members`.
2. **Multi-line group** — [collect_group_block](collect_group_block.md) calls `collect_members` once per non-empty, non-comment line within the `{ ... }` block, accumulating all members across lines.

### Examples

| Input `text` | Resulting entries appended |
|--------------|---------------------------|
| `"game.exe: helper.exe"` | `["game.exe", "helper.exe"]` |
| `"  GAME.EXE : Helper.EXE "` | `["game.exe", "helper.exe"]` |
| `"single.exe"` | `["single.exe"]` |
| `""` | *(none)* |
| `"# comment"` | *(none)* |
| `"a.exe: # comment: b.exe"` | `["a.exe", "b.exe"]` |

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [read_config](read_config.md), [collect_group_block](collect_group_block.md), [sort_and_group_config](sort_and_group_config.md) |
| **Callees** | None (standard library string operations only) |
| **API** | Pure function — no I/O, no Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Multi-line group block collector | [collect_group_block](collect_group_block.md) |
| Main config file parser | [read_config](read_config.md) |
| Rule field parsing (consumes member list) | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Auto-grouping utility | [sort_and_group_config](sort_and_group_config.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd