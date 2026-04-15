# IdealProcessorRule type (config.rs)

The `IdealProcessorRule` struct maps a set of CPU indices to an optional list of thread start-module name prefixes for ideal-processor assignment. When the scheduler assigns ideal processors to a process's threads, it uses these rules to determine which CPUs should be preferred and, optionally, restricts the assignment to threads whose start module matches one of the specified prefixes.

## Syntax

```AffinityServiceRust/src/config.rs#L24-27
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: List<[u32; CONSUMER_CPUS]>,
    pub prefixes: Vec<String>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `cpus` | `List<[u32; CONSUMER_CPUS]>` | A sorted list of CPU indices that the scheduler should assign as ideal processors to qualifying threads. The number of CPUs in the list determines how many top threads (ranked by total CPU time) receive an ideal-processor assignment from this rule. |
| `prefixes` | `Vec<String>` | A list of lowercase module-name prefixes used to filter which threads this rule applies to. When empty, the rule applies to **all** threads of the process. When non-empty, only threads whose start module name begins with one of these prefixes are considered. |

## Remarks

### Relationship to the ideal-processor specification

`IdealProcessorRule` instances are produced by [`parse_ideal_processor_spec`](parse_ideal_processor_spec.md) when the configuration file contains an ideal-processor field of the form `*alias[@prefix1;prefix2]`. A single specification string can produce multiple rules when multiple `*`-delimited segments are present (e.g., `*p@engine.dll*e@helper.dll` produces two rules).

### Thread selection algorithm

The scheduler uses the `cpus` list to determine the number of top threads (N = `cpus.len()`) that should receive ideal-processor assignments from this rule. Threads are ranked by their cumulative CPU time. When a thread drops out of the top N, it falls back to its previous ideal processor value.

### Prefix matching

Prefix strings are stored in lowercase and compared against the lowercase form of the thread's start module name. This allows matching partial module names — for example, a prefix of `engine` matches `engine.dll`, `engine_worker.dll`, etc.

### Edge cases

- If `cpus` is empty (e.g., because the referenced alias resolved to an empty CPU set), the rule is **skipped** during parsing and will not appear in the final `Vec<IdealProcessorRule>`.
- If `prefixes` is empty, the rule is a catch-all that applies to every thread in the process.
- Multiple rules can target the same thread if their prefix filters overlap; the scheduler processes rules in order.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Constructed by | [`parse_ideal_processor_spec`](parse_ideal_processor_spec.md) |
| Stored in | [`ThreadLevelConfig.ideal_processor_rules`](ThreadLevelConfig.md) |
| Consumed by | `scheduler.rs` (prime-thread scheduler ideal-processor assignment) |
| Dependencies | `List` from `collections.rs`, `CONSUMER_CPUS` constant |

## See Also

| Resource | Link |
|----------|------|
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| parse_ideal_processor_spec | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| PrimePrefix | [PrimePrefix](PrimePrefix.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| scheduler module | [scheduler.rs overview](../scheduler.rs/README.md) |
| config module overview | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*