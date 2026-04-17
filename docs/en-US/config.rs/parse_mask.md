# parse_mask function (config.rs)

Convenience function that parses a CPU specification string and returns the corresponding `usize` bitmask. This is a shorthand composition of [`parse_cpu_spec`](parse_cpu_spec.md) followed by [`cpu_indices_to_mask`](cpu_indices_to_mask.md).

## Syntax

```AffinityServiceRust/src/config.rs#L895-898
#[allow(dead_code)]
pub fn parse_mask(s: &str) -> usize {
    let cpus = parse_cpu_spec(s);
    cpu_indices_to_mask(&cpus)
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | A CPU specification string in any format accepted by [`parse_cpu_spec`](parse_cpu_spec.md): ranges (`"0-7"`), individual indices (`"0;4;8"`), hex bitmasks (`"0xFF"`), a single CPU index (`"7"`), or the special value `"0"` (no CPUs). |

## Return value

Type: `usize`

A bitmask where each set bit represents a logical processor index from the parsed specification. Bit 0 corresponds to CPU 0, bit 1 to CPU 1, and so on. Returns `0` when the input is empty, `"0"`, or unparseable.

### Examples

| Input | Intermediate CPU indices | Output (hex) |
|-------|--------------------------|--------------|
| `"0-3"` | `[0, 1, 2, 3]` | `0x0F` |
| `"0;2;4"` | `[0, 2, 4]` | `0x15` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0x00` |
| `""` | `[]` | `0x00` |

## Remarks

- This function is annotated with `#[allow(dead_code)]` in the source, indicating it may not be actively called in the current codebase but is retained as a utility for testing, debugging, or future use.

- **64-core limitation**: Because the return type is `usize` and [`cpu_indices_to_mask`](cpu_indices_to_mask.md) only sets bits for CPU indices less than 64, any CPU indices ≥ 64 in the parsed specification are silently dropped from the resulting bitmask. For systems with more than 64 logical processors, use [`parse_cpu_spec`](parse_cpu_spec.md) directly to work with the full index list.

- The function is `pub`, making it available to other modules in the crate, even though internal usage may be limited.

- Round-trip fidelity: For inputs that produce only indices in the range 0–63, the bitmask faithfully represents the parsed specification. However, `parse_mask` is a lossy operation for >64-core specs and does not preserve the original string format (ranges vs. individual indices vs. hex notation).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | Currently unused (`#[allow(dead_code)]`); available as a utility |
| Callees | [`parse_cpu_spec`](parse_cpu_spec.md), [`cpu_indices_to_mask`](cpu_indices_to_mask.md) |
| API | Standard library only |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| config module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*