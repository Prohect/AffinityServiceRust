# ConfigResult struct (config.rs)

Result container returned by [read_config](read_config.md) that holds all parsed configuration data, statistics counters, and any errors or warnings encountered during parsing.

## Syntax

```rust
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

## Fields

`configs`

A two-level map of parsed process configurations. The outer key is the grade level (`u32`, starting at 1), and the inner key is the lowercased process name. Each value is a [ProcessConfig](ProcessConfig.md) instance. Grade levels allow tiered rule application — grade 1 rules are applied first, higher grades are applied later for secondary matching.

`constants`

A [ConfigConstants](ConfigConstants.md) instance holding scheduler tuning parameters parsed from `@CONSTANT = value` lines in the config file. Uses default values if no constants are specified.

`constants_count`

Number of constants successfully parsed from `@CONSTANT` directives.

`aliases_count`

Number of CPU aliases successfully parsed from `*alias = cpu_spec` directives.

`groups_count`

Number of process groups (curly-brace blocks) parsed from the config file.

`group_members_count`

Total number of individual process names found across all groups.

`process_rules_count`

Total number of process rules inserted into `configs`, including both individual rules and expanded group members.

`redundant_rules_count`

Number of rules that overwrote a previously defined rule for the same process name. Each redundant rule also generates a warning.

`errors`

List of error messages encountered during parsing. If this list is non-empty, the configuration is considered invalid and should not be used for process management.

`warnings`

List of warning messages for non-fatal issues such as unknown priority names, empty groups, or redundant rules.

## Remarks

`ConfigResult` implements `Default`, initializing `configs` as an empty map, `constants` with [ConfigConstants](ConfigConstants.md) defaults, all counters to zero, and both `errors` and `warnings` as empty vectors.

The struct provides three convenience methods:

- **`is_valid()`** — Returns `true` if `errors` is empty, indicating the config can be safely used.
- **`total_rules()`** — Returns the sum of all rules across all grade levels.
- **`print_report()`** — Logs a summary of parsing results. On success, prints group and rule counts. On failure, prints all errors and warnings.

The `configs` map is keyed by grade to support tiered rule matching. A typical configuration uses grade 1 (the default). The grade field is the last optional field in a rule line, parsed by [parse_and_insert_rules](parse_and_insert_rules.md).

### Validation flow

```
read_config() → ConfigResult
    if result.is_valid()
        result.print_report()   // logs success summary
        // proceed with configs
    else
        result.print_report()   // logs errors
        // abort
```

### Error vs. Warning examples

| Condition | Severity |
| --- | --- |
| Cannot open config file | Error |
| Undefined CPU alias reference | Error |
| Too few fields in a rule line | Error |
| Unclosed group block (missing `}`) | Error |
| Unknown priority string | Warning |
| Empty group with no members | Warning |
| Redundant/duplicate process rule | Warning |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Derives** | `Debug`, `Default` |
| **Used by** | [read_config](read_config.md), [parse_constant](parse_constant.md), [parse_alias](parse_alias.md), [parse_and_insert_rules](parse_and_insert_rules.md), [main](../main.md) |

## See also

- [read_config](read_config.md)
- [ProcessConfig](ProcessConfig.md)
- [ConfigConstants](ConfigConstants.md)
- [parse_and_insert_rules](parse_and_insert_rules.md)