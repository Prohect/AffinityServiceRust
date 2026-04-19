# mask_to_cpu_indices function (config.rs)

Converts a 64-bit CPU bitmask into a sorted list of individual CPU indices. Each set bit in the mask corresponds to a logical processor number in the output list.

## Syntax

```rust
fn mask_to_cpu_indices(mask: u64) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

`mask: u64`

A 64-bit bitmask where bit *N* being set indicates that logical processor *N* should be included in the output list. For example, `0x15` (binary `10101`) represents processors 0, 2, and 4.

## Return value

`List<[u32; CONSUMER_CPUS]>` — A stack-allocated small vector of CPU indices, sorted in ascending order. If `mask` is `0`, the returned list is empty.

## Remarks

The function iterates over bit positions 0 through 63, testing each bit with `(mask >> i) & 1 == 1`. Matching positions are collected into the output list via the `Iterator::collect()` trait. Because bits are tested from LSB to MSB, the resulting list is inherently sorted in ascending order.

### Limitations

- The function supports a maximum of 64 logical processors, which is the limit of a single Windows processor group. Systems with more than 64 processors require range-based CPU specifications (`"0-7;64-71"`) handled by [parse_cpu_spec](parse_cpu_spec.md) instead of bitmask notation.
- The return type uses `CONSUMER_CPUS` (32) as the inline capacity. If more than 32 CPUs are set in the mask, the list spills to heap allocation, which is acceptable for the configuration parsing hot path.

### Visibility

This function is module-private (`fn`, not `pub fn`). It is called exclusively by [parse_cpu_spec](parse_cpu_spec.md) when parsing hexadecimal CPU mask strings (e.g., `"0xFF"`).

### Inverse operation

The inverse of this function is [cpu_indices_to_mask](cpu_indices_to_mask.md), which converts a slice of CPU indices back to a `usize` bitmask.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Callers** | [parse_cpu_spec](parse_cpu_spec.md) (hex mask branch) |
| **Callees** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Inverse conversion | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| CPU index formatter | [format_cpu_indices](format_cpu_indices.md) |
| Hex mask parser | [parse_mask](parse_mask.md) |
| Collection types | [List](../collections.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*