# mask_to_cpu_indices function (config.rs)

Converts a 64-bit bitmask into a sorted list of CPU index positions where bits are set. This is a private helper used internally by [`parse_cpu_spec`](parse_cpu_spec.md) when parsing hexadecimal CPU mask values.

## Syntax

```AffinityServiceRust/src/config.rs#L115-117
fn mask_to_cpu_indices(mask: u64) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `mask` | `u64` | A 64-bit unsigned integer where each set bit represents a logical processor. Bit 0 corresponds to CPU 0, bit 1 to CPU 1, and so on up to bit 63 for CPU 63. |

## Return value

Type: `List<[u32; CONSUMER_CPUS]>`

A list of `u32` CPU indices corresponding to the set bits in `mask`, sorted in ascending order. If `mask` is `0`, the returned list is empty.

### Examples

| Input (`mask`) | Output |
|----------------|--------|
| `0x0F` | `[0, 1, 2, 3]` |
| `0x8001` | `[0, 15]` |
| `0x00` | `[]` |
| `0xFFFFFFFFFFFFFFFF` | `[0, 1, 2, ..., 63]` |

## Remarks

- The function iterates over bit positions 0 through 63 and collects those where `(mask >> i) & 1 == 1`. It uses Rust's `Iterator::filter` and `Iterator::collect` for a concise implementation.

- Because it iterates from bit 0 to bit 63 in order, the resulting list is inherently sorted in ascending order without requiring a separate sort step.

- This function only supports up to 64 logical processors (a single Windows processor group). For systems with more than 64 logical processors, the range-based CPU specification format (`0-7;64-71`) should be used instead of hex bitmasks. The hex mask format is considered legacy and is preserved for backward compatibility.

- This function is module-private (`fn`, not `pub fn`) and is not accessible outside of `config.rs`.

### Algorithm

```/dev/null/pseudocode.txt#L1-3
for i in 0..64:
    if bit i of mask is set:
        append i to result list
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (crate-internal) |
| Callers | [`parse_cpu_spec`](parse_cpu_spec.md) |
| Callees | Standard iterator methods (`filter`, `collect`) |
| Dependencies | `List` and `CONSUMER_CPUS` from [`collections.rs`](../collections.rs/README.md) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| config module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*