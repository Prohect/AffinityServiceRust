# resolve_cpu_spec function (config.rs)

Resolves a CPU specification string that may be either a literal CPU spec (ranges, hex masks, individual indices) or an alias reference (`*name`) into a sorted list of CPU indices. This is the internal dispatcher that sits between raw config field values and the parsed CPU index lists stored in configuration structs.

## Syntax

```AffinityServiceRust/src/config.rs#L220-240
fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `spec` | `&str` | The CPU specification string to resolve. Leading/trailing whitespace is trimmed. If the string begins with `*`, it is treated as an alias reference; otherwise it is parsed as a literal CPU spec by [`parse_cpu_spec`](parse_cpu_spec.md). |
| `field_name` | `&str` | A human-readable name identifying which config field is being resolved (e.g., `"affinity"`, `"cpuset"`, `"prime_cpus"`). Used only in error messages for context. |
| `line_number` | `usize` | The 1-based line number in the configuration file where this spec appears. Used in error messages to help users locate problems. |
| `cpu_aliases` | `&HashMap<String, List<[u32; CONSUMER_CPUS]>>` | The alias lookup table populated by earlier `*name = cpu_spec` lines in the config file. Keys are lowercase alias names (without the leading `*`). |
| `errors` | `&mut Vec<String>` | A mutable reference to the error accumulator. An error is pushed when an alias reference cannot be found in `cpu_aliases`. |

## Return value

Type: `List<[u32; CONSUMER_CPUS]>`

A sorted list of CPU indices. Returns an empty list when:
- The spec is `"0"` or empty (passed through to [`parse_cpu_spec`](parse_cpu_spec.md)).
- The spec references an undefined alias (an error is also recorded).

## Remarks

### Alias resolution

When the trimmed `spec` starts with `*`, the function:

1. Strips the leading `*` character.
2. Converts the remainder to lowercase.
3. Looks up the result in `cpu_aliases`.
4. If the alias exists, returns a clone of the stored CPU list.
5. If the alias does not exist, pushes a descriptive error message to `errors` and returns an empty default list.

The error message follows the format:

```/dev/null/example.txt#L1
Line {line_number}: Undefined alias '*{alias}' in {field_name} field
```

### Literal spec pass-through

When the spec does not start with `*`, the function delegates directly to [`parse_cpu_spec`](parse_cpu_spec.md), which handles ranges (`0-7`), individual indices (`0;4;8`), hex bitmasks (`0xFF`), and the special value `"0"` (no change).

### Visibility

This function is module-private (`fn`, not `pub fn`). It is called by [`parse_and_insert_rules`](parse_and_insert_rules.md) for the affinity and cpuset fields and is not accessible outside the `config` module.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (crate-internal) |
| Callers | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| Callees | [`parse_cpu_spec`](parse_cpu_spec.md), `HashMap::get`, `HashMap::contains_key` |
| API | None |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| config module overview | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*