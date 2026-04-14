# parse_cpu_spec function (config.rs)

Parses a CPU specification string into a sorted, deduplicated vector of logical CPU indices. This is the foundational CPU-spec parser used throughout the configuration module; it accepts hex bitmasks, inclusive ranges, and semicolon-separated individual indices. All other CPU-spec–aware functions ultimately delegate to `parse_cpu_spec` or consume its output.

## Syntax

```rust
pub fn parse_cpu_spec(s: &str) -> Vec<u32>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | A CPU specification string in one of the supported formats described below. Leading and trailing whitespace is trimmed before parsing. |

## Return value

Returns a `Vec<u32>` of logical CPU indices in ascending sorted order with no duplicates. Returns an empty vector when:

- `s` is empty or contains only whitespace.
- `s` is `"0"` (the canonical "no change" sentinel).
- `s` is a hex prefix (`0x` / `0X`) followed by a value that fails to parse as `u64`.

## Remarks

### Supported formats

| Format | Example | Result | Notes |
|--------|---------|--------|-------|
| Empty / `"0"` | `""`, `"0"` | `[]` | Sentinel meaning "do not modify this setting." |
| Hex bitmask | `"0xFF"`, `"0x0F"` | `[0,1,2,3,4,5,6,7]`, `[0,1,2,3]` | Legacy format; limited to 64 CPUs because the mask is parsed as `u64`. Prefix is case-insensitive (`0x` or `0X`). Delegates to [mask_to_cpu_indices](mask_to_cpu_indices.md) for bit extraction. |
| Inclusive range | `"0-7"` | `[0,1,2,3,4,5,6,7]` | Start and end are inclusive. If the end value fails to parse, it defaults to the start value, producing a single-element range. |
| Individual indices | `"0;4;8"` | `[0,4,8]` | Semicolon-separated. Whitespace around each part is trimmed. Non-numeric parts are silently skipped. |
| Mixed ranges and indices | `"0-3;8;12-15"` | `[0,1,2,3,8,12,13,14,15]` | Ranges and individual indices can be freely combined. |

### Deduplication and sorting

Duplicate CPU indices that arise from overlapping ranges or repeated values are suppressed during accumulation (each index is checked against the existing vector before insertion). The final vector is sorted in ascending order before being returned.

### Systems with more than 64 logical processors

The range and semicolon formats support arbitrary `u32` indices, making them suitable for systems with more than 64 logical processors (e.g., `"0-7;64-71"` for a dual-processor-group configuration). The hex bitmask format is limited to 64 bits and should be considered a legacy compatibility path.

### Error handling

`parse_cpu_spec` is intentionally lenient. Unparseable range endpoints default to `0`, unparseable individual indices are skipped, and an invalid hex value after `0x` returns an empty vector. No errors are reported to the caller; validation with user-facing error messages is the responsibility of higher-level functions like [resolve_cpu_spec](resolve_cpu_spec.md).

### Examples

```text
parse_cpu_spec("0-3")       → [0, 1, 2, 3]
parse_cpu_spec("0;2;4")     → [0, 2, 4]
parse_cpu_spec("0x0F")      → [0, 1, 2, 3]
parse_cpu_spec("0-3;8;12")  → [0, 1, 2, 3, 8, 12]
parse_cpu_spec("0")         → []
parse_cpu_spec("")          → []
```

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [resolve_cpu_spec](resolve_cpu_spec.md), [parse_alias](parse_alias.md), [parse_mask](parse_mask.md), [convert](convert.md) |
| **Callees** | [mask_to_cpu_indices](mask_to_cpu_indices.md) (for hex bitmask inputs) |
| **API** | Pure function — no I/O, no Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Bitmask to CPU index conversion | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU index list to bitmask conversion | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| Compact range formatting (inverse operation) | [format_cpu_indices](format_cpu_indices.md) |
| Alias-aware CPU spec resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Convenience parse-to-mask wrapper | [parse_mask](parse_mask.md) |
| Module overview | [config module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd