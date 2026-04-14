# IdealProcessorRule struct (config.rs)

Defines a mapping from a set of CPU indices to optional module-name prefix filters for ideal processor assignment. When the service applies ideal processor rules to a process's threads, each thread's start-address module name is checked against the `prefixes` list. If the list is empty (no filter), all threads are eligible; otherwise, only threads whose start module matches one of the prefixes are assigned ideal processors from the `cpus` set.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `cpus` | `Vec<u32>` | Sorted list of logical CPU indices that threads matching this rule should be distributed across as ideal processors. These indices are resolved from a CPU alias at parse time via [parse_ideal_processor_spec](parse_ideal_processor_spec.md). The apply module round-robins thread ideal processor assignments across this set. |
| `prefixes` | `Vec<String>` | Module-name prefix filters. Each entry is a lowercased string matched against the resolved module name of a thread's start address (e.g., `"engine.dll"`, `"ntdll"`). An empty vector means the rule applies to all threads unconditionally. When non-empty, only threads whose start-address module begins with one of these prefixes are assigned ideal processors from `cpus`. |

## Remarks

### Config syntax

Ideal processor rules appear in field 7 (the `ideal_processor` position) of a rule line. The specification format is:

```
*alias[@prefix1;prefix2]*alias2[@prefix3]
```

Where:

- Each segment begins with `*` followed by a CPU alias name (must be previously defined with `*alias = cpu_spec`).
- The optional `@` suffix provides semicolon-separated module-name prefixes that filter which threads receive ideal processor assignment from this segment's CPU set.
- Multiple segments can be chained to create multiple `IdealProcessorRule` entries for a single process.

**Examples:**

| Spec string | Result |
|-------------|--------|
| `*perf` | One rule: all threads get ideal processors from alias `perf`'s CPUs. |
| `*perf@engine.dll` | One rule: only threads starting in `engine.dll` get ideal processors from `perf`. |
| `*p@engine.dll;render.dll*e@audio.dll` | Two rules: `engine.dll`/`render.dll` threads → alias `p` CPUs; `audio.dll` threads → alias `e` CPUs. |
| `0` | No rules (ideal processor assignment disabled). |

### Distribution behavior

At runtime, the apply module iterates over a process's threads and, for each `IdealProcessorRule`, assigns ideal processors by cycling through `cpus` in order. This distributes thread scheduling hints across the specified cores. The assignment uses `SetThreadIdealProcessorEx` and only applies to threads that match the prefix filter.

### Empty cpus

If the referenced alias resolves to an empty CPU set, the rule is silently skipped during parsing and no `IdealProcessorRule` entry is produced for that segment.

### Interaction with CPU sets

When a process also has a `cpu_set_cpus` configured with the `@` prefix (i.e., `cpu_set_reset_ideal` is true), `reset_thread_ideal_processors` is called first, distributing ideal processors across the CPU set. Ideal processor rules from this struct are applied separately and may override those assignments for specific threads.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Constructed by** | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| **Consumed by** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Parent struct** | [ProcessConfig](ProcessConfig.md) (field `ideal_processor_rules`) |

## See Also

| Topic | Link |
|-------|------|
| Per-process configuration record | [ProcessConfig](ProcessConfig.md) |
| Ideal processor spec parser | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| Ideal processor application logic | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| CPU alias parsing | [parse_alias](parse_alias.md) |
| CPU alias resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Module overview | [config module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd