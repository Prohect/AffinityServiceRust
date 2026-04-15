# PrimePrefix type (config.rs)

The `PrimePrefix` struct associates a thread start-module prefix string with an optional set of CPU indices and a thread priority override. It is used within [`ThreadLevelConfig`](ThreadLevelConfig.md) to control which CPUs are assigned to prime threads whose start module matches the given prefix, and to optionally boost or lower those threads' scheduling priority.

## Syntax

```AffinityServiceRust/src/config.rs#L17-21
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<List<[u32; CONSUMER_CPUS]>>,
    pub thread_priority: ThreadPriority,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `prefix` | `String` | A case-insensitive thread start-module name prefix to match against (e.g., `"engine.dll"`, `"render"`). An empty string matches all threads regardless of their start module. |
| `cpus` | `Option<List<[u32; CONSUMER_CPUS]>>` | An optional list of CPU indices that threads matching this prefix should be scheduled on. When `Some`, overrides the parent rule's base `prime_threads_cpus`. When `None`, the thread inherits the base CPU set from the containing [`ThreadLevelConfig`](ThreadLevelConfig.md). |
| `thread_priority` | `ThreadPriority` | An optional thread-level priority to apply to matching threads. `ThreadPriority::None` means no priority modification (auto-boost behavior is preserved). Set via the `!priority` suffix in the config syntax (e.g., `engine.dll!above normal`). |

## Remarks

### Configuration syntax

In the configuration file, `PrimePrefix` values are specified within the prime-threads field (field 4) of a rule line. The general syntax is:

```/dev/null/example.ini#L1-3
process.exe:normal:0:0:*alias@prefix1;prefix2!priority:none:none:0:1
```

The `@prefix` portion is split on `;` to produce one `PrimePrefix` per entry. The `!priority` suffix is optional and parsed via `ThreadPriority::from_str`.

### Multi-segment prefix rules

When multiple `*alias@prefix` segments are chained (e.g., `*p@engine.dll*e@helper.dll`), each segment produces its own set of `PrimePrefix` instances with the corresponding alias CPUs stored in `cpus`. This allows different groups of threads to be scheduled on different CPU sets based on their start module.

### Default behavior

When a rule specifies prime CPUs without any `@prefix` filter, a single `PrimePrefix` is created with an empty `prefix` (matches all threads), `cpus` set to `None` (inherits from parent), and `thread_priority` set to `ThreadPriority::None` (no override).

### Lifetime

`PrimePrefix` instances are created during configuration parsing by [`parse_and_insert_rules`](parse_and_insert_rules.md) and stored inside [`ThreadLevelConfig`](ThreadLevelConfig.md). They are consumed at runtime by the [`PrimeThreadScheduler`](../scheduler.rs/README.md) when deciding which threads qualify as "prime" and how to schedule them.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Constructed by | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| Contained in | [`ThreadLevelConfig`](ThreadLevelConfig.md) (field `prime_threads_prefixes`) |
| Consumed by | `PrimeThreadScheduler` (see [scheduler.rs](../scheduler.rs/README.md)) |
| Derives | `Debug`, `Clone` |

## See Also

| Resource | Link |
|----------|------|
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| IdealProcessorRule | [IdealProcessorRule](IdealProcessorRule.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| priority module | [priority.rs overview](../priority.rs/README.md) |
| config module overview | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*