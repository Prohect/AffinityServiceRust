# parse_ideal_processor_spec function (config.rs)

Parses an ideal-processor specification string into a list of [`IdealProcessorRule`](IdealProcessorRule.md) entries. Each rule maps a set of CPU indices (resolved from an alias) to optional module-name prefixes that filter which threads receive ideal-processor assignments. The specification supports chaining multiple segments to assign different CPU sets to threads based on their start module.

## Syntax

```AffinityServiceRust/src/config.rs#L323-384
fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `spec` | `&str` | The ideal-processor specification string. Must start with `*` to indicate alias-based rules. The special value `"0"` or an empty string means no ideal-processor assignment. Leading and trailing whitespace is trimmed. |
| `line_number` | `usize` | The 1-based line number in the configuration file. Used in error messages to help the user locate problems. |
| `cpu_aliases` | `&HashMap<String, List<[u32; CONSUMER_CPUS]>>` | The alias lookup table populated by earlier `*name = cpu_spec` lines. Keys are lowercase alias names without the leading `*`. |
| `errors` | `&mut Vec<String>` | A mutable reference to the error accumulator. Errors are pushed when the spec does not start with `*`, when an alias name is empty, or when a referenced alias is not defined. |

## Return value

Type: `Vec<IdealProcessorRule>`

A vector of [`IdealProcessorRule`](IdealProcessorRule.md) entries. Each rule contains:
- `cpus`: A list of CPU indices resolved from the alias.
- `prefixes`: A list of lowercase module-name prefixes that restrict which threads the rule applies to. An empty vector means the rule applies to all threads.

Returns an empty vector when:
- `spec` is empty or `"0"` (no ideal-processor assignment requested).
- `spec` does not start with `*` (an error is recorded).
- All segments resolve to empty CPU sets (segments with empty CPU sets are silently skipped).

## Remarks

### Specification format

The general format is one or more `*`-delimited segments:

```/dev/null/syntax.txt#L1-3
*alias[@prefix1;prefix2;...]
*alias1[@prefix1]*alias2[@prefix2;prefix3]
```

| Component | Required | Description |
|-----------|----------|-------------|
| `*` | Yes | Prefix marker that begins each rule segment. |
| `alias` | Yes | A CPU alias name (case-insensitive) that must be defined in the `[ALIAS]` section of the config file. |
| `@` | No | Separator between the alias name and the prefix filter list. |
| `prefix1;prefix2` | No | Semicolon-separated list of module-name prefixes. When present, only threads whose start module begins with one of these strings are eligible for ideal-processor assignment from this rule. |

### Parsing algorithm

1. The input is trimmed. If empty or `"0"`, an empty vector is returned immediately.
2. If the string does not start with `*`, an error is pushed and an empty vector is returned.
3. The string is split on `*` (the first empty element from the leading `*` is skipped).
4. For each non-empty segment:
   a. If the segment contains `@`, it is split into an alias part (before `@`) and a prefixes part (after `@`).
   b. If no `@` is present, the entire segment is the alias name and the prefix list is empty.
   c. The alias name is trimmed, lowercased, and looked up in `cpu_aliases`.
   d. If the alias is empty, an error is pushed and the segment is skipped.
   e. If the alias is not found in the map, an error is pushed and an empty CPU list is used.
   f. If the resolved CPU list is empty, the segment is silently skipped (no rule is created).
   g. The prefixes string is split on `;`, trimmed, lowercased, and filtered for non-empty entries.
   h. An `IdealProcessorRule { cpus, prefixes }` is created and pushed to the result vector.

### Multi-segment chaining

Multiple segments can be chained to assign different CPU sets to different groups of threads. For example:

```/dev/null/example.ini#L1
*p@engine.dll;render.dll*e@helper.dll
```

This produces two rules:
1. Threads whose start module begins with `engine.dll` or `render.dll` → CPUs from alias `p`.
2. Threads whose start module begins with `helper.dll` → CPUs from alias `e`.

### Catch-all rules

A segment without an `@` prefix filter creates a catch-all rule that applies to all threads:

```/dev/null/example.ini#L1
*pN01
```

This produces one rule with an empty `prefixes` vector, meaning all threads in the process are eligible.

### Edge cases

| Input | Result | Notes |
|-------|--------|-------|
| `""` or `"0"` | `[]` | No ideal-processor assignment. |
| `"7"` (no leading `*`) | `[]` + error | Spec must start with `*`. |
| `"*undefined_alias"` | `[]` | Error pushed; alias not found, empty CPU set, segment skipped. |
| `"**"` | `[]` | Both segments have empty alias names; errors pushed, segments skipped. |
| `"*p@"` | Rule with `cpus` from `p`, empty `prefixes` | The `@` is present but no prefixes follow; the prefix filter is effectively empty (catch-all). |

### Visibility

This function is module-private (`fn`, not `pub fn`) and is only called from within `config.rs`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (crate-internal) |
| Callers | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| Callees | `HashMap::get`, `str::split`, `str::find`, `str::trim`, `str::to_lowercase` |
| Dependencies | [`IdealProcessorRule`](IdealProcessorRule.md), `List` and `CONSUMER_CPUS` from [`collections.rs`](../collections.rs/README.md) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| IdealProcessorRule | [IdealProcessorRule](IdealProcessorRule.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| config module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*