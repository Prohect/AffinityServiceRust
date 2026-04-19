# PrimePrefix struct (config.rs)

Associates a module-name prefix with a CPU set and optional thread priority boost for prime thread matching. When the prime thread scheduler identifies a thread as "prime" (high-activity), it checks the thread's start module against stored `PrimePrefix` entries to determine which CPUs the thread should be pinned to and whether its priority should be elevated.

## Syntax

```rust
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
| `prefix` | `String` | Module name prefix to match against a thread's start module (e.g., `"engine.dll"`). An empty string matches all threads regardless of their start module. Comparison is case-insensitive. |
| `cpus` | `Option<List<[u32; CONSUMER_CPUS]>>` | CPU indices to pin matching prime threads to. When `Some`, overrides the parent [ThreadLevelConfig](ThreadLevelConfig.md)'s `prime_threads_cpus` for threads matching this prefix. When `None`, the parent's `prime_threads_cpus` is used as fallback. |
| `thread_priority` | [ThreadPriority](../priority.rs/ThreadPriority.md) | Optional priority boost applied to the thread when it is promoted to prime status. `ThreadPriority::None` means no priority change (auto-boost behavior). Specified in config with the `!priority` suffix syntax (e.g., `engine.dll!above normal`). |

## Remarks

### Config syntax

`PrimePrefix` entries are parsed from the prime field (field 4) of a rule line. The format supports per-prefix CPU overrides and priority boosts:

```
process.exe:normal:0:0:*pcore@engine.dll;helper.dll!above normal:none:none:0:1
```

In this example:
- `*pcore` references a CPU alias defining which CPUs to assign.
- `@engine.dll;helper.dll!above normal` defines two prefixes: `engine.dll` (no priority boost) and `helper.dll` (boosted to above normal).

### Matching behavior

- When `prefix` is empty (`""`), the entry acts as a catch-all that matches any thread regardless of its start module.
- Multiple `PrimePrefix` entries can exist for a single process rule. The scheduler evaluates them in order, and a thread matches the first prefix whose string is a prefix of the thread's start module name.
- The `cpus` field allows directing threads from different modules to different CPU cores within the same process rule. For example, render threads can go to P-cores while audio threads go to E-cores.

### Default construction

When no `@prefix` syntax is used in the prime field, a single `PrimePrefix` is created with an empty `prefix`, `cpus` set to `None`, and `thread_priority` set to `ThreadPriority::None`. This means all prime threads for that process use the parent config's CPU set with no priority boost.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Consumers** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) |
| **Dependencies** | [ThreadPriority](../priority.rs/ThreadPriority.md), [List](../collections.rs/README.md) |
| **Privileges** | None (data structure only) |

## See Also

| Topic | Link |
|-------|------|
| Parent config struct | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Prime thread promotion | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| Thread priority enum | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Rule parser | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Module overview | [config.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*