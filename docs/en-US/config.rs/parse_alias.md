# parse_alias function (config.rs)

Parses and registers a `*name = cpu_spec` alias definition from the configuration file. Aliases provide named shortcuts for CPU specifications that can be referenced elsewhere in the config using the `*name` syntax, allowing users to define their CPU topology once and reuse it across multiple rules.

## Syntax

```AffinityServiceRust/src/config.rs#L293-313
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `&str` | The alias name (without the leading `*`) already lowercased and trimmed by the caller. For example, given the config line `*pcore = 0-7`, the caller passes `"pcore"`. |
| `value` | `&str` | The CPU specification string to associate with this alias (e.g., `"0-7"`, `"0;4;8"`, `"0xFF"`). Parsed by [`parse_cpu_spec`](parse_cpu_spec.md). |
| `line_number` | `usize` | The 1-based line number in the configuration file where this alias definition appears. Used in error and warning messages. |
| `cpu_aliases` | `&mut HashMap<String, List<[u32; CONSUMER_CPUS]>>` | The mutable alias lookup table. The new alias is inserted (or overwrites a prior definition with the same name) as a key-value pair of `(name, parsed_cpus)`. |
| `result` | `&mut ConfigResult` | The mutable configuration result accumulator. Errors are pushed to `result.errors`; warnings to `result.warnings`; and `result.aliases_count` is incremented on success. |

## Return value

This function does not return a value. Side effects are applied to `cpu_aliases` and `result`.

## Remarks

### Processing steps

1. If `name` is empty, an error is pushed: `"Line {line_number}: Empty alias name"`.
2. Otherwise, the `value` string is parsed via [`parse_cpu_spec`](parse_cpu_spec.md) into a sorted list of CPU indices.
3. If the parsed CPU list is empty **and** the original value is not the literal string `"0"`, a warning is pushed indicating the alias resolved to an empty CPU set. This distinguishes between an intentional no-op alias (`*empty = 0`) and a parse failure.
4. The alias is inserted into `cpu_aliases` under the provided `name`. If an alias with the same name already exists, it is silently overwritten.
5. `result.aliases_count` is incremented by 1.

### Configuration file syntax

Aliases are defined in the configuration file with the `*` prefix:

```/dev/null/example.ini#L1-3
*pcore = 0-7
*ecore = 8-19
*all = 0-19
```

These can then be referenced in rule lines:

```/dev/null/example.ini#L1
game.exe:high:*pcore:*ecore:0:none:none:0:1
```

### Name collisions

If two alias definitions share the same name, the later definition overwrites the earlier one without producing a warning or error. This allows users to redefine aliases in config file sections that are conditionally included.

### Empty alias warning

The warning message follows the format:

```/dev/null/example.txt#L1
Line {line_number}: Alias '*{name}' has empty CPU set from '{value}'
```

This alerts the user that the alias will effectively be a no-op when referenced, which is likely a configuration mistake (e.g., a typo in a range expression).

### Visibility

This function is module-private (`fn`, not `pub fn`). It is called exclusively by [`read_config`](read_config.md) during the line-by-line parsing pass.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (crate-internal) |
| Callers | [`read_config`](read_config.md) |
| Callees | [`parse_cpu_spec`](parse_cpu_spec.md) |
| Dependencies | `HashMap` from [`collections.rs`](../collections.rs/README.md), `List` and `CONSUMER_CPUS` |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_constant | [parse_constant](parse_constant.md) |
| read_config | [read_config](read_config.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| config module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*