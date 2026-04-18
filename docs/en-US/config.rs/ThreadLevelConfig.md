# ThreadLevelConfig type (config.rs)

The `ThreadLevelConfig` struct holds all thread-level scheduling settings for a single process rule. It defines the prime-thread CPU set, module-prefix filters with optional per-prefix CPU overrides and thread priorities, the number of top threads to track, and ideal-processor assignment rules. Thread-level configs are separated from process-level configs so the scheduler can independently manage per-thread CPU placement on each polling iteration.

## Syntax

```AffinityServiceRust/src/config.rs#L40-46
#[derive(Debug, Clone)]
pub struct ThreadLevelConfig {
    pub name: String,
    pub prime_threads_cpus: List<[u32; CONSUMER_CPUS]>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `name` | `String` | The lowercase executable name this rule applies to (e.g., `"game.exe"`). Matches the corresponding [ProcessLevelConfig](ProcessLevelConfig.md) entry when both process-level and thread-level rules exist for the same process. |
| `prime_threads_cpus` | `List<[u32; CONSUMER_CPUS]>` | The combined set of CPU indices available for prime-thread scheduling. When a process has CPU-intensive threads, the scheduler assigns their affinity to these CPUs. This is the union of all per-segment CPU aliases specified in the `prime_cpus` config field. An empty list means no prime-thread scheduling is active (unless `track_top_x_threads` is negative for tracking-only mode). |
| `prime_threads_prefixes` | `Vec<PrimePrefix>` | A list of [PrimePrefix](PrimePrefix.md) entries that filter which threads are eligible for prime scheduling based on their start module name. Each prefix can optionally override the CPU set and thread priority for matching threads. When the list contains a single entry with an empty prefix string, all threads are eligible. |
| `track_top_x_threads` | `i32` | Controls how many top CPU-consuming threads are tracked and optionally scheduled. **Positive values** (`?N`): track the top N threads and apply prime-thread scheduling to them. **Negative values** (`??N`): track the top N threads for monitoring purposes only, without applying prime scheduling. **Zero**: tracking count is derived from the number of CPUs in `prime_threads_cpus`. |
| `ideal_processor_rules` | `Vec<IdealProcessorRule>` | A list of [IdealProcessorRule](IdealProcessorRule.md) entries that assign ideal processors to threads based on their start module name. The scheduler uses these rules to set the preferred CPU for each qualifying thread, distributing the top N threads (where N = number of CPUs in the rule) across the specified CPUs by total CPU time ranking. |

## Remarks

### Relationship to ProcessLevelConfig

A single config line can produce both a [ProcessLevelConfig](ProcessLevelConfig.md) and a `ThreadLevelConfig` for the same process name. The process-level entry is created when any of the process-wide settings (priority, affinity, CPU set, I/O priority, memory priority) are non-default. The thread-level entry is created when any of the thread-specific settings (prime CPUs, tracking count, ideal processor rules) are non-default. The two configs are stored in separate `HashMap` collections keyed by grade within [ConfigResult](ConfigResult.md).

### Grade-based scheduling

Thread-level configs are organized by grade (polling frequency). A rule with grade `1` runs every iteration; grade `2` runs every other iteration, and so on. This allows expensive thread-level analysis to be performed less frequently than process-level settings.

### Prime-thread scheduling flow

1. The scheduler identifies the process by matching `name` against running process executables.
2. It enumerates the process's threads and filters them through `prime_threads_prefixes` by matching each thread's start module name.
3. Threads are ranked by total CPU time, and the top N are selected (where N is determined by `track_top_x_threads` or the CPU count).
4. Each selected thread's affinity is set to the appropriate CPU from `prime_threads_cpus` (or the per-prefix CPU override in the matching `PrimePrefix`).
5. If `ideal_processor_rules` are defined, ideal processor assignment is applied separately using a similar ranking mechanism filtered by the rule's prefixes.

### track_top_x_threads sign convention

| Value | Syntax in config | Behavior |
|-------|-----------------|----------|
| Positive (`> 0`) | `?Nx*alias` | Track top N threads **and** apply prime scheduling |
| Negative (`< 0`) | `??N` | Track top N threads for monitoring **only** (no prime scheduling) |
| Zero (`0`) | (default) | Derive tracking count from `prime_threads_cpus.len()` |

### Validity check

A `ThreadLevelConfig` is only inserted into the result when at least one of the following conditions is true:
- `prime_threads_cpus` is non-empty
- `track_top_x_threads` is non-zero
- `ideal_processor_rules` is non-empty

If none of these conditions hold, no thread-level entry is created, even if the config line was otherwise valid.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Constructed by | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Stored in | [ConfigResult](ConfigResult.md)`.thread_level_configs` |
| Consumed by | `scheduler.rs` (prime-thread scheduler), `apply.rs` |
| Related types | [PrimePrefix](PrimePrefix.md), [IdealProcessorRule](IdealProcessorRule.md) |

## See Also

| Resource | Link |
|----------|------|
| PrimePrefix | [PrimePrefix](PrimePrefix.md) |
| IdealProcessorRule | [IdealProcessorRule](IdealProcessorRule.md) |
| ProcessLevelConfig | [ProcessLevelConfig](ProcessLevelConfig.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| scheduler module | [scheduler.rs overview](../scheduler.rs/README.md) |
| config module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*