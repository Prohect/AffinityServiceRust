# ConfigResult struct (config.rs)

Aggregated output of the configuration parser. Contains all parsed process-level and thread-level rules organized by grade (application frequency), along with parsing statistics, errors, and warnings. This is the primary return type of [read_config](read_config.md) and the struct consumed by the main service loop and [hotreload_config](hotreload_config.md).

## Syntax

```rust
#[derive(Debug, Default)]
pub struct ConfigResult {
    pub process_level_configs: HashMap<u32, HashMap<String, ProcessLevelConfig>>,
    pub thread_level_configs: HashMap<u32, HashMap<String, ThreadLevelConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub thread_level_configs_count: usize,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `process_level_configs` | `HashMap<u32, HashMap<String, ProcessLevelConfig>>` | Process-level rules keyed by grade (outer) and lowercased process name (inner). Grade `1` is the default and means the rule is applied on every polling iteration; higher grades reduce application frequency. |
| `thread_level_configs` | `HashMap<u32, HashMap<String, ThreadLevelConfig>>` | Thread-level rules keyed by grade (outer) and lowercased process name (inner). These rules are evaluated every polling iteration for thread-level scheduling (prime threads, ideal processors). |
| `constants` | [ConfigConstants](ConfigConstants.md) | Tuning constants for prime thread hysteresis. Populated from `@CONSTANT = value` lines in the config file; uses `Default` values when not specified. |
| `constants_count` | `usize` | Number of `@CONSTANT` directives successfully parsed. |
| `aliases_count` | `usize` | Number of `*alias = cpu_spec` alias definitions successfully parsed. |
| `groups_count` | `usize` | Number of `{ }` group blocks successfully parsed. |
| `group_members_count` | `usize` | Total number of process names contained across all parsed group blocks. |
| `process_rules_count` | `usize` | Total number of process names for which rules were attempted (includes both singles and group members). |
| `redundant_rules_count` | `usize` | Number of rules that overwrote a previously defined rule for the same process name. Each redundant rule also generates a warning. |
| `errors` | `Vec<String>` | Fatal parsing errors. Any non-empty error list causes [is_valid](#is_valid) to return `false` and prevents the configuration from being applied. Messages include line numbers. |
| `warnings` | `Vec<String>` | Non-fatal parsing warnings (e.g., unknown priority names treated as `none`, redundant rules, empty groups). The configuration is still usable when warnings are present. |
| `thread_level_configs_count` | `usize` | Running count of thread-level config entries inserted. Incremented per process name that produces a valid thread-level rule. |

## Methods

### is_valid

```rust
pub fn is_valid(&self) -> bool
```

Returns `true` if the `errors` vector is empty, indicating the configuration was parsed without fatal errors and is safe to apply.

**Return value**

`bool` — `true` when no parsing errors were recorded; `false` otherwise.

### total_rules

```rust
pub fn total_rules(&self) -> usize
```

Returns the total number of active rules across all grades, combining both process-level and thread-level configurations. This counts entries in the inner `HashMap`s, not the raw `process_rules_count` counter.

**Return value**

`usize` — Sum of all process-level config entries and all thread-level config entries across every grade.

### print_report

```rust
pub fn print_report(&self)
```

Logs a human-readable summary of the parsing result. Behavior depends on validity:

- **Valid configuration:** Logs group statistics (if any groups exist), total process rules count, and any warnings prefixed with `⚠`.
- **Invalid configuration:** Logs all errors prefixed with `✗`, all warnings prefixed with `⚠`, and a final summary of the error count.
- **Redundant rules present:** Also prints warnings even for valid configurations to alert the user about duplicate definitions.

## Remarks

### Two-tier HashMap structure

The outer `HashMap<u32, ...>` key is the **grade**, an unsigned integer ≥ 1 that controls how often rules are applied. Grade 1 rules run every polling cycle; grade *N* rules run every *N*th cycle. This allows users to configure less frequent enforcement for processes that don't need constant monitoring. The inner `HashMap<String, ...>` maps lowercased process names to their respective configurations.

### Process-level vs. thread-level separation

A single config line can produce both a [ProcessLevelConfig](ProcessLevelConfig.md) and a [ThreadLevelConfig](ThreadLevelConfig.md). The parser in [parse_and_insert_rules](parse_and_insert_rules.md) determines validity independently: a process-level entry is created only if at least one of priority, affinity, CPU set, IO priority, or memory priority is non-default; a thread-level entry is created only if prime CPUs, tracking, or ideal processor rules are specified. A warning is emitted if neither is valid.

### Redundancy detection

When a rule is inserted for a process name that already exists in any grade, the old entry is overwritten and `redundant_rules_count` is incremented. This is a warning, not an error — the last definition wins.

### Default derivation

The struct derives `Default`, which initializes all `HashMap`s and `Vec`s to empty, all counters to zero, and `constants` to `ConfigConstants::default()`. This is used by [read_config](read_config.md) to create the initial accumulator before parsing begins.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Callers** | [read_config](read_config.md), [hotreload_config](hotreload_config.md), main polling loop, [parse_and_insert_rules](parse_and_insert_rules.md), [parse_constant](parse_constant.md), [parse_alias](parse_alias.md) |
| **Dependencies** | [ProcessLevelConfig](ProcessLevelConfig.md), [ThreadLevelConfig](ThreadLevelConfig.md), [ConfigConstants](ConfigConstants.md), [HashMap](../collections.rs/README.md) |
| **Privileges** | None (data structure only) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Process-level rule struct | [ProcessLevelConfig](ProcessLevelConfig.md) |
| Thread-level rule struct | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Tuning constants | [ConfigConstants](ConfigConstants.md) |
| Config file parser | [read_config](read_config.md) |
| Rule insertion logic | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Hot-reload with validation | [hotreload_config](hotreload_config.md) |
| Apply engine | [apply.rs](../apply.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*