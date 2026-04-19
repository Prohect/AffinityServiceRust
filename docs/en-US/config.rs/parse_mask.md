# parse_mask function (config.rs)

Convenience function that parses a CPU specification string and returns the corresponding bitmask. Combines [parse_cpu_spec](parse_cpu_spec.md) and [cpu_indices_to_mask](cpu_indices_to_mask.md) into a single call.

## Syntax

```rust
pub fn parse_mask(s: &str) -> usize
```

## Parameters

`s: &str`

A CPU specification string in any format accepted by [parse_cpu_spec](parse_cpu_spec.md): ranges (`"0-7"`), semicolon-separated indices (`"0;4;8"`), hex masks (`"0xFF"`), or the sentinel value `"0"` (no CPUs).

## Return value

`usize` — A bitmask where bit *N* is set if CPU index *N* was present in the parsed specification. Returns `0` when the input is empty, `"0"`, or unparseable.

## Remarks

This function is a two-step pipeline:

1. The input string `s` is passed to [parse_cpu_spec](parse_cpu_spec.md), producing a sorted `List<[u32; CONSUMER_CPUS]>` of CPU indices.
2. The resulting index list is passed to [cpu_indices_to_mask](cpu_indices_to_mask.md), producing the `usize` bitmask.

### Example conversions

| Input | Intermediate indices | Output mask |
|-------|---------------------|-------------|
| `"0-3"` | `[0, 1, 2, 3]` | `0xF` |
| `"0;2;4"` | `[0, 2, 4]` | `0x15` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0x0` |

### 64-core limitation

Because [cpu_indices_to_mask](cpu_indices_to_mask.md) silently drops indices ≥ 64, any CPU indices beyond processor 63 in the parsed spec will not be represented in the returned bitmask. For systems with more than 64 logical processors, prefer working with the CPU index list from [parse_cpu_spec](parse_cpu_spec.md) directly.

### Dead code allowance

The function is annotated with `#[allow(dead_code)]`, indicating it may not have active callers in all build configurations but is retained as a public utility.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Public (`pub fn`) |
| **Callers** | Utility / external consumers |
| **Callees** | [parse_cpu_spec](parse_cpu_spec.md), [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Index-to-mask conversion | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| Mask-to-index conversion | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| Display formatting | [format_cpu_indices](format_cpu_indices.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*