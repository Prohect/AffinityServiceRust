# ConfigResult struct (config.rs)

Aggregates the complete output of a configuration file parse pass. `ConfigResult` holds the parsed process rule map (keyed by grade then by process name), the resolved tuning constants, statistical counters for reporting, and any errors or warnings encountered during parsing. It is the return type of [read_config](read_config.md) and provides helper methods to check validity, count total rules, and print a human-readable report.

## Syntax

```rust
#[derive(Debug, Default)]
pub struct ConfigResult {
    pub configs: HashMap<u32, HashMap<String, ProcessConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `configs` | `HashMap<u32, HashMap<String, ProcessConfig>>` | Two-level map of parsed process rules. The outer key is the **grade** (a positive `u32`, default `1`), and the inner key is the lowercased process name. Grade ordering allows the service loop to apply higher-grade rules before lower-grade ones, controlling evaluation priority when multiple rules could match. |
| `constants` | `ConfigConstants` | Hysteresis tuning constants parsed from `@CONSTANT = value` lines. Initialized to [ConfigConstants::default()](ConfigConstants.md) and overwritten as constants are encountered in the file. |
| `constants_count` | `usize` | Number of `@CONSTANT` lines successfully parsed. |
| `aliases_count` | `usize` | Number of `*alias = cpu_spec` lines successfully parsed. |
| `groups_count` | `usize` | Number of `{ ... }` process group blocks parsed. |
| `group_members_count` | `usize` | Total number of individual process names expanded from all group blocks. |
| `process_rules_count` | `usize` | Total number of [ProcessConfig](ProcessConfig.md) entries inserted (individual rules plus group-expanded members). |
| `redundant_rules_count` | `usize` | Number of rules that overwrote a previously defined entry for the same process name. Each redundant rule generates a warning. |
| `errors` | `Vec<String>` | Fatal parse errors. When non-empty, the configuration is considered invalid and must not be applied. Each string includes the line number and a human-readable description. |
| `warnings` | `Vec<String>` | Non-fatal parse warnings (e.g., unknown priority strings defaulting to `none`, redundant rules, unknown constants). Warnings do not prevent the configuration from being applied. |

## Methods

### `is_valid`

```rust
pub fn is_valid(&self) -> bool
```

Returns `true` if `errors` is empty, indicating the parsed configuration can be safely applied.

### `total_rules`

```rust
pub fn total_rules(&self) -> usize
```

Returns the sum of all [ProcessConfig](ProcessConfig.md) entries across all grades. This counts the actual map entries rather than the `process_rules_count` counter, so it reflects the final de-duplicated count after any overwrites.

### `print_report`

```rust
pub fn print_report(&self)
```

Logs a summary of the parse results. On success, it logs group and rule counts. On failure, it logs all errors and warnings, followed by a final count of errors with an instruction to fix them. Redundant-rule warnings are always printed when `redundant_rules_count > 0`.

## Remarks

`ConfigResult` implements the `Default` trait. The default value has an empty `configs` map, default [ConfigConstants](ConfigConstants.md), all counters at zero, and empty error/warning vectors. This default is used as the starting state in [read_config](read_config.md) and is also returned early when the config file cannot be opened (with an error pushed into `errors`).

### Grade system

The `configs` map uses a grade as its outer key. Grades are parsed from the optional eighth field in a rule line. Grade `1` is the default. The service main loop iterates grades in order so that higher-grade rules are evaluated first. This allows "priority lanes" — for example, game processes at grade `2` are applied before background utilities at grade `1`.

### Hot-reload behavior

During [hotreload_config](hotreload_config.md), a new `ConfigResult` is parsed from the modified file. If `is_valid()` returns `true`, the new `configs` map and `constants` replace the live state. If errors are present, the previous configuration is kept and the errors are logged. This ensures the service never operates with an invalid rule set.

### Error format

All error and warning strings follow the pattern `"Line {N}: {description}"`, where `N` is the 1-based line number in the config file. This makes it straightforward for users to locate and fix problems in their configuration.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Returned by** | [read_config](read_config.md) |
| **Consumed by** | [hotreload_config](hotreload_config.md), [main](../main.rs/main.md) |
| **Key dependencies** | [ProcessConfig](ProcessConfig.md), [ConfigConstants](ConfigConstants.md) |

## See Also

| Topic | Link |
|-------|------|
| Main config parser | [read_config](read_config.md) |
| Per-process rule record | [ProcessConfig](ProcessConfig.md) |
| Hysteresis tuning constants | [ConfigConstants](ConfigConstants.md) |
| Hot-reload of config file | [hotreload_config](hotreload_config.md) |
| Rule field parsing | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd