# parse_and_insert_rules function (config.rs)

The primary rule field parser and insertion function. Takes an array of process names (either a single name or members of a group block) and a colon-split array of rule fields, validates each field, constructs [ProcessLevelConfig](ProcessLevelConfig.md) and/or [ThreadLevelConfig](ThreadLevelConfig.md) instances, and inserts them into the appropriate grade-keyed maps in [ConfigResult](ConfigResult.md).

## Syntax

```rust
fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## Parameters

`members: &[String]`

A slice of lowercase process names to which the parsed rule should be applied. For a single-line rule, this contains one element (e.g., `["game.exe"]`). For a group rule, this contains all member names extracted from the `{ }` block.

`rule_parts: &[&str]`

A slice of field strings split from the rule portion of a config line. Expected field positions:

| Index | Field | Example | Description |
|-------|-------|---------|-------------|
| 0 | priority | `"normal"` | Process priority class |
| 1 | affinity | `"*pcore"` or `"0-7"` | CPU affinity specification |
| 2 | cpuset | `"@*ecore"` or `"0"` | CPU set specification; `@` prefix enables ideal processor reset |
| 3 | prime | `"?8x*p@engine.dll"` | Prime thread spec (CPUs, prefixes, tracking count) |
| 4 | io_priority | `"low"` | IO priority level |
| 5 | memory_priority | `"none"` | Memory priority level |
| 6 | ideal / grade | `"*p@render.dll"` or `"2"` | Ideal processor spec, or grade if numeric |
| 7 | grade | `"1"` | Application frequency tier (only if field 6 is an ideal spec) |

A minimum of 2 fields (priority and affinity) is required; all subsequent fields are optional with "no change" defaults.

`line_number: usize`

The 1-based line number in the configuration file where this rule originates. Used in all error and warning messages for user diagnostics.

`cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>`

Map of defined CPU aliases, populated by earlier [parse_alias](parse_alias.md) calls. Passed through to [resolve_cpu_spec](resolve_cpu_spec.md) for alias-aware CPU field resolution.

`result: &mut ConfigResult`

**\[in, out\]** The accumulated parsing result. This function reads and writes `errors`, `warnings`, `process_level_configs`, `thread_level_configs`, `process_rules_count`, `redundant_rules_count`, and `thread_level_configs_count`.

## Return value

This function does not return a value. All outputs are communicated through mutations to `result`.

## Remarks

### Field parsing pipeline

Each field is parsed independently in order:

1. **Priority (field 0):** Parsed via `ProcessPriority::from_str`. Unknown values produce a warning and default to `ProcessPriority::None`.

2. **Affinity (field 1):** Resolved via [resolve_cpu_spec](resolve_cpu_spec.md), which handles both `*alias` references and literal CPU specs.

3. **CPU set (field 2):** If the field starts with `@`, the `@` is stripped and `cpu_set_reset_ideal` is set to `true`; the remainder is resolved as a CPU spec. This flag causes thread ideal processors to be redistributed after the CPU set is applied.

4. **Prime spec (field 3):** The most complex field. Supports several sub-formats:
   - `"0"` — No prime thread scheduling.
   - `"*alias"` — Pin prime threads to the alias's CPUs, matching all threads.
   - `"?Nx*alias"` — Track top N threads by CPU cycles and prime them to alias CPUs. The `?` prefix with a number sets positive `track_top_x_threads`.
   - `"??N"` — Track top N threads without prime pinning. The `??` prefix sets negative `track_top_x_threads` (tracking-only mode).
   - `"*alias@prefix1;prefix2!priority"` — Per-prefix CPU routing with optional thread priority boost. Each `*alias@` segment produces [PrimePrefix](PrimePrefix.md) entries. The `!` separator within a prefix sets a [ThreadPriority](../priority.rs/ThreadPriority.md) override.

5. **IO priority (field 4):** Parsed via `IOPriority::from_str`. Unknown values produce a warning.

6. **Memory priority (field 5):** Parsed via `MemoryPriority::from_str`. Unknown values produce a warning.

7. **Ideal processor / grade (field 6):** Ambiguous field — if it starts with `*` or is `"0"`, it is parsed as an ideal processor spec via [parse_ideal_processor_spec](parse_ideal_processor_spec.md). If it parses as a plain integer, it is treated as the grade. Otherwise, it is attempted as an ideal processor spec with grade defaulting to 1.

8. **Grade (field 7):** If field 6 was an ideal processor spec and field 7 exists, it is parsed as the grade. Grade must be ≥ 1; a value of 0 produces a warning and defaults to 1.

### Config insertion logic

For each member name, the function performs two independent validity checks:

- **Process-level valid:** At least one of `priority`, `affinity_cpus`, `cpu_set_cpus`, `io_priority`, or `memory_priority` is non-default. If valid, a [ProcessLevelConfig](ProcessLevelConfig.md) is created and inserted into `result.process_level_configs` under the appropriate grade.

- **Thread-level valid:** At least one of `prime_threads_cpus` (non-empty), `track_top_x_threads` (non-zero), or `ideal_processor_rules` (non-empty) is active. If valid, a [ThreadLevelConfig](ThreadLevelConfig.md) is created and inserted into `result.thread_level_configs`.

If neither check passes, a warning is emitted indicating no valid rules exist for that process.

### Redundancy detection

Before insertion, the function checks all existing grade maps for an entry with the same process name. If found, `redundant_rules_count` is incremented and a warning is emitted. The new entry overwrites the old one — last definition wins.

### Prime prefix construction

When the prime spec contains `@` segments, the parser builds a `Vec<PrimePrefix>` with per-segment CPU lists and optional thread priority overrides. The `prime_threads_cpus` field is set to the union of all segment CPU sets. When no `@` segments are present, a single default [PrimePrefix](PrimePrefix.md) is created with an empty prefix (matching all threads), `None` cpus (inheriting from `prime_threads_cpus`), and `ThreadPriority::None`.

### Default PrimePrefix for no-prefix specs

Even when the prime spec is a simple `*alias` without any `@` prefix filter, the function still creates a `Vec<PrimePrefix>` containing one default entry. This ensures the downstream [apply_prime_threads](../apply.rs/apply_prime_threads.md) function always has at least one prefix entry to iterate over.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Private (`fn`) |
| **Callers** | [read_config](read_config.md) (for both single-line rules and group rules) |
| **Callees** | [resolve_cpu_spec](resolve_cpu_spec.md), [parse_ideal_processor_spec](parse_ideal_processor_spec.md), `ProcessPriority::from_str`, `IOPriority::from_str`, `MemoryPriority::from_str`, `ThreadPriority::from_str` |
| **Writes to** | [ConfigResult](ConfigResult.md) (`.process_level_configs`, `.thread_level_configs`, `.errors`, `.warnings`, counters) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Process-level config struct | [ProcessLevelConfig](ProcessLevelConfig.md) |
| Thread-level config struct | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Prime prefix struct | [PrimePrefix](PrimePrefix.md) |
| Ideal processor rule struct | [IdealProcessorRule](IdealProcessorRule.md) |
| CPU alias resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Ideal processor spec parser | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| Config file reader | [read_config](read_config.md) |
| Config result container | [ConfigResult](ConfigResult.md) |
| Priority enums | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*