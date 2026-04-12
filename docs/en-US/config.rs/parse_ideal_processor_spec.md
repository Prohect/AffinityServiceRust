# parse_ideal_processor_spec function (config.rs)

Parses an ideal processor specification string into a list of [IdealProcessorRule](IdealProcessorRule.md) entries, resolving CPU aliases and optional module prefix filters.

## Syntax

```rust
fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule>
```

## Parameters

`spec`

The ideal processor specification string to parse. Must start with `*` for each rule segment. The format is `*alias[@prefix1;prefix2]` where segments can be chained: `*p@engine.dll*e@helper.dll`. A value of `"0"` or an empty string means no ideal processor rules.

`line_number`

The 1-based line number in the config file where this specification appears, used for error reporting.

`cpu_aliases`

A map of alias names (without the `*` prefix) to their resolved CPU index vectors. Each alias referenced in the spec must exist in this map or an error is recorded.

`errors`

A mutable vector to which parsing error messages are appended when the spec is malformed or references undefined aliases.

## Return value

Returns a `Vec<`[IdealProcessorRule](IdealProcessorRule.md)`>` containing the parsed rules. Each rule has a `cpus` field (resolved from the alias) and a `prefixes` field (module name filters parsed from the `@` section). Returns an empty vector if the spec is `"0"`, empty, or entirely invalid.

## Remarks

The specification uses a segment-based format where each segment begins with `*`:

| Token | Meaning |
| --- | --- |
| `*` | Segment delimiter and alias marker |
| `alias` | CPU alias name (resolved via `cpu_aliases`) |
| `@` | Separator between alias and prefix filter list |
| `;` | Separator between multiple prefix filters |

### Parsing algorithm

1. If `spec` is empty or `"0"`, return an empty vector immediately.
2. If `spec` does not start with `*`, push an error and return empty.
3. Split `spec` on `*` and skip the first empty segment.
4. For each segment:
   - Split on `@` to separate the alias part from the optional prefixes part.
   - Look up the alias (lowercased) in `cpu_aliases`. If not found, push an error and continue.
   - If the resolved CPU list is empty, skip the segment.
   - Split the prefixes part on `;`, trim and lowercase each entry, and filter out empties.
   - Construct an [IdealProcessorRule](IdealProcessorRule.md) with the resolved CPUs and prefix list.

### Examples

| Spec | Result |
| --- | --- |
| `*p` | One rule: alias `p` CPUs, no prefix filter (applies to all threads) |
| `*p@engine.dll` | One rule: alias `p` CPUs, only for threads starting in `engine.dll` |
| `*p@engine.dll;render.dll` | One rule: alias `p` CPUs, for `engine.dll` or `render.dll` threads |
| `*p@engine.dll*e@helper.dll` | Two rules: alias `p` for `engine.dll`, alias `e` for `helper.dll` |

When a rule has an empty `prefixes` vector, it acts as a catch-all rule that applies to all threads not matched by more specific prefix rules. This is used by [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) during thread scheduling.

This function is called internally by [parse_and_insert_rules](parse_and_insert_rules.md) when processing the ideal processor field (field index 6) of a config rule line.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | Private (`fn`) |
| **Line** | L310–L379 |
| **Called by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Depends on** | [IdealProcessorRule](IdealProcessorRule.md), [resolve_cpu_spec](resolve_cpu_spec.md) (indirectly via alias map) |

## See also

- [IdealProcessorRule](IdealProcessorRule.md) — the struct produced by this function
- [parse_and_insert_rules](parse_and_insert_rules.md) — calls this function during rule parsing
- [ProcessConfig](ProcessConfig.md) — stores the parsed rules in the `ideal_processor_rules` field
