# parse_alias function (config.rs)

Parses a CPU alias definition line and registers the alias name with its resolved CPU indices in the aliases map.

## Syntax

```rust
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
)
```

## Parameters

`name`

The alias name (without the leading `*` prefix). Must be non-empty. The name is stored in lowercase in the aliases map.

`value`

The CPU specification string to associate with this alias. Parsed via [parse_cpu_spec](parse_cpu_spec.md) to produce a `Vec<u32>` of CPU indices. Supports all CPU spec formats (ranges, individual indices, hex masks).

`line_number`

The 1-based line number in the config file where this alias definition appears. Used for error and warning messages.

`cpu_aliases`

Mutable reference to the `HashMap<String, Vec<u32>>` that stores all registered CPU aliases. The new alias is inserted (or overwrites an existing entry) keyed by the lowercase alias name.

`result`

Mutable reference to the [ConfigResult](ConfigResult.md) accumulator. On success, `aliases_count` is incremented. Errors are pushed to `result.errors` and warnings to `result.warnings`.

## Return value

This function does not return a value. Results are communicated through mutations to `cpu_aliases` and `result`.

## Remarks

Alias definitions appear in the config file with the format:

```
*alias_name = cpu_spec
```

The leading `*` and `=` are stripped by the caller ([read_config](read_config.md)) before `parse_alias` is invoked. The `name` parameter receives only the identifier portion (e.g., `"p"` from `*p = 0-7`).

### Validation rules

- If `name` is empty, an error is pushed: `"Line N: Empty alias name"`.
- If the parsed CPU set is empty and the raw value is not `"0"`, a warning is emitted indicating the alias resolved to an empty set. This catches typos or malformed specs without failing hard.

### Usage in config files

Aliases are referenced elsewhere in config rules by prefixing with `*`:

```
*p = 0-7
*e = 8-19
chrome.exe:high:*p:@*e:?8x*p
```

Aliases are resolved by [resolve_cpu_spec](resolve_cpu_spec.md), [parse_ideal_processor_spec](parse_ideal_processor_spec.md), and inline within [parse_and_insert_rules](parse_and_insert_rules.md) when parsing prime thread and ideal processor fields.

### Redefinition behavior

If an alias name is defined more than once, the later definition silently overwrites the earlier one. The `aliases_count` is incremented for each definition regardless.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | Private (`fn`) |
| **Line** | L294 |
| **Called by** | [read_config](read_config.md) |
| **Calls** | [parse_cpu_spec](parse_cpu_spec.md) |

## See also

- [parse_constant](parse_constant.md) — parses `@CONSTANT = value` definitions
- [resolve_cpu_spec](resolve_cpu_spec.md) — resolves `*alias` references at rule parse time
- [ConfigResult](ConfigResult.md) — accumulator for parse results