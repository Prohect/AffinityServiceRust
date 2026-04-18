# parse_and_insert_rules function (config.rs)

Parses the rule fields from a configuration line and inserts both process-level and thread-level config entries into the [`ConfigResult`](ConfigResult.md) for every member in the provided list. This is the central rule-interpretation function that converts raw colon-separated field strings into fully resolved [`ProcessLevelConfig`](ProcessLevelConfig.md) and [`ThreadLevelConfig`](ThreadLevelConfig.md) instances.

## Syntax

```AffinityServiceRust/src/config.rs#L424-741
fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `members` | `&[String]` | A slice of lowercase process names that this rule applies to. For a single-process line this contains one element; for a `{ }` group block it contains all collected member names. |
| `rule_parts` | `&[&str]` | The colon-split fields **after** the process name (or after the closing `}:` of a group). Expected field order is: `priority`, `affinity`, `cpuset`, `prime_cpus`, `io_priority`, `memory_priority`, `ideal_processor`, `grade`. At least 2 fields are required. |
| `line_number` | `usize` | The 1-based line number in the configuration file, used for error and warning messages. |
| `cpu_aliases` | `&HashMap<String, List<[u32; CONSUMER_CPUS]>>` | The alias lookup table populated by `*name = cpu_spec` lines that appeared earlier in the config file. |
| `result` | `&mut ConfigResult` | The mutable configuration result accumulator. Parsed entries are inserted into `result.process_level_configs` and/or `result.thread_level_configs`, and any errors or warnings are appended to the corresponding vectors. |

## Return value

This function does not return a value. All output is accumulated in the `result` parameter.

## Remarks

### Field parsing order

The function interprets `rule_parts` positionally. Fields beyond the minimum of 2 are optional; missing fields receive sensible defaults.

| Index | Field | Default | Description |
|-------|-------|---------|-------------|
| 0 | `priority` | *(required)* | Process priority class string (e.g., `"normal"`, `"high"`, `"none"`). Resolved via `ProcessPriority::from_str`. Unknown values are treated as `None` with a warning. |
| 1 | `affinity` | *(required)* | CPU affinity specification. Resolved via [`resolve_cpu_spec`](resolve_cpu_spec.md); supports alias references (`*pcore`) and literal specs (`0-7`). |
| 2 | `cpuset` | `List::new()` (empty) | CPU set specification. If prefixed with `@`, the `cpu_set_reset_ideal` flag is set to `true` and the `@` is stripped before resolution. |
| 3 | `prime_cpus` | Empty CPU list, default prefix | Prime-thread CPU specification with optional module-prefix filtering and tracking directives. See **Prime-thread parsing** below. |
| 4 | `io_priority` | `IOPriority::None` | I/O priority string (e.g., `"low"`, `"normal"`, `"high"`). Unknown values produce a warning and default to `None`. |
| 5 | `memory_priority` | `MemoryPriority::None` | Memory priority string (e.g., `"very low"`, `"normal"`). Unknown values produce a warning and default to `None`. |
| 6 | `ideal_processor` or `grade` | `Vec::new()` / `1` | This field is polymorphic. If it starts with `*` or equals `"0"`, it is parsed as an ideal-processor specification via [`parse_ideal_processor_spec`](parse_ideal_processor_spec.md); grade is then read from index 7. If it parses as an integer, it is interpreted as the grade directly. |
| 7 | `grade` | `1` | Rule application frequency. `1` = every loop, `N` = every Nth loop. Values of `0` are corrected to `1` with a warning. Only used when field 6 is an ideal-processor spec. |

### Prime-thread parsing (field 3)

The prime-thread field supports a rich mini-syntax:

- **`"0"`** — No prime-thread scheduling.
- **`?N`** prefix — Track top N threads **with** prime scheduling (positive `track_top_x_threads`).
- **`??N`** prefix — Track top N threads **without** prime scheduling (negative `track_top_x_threads`).
- **`*alias@prefix1;prefix2`** segments — Per-alias CPU sets with module-prefix filters. Multiple `*alias@...` segments can be chained. Each segment produces one or more [`PrimePrefix`](PrimePrefix.md) entries.
- **`prefix!priority`** suffix — Optional thread priority override for a specific prefix, parsed via `ThreadPriority::from_str`.
- **Plain `*alias`** — A single alias without prefix filtering; all threads are eligible.

When `@` is present, the parser splits on `*` to extract segments, resolves each alias from `cpu_aliases`, and builds a `Vec<PrimePrefix>`. A `!` within a prefix string separates the module name from an optional thread priority override.

### Split into process-level and thread-level

After parsing all fields, the function checks which settings are non-default:

- **Process-level valid**: `priority != None`, `affinity_cpus` non-empty, `cpu_set_cpus` non-empty, `io_priority != None`, or `memory_priority != None`.
- **Thread-level valid**: `prime_threads_cpus` non-empty, `track_top_x_threads != 0`, or `ideal_processor_rules` non-empty.

For each member in `members`:

1. If process-level settings are valid, a [`ProcessLevelConfig`](ProcessLevelConfig.md) entry is inserted into `result.process_level_configs` under the corresponding grade key.
2. If thread-level settings are valid, a [`ThreadLevelConfig`](ThreadLevelConfig.md) entry is inserted into `result.thread_level_configs` under the corresponding grade key and `thread_level_configs_count` is incremented.
3. If neither is valid, a warning is emitted noting that the process has no effective rules.

### Redundant rule detection

Before inserting, the function checks whether the process name already exists in any grade bucket of `process_level_configs` or `thread_level_configs`. If so, it increments `result.redundant_rules_count` and pushes a warning indicating that the previous definition will be overwritten.

### Error conditions

| Condition | Severity | Message pattern |
|-----------|----------|-----------------|
| Fewer than 2 rule fields | Error | `"Line {n}: Too few fields ({count}) - expected at least 2"` |
| Unknown priority string | Warning | `"Line {n}: Unknown priority '...' - will be treated as 'none'"` |
| Undefined CPU alias | Error | `"Line {n}: Undefined alias '*...' in {field} field"` |
| Unknown I/O priority | Warning | `"Line {n}: Unknown IO priority '...' - will be treated as 'none'"` |
| Unknown memory priority | Warning | `"Line {n}: Unknown memory priority '...' - will be treated as 'none'"` |
| Grade value of 0 | Warning | `"Line {n}: Grade cannot be 0, using 1 instead"` |
| Invalid grade string | Warning | `"Line {n}: Invalid grade '...', using 1"` |
| Redundant process-level rule | Warning | `"Line {n}: Redundant process level rule - '...' already defined"` |
| Redundant thread-level rule | Warning | `"Line {n}: Redundant thread level rule - '...' already defined"` |
| All fields are none/0 | Warning | `"No valid rules(all none/0) for process '...'"` |

### Side effects

- Modifies `result.process_level_configs` and `result.thread_level_configs` by inserting new entries.
- Modifies `result.errors` and `result.warnings` by appending diagnostic messages.
- Increments `result.process_rules_count` by `members.len()`.
- Increments `result.redundant_rules_count` for each duplicate detected.
- Increments `result.thread_level_configs_count` for each thread-level entry created.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (`fn`, not `pub fn`) |
| Callers | [`read_config`](read_config.md), [`sort_and_group_config`](sort_and_group_config.md) (indirectly via `read_config`) |
| Callees | [`resolve_cpu_spec`](resolve_cpu_spec.md), [`parse_ideal_processor_spec`](parse_ideal_processor_spec.md), `ProcessPriority::from_str`, `IOPriority::from_str`, `MemoryPriority::from_str`, `ThreadPriority::from_str` |
| Dependencies | [`ProcessLevelConfig`](ProcessLevelConfig.md), [`ThreadLevelConfig`](ThreadLevelConfig.md), [`PrimePrefix`](PrimePrefix.md), [`IdealProcessorRule`](IdealProcessorRule.md), [`ConfigResult`](ConfigResult.md) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| read_config | [read_config](read_config.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_ideal_processor_spec | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| collect_group_block | [collect_group_block](collect_group_block.md) |
| ProcessLevelConfig | [ProcessLevelConfig](ProcessLevelConfig.md) |
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| PrimePrefix | [PrimePrefix](PrimePrefix.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| config module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*