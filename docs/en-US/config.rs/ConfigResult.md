# ConfigResult type (config.rs)

`ConfigResult` is the aggregated output of the configuration parser. It contains all parsed process-level and thread-level rules organized by grade, tunable constants, alias/group/rule statistics, and any errors or warnings produced during parsing. It is the primary return type of [`read_config`](read_config.md) and serves as the central data structure that the main loop consults when applying rules to running processes.

## Syntax

```AffinityServiceRust/src/config.rs#L162-175
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
| `process_level_configs` | `HashMap<u32, HashMap<String, ProcessLevelConfig>>` | Process-level rules keyed by grade (polling frequency), then by lowercase process name. Each entry contains priority, affinity, CPU set, I/O priority, and memory priority settings. |
| `thread_level_configs` | `HashMap<u32, HashMap<String, ThreadLevelConfig>>` | Thread-level rules keyed by grade, then by lowercase process name. Each entry contains prime-thread CPUs, prefix filters, tracking counts, and ideal-processor rules. |
| `constants` | [`ConfigConstants`](ConfigConstants.md) | Tunable numeric constants (`MIN_ACTIVE_STREAK`, `KEEP_THRESHOLD`, `ENTRY_THRESHOLD`) that control the prime-thread scheduler's behavior. Populated from `@NAME = value` lines, or defaults if none are specified. |
| `constants_count` | `usize` | Number of `@CONSTANT` directives successfully parsed from the config file. |
| `aliases_count` | `usize` | Number of `*alias = cpu_spec` directives successfully parsed from the config file. |
| `groups_count` | `usize` | Number of `{ }` process group blocks encountered in the config file. |
| `group_members_count` | `usize` | Total number of individual process names collected from all group blocks. |
| `process_rules_count` | `usize` | Total number of process rules parsed (includes both individual rules and group-expanded members). |
| `redundant_rules_count` | `usize` | Count of rules that were defined more than once for the same process name. When a redundant rule is encountered, the earlier definition is overwritten and a warning is emitted. |
| `errors` | `Vec<String>` | List of fatal parsing errors. When this vector is non-empty, the configuration is considered invalid and should not be applied. |
| `warnings` | `Vec<String>` | List of non-fatal warnings (e.g., unknown priority strings, redundant rules, empty groups). The configuration is still usable when warnings are present. |
| `thread_level_configs_count` | `usize` | Total number of thread-level configuration entries inserted across all grades. |

## Methods

### `is_valid`

```AffinityServiceRust/src/config.rs#L178-180
pub fn is_valid(&self) -> bool {
    self.errors.is_empty()
}
```

Returns `true` if the configuration contains no errors and is safe to apply. Warnings alone do not cause `is_valid` to return `false`.

### `total_rules`

```AffinityServiceRust/src/config.rs#L182-186
pub fn total_rules(&self) -> usize {
    let a: usize = self.process_level_configs.values().map(|grade_configs| grade_configs.len()).sum();
    let b: usize = self.thread_level_configs.values().map(|grade_configs| grade_configs.len()).sum();
    a + b
}
```

Returns the total number of active rules across all grades by summing the lengths of all inner `HashMap` entries for both process-level and thread-level configs.

### `print_report`

```AffinityServiceRust/src/config.rs#L188-217
pub fn print_report(&self)
```

Prints a human-readable summary of the parse result to the log. When the configuration is valid, it reports the count of process groups and process rules. When errors exist, it prints each error and warning prefixed with `✗` or `⚠` respectively, followed by a count of errors. Redundant-rule warnings are also printed when `redundant_rules_count > 0`.

## Remarks

- **Grade-based organization**: Both `process_level_configs` and `thread_level_configs` use a `HashMap<u32, HashMap<String, ...>>` structure. The outer key is the *grade* — a positive integer (≥ 1) that determines how frequently a rule is applied. A grade of `1` means the rule runs every polling loop; a grade of `N` means it runs every N-th loop. This design allows the main loop to efficiently select only the rules that apply to the current iteration.

- **Deriving Default**: `ConfigResult` derives `Default`, which initializes both `HashMap` collections as empty, sets all counters to `0`, and creates empty `Vec`s for errors and warnings. The `constants` field receives the `ConfigConstants::default()` values (`min_active_streak = 2`, `keep_threshold = 0.69`, `entry_threshold = 0.42`).

- **Redundant rule handling**: When a process name appears in more than one rule, [`parse_and_insert_rules`](parse_and_insert_rules.md) overwrites the earlier entry and increments `redundant_rules_count`. A warning is also pushed to `warnings` identifying the line number and process name.

- **Hot-reload safety**: [`hotreload_config`](hotreload_config.md) parses the config file into a new `ConfigResult` and only replaces the active configuration if `is_valid()` returns `true`. If the reload fails validation, the previous `ConfigResult` is retained and an error is logged.

- **Split into process-level and thread-level**: A single config line may produce entries in both `process_level_configs` and `thread_level_configs`. The split occurs inside [`parse_and_insert_rules`](parse_and_insert_rules.md): process-level settings (priority, affinity, CPU set, I/O priority, memory priority) go into `process_level_configs`, while thread-level settings (prime CPUs, prefixes, tracking, ideal processor) go into `thread_level_configs`. A process with only thread-level settings (e.g., only ideal processor rules) will have no entry in `process_level_configs`, and vice versa.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Constructed by | [`read_config`](read_config.md) (via `ConfigResult::default()`) |
| Populated by | [`parse_and_insert_rules`](parse_and_insert_rules.md), [`parse_constant`](parse_constant.md), [`parse_alias`](parse_alias.md), [`collect_group_block`](collect_group_block.md) |
| Consumed by | `main.rs` main loop, [`hotreload_config`](hotreload_config.md), `apply.rs` |
| API | Internal |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| ProcessLevelConfig | [ProcessLevelConfig](ProcessLevelConfig.md) |
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| ConfigConstants | [ConfigConstants](ConfigConstants.md) |
| read_config | [read_config](read_config.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| hotreload_config | [hotreload_config](hotreload_config.md) |
| config module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*