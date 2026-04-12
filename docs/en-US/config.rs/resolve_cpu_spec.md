# resolve_cpu_spec function (config.rs)

Resolves a CPU specification string that may contain a CPU alias reference, returning the corresponding list of CPU indices.

## Syntax

```rust
fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<u32>
```

## Parameters

`spec`

The CPU specification string to resolve. If it starts with `*`, it is treated as a CPU alias reference (e.g., `*p`). Otherwise, it is forwarded to [parse_cpu_spec](parse_cpu_spec.md) for direct parsing.

`field_name`

The name of the config field being parsed (e.g., `"affinity"`, `"cpuset"`, `"prime_cpus"`). Used in error messages to help the user locate the problem.

`line_number`

The 1-based line number in the config file where this specification appears. Included in error messages for diagnostics.

`cpu_aliases`

A map of alias names (without the `*` prefix) to their resolved CPU index vectors. Built from `*name = cpu_spec` lines parsed earlier in the config file by [parse_alias](parse_alias.md).

`errors`

A mutable reference to the error list from [ConfigResult](ConfigResult.md). If an alias is referenced but not defined, an error message is pushed here.

## Return value

Returns a `Vec<u32>` containing sorted CPU indices. If the spec is an alias reference, returns the alias's CPU list. If the alias is undefined, returns an empty vector and records an error. If the spec is not an alias, delegates to [parse_cpu_spec](parse_cpu_spec.md).

## Remarks

This function is the primary entry point for resolving CPU specifications within config rule fields. It bridges the alias system with the raw CPU spec parser, providing a unified interface for all rule-parsing code.

### Alias resolution

When `spec` begins with `*`, the remainder (after trimming and lowercasing) is looked up in the `cpu_aliases` map. The `*` prefix is stripped before lookup, so `*P` and `*p` both resolve to alias key `"p"`.

If the alias is not found, the function pushes an error in the format:

> Line {line_number}: Undefined alias '\*{alias}' in {field_name} field

and returns an empty `Vec<u32>`.

### Direct spec passthrough

When `spec` does not start with `*`, it is passed directly to [parse_cpu_spec](parse_cpu_spec.md), which handles all supported CPU specification formats including ranges (`0-7`), semicolon-separated indices (`0;4;8`), and hex bitmasks (`0xFF`).

### Usage in rule parsing

This function is called by [parse_and_insert_rules](parse_and_insert_rules.md) for the affinity, cpuset, and prime CPU fields of each process rule. It is also used internally by [parse_ideal_processor_spec](parse_ideal_processor_spec.md) for ideal processor alias lookups (though that function handles the `*` prefix splitting itself).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | Private (`fn`) |
| **Line** | L221–L241 |
| **Called by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Calls** | [parse_cpu_spec](parse_cpu_spec.md) |