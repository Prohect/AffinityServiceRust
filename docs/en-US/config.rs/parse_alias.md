# parse_alias function (config.rs)

Parses a `*name = cpu_spec` alias definition line from the configuration file and inserts the resulting CPU index list into the alias map. Aliases provide named shortcuts for CPU specifications that can be referenced elsewhere in the config using the `*name` syntax.

## Syntax

```rust
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## Parameters

`name: &str`

The alias name (without the leading `*` prefix), already lowercased by the caller. For example, given the config line `*PCore = 0-7`, the caller passes `"pcore"`. An empty name triggers an error.

`value: &str`

The CPU specification string to the right of the `=` sign, already trimmed by the caller. This is forwarded to [parse_cpu_spec](parse_cpu_spec.md) for parsing into a list of CPU indices. Supports ranges (`0-7`), semicolon-separated values (`0;4;8`), hex masks (`0xFF`), or `"0"` (empty set).

`line_number: usize`

The 1-based line number in the config file where this alias definition appears. Included in error and warning messages.

`cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>`

**\[in, out\]** The alias map being built during config parsing. On success, a new entry is inserted with `name` as the key and the parsed CPU list as the value. If an alias with the same name already exists, it is silently overwritten.

`result: &mut ConfigResult`

**\[in, out\]** The parse result accumulator. On success, `result.aliases_count` is incremented. On failure (empty name), an error is pushed to `result.errors`. If the parsed CPU set is empty for a non-`"0"` value, a warning is pushed to `result.warnings`.

## Return value

This function does not return a value. Results are communicated through `cpu_aliases` and `result`.

## Remarks

### Validation

The function performs two validation checks:

1. **Empty name:** If `name` is empty, an error `"Line {N}: Empty alias name"` is pushed and the function returns without inserting anything.
2. **Empty CPU set with non-zero value:** If [parse_cpu_spec](parse_cpu_spec.md) returns an empty list but `value` is not literally `"0"`, a warning is emitted because the user likely made a typo in the CPU specification. The alias is still inserted with an empty CPU list.

### Alias naming conventions

- Alias names are case-insensitive. The caller lowercases the name before passing it.
- There are no restrictions on alias name characters beyond what the config parser's line-splitting logic imposes (the name is everything between `*` and `=`).
- Common conventions include `pcore`, `ecore`, `perf`, `efficiency`, or application-specific names like `game_cpus`.

### Overwriting behavior

If the same alias name is defined multiple times in the config file, each subsequent definition silently overwrites the previous one. No warning is emitted for redefinition — only the final value is retained in `cpu_aliases`. Rule lines that reference the alias will use whatever value was current at their line position (since aliases are parsed before rules in sequential order).

### Config syntax example

```
*pcore = 0-7
*ecore = 8-19
*all = 0-19
*fast = 0;2;4;6
```

### Interaction with resolve_cpu_spec

Once defined, aliases are consumed by [resolve_cpu_spec](resolve_cpu_spec.md) whenever a CPU field value starts with `*`. For example, the affinity field `*pcore` in a rule line causes `resolve_cpu_spec` to look up `"pcore"` in the alias map and return the stored CPU list `[0, 1, 2, 3, 4, 5, 6, 7]`.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Private (`fn`) |
| **Callers** | [read_config](read_config.md) (for each `*name = value` line encountered) |
| **Callees** | [parse_cpu_spec](parse_cpu_spec.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Alias consumer | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Constant parser (similar pattern) | [parse_constant](parse_constant.md) |
| Config file reader | [read_config](read_config.md) |
| Aggregate result | [ConfigResult](ConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*