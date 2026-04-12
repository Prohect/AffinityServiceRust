# PrimePrefix struct (config.rs)

Module-specific prefix rule for prime thread scheduling. Associates a thread module name prefix with optional dedicated CPU cores and an optional thread priority override.

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

`prefix`

A string used to match against a thread's start module name. When empty, the rule applies to all threads regardless of their start module. When non-empty, only threads whose resolved start address module name contains this prefix will be matched by this rule.

`cpus`

An optional list of CPU indices that this prefix's matched threads should be scheduled on. When `Some`, these CPUs override the process-level `prime_threads_cpus` for matching threads. When `None`, the process-level prime CPUs are used instead.

`thread_priority`

A [ThreadPriority](../priority.rs/ThreadPriority.md) value to apply to threads matching this prefix when they are promoted by the prime thread scheduler. When set to `ThreadPriority::None`, no priority boost is applied (auto-boost behavior).

## Remarks

`PrimePrefix` is used within [ProcessConfig](ProcessConfig.md)`::`[prime_threads_prefixes](ProcessConfig.md) to enable fine-grained control over which threads receive prime scheduling treatment and where they are placed.

### Parsing format

Prime prefix rules are specified in the prime field of a config rule using the syntax:

`*alias@module1[!priority];module2[!priority]`

Where:
- `*alias` references a CPU alias defining the target CPUs
- `@module1` is the module prefix filter
- `!priority` is an optional thread priority suffix (e.g., `!highest`, `!above normal`)
- Multiple prefixes are separated by `;`

Multiple alias-prefix groups can be chained:

`*p@engine.dll!highest;render.dll*e@helper.dll`

### Examples

A rule like `*p@engine.dll!highest;audio.dll` produces two `PrimePrefix` entries:
1. `PrimePrefix { prefix: "engine.dll", cpus: Some([0,1,2,3]), thread_priority: Highest }`
2. `PrimePrefix { prefix: "audio.dll", cpus: Some([0,1,2,3]), thread_priority: None }`

When `prefix` is empty (no `@` filtering), a single catch-all entry is created:

`PrimePrefix { prefix: "", cpus: None, thread_priority: None }`

This default entry matches all threads in the process.

### Thread matching

During [apply_prime_threads](../apply.rs/apply_prime_threads.md), each candidate thread's start address is resolved to a module name via `resolve_address_to_module`. The scheduler then checks each `PrimePrefix` in order to find a matching prefix. The first match determines the CPU set and priority for that thread.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Used by** | [ProcessConfig](ProcessConfig.md)::`prime_threads_prefixes` |
| **Parsed by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Consumed by** | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |