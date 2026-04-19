# resolve_cpu_spec function (config.rs)

Resolves a CPU specification string that may contain an alias reference. If the spec begins with `*`, it is treated as an alias lookup; otherwise it is parsed directly as a CPU specification.

## Syntax

```rust
fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

`spec: &str`

The CPU specification string to resolve. If it starts with `*`, the remainder is treated as an alias name (case-insensitive). Otherwise the string is forwarded to [parse_cpu_spec](parse_cpu_spec.md) for direct parsing.

`field_name: &str`

A human-readable name for the config field being parsed (e.g., `"affinity"`, `"cpuset"`, `"prime_cpus"`). Used in error messages to indicate which field contained the invalid alias.

`line_number: usize`

The 1-based line number in the config file where this specification appears. Included in error messages for user diagnostics.

`cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>`

The map of currently defined CPU aliases, populated by earlier [parse_alias](parse_alias.md) calls. Keys are lowercase alias names (without the `*` prefix).

`errors: &mut Vec<String>`

**[out]** Accumulator for error messages. An error is appended if an alias reference (`*name`) refers to an undefined alias.

## Return value

`List<[u32; CONSUMER_CPUS]>` — A sorted list of CPU indices. Returns the alias's CPU list if the alias exists, an empty list if the alias is undefined, or the result of [parse_cpu_spec](parse_cpu_spec.md) for non-alias specs.

## Remarks

This function is the central indirection layer between raw config field values and the CPU index lists stored in [ProcessLevelConfig](ProcessLevelConfig.md) and [ThreadLevelConfig](ThreadLevelConfig.md). It is called by [parse_and_insert_rules](parse_and_insert_rules.md) for the affinity, cpuset, and prime CPU fields.

### Alias resolution

1. The input `spec` is trimmed of leading/trailing whitespace.
2. If the trimmed string starts with `*`, the `*` prefix is stripped and the remainder is lowercased to form the alias key.
3. The alias key is looked up in `cpu_aliases`. If found, the associated CPU list is cloned and returned.
4. If not found, an error is pushed in the format `"Line {N}: Undefined alias '*{name}' in {field_name} field"` and an empty list is returned.

### Direct parsing

If the spec does not start with `*`, it is passed directly to [parse_cpu_spec](parse_cpu_spec.md), which handles ranges (`0-7`), individual CPUs (`0;4;8`), hex masks (`0xFF`), and the special value `"0"` (empty list / no change).

### Visibility

This function is module-private (`fn`, not `pub fn`). It is an internal helper used only within `config.rs`.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Private |
| **Callers** | [parse_and_insert_rules](parse_and_insert_rules.md) (affinity, cpuset, prime CPU fields) |
| **Callees** | [parse_cpu_spec](parse_cpu_spec.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Alias definition | [parse_alias](parse_alias.md) |
| Rule insertion | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Config reader | [read_config](read_config.md) |
| Module overview | [config.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*