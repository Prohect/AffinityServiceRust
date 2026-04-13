# resolve_cpu_spec function (config.rs)

Resolves a CPU specification string that may be either a literal CPU spec or a reference to a previously defined CPU alias. When the spec begins with `*`, the remainder is treated as an alias name and looked up in the provided alias map. Otherwise, the spec is forwarded to [parse_cpu_spec](parse_cpu_spec.md) for direct parsing.

## Syntax

```rust
fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<u32>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `spec` | `&str` | The CPU specification string to resolve. Leading and trailing whitespace is trimmed. If it starts with `*`, the rest is treated as a case-insensitive alias name. Otherwise it is parsed as a literal CPU spec by [parse_cpu_spec](parse_cpu_spec.md). |
| `field_name` | `&str` | The name of the rule field being parsed (e.g., `"affinity"`, `"cpuset"`, `"prime_cpus"`). Used only in error messages to help the user locate the problem. |
| `line_number` | `usize` | The 1-based line number in the configuration file where this spec was encountered. Included in error messages. |
| `cpu_aliases` | `&HashMap<String, Vec<u32>>` | Map of defined CPU aliases, keyed by lowercased alias name (without the leading `*`), with values being the resolved `Vec<u32>` of CPU indices. Built up by earlier [parse_alias](parse_alias.md) calls during the parse pass. |
| `errors` | `&mut Vec<String>` | Mutable reference to the error list in the current parse context. If an alias reference cannot be resolved, a descriptive error string is pushed here. |

## Return value

Returns a `Vec<u32>` of sorted CPU indices.

- **Alias path (`*name`):** Returns the cloned CPU index vector from the alias map if the alias exists. Returns an empty vector and pushes an error if the alias is undefined.
- **Literal path:** Returns the output of [parse_cpu_spec](parse_cpu_spec.md) applied to the trimmed spec string.

## Remarks

### Alias resolution

Alias names are matched case-insensitively. The leading `*` is stripped, the remainder is lowercased, and the result is looked up in `cpu_aliases`. This means `*Perf`, `*PERF`, and `*perf` all resolve to the same alias entry.

When the alias is not found, the error message follows the format:

```text
Line {line_number}: Undefined alias '*{alias}' in {field_name} field
```

The function still returns an empty vector in this case, allowing parsing to continue and accumulate additional errors rather than aborting on the first failure.

### Literal spec pass-through

When the spec does not start with `*`, it is forwarded directly to [parse_cpu_spec](parse_cpu_spec.md) without any additional validation or error reporting. Any issues with the literal spec (e.g., malformed ranges) are handled silently by `parse_cpu_spec`'s lenient parsing behavior.

### Visibility

This function has **crate-private** visibility (`fn`, not `pub fn`). It is called internally by [parse_and_insert_rules](parse_and_insert_rules.md) when resolving the `affinity`, `cpuset`, and `prime_cpus` fields of a rule line.

### Whitespace handling

The `spec` parameter is trimmed at the start of the function. This ensures that specs like `" *perf "` or `" 0-7 "` are handled correctly regardless of whitespace in the config file.

### Examples

Given the alias map `{ "perf": [0, 1, 2, 3], "eff": [4, 5, 6, 7] }`:

| Input `spec` | Result | Error pushed? |
|--------------|--------|---------------|
| `"*perf"` | `[0, 1, 2, 3]` | No |
| `"*EFF"` | `[4, 5, 6, 7]` | No |
| `"*unknown"` | `[]` | Yes — `"Line N: Undefined alias '*unknown' in {field_name} field"` |
| `"0-7"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | No |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | No |
| `"0"` | `[]` | No |

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Calls** | [parse_cpu_spec](parse_cpu_spec.md) |
| **API** | Pure function — no I/O, no Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| CPU spec string parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Alias definition parsing | [parse_alias](parse_alias.md) |
| Rule field parsing (primary caller) | [parse_and_insert_rules](parse_and_insert_rules.md) |
| CPU indices to bitmask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| Per-process config (fields that use aliases) | [ProcessConfig](ProcessConfig.md) |
| Config module overview | [README](README.md) |