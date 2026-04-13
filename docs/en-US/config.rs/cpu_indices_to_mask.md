# cpu_indices_to_mask function (config.rs)

Converts a slice of CPU indices into a `usize` bitmask where each set bit corresponds to a CPU index present in the input. Indices greater than or equal to 64 are silently ignored because they cannot be represented in a single 64-bit mask.

## Syntax

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpus` | `&[u32]` | Slice of logical CPU indices to encode. Duplicate values are harmless (the bit is set idempotently). The slice does not need to be sorted. |

## Return value

Returns a `usize` bitmask. Bit *N* is set if the value *N* appears in `cpus` and *N* < 64. Returns `0` if `cpus` is empty or all indices are ≥ 64.

## Remarks

### Bit layout

The least-significant bit (bit 0) represents CPU 0. For example, the input `[0, 2, 4]` produces the mask `0b10101` (`0x15`).

### 64-core limit

Because the return type is `usize` (64 bits on x86-64 Windows), indices ≥ 64 are silently dropped. This mirrors the Windows `DWORD_PTR` affinity mask, which also supports at most 64 processors per processor group. For systems with more than 64 logical processors, use CPU set APIs ([cpu_set_cpus](ProcessConfig.md) field) rather than affinity masks.

### Inverse operation

[mask_to_cpu_indices](mask_to_cpu_indices.md) performs the reverse conversion, expanding a bitmask back into a sorted vector of CPU indices.

### Usage in the codebase

This function is called by:

- [parse_mask](parse_mask.md), which chains [parse_cpu_spec](parse_cpu_spec.md) → `cpu_indices_to_mask` as a convenience wrapper.
- The apply module when building affinity masks from [ProcessConfig](ProcessConfig.md)`.affinity_cpus` for `SetProcessAffinityMask`.
- The winapi module for CPU set and mask conversions.

### Examples

| Input | Output | Notes |
|-------|--------|-------|
| `&[]` | `0` | Empty input yields zero mask. |
| `&[0]` | `1` | Single CPU 0. |
| `&[0, 1, 2, 3]` | `0xF` | Contiguous range. |
| `&[0, 2, 4, 63]` | `0x8000000000000015` | Sparse indices including the last representable bit. |
| `&[64, 128]` | `0` | All indices out of range, silently ignored. |
| `&[0, 0, 0]` | `1` | Duplicates are harmless. |

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Callers** | [parse_mask](parse_mask.md), apply module (`apply_affinity`, `apply_prime_threads_promote`), winapi module (`cpusetids_from_mask`) |
| **Callees** | None |
| **Inverse** | [mask_to_cpu_indices](mask_to_cpu_indices.md) |

## See Also

| Topic | Link |
|-------|------|
| Bitmask to index list | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU spec string parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Convenience spec-to-mask | [parse_mask](parse_mask.md) |
| Compact range formatter | [format_cpu_indices](format_cpu_indices.md) |
| Per-process config (affinity_cpus field) | [ProcessConfig](ProcessConfig.md) |
| Module overview | [config module](README.md) |