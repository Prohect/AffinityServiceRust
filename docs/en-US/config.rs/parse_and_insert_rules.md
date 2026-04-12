# parse_and_insert_rules function (config.rs)

Parses rule fields from a split config line and inserts [ProcessConfig](ProcessConfig.md) entries for all group members into the [ConfigResult](ConfigResult.md). This is the central rule-construction function called by [read_config](read_config.md) for both individual process lines and group blocks.

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

`members`

A slice of process name strings (lowercased) that this rule applies to. For a single process line, this contains one element. For a group block, it contains all members collected by [collect_members](collect_members.md) / [collect_group_block](collect_group_block.md).

`rule_parts`

A slice of field strings split by `:` from the rule portion of the config line. Expected field order:

| Index | Field | Type | Default |
| --- | --- | --- | --- |
| 0 | priority | `ProcessPriority` | `None` |
| 1 | affinity | CPU spec or alias | empty |
| 2 | cpuset | CPU spec or alias (prefix `@` for reset ideal) | empty |
| 3 | prime specification | tracking + CPU spec + prefixes | empty |
| 4 | io_priority | `IOPriority` | `None` |
| 5 | memory_priority | `MemoryPriority` | `None` |
| 6 | ideal processor / grade | ideal spec or grade number | empty / 1 |
| 7 | grade | `u32` (≥1) | 1 |

At least 2 fields (priority and affinity) are required; additional fields are optional.

`line_number`

The 1-based line number in the config file, used for error and warning messages.

`cpu_aliases`

A map of alias names to CPU index vectors, populated by [parse_alias](parse_alias.md) during config parsing. Aliases are referenced in CPU spec fields with the `*` prefix.

`result`

The mutable [ConfigResult](ConfigResult.md) accumulator. Parsed configs are inserted into `result.configs`, keyed by grade then by process name. Errors, warnings, and counters are updated in place.

## Return value

This function does not return a value. Results are written into the `result` parameter.

## Remarks

### Field parsing details

**Priority (field 0):** Parsed via `ProcessPriority::from_str`. Unknown values generate a warning and default to `None`.

**Affinity (field 1):** Resolved through [resolve_cpu_spec](resolve_cpu_spec.md), supporting both literal CPU specifications and `*alias` references.

**CPU Set (field 2):** Also resolved through [resolve_cpu_spec](resolve_cpu_spec.md). If the field value is prefixed with `@`, the `cpu_set_reset_ideal` flag is set to `true`, which causes thread ideal processors to be redistributed across the CPU set after application.

**Prime specification (field 3):** The most complex field, supporting several sub-formats:

- `0` — No prime thread scheduling.
- `?Nx*alias` — Track top N threads with prime scheduling to the given alias CPUs. The `x` separator is required between the count and the alias.
- `??Nx*alias` — Track top N threads without prime scheduling (negative `track_top_x_threads`).
- `*alias@module1;module2!priority` — Module-filtered prime scheduling. Each `*alias@prefixes` segment defines a [PrimePrefix](PrimePrefix.md) rule. The `!priority` suffix on a prefix sets a thread priority boost.
- `*alias` — Simple prime scheduling to alias CPUs for all threads.

**IO Priority (field 4):** Parsed via `IOPriority::from_str`. Unknown values generate a warning.

**Memory Priority (field 5):** Parsed via `MemoryPriority::from_str`. Unknown values generate a warning.

**Ideal processor / Grade (fields 6–7):** Field 6 is overloaded — if it starts with `*` or equals `0`, it is parsed as an ideal processor specification via [parse_ideal_processor_spec](parse_ideal_processor_spec.md), with field 7 used as the grade. If field 6 is a plain number, it is treated as the grade directly. Grade must be ≥1; a value of 0 is corrected to 1 with a warning.

### Redundant rule detection

If a process name already exists in any grade within `result.configs`, a warning is emitted and `redundant_rules_count` is incremented. The new rule overwrites the previous definition.

### Config insertion

For each member, a [ProcessConfig](ProcessConfig.md) struct is constructed with all parsed fields (cloning shared vectors) and inserted into `result.configs` under the appropriate grade key. The `process_rules_count` is incremented by the number of members.

### Example config lines

Single process:
```
chrome.exe:high:*p:0:0:none:none:*e:1
```

Group with prime thread scheduling:
```
games { game1.exe: game2.exe }:high:*p:@*c:?8x*p@engine.dll;render.dll:none:none:*e
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | Private (`fn`) |
| **Called by** | [read_config](read_config.md) |
| **Calls** | [resolve_cpu_spec](resolve_cpu_spec.md), [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| **Related types** | [ProcessConfig](ProcessConfig.md), [PrimePrefix](PrimePrefix.md), [ConfigResult](ConfigResult.md), [IdealProcessorRule](IdealProcessorRule.md) |