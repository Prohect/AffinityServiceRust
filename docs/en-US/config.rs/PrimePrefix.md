# PrimePrefix struct (config.rs)

Represents a module-name prefix filter combined with an optional CPU set override and thread priority boost for prime-thread scheduling. When the prime-thread scheduler identifies a hot thread, it checks the thread's start-address module name against the `prefix` field. If the prefix matches (or is empty, meaning "match all"), the thread is pinned to the CPUs specified in `cpus` and optionally boosted to `thread_priority`.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `prefix` | `String` | Module-name prefix used to filter threads by their start address. An empty string matches all threads. The match is case-insensitive and compared against the resolved module name of the thread's start address (e.g., `"engine.dll"`). |
| `cpus` | `Option<Vec<u32>>` | Optional per-prefix CPU set override. When `Some`, the matched thread is pinned to these specific CPU indices instead of the parent rule's `prime_threads_cpus`. When `None`, the parent rule's CPU list is used. |
| `thread_priority` | `ThreadPriority` | Thread priority to apply to matched prime threads. `ThreadPriority::None` means no priority change (the OS auto-boost behavior is preserved). Non-`None` values (e.g., `AboveNormal`, `Highest`) explicitly set the thread's scheduling priority via `SetThreadPriority`. |

## Remarks

`PrimePrefix` entries are constructed during rule parsing in [parse_and_insert_rules](parse_and_insert_rules.md) when the prime-threads field contains `@`-delimited prefix specifications.

### Config syntax

The prime-threads field (field 4 in a rule line) supports prefix-qualified specifications of the form:

```
*alias@prefix1;prefix2!priority*alias2@prefix3
```

Where:

- `*alias` references a CPU alias defined with `*alias = cpu_spec`.
- `@prefix1;prefix2` lists semicolon-separated module-name prefixes.
- `!priority` optionally appends a thread priority to a specific prefix (e.g., `engine.dll!highest`).

For example, the spec `*p@engine.dll;render.dll!above normal*e@audio.dll` produces three `PrimePrefix` entries:

1. `{ prefix: "engine.dll", cpus: Some(<p cpus>), thread_priority: None }`
2. `{ prefix: "render.dll", cpus: Some(<p cpus>), thread_priority: AboveNormal }`
3. `{ prefix: "audio.dll", cpus: Some(<e cpus>), thread_priority: None }`

When no `@` is present in the prime-threads field, a single `PrimePrefix` with an empty prefix and `cpus: None` is created, meaning all threads are eligible and the parent rule's CPU list is used.

### Thread matching

At runtime, the apply module resolves each thread's start address to a module name and compares it against the `prefix` field using a case-insensitive prefix match. An empty `prefix` acts as a wildcard.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Constructed by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Consumed by** | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **Parent struct** | [ProcessConfig](ProcessConfig.md) (field `prime_threads_prefixes`) |

## See Also

| Topic | Link |
|-------|------|
| Per-process configuration record | [ProcessConfig](ProcessConfig.md) |
| Rule parsing and prefix extraction | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Prime-thread application logic | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| Thread priority enum | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| CPU alias parsing | [parse_alias](parse_alias.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd