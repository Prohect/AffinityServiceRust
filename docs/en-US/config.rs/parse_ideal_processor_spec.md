# parse_ideal_processor_spec function (config.rs)

Parses an ideal processor specification string into a vector of [IdealProcessorRule](IdealProcessorRule.md) entries. The specification format uses `*alias` segments to reference CPU aliases, with optional `@prefix` suffixes to filter which threads receive ideal processor assignment based on their start-address module name.

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

| Parameter | Type | Description |
|-----------|------|-------------|
| `spec` | `&str` | The ideal processor specification string from field 7 of a rule line. Must start with `*` unless it is `"0"` or empty. Leading and trailing whitespace is trimmed. |
| `line_number` | `usize` | 1-based line number in the configuration file. Used in error messages to help the user locate problems. |
| `cpu_aliases` | `&HashMap<String, Vec<u32>>` | Map of previously defined CPU aliases, keyed by lowercased alias name (without the `*` prefix). Built by earlier [parse_alias](parse_alias.md) calls during the parse pass. |
| `errors` | `&mut Vec<String>` | Mutable reference to the error accumulator. Errors for undefined aliases, empty alias names, or specs that do not start with `*` are pushed here. |

## Return value

Returns a `Vec<IdealProcessorRule>`. Each element maps a set of CPU indices to an optional list of module-name prefix filters.

Returns an empty vector when:

- `spec` is empty or contains only whitespace.
- `spec` is `"0"` (the canonical "disabled" sentinel).
- `spec` does not start with `*` (an error is also pushed).
- All referenced aliases resolve to empty CPU sets (segments with empty CPU lists are silently skipped).

## Remarks

### Specification format

The spec string consists of one or more `*`-delimited segments:

```
*alias[@prefix1;prefix2]*alias2[@prefix3;prefix4]
```

Each segment has the following structure:

| Component | Required | Description |
|-----------|----------|-------------|
| `*` | Yes | Segment delimiter and alias marker. The spec must start with `*`. |
| `alias` | Yes | A CPU alias name (case-insensitive). Looked up in `cpu_aliases` after lowercasing. Must have been defined earlier in the config file with `*alias = cpu_spec`. |
| `@` | No | Separator between the alias name and the prefix filter list. When absent, the rule applies to all threads. |
| `prefix1;prefix2` | No | Semicolon-separated list of module-name prefixes. Each prefix is trimmed, lowercased, and stored in the resulting [IdealProcessorRule](IdealProcessorRule.md)`.prefixes` vector. |

### Parsing algorithm

1. **Trim and early-exit** — The spec is trimmed. If empty or equal to `"0"`, return an empty vector immediately.
2. **Prefix validation** — If the spec does not start with `*`, push an error and return an empty vector.
3. **Segment splitting** — Split on `*` and skip the first (empty) element produced by the leading `*`.
4. **Per-segment processing:**
   a. Skip empty segments (produced by consecutive `**`).
   b. Split on `@` to separate the alias part from the optional prefixes part.
   c. Lowercase the alias name and look it up in `cpu_aliases`.
   d. If the alias is unknown, push an error and use an empty CPU vector.
   e. If the resolved CPU vector is empty, skip the segment (no rule is emitted).
   f. Split the prefixes part on `;`, trim and lowercase each entry, and filter out empty strings.
   g. Construct an [IdealProcessorRule](IdealProcessorRule.md) with the resolved `cpus` and `prefixes`, and push it onto the result vector.

### Error conditions

| Condition | Error message format | Behavior |
|-----------|---------------------|----------|
| Spec does not start with `*` | `"Line {N}: Ideal processor spec must start with '*', got '{spec}'"` | Returns empty vector. |
| Empty alias name in a segment | `"Line {N}: Empty alias in ideal processor rule '*{segment}'"` | Segment is skipped; parsing continues. |
| Undefined alias name | `"Line {N}: Unknown CPU alias '*{alias}' in ideal processor specification"` | Segment produces an empty CPU vector and is skipped; parsing continues. |

### Examples

Given aliases `{ "p": [0, 1, 2, 3], "e": [4, 5, 6, 7] }`:

| Spec string | Result | Notes |
|-------------|--------|-------|
| `"0"` | `[]` | Disabled. |
| `""` | `[]` | Disabled. |
| `"*p"` | `[IdealProcessorRule { cpus: [0,1,2,3], prefixes: [] }]` | All threads → CPUs 0–3. |
| `"*p@engine.dll"` | `[IdealProcessorRule { cpus: [0,1,2,3], prefixes: ["engine.dll"] }]` | Only `engine.dll` threads → CPUs 0–3. |
| `"*p@engine.dll;render.dll*e@audio.dll"` | Two rules: `engine.dll`/`render.dll` → `p`, `audio.dll` → `e`. | Multiple segments chained. |
| `"*p@*e"` | `[{ cpus: [0..3], prefixes: [] }, { cpus: [4..7], prefixes: [] }]` | No `@` in second segment — `e` is the alias name; no prefix filter. |
| `"*unknown"` | `[]` | Error pushed for undefined alias; empty CPUs cause segment to be skipped. |
| `"0-7"` | `[]` | Error pushed (does not start with `*`). |

### Interaction with rule parsing

`parse_ideal_processor_spec` is called from [parse_and_insert_rules](parse_and_insert_rules.md) when processing field 7 of a rule line. The returned vector is stored directly in [ProcessConfig](ProcessConfig.md)`.ideal_processor_rules`. If field 7 can be parsed as a plain integer (grade), it is not interpreted as an ideal processor spec — the function is only called when the field starts with `*` or cannot be parsed as a grade.

### Runtime application

At apply time, the [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) function iterates over the returned rules. For each rule, it walks the process's threads, checks each thread's start-address module name against `prefixes` (or matches all threads if `prefixes` is empty), and assigns ideal processors by round-robining through `cpus` using `SetThreadIdealProcessorEx`.

### Empty prefixes vs. no prefixes

An [IdealProcessorRule](IdealProcessorRule.md) with an empty `prefixes` vector means "match all threads." This is distinct from a rule that was skipped entirely (which produces no entry in the vector at all). The empty-prefixes case arises when a segment has no `@` delimiter — e.g., `*p` creates a rule that applies to every thread in the process.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Callees** | None (alias lookup via `HashMap::get`) |
| **API** | Pure function — no I/O, no Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Ideal processor rule struct | [IdealProcessorRule](IdealProcessorRule.md) |
| Rule field parsing (caller) | [parse_and_insert_rules](parse_and_insert_rules.md) |
| CPU alias definition | [parse_alias](parse_alias.md) |
| Alias-aware CPU spec resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Per-process configuration | [ProcessConfig](ProcessConfig.md) |
| Ideal processor application at runtime | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd