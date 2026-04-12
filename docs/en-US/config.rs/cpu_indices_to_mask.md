# cpu_indices_to_mask function (config.rs)

Converts a slice of CPU indices into a bitmask representation suitable for legacy Windows affinity APIs that operate on systems with 64 or fewer logical processors.

## Syntax

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

## Parameters

`cpus`

A slice of CPU indices to encode into a bitmask. Each value represents a zero-based logical processor number. Values of 64 or greater are silently ignored since they cannot be represented in a single `usize` bitmask.

## Return value

Returns a `usize` bitmask where bit N is set if CPU index N is present in the input slice. For example, indices `[0, 2, 3]` produce the bitmask `0b1101` (decimal 13).

## Remarks

This function is the inverse of [mask_to_cpu_indices](mask_to_cpu_indices.md). It is used when interacting with Windows APIs such as `SetProcessAffinityMask` that require a bitmask rather than a list of processor indices.

CPU indices >= 64 are skipped because the bitmask is stored in a `usize`, which is 64 bits on 64-bit Windows. For systems with more than 64 logical processors, use CPU set APIs instead (see `cpu_set_cpus` in [ProcessConfig](ProcessConfig.md)).

The function does not require the input to be sorted or deduplicated. Setting the same bit twice is a no-op due to the bitwise OR operation.

### Algorithm

For each CPU index in the input slice:
1. If the index is less than 64, set the corresponding bit in the mask via `mask |= 1usize << cpu`.
2. Otherwise, skip the index.

### Example

Given CPUs `[0, 1, 2, 3]`, the result is `0x0F` (binary `00001111`).

Given CPUs `[4, 8, 12]`, the result is `0x1110` (binary `0001_0001_0001_0000`).

### Related functions

| Function | Purpose |
| --- | --- |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | Inverse operation — converts a bitmask to a list of CPU indices |
| [parse_cpu_spec](parse_cpu_spec.md) | Parses a CPU specification string into CPU indices |
| [format_cpu_indices](format_cpu_indices.md) | Formats CPU indices as a human-readable compact string |
| [parse_mask](parse_mask.md) | Convenience wrapper that parses a spec string directly into a mask |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | `pub` |
| **Called by** | [parse_mask](parse_mask.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **Depends on** | — |