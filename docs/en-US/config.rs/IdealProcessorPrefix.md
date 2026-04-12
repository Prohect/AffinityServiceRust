# IdealProcessorPrefix struct (config.rs)

Internal helper struct used during parsing of ideal processor specifications. Associates a module name prefix string with a specific set of CPU indices.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorPrefix {
    pub prefix: String,
    pub cpus: Vec<u32>,
}
```

## Members

`prefix`

The module name prefix string used to match against thread start address module names. For example, `"engine.dll"` would match threads whose start address resolves to that module. Case-insensitive matching is applied during comparison.

`cpus`

A vector of CPU indices that threads matching this prefix should have their ideal processor set to. Unlike [PrimePrefix](PrimePrefix.md) where `cpus` is optional, this field is always required since the purpose of the struct is to bind a prefix to specific CPUs.

## Remarks

`IdealProcessorPrefix` is an intermediate representation used internally by [parse_ideal_processor_spec](parse_ideal_processor_spec.md) during the parsing phase. The parser converts the raw specification string into `IdealProcessorPrefix` instances, which are then consolidated into [IdealProcessorRule](IdealProcessorRule.md) entries that are stored in [ProcessConfig](ProcessConfig.md).

In the final [IdealProcessorRule](IdealProcessorRule.md) struct, the `cpus` and `prefixes` fields are separated — multiple prefixes may share the same CPU set. `IdealProcessorPrefix` keeps them paired during parsing before this consolidation occurs.

This struct is marked `#[allow(dead_code)]` in the source because its usage is confined to internal parsing logic and it does not appear in the public-facing configuration output.

### Relationship to IdealProcessorRule

| Struct | Purpose |
| --- | --- |
| **IdealProcessorPrefix** | Intermediate parse-time pairing of one prefix to one CPU set |
| [IdealProcessorRule](IdealProcessorRule.md) | Final representation grouping multiple prefixes under one CPU set |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Source lines** | L24–L27 |
| **Used by** | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |

## See also

- [IdealProcessorRule](IdealProcessorRule.md) — final rule struct stored in process configs
- [ProcessConfig](ProcessConfig.md) — contains `ideal_processor_rules` field
- [parse_ideal_processor_spec](parse_ideal_processor_spec.md) — parser that produces these structs