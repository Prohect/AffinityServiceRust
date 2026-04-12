# parse_cpu_spec function (config.rs)

Parses a CPU specification string into a sorted list of CPU indices. Supports multiple input formats including hex bitmasks, inclusive ranges, individual CPU indices, and combinations thereof.

## Syntax

```rust
pub fn parse_cpu_spec(s: &str) -> Vec<u32>
```

## Parameters

`s`

A string slice containing the CPU specification to parse. Leading and trailing whitespace is trimmed automatically. The following formats are supported:

| Format | Example | Result |
| --- | --- | --- |
| Empty or `"0"` | `""`, `"0"` | `[]` (empty vec) |
| Hex bitmask (legacy, ≤64 cores) | `"0xFF"`, `"0x0F"` | `[0, 1, 2, 3, 4, 5, 6, 7]`, `[0, 1, 2, 3]` |
| Inclusive range | `"0-7"` | `[0, 1, 2, 3, 4, 5, 6, 7]` |
| Individual CPUs (semicolon-separated) | `"0;4;8"` | `[0, 4, 8]` |
| Multiple ranges | `"0-7;64-71"` | `[0, 1, 2, 3, 4, 5, 6, 7, 64, 65, 66, 67, 68, 69, 70, 71]` |

## Return value

Returns a `Vec<u32>` containing the sorted, deduplicated list of CPU indices parsed from the specification string. Returns an empty vector when the input is empty, `"0"`, or contains an unparseable hex mask.

## Remarks

This is the foundational CPU specification parser used throughout the configuration system. All other functions that accept CPU specifications ultimately delegate to `parse_cpu_spec` (either directly or via [resolve_cpu_spec](resolve_cpu_spec.md)).

**Hex bitmask format** is considered legacy and only supports up to 64 cores. It is recognized by the `0x` or `0X` prefix. Each set bit in the mask corresponds to a CPU index. For example, `0x0F` (binary `00001111`) maps to CPUs `[0, 1, 2, 3]`. The conversion is performed internally by [mask_to_cpu_indices](mask_to_cpu_indices.md).

**Range and individual formats** use semicolons (`;`) as delimiters between segments. Each segment can be either a single CPU number or an inclusive range with a dash (`-`). Duplicate CPU indices across segments are automatically removed. The final result is always sorted in ascending order.

This function does **not** resolve CPU aliases (names prefixed with `*`). Alias resolution is handled by [resolve_cpu_spec](resolve_cpu_spec.md), which calls `parse_cpu_spec` internally for non-alias specifications.

### Examples

```text
"0-3"       → [0, 1, 2, 3]
"0;2;4"     → [0, 2, 4]
"0x0F"      → [0, 1, 2, 3]
"0-7;64-71" → [0, 1, 2, 3, 4, 5, 6, 7, 64, 65, 66, 67, 68, 69, 70, 71]
""          → []
"0"         → []
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | `pub` |
| **Lines** | L70–L118 |
| **Called by** | [resolve_cpu_spec](resolve_cpu_spec.md), [parse_alias](parse_alias.md), [parse_mask](parse_mask.md), [convert](convert.md) |
| **Calls** | [mask_to_cpu_indices](mask_to_cpu_indices.md) |

## See also

- [resolve_cpu_spec](resolve_cpu_spec.md) — wrapper that adds CPU alias resolution
- [mask_to_cpu_indices](mask_to_cpu_indices.md) — converts a bitmask to CPU index list
- [cpu_indices_to_mask](cpu_indices_to_mask.md) — inverse operation (indices to bitmask)
- [format_cpu_indices](format_cpu_indices.md) — formats CPU indices back to a compact string