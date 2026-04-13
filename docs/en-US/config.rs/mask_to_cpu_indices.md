# mask_to_cpu_indices function (config.rs)

Converts a 64-bit bitmask into a sorted vector of CPU indices. Each set bit in the mask corresponds to a logical CPU index in the returned vector. This is the inverse of [cpu_indices_to_mask](cpu_indices_to_mask.md) (for values that fit in 64 bits).

## Syntax

```rust
fn mask_to_cpu_indices(mask: u64) -> Vec<u32>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `mask` | `u64` | A bitmask where bit *N* being set indicates that logical CPU *N* should be included in the output. Bits 0–63 are examined; higher CPUs cannot be represented in this format. |

## Return value

Returns a `Vec<u32>` containing the zero-based indices of all set bits in `mask`, in ascending order.

If `mask` is `0`, the returned vector is empty.

## Remarks

This function is the low-level workhorse behind the hex-mask branch of [parse_cpu_spec](parse_cpu_spec.md). When a CPU specification string begins with `0x` or `0X`, `parse_cpu_spec` parses the hex value into a `u64` and delegates to `mask_to_cpu_indices` for bit extraction.

### Implementation

The function filters the range `0..64`, collecting every index `i` for which `(mask >> i) & 1 == 1`. The result is naturally sorted because the range is iterated in ascending order.

```rust
fn mask_to_cpu_indices(mask: u64) -> Vec<u32> {
    (0..64).filter(|i| (mask >> i) & 1 == 1).collect()
}
```

### Visibility

This function has **crate-private** visibility (`fn`, not `pub fn`). It is called only from [parse_cpu_spec](parse_cpu_spec.md) within the `config` module.

### 64-core limitation

Because the input is a `u64`, this function can only represent CPUs 0–63. Systems with more than 64 logical processors should use the range/semicolon syntax (`0-7;64-71`) in their CPU specifications instead of hex masks. The range syntax in [parse_cpu_spec](parse_cpu_spec.md) has no upper-bound limitation.

### Examples

| Input mask | Output |
|------------|--------|
| `0x00` | `[]` |
| `0x01` | `[0]` |
| `0x0F` | `[0, 1, 2, 3]` |
| `0xFF00` | `[8, 9, 10, 11, 12, 13, 14, 15]` |
| `0x8000_0000_0000_0001` | `[0, 63]` |

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [parse_cpu_spec](parse_cpu_spec.md) |
| **Inverse** | [cpu_indices_to_mask](cpu_indices_to_mask.md) (truncates to `usize`) |

## See Also

| Topic | Link |
|-------|------|
| CPU spec string parser | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU indices → bitmask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| CPU indices → display string | [format_cpu_indices](format_cpu_indices.md) |
| Convenience parse-to-mask | [parse_mask](parse_mask.md) |
| Module overview | [config module](README.md) |