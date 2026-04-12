# IdealProcessorRule struct (config.rs)

Rule for assigning ideal processors to threads based on module prefix matching. Each rule maps a set of CPU indices to an optional list of module prefixes that filter which threads receive the assignment.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
```

## Members

`cpus`

Type: `Vec<u32>`

A sorted list of CPU indices that threads matching this rule should be assigned to as their ideal processors. The scheduler distributes threads across these CPUs in a round-robin fashion.

`prefixes`

Type: `Vec<String>`

A list of module name prefixes used to filter which threads this rule applies to. Thread start addresses are resolved to their owning module, and only threads whose module name starts with one of these prefixes are assigned the ideal processors in `cpus`. When this vector is empty, the rule applies to **all** threads of the process.

## Remarks

`IdealProcessorRule` is the final, resolved form of ideal processor specifications after config parsing. It is consumed by [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) to set each thread's ideal processor via the Windows `SetThreadIdealProcessorEx` API.

### Rule application order

Rules are evaluated in the order they appear in the config. A thread is matched by the **first** rule whose prefix list contains a matching entry (or the first rule with an empty prefix list). This means more specific rules (with prefixes) should be listed before catch-all rules (without prefixes).

### Config syntax

Ideal processor rules are specified in the 7th field of a process rule line using the format:

`*alias[@prefix1;prefix2]*alias2[@prefix3]`

For example, `*p@engine.dll*e@helper.dll` creates two rules:
- Threads starting in `engine.dll` get CPUs from alias `*p`
- Threads starting in `helper.dll` get CPUs from alias `*e`

A rule without `@` prefixes (e.g., `*p`) applies to all threads as a catch-all.

### Relationship to other types

- [IdealProcessorPrefix](IdealProcessorPrefix.md) is an intermediate parsing helper; `IdealProcessorRule` is the resolved output stored in [ProcessConfig](ProcessConfig.md).
- Rules are parsed by [parse_ideal_processor_spec](parse_ideal_processor_spec.md) and stored in `ProcessConfig::ideal_processor_rules`.
- CPU alias references (e.g., `*p`) are resolved against aliases defined via [parse_alias](parse_alias.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Derives** | `Debug`, `Clone` |
| **Used by** | [ProcessConfig](ProcessConfig.md), [parse_ideal_processor_spec](parse_ideal_processor_spec.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |