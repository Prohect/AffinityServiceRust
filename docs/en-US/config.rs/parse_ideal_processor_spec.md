# parse_ideal_processor_spec function (config.rs)

Parses an ideal processor specification string into a list of [IdealProcessorRule](IdealProcessorRule.md) entries. Each rule maps a set of CPU indices (resolved from an alias) to a list of thread start-module prefixes, enabling per-module ideal processor assignment within a single process.

## Syntax

```rust
fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule>
```

## Parameters

`spec: &str`

The ideal processor specification string from field 7 of a config rule line. The format consists of one or more segments, each beginning with `*` followed by a CPU alias name and optionally an `@` delimiter with semicolon-separated module prefixes. Examples:

- `"0"` — No ideal processor rules (returns empty vector).
- `"*pcore"` — Assign all threads to the alias `pcore`'s CPUs.
- `"*pcore@engine.dll;render.dll"` — Assign only threads from `engine.dll` or `render.dll` to `pcore` CPUs.
- `"*pcore@engine.dll*ecore@audio.dll"` — Two rules: engine threads to P-cores, audio threads to E-cores.

`line_number: usize`

The 1-based line number in the configuration file where the specification appears. Included in error messages for user diagnostics.

`cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>`

The map of currently defined CPU aliases, populated by earlier [parse_alias](parse_alias.md) calls. Keys are lowercase alias names (without the `*` prefix).

`errors: &mut Vec<String>`

**\[out\]** Accumulator for error messages. Errors are appended when the spec does not start with `*`, when an alias name is empty, or when an alias is not found in `cpu_aliases`.

## Return value

`Vec<IdealProcessorRule>` — A list of [IdealProcessorRule](IdealProcessorRule.md) entries, each containing a CPU index list and a prefix filter list. Returns an empty vector if the spec is `"0"`, empty, or entirely invalid.

## Remarks

### Parsing algorithm

1. **Trim and early exit:** The spec is trimmed. If empty or equal to `"0"`, an empty vector is returned immediately.
2. **Prefix validation:** If the spec does not start with `*`, an error is pushed and an empty vector is returned.
3. **Segment splitting:** The spec is split on `*` (the first empty segment from the leading `*` is skipped). Each non-empty segment represents one rule.
4. **Alias and prefix extraction:** Within each segment:
   - If `@` is present, the portion before `@` is the alias name and the portion after is a semicolon-separated list of module prefixes.
   - If `@` is absent, the entire segment is the alias name and the prefix list is empty (matching all threads).
5. **Alias resolution:** The alias name is lowercased and looked up in `cpu_aliases`. If the alias does not exist, an error is pushed and the segment is skipped.
6. **Empty CPU check:** If the resolved CPU list is empty (the alias maps to no CPUs), the segment is skipped entirely — no rule is produced.
7. **Prefix normalization:** Each prefix string is trimmed, lowercased, and empty strings are filtered out.
8. **Rule construction:** An [IdealProcessorRule](IdealProcessorRule.md) is created with the resolved CPUs and the prefix list, then pushed onto the result vector.

### Relationship with prime thread prefixes

The ideal processor specification syntax is similar to — but independent from — the prime thread prefix syntax parsed by [parse_and_insert_rules](parse_and_insert_rules.md) in field 4. Both use the `*alias@prefix` pattern, but they serve different purposes:

| Feature | Prime threads (field 4) | Ideal processors (field 7) |
|---------|------------------------|---------------------------|
| **Purpose** | Pin high-activity threads to dedicated CPUs via CPU sets | Set ideal processor hints for all matching threads |
| **Enforcement** | Hard (CPU set restriction) | Soft (scheduler hint) |
| **Requires tracking** | Yes (`track_top_x_threads`) | No |
| **Per-prefix priority boost** | Yes (`!priority` suffix) | No |
| **Struct produced** | [PrimePrefix](PrimePrefix.md) | [IdealProcessorRule](IdealProcessorRule.md) |

### Field position ambiguity

Field 7 in the rule format can contain either an ideal processor spec (starting with `*`) or a grade number. The caller [parse_and_insert_rules](parse_and_insert_rules.md) disambiguates: if the field starts with `*` or equals `"0"`, it is treated as an ideal processor spec and the grade is read from field 8 (defaulting to 1). If the field parses as a plain integer, it is treated as the grade and no ideal processor rules are created.

### Error reporting

Errors from this function are appended to the `errors` vector and eventually appear in the [ConfigResult](ConfigResult.md)'s error list. The following conditions produce errors:

- **Missing `*` prefix:** `"Line {N}: Ideal processor spec must start with '*', got '{spec}'"` — The entire spec is rejected.
- **Empty alias name:** `"Line {N}: Empty alias in ideal processor rule '*{segment}'"` — The individual segment is skipped; other segments may still succeed.
- **Unknown alias:** `"Line {N}: Unknown CPU alias '*{alias}' in ideal processor specification"` — The segment is skipped.

### Config syntax example

```
*pcore = 0-7
*ecore = 8-19

# All threads get ideal processor hints on P-cores
game.exe:normal:0:0:0:none:none:*pcore:1

# Per-module ideal processor hints
game2.exe:normal:0:0:0:none:none:*pcore@engine.dll;render.dll*ecore@audio.dll:1
```

In the second example, two [IdealProcessorRule](IdealProcessorRule.md) entries are created:

1. `cpus: [0, 1, 2, 3, 4, 5, 6, 7], prefixes: ["engine.dll", "render.dll"]`
2. `cpus: [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19], prefixes: ["audio.dll"]`

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Private (`fn`) — internal to the config module |
| **Callers** | [parse_and_insert_rules](parse_and_insert_rules.md) (field 7 of rule lines) |
| **Callees** | CPU alias map lookup (no function calls; inline alias resolution) |
| **Dependencies** | [IdealProcessorRule](IdealProcessorRule.md), [HashMap](../collections.rs/README.md), [List](../collections.rs/README.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Ideal processor rule struct | [IdealProcessorRule](IdealProcessorRule.md) |
| Rule insertion function | [parse_and_insert_rules](parse_and_insert_rules.md) |
| CPU alias definitions | [parse_alias](parse_alias.md) |
| Runtime ideal processor application | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| Prime thread prefix (related concept) | [PrimePrefix](PrimePrefix.md) |
| Alias resolution for other fields | [resolve_cpu_spec](resolve_cpu_spec.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*