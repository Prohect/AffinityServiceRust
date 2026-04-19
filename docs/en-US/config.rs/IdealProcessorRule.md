# IdealProcessorRule struct (config.rs)

Maps a set of CPU indices to a list of thread start-module prefixes, forming a single rule for ideal processor assignment. When the service iterates a process's threads, any thread whose start module matches one of the `prefixes` receives an ideal processor hint drawn from `cpus`. If `prefixes` is empty, the rule applies to all threads unconditionally.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: List<[u32; CONSUMER_CPUS]>,
    pub prefixes: Vec<String>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `cpus` | `List<[u32; CONSUMER_CPUS]>` | Sorted list of logical processor indices to use when assigning ideal processor hints. Resolved from a CPU alias at parse time. Must be non-empty for the rule to take effect. |
| `prefixes` | `Vec<String>` | Lowercase module-name prefixes used to filter threads. A thread matches if its start module begins with any entry. An empty vector means the rule matches every thread in the process. |

## Remarks

`IdealProcessorRule` is produced by [parse_ideal_processor_spec](parse_ideal_processor_spec.md) from the ideal processor field (field 7) of a config rule line. The spec format uses `*alias@prefix1;prefix2` segments, where each segment becomes one `IdealProcessorRule` instance. Multiple segments can be chained to assign different CPU sets to threads from different modules within the same process.

### Ideal processor vs. affinity

An ideal processor hint is a *soft preference* — the Windows scheduler favours the hinted core but may schedule the thread elsewhere under load. This contrasts with hard affinity, which restricts the thread to a strict set of CPUs. Ideal processor rules are therefore useful for guiding thread placement without introducing starvation risk.

### Thread matching

Thread-to-rule matching is performed by [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) during every polling iteration. For each thread, the service queries the thread's start address module via cached data from the Event Tracing for Windows (ETW) subsystem. The first rule whose `prefixes` list contains a matching prefix (or whose list is empty) determines the ideal CPU.

### CPU distribution

When multiple threads match the same rule, the service distributes ideal processor assignments round-robin across the `cpus` list to spread load evenly.

### Config syntax example

```
*pcore = 0-7
*ecore = 8-19
game.exe:normal:0:0:0:none:none:*pcore@engine.dll;render.dll*ecore@audio.dll
```

This produces two `IdealProcessorRule` entries:

1. `cpus: [0..7], prefixes: ["engine.dll", "render.dll"]`
2. `cpus: [8..19], prefixes: ["audio.dll"]`

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Produced by** | [parse_ideal_processor_spec](parse_ideal_processor_spec.md), [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Stored in** | [ThreadLevelConfig](ThreadLevelConfig.md)`.ideal_processor_rules` |
| **Consumed by** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Collection type** | [List](../collections.rs/README.md) (`SmallVec<[u32; CONSUMER_CPUS]>`) |
| **Privileges** | None (data struct) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Parent config struct | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Spec parser | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| Runtime application | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| CPU alias resolution | [parse_alias](parse_alias.md) |
| Prime thread prefixes (related concept) | [PrimePrefix](PrimePrefix.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*