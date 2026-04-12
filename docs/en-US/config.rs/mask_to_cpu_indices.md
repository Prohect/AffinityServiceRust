# mask_to_cpu_indices function (config.rs)

Converts a 64-bit bitmask into a sorted vector of CPU indices corresponding to each set bit.

## Syntax

```rust
fn mask_to_cpu_indices(mask: u64) -> Vec<u32>
```

## Parameters

`mask`

A 64-bit unsigned integer bitmask where each set bit represents an active CPU. Bit 0 corresponds to CPU 0, bit 1 to CPU 1, and so on up to bit 63 for CPU 63.

## Return value

Returns a `Vec<u32>` containing the zero-based indices of all set bits in the mask, sorted in ascending order. Returns an empty vector if the mask is `0`.

## Remarks

This is an internal helper function used by [parse_cpu_spec](parse_cpu_spec.md) to handle legacy hexadecimal bitmask notation (e.g., `0xFF`). When `parse_cpu_spec` encounters a string starting with `0x` or `0X`, it parses the hex value into a `u64` and delegates to this function to extract individual CPU indices.

The function iterates over bit positions 0 through 63, checking each bit with `(mask >> i) & 1 == 1`. This limits support to systems with at most 64 logical processors when using bitmask notation. For systems with more than 64 cores, use range or semicolon-separated syntax instead (e.g., `0-7;64-71`).

This function is the inverse of [cpu_indices_to_mask](cpu_indices_to_mask.md).

### Examples

| Input mask | Output |
| --- | --- |
| `0x0F` (15) | `[0, 1, 2, 3]` |
| `0xFF` (255) | `[0, 1, 2, 3, 4, 5, 6, 7]` |
| `0x05` (5) | `[0, 2]` |
| `0x00` (0) | `[]` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | Private (`fn`) |
| **Called by** | [parse_cpu_spec](parse_cpu_spec.md) |
| **See also** | [cpu_indices_to_mask](cpu_indices_to_mask.md), [format_cpu_indices](format_cpu_indices.md) |