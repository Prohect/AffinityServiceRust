# parse_and_insert_rules function (config.rs)

Parses the colon-separated rule fields from a configuration line and inserts a fully constructed [ProcessConfig](ProcessConfig.md) entry into the [ConfigResult](ConfigResult.md) rule map for every member in the provided list. This is the central rule-construction function: it validates each field, resolves CPU aliases, parses prime-thread prefix specifications, and handles the grade-based partitioning of rules.

## Syntax

```rust
fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `members` | `&[String]` | Slice of lowercased process names (or group member names) that this rule applies to. Each member receives its own clone of the parsed [ProcessConfig](ProcessConfig.md). For a single-process rule line, this is a one-element slice. For a group, it contains all expanded member names from the `{ ... }` block. |
| `rule_parts` | `&[&str]` | Slice of colon-separated field strings extracted from the rule portion of the config line. Index mapping: `[0]` = priority, `[1]` = affinity, `[2]` = cpuset, `[3]` = prime_cpus, `[4]` = io_priority, `[5]` = memory_priority, `[6]` = ideal_processor or grade, `[7]` = grade (when field 6 is ideal_processor). At least 2 elements are required; remaining fields default to `None`/empty/`0`/`1` when omitted. |
| `line_number` | `usize` | 1-based line number in the configuration file where the rule was defined. Used for all error and warning messages produced during parsing. |
| `cpu_aliases` | `&HashMap<String, Vec<u32>>` | Map of defined CPU aliases (keyed by lowercased alias name) built by earlier [parse_alias](parse_alias.md) calls. Used by [resolve_cpu_spec](resolve_cpu_spec.md) and the inline prime/ideal-processor alias resolution logic. |
| `result` | `&mut ConfigResult` | Mutable reference to the running parse-result accumulator. On success, [ProcessConfig](ProcessConfig.md) entries are inserted into `result.configs`. Errors and warnings are pushed to `result.errors` and `result.warnings`. The `process_rules_count` and `redundant_rules_count` counters are updated. |

## Return value

This function does not return a value. All output is communicated through mutations to `result`.

## Remarks

### Rule field format

The full rule format is:

```text
priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade
```

Fields are positional and colon-separated. The first two fields (priority and affinity) are required; all subsequent fields are optional.

### Field parsing details

| Index | Field | Parser | Default | Notes |
|-------|-------|--------|---------|-------|
| 0 | `priority` | `ProcessPriority::from_str` | — | **Required.** Unrecognized values produce a warning and default to `ProcessPriority::None`. |
| 1 | `affinity` | [resolve_cpu_spec](resolve_cpu_spec.md) | — | **Required.** Supports `*alias` references and all formats accepted by [parse_cpu_spec](parse_cpu_spec.md). |
| 2 | `cpuset` | [resolve_cpu_spec](resolve_cpu_spec.md) | `[]` (empty) | A leading `@` enables `cpu_set_reset_ideal`, which distributes thread ideal processors across the CPU set after applying it. The `@` is stripped before spec resolution. |
| 3 | `prime_cpus` | Complex (see below) | `[]` / wildcard prefix | Supports tracking prefixes (`?N`, `??N`), alias-qualified module-name prefix specs (`*alias@prefix!priority`), and plain CPU specs. |
| 4 | `io_priority` | `IOPriority::from_str` | `IOPriority::None` | Unrecognized values produce a warning. |
| 5 | `memory_priority` | `MemoryPriority::from_str` | `MemoryPriority::None` | Unrecognized values produce a warning. |
| 6 | `ideal_processor` or `grade` | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) or `u32::parse` | `[]` / `1` | If the value starts with `*` or equals `"0"`, it is parsed as an ideal processor spec and grade is read from field 7. If it parses as a `u32`, it is treated as a bare grade with no ideal processor rules. Otherwise, it is attempted as an ideal processor spec with grade defaulting to `1`. |
| 7 | `grade` | `u32::parse` | `1` | Only present when field 6 is an ideal processor spec. Grade `0` is corrected to `1` with a warning. Non-numeric values produce a warning and default to `1`. |

### Prime-threads field (field 3) sub-syntax

The prime-threads field supports a rich sub-syntax for controlling which threads are tracked and how they are pinned:

**Tracking prefix:**

| Prefix | Meaning |
|--------|---------|
| `?N` | Track the top *N* threads by cycle delta **and** apply prime scheduling. `track_top_x_threads` is set to positive *N*. |
| `??N` | Track the top *N* threads **without** applying prime scheduling (observation-only). `track_top_x_threads` is set to negative *N*. |

The tracking prefix is consumed first. An optional `x` or `X` separator may follow the number before the CPU spec (e.g., `?8x*p@engine.dll`).

**Module-prefix–qualified specs:**

When the field contains `@`, it is parsed as one or more `*alias@prefix1;prefix2!priority` segments:

- `*alias` references a CPU alias for the segment's CPU set.
- `@prefix1;prefix2` provides semicolon-separated module-name prefix filters.
- `!priority` optionally sets a per-prefix thread priority (e.g., `!highest`, `!above normal`).

Each segment produces one or more [PrimePrefix](PrimePrefix.md) entries with `cpus: Some(...)` pointing to the segment's resolved CPU set. The `prime_threads_cpus` field is the union of all segments' CPU sets.

**Plain CPU spec:**

When no `@` is present, the field is resolved via [resolve_cpu_spec](resolve_cpu_spec.md) and a single wildcard [PrimePrefix](PrimePrefix.md) with an empty prefix string and `cpus: None` is created.

**Value `"0"`:**

Produces empty prime CPU list and empty prefix list, effectively disabling prime-thread scheduling for this rule.

### Ideal processor / grade disambiguation (field 6)

Field 6 is ambiguous because it may contain either an ideal processor specification or a bare grade integer. The parser uses the following heuristic:

1. If the value starts with `*` or equals `"0"` → parse as ideal processor spec, read grade from field 7.
2. Else if the value parses as `u32` → treat it as a grade (no ideal processor rules).
3. Otherwise → attempt to parse as an ideal processor spec with grade defaulting to `1`.

Grade `0` is invalid and is corrected to `1` with a warning in all code paths.

### Redundant rule detection

Before inserting each member, the function checks whether the process name already exists in any grade map within `result.configs`. If a duplicate is found:

- `result.redundant_rules_count` is incremented.
- A warning is pushed: `"Line {N}: Redundant rule - '{name}' already defined (previous definition will be overwritten)"`.
- The previous entry is overwritten by the new insertion.

### Grade-based insertion

Each [ProcessConfig](ProcessConfig.md) is inserted into `result.configs.entry(grade).or_default()`, a two-level `HashMap<u32, HashMap<String, ProcessConfig>>`. The grade (default `1`) determines which sub-map the entry is placed into. The service main loop iterates grades to control rule evaluation priority.

### Error conditions

| Condition | Severity | Message pattern |
|-----------|----------|-----------------|
| Fewer than 2 `rule_parts` elements | Error | `"Line {N}: Too few fields ({count}) - expected at least 2 (priority,affinity)"` |
| Unrecognized priority string | Warning | `"Line {N}: Unknown priority '{val}' - will be treated as 'none'"` |
| Undefined `*alias` in affinity/cpuset/prime field | Error | (from [resolve_cpu_spec](resolve_cpu_spec.md)) |
| Unknown CPU alias in prime `*alias@` segment | Error | `"Line {N}: Unknown CPU alias '*{alias}' in prime specification"` |
| Unrecognized thread priority in `!priority` suffix | Warning | `"Line {N}: Unknown thread priority '{val}' in prefix - will be treated as 'none' (auto-boost)"` |
| Unrecognized IO priority | Warning | `"Line {N}: Unknown IO priority '{val}' - will be treated as 'none'"` |
| Unrecognized memory priority | Warning | `"Line {N}: Unknown memory priority '{val}' - will be treated as 'none'"` |
| Grade value of `0` | Warning | `"Line {N}: Grade cannot be 0, using 1 instead"` |
| Invalid grade string | Warning | `"Line {N}: Invalid grade '{val}', using 1"` |
| Redundant process name | Warning | `"Line {N}: Redundant rule - '{name}' already defined (previous definition will be overwritten)"` |

### Example rule lines

```text
# Simple: set game.exe to High priority on CPUs 0-7
game.exe:high:0-7

# Full rule with all fields
game.exe:high:*perf:@*perf:?8x*p@engine.dll;render.dll!above normal*e@audio.dll:normal:normal:*p@engine.dll:2

# Group with shared rule
{ game.exe: helper.exe: launcher.exe }:above normal:*perf:0:0:none:none
```

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [read_config](read_config.md) |
| **Callees** | [resolve_cpu_spec](resolve_cpu_spec.md), [parse_ideal_processor_spec](parse_ideal_processor_spec.md), `ProcessPriority::from_str`, `IOPriority::from_str`, `MemoryPriority::from_str`, `ThreadPriority::from_str` |
| **Produces** | [ProcessConfig](ProcessConfig.md) entries (inserted into [ConfigResult](ConfigResult.md)`.configs`) |
| **API** | Pure function — no I/O, no Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Per-process configuration record | [ProcessConfig](ProcessConfig.md) |
| Prime-thread prefix filter | [PrimePrefix](PrimePrefix.md) |
| Ideal processor rule | [IdealProcessorRule](IdealProcessorRule.md) |
| Parsed config aggregate | [ConfigResult](ConfigResult.md) |
| Alias-aware CPU spec resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Ideal processor spec parser | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| Main config file reader (caller) | [read_config](read_config.md) |
| CPU alias definition parser | [parse_alias](parse_alias.md) |
| Process priority enum | [ProcessPriority](../priority.rs/ProcessPriority.md) |
| I/O priority enum | [IOPriority](../priority.rs/IOPriority.md) |
| Memory priority enum | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| Thread priority enum | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd