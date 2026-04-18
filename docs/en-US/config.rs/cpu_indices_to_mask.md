# cpu_indices_to_mask function (config.rs)

Converts a slice of CPU indices into a `usize` bitmask suitable for use with Windows affinity APIs. Each CPU index sets the corresponding bit in the returned mask. This is the inverse operation of [`mask_to_cpu_indices`](mask_to_cpu_indices.md).

## Syntax

```AffinityServiceRust/src/config.rs#L119-127
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize {
    let mut mask: usize = 0;
    for &cpu in cpus {
        if cpu < 64 {
            mask |= 1usize << cpu;
        }
    }
    mask
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpus` | `&[u32]` | A slice of CPU index values. Each value represents a logical processor number (0-based). Values >= 64 are silently ignored because they cannot be represented in a single `usize` bitmask on 64-bit platforms. |

## Return value

Type: `usize`

A bitmask where bit N is set if N appears in the input `cpus` slice. Returns `0` when `cpus` is empty or contains only indices >= 64.

### Examples

| Input | Output (hex) |
|-------|--------------|
| `[0, 1, 2, 3]` | `0x0F` |
| `[0, 15]` | `0x8001` |
| `[]` | `0x0` |
| `[0, 2, 4]` | `0x15` |
| `[64, 65]` | `0x0` (indices >= 64 are dropped) |

## Remarks

- **64-core limitation**: The function guards against overflow by checking `cpu < 64` before shifting. On a 64-bit Windows system, `usize` is 64 bits wide, so bits 0-63 are representable. CPU indices >= 64 (which occur on systems with multiple processor groups) are silently dropped. For such systems, the range-based CPU specification format should be used with the full `List<[u32; CONSUMER_CPUS]>` representation rather than bitmasks.

- **Relationship to Windows APIs**: The returned `usize` value can be passed directly to Windows API functions like `SetProcessAffinityMask` (which expects a `ULONG_PTR` affinity mask) after casting.

- **No deduplication**: Duplicate CPU indices in the input do not cause errors -- setting a bit that is already set is a no-op. The result is the same regardless of whether duplicates are present.

- This function is `pub` and is used by [`parse_mask`](parse_mask.md) and by the process-application logic in `apply.rs`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | [`parse_mask`](parse_mask.md), `apply.rs` (affinity application logic) |
| Callees | None (bit manipulation only) |
| API | Standard library only |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| parse_mask | [parse_mask](parse_mask.md) |
| config module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
