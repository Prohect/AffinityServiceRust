# parse_mask function (config.rs)

Parses a CPU specification string and converts it directly to a bitmask representation.

## Syntax

```rust
pub fn parse_mask(s: &str) -> usize
```

## Parameters

`s`

A CPU specification string. Supports all formats accepted by [parse_cpu_spec](parse_cpu_spec.md), including hex masks, ranges, and semicolon-separated individual CPUs.

## Return value

Returns a `usize` bitmask where each set bit corresponds to a CPU index from the parsed specification. For example, CPUs `[0, 1, 2, 3]` produce a mask of `0x0F`. Only CPUs with indices less than 64 are represented in the mask.

## Remarks

This is a convenience wrapper that combines [parse_cpu_spec](parse_cpu_spec.md) and [cpu_indices_to_mask](cpu_indices_to_mask.md) into a single call. It parses the input string into a list of CPU indices and then converts those indices into a bitmask.

Because the return type is `usize`, this function is limited to representing CPUs 0–63. On systems with more than 64 logical processors, higher-numbered CPUs will be silently excluded from the resulting mask. For >64 core systems, prefer working with CPU index vectors via [parse_cpu_spec](parse_cpu_spec.md) directly.

This function is currently marked `#[allow(dead_code)]` in the source, indicating it may be reserved for future use or external tooling.

### Examples

| Input | Parsed CPUs | Returned Mask |
| --- | --- | --- |
| `"0-3"` | `[0, 1, 2, 3]` | `0x0F` |
| `"0;2;4"` | `[0, 2, 4]` | `0x15` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0x00` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Declared at** | Line 860 |
| **Visibility** | `pub` |
| **Calls** | [parse_cpu_spec](parse_cpu_spec.md), [cpu_indices_to_mask](cpu_indices_to_mask.md) |

## See also

- [parse_cpu_spec](parse_cpu_spec.md) — Parses CPU specification strings into index vectors.
- [cpu_indices_to_mask](cpu_indices_to_mask.md) — Converts CPU index slices to bitmasks.
- [mask_to_cpu_indices](mask_to_cpu_indices.md) — Inverse operation: bitmask to CPU indices.