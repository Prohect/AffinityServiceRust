# parse_cpu_spec function (config.rs)

Parses a CPU specification string into a sorted list of CPU indices. This is the central CPU-set parser used throughout the configuration system — every CPU alias definition, affinity field, CPU set field, and prime thread field ultimately passes through this function.

## Syntax

```rust
pub fn parse_cpu_spec(s: &str) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

`s: &str`

A string describing a set of logical processor indices. The following formats are accepted:

| Format | Example | Result |
|--------|---------|--------|
| Empty or `"0"` | `""`, `"0"` | Empty list (no CPUs / no change) |
| Hex bitmask | `"0xFF"`, `"0x15"` | CPU indices derived from set bits (legacy, ≤64 cores) |
| Inclusive range | `"0-7"` | `[0, 1, 2, 3, 4, 5, 6, 7]` |
| Semicolon-separated individuals | `"0;4;8"` | `[0, 4, 8]` |
| Mixed ranges and individuals | `"0-3;8;12-15"` | `[0, 1, 2, 3, 8, 12, 13, 14, 15]` |

## Return value

`List<[u32; CONSUMER_CPUS]>` — A sorted, deduplicated [SmallVec](../collections.rs/README.md)-backed list of `u32` CPU indices. Returns an empty list when the input is empty, `"0"`, or unparseable.

## Remarks

### Parsing algorithm

1. **Trim** the input string.
2. **Empty / zero check:** If the trimmed string is empty or exactly `"0"`, return an empty list immediately. The value `"0"` is the config convention for "no CPU restriction".
3. **Hex prefix detection:** If the string starts with `"0x"` or `"0X"`, parse the remainder as a hexadecimal `u64` and delegate to [mask_to_cpu_indices](mask_to_cpu_indices.md) to extract set bit positions. On parse failure, return an empty list.
4. **Semicolon splitting:** Split the string on `';'`. Each segment is processed independently:
   - If the segment contains `'-'`, parse the portions before and after the dash as `u32` start and end values and insert every integer in the inclusive range `[start, end]`.
   - Otherwise, parse the segment as a single `u32` CPU index.
   - Duplicates are skipped during insertion (`contains` check).
5. **Sort** the resulting list in ascending order before returning.

### Design choices

- **Semicolons as delimiters:** Colons (`:`) are reserved as field separators in the config rule format, so CPU specs use semicolons to separate individual values or ranges.
- **`"0"` means no change:** This is a deliberate sentinel value. A config field of `0` signals that the service should not modify the corresponding setting. This is distinct from a hex mask `0x0` (which also results in an empty list).
- **Hex masks limited to 64 cores:** The hex bitmask format is a legacy convenience inherited from Process Lasso compatibility. On systems with more than 64 logical processors, use range syntax (`"0-7;64-71"`) instead.
- **No error reporting:** Invalid tokens within a semicolon-delimited spec are silently skipped (`unwrap_or` to `0` or parse failure ignored). Validation errors are caught at a higher level by checking whether the resulting list is unexpectedly empty.

### Stack allocation

The return type `List<[u32; CONSUMER_CPUS]>` is a `SmallVec` that stores up to `CONSUMER_CPUS` (32) CPU indices inline on the stack. Typical consumer systems (≤32 cores) never heap-allocate.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Callers** | [parse_alias](parse_alias.md), [resolve_cpu_spec](resolve_cpu_spec.md), [parse_mask](parse_mask.md), [convert](convert.md) |
| **Callees** | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Bitmask to CPU indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU indices to bitmask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| Format CPU indices for display | [format_cpu_indices](format_cpu_indices.md) |
| Alias resolution wrapper | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Collection types | [List / CONSUMER_CPUS](../collections.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*