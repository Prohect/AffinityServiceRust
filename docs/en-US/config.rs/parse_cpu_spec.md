# parse_cpu_spec function (config.rs)

Parses a CPU specification string into a sorted list of CPU indices. This function is the primary entry point for interpreting the various CPU specification formats supported by the AffinityServiceRust configuration file, including numeric ranges, semicolon-separated individual indices, and legacy hexadecimal bitmasks.

## Syntax

```AffinityServiceRust/src/config.rs#L78-113
pub fn parse_cpu_spec(s: &str) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | A CPU specification string in one of the supported formats described below. Leading and trailing whitespace is trimmed before parsing. |

## Return value

Type: `List<[u32; CONSUMER_CPUS]>`

A sorted, deduplicated list of `u32` CPU indices. Returns an empty list when the input is empty, `"0"`, or cannot be parsed.

## Remarks

### Supported formats

| Format | Example | Result | Notes |
|--------|---------|--------|-------|
| Empty or `"0"` | `""`, `"0"` | `[]` (empty) | Indicates "no change" — the current CPU assignment is left unmodified. |
| Hex bitmask | `"0xFF"`, `"0X0F"` | `[0,1,2,3,4,5,6,7]`, `[0,1,2,3]` | Legacy format for ≤ 64 logical processors. Must start with `0x` or `0X`. Parsed via `u64::from_str_radix`. |
| CPU range | `"0-7"` | `[0,1,2,3,4,5,6,7]` | Inclusive range. The recommended format for modern configurations. |
| Individual CPUs | `"0;4;8"` | `[0,4,8]` | Semicolon-separated list of individual CPU indices. |
| Mixed ranges | `"0-3;8-11"` | `[0,1,2,3,8,9,10,11]` | Multiple ranges and/or individual indices separated by semicolons. Supports >64 core systems. |
| Single CPU | `"7"` | `[7]` | Interpreted as CPU index 7, **not** as a bitmask. Use `"0x7"` or `"0-2"` for cores 0–2. |

### Algorithm

1. The input string is trimmed of whitespace.
2. If the result is empty or exactly `"0"`, an empty list is returned immediately.
3. If the string starts with `"0x"` or `"0X"`, it is treated as a hexadecimal bitmask:
   - The hex portion is parsed as a `u64`.
   - Bit positions that are set (`1`) are converted to CPU indices via [`mask_to_cpu_indices`](mask_to_cpu_indices.md).
   - If hex parsing fails, an empty list is returned.
4. Otherwise, the string is split on `';'` and each segment is processed:
   - Empty segments (from trailing or double semicolons) are skipped.
   - If the segment contains a `'-'`, it is parsed as an inclusive range `start-end`. Both bounds default to `0` on parse failure; `start` defaults the end on failure.
   - If the segment is a plain integer, it is added as a single CPU index.
   - Duplicate CPU indices are suppressed during insertion.
5. The resulting list is sorted in ascending order before being returned.

### Important disambiguation

The value `"7"` means **CPU index 7** (a single logical processor), not a bitmask for CPUs 0–2. This is a deliberate design choice to avoid ambiguity between bitmask and index interpretations. To specify CPUs 0, 1, and 2, use either the hex notation `"0x7"` or the range notation `"0-2"`.

### Edge cases

- Invalid hex strings after the `0x`/`0X` prefix return an empty list without error.
- Non-numeric range bounds (e.g., `"a-z"`) fall back to `0` via `unwrap_or(0)`.
- Duplicate indices within a specification are silently deduplicated.
- The function does not validate that CPU indices correspond to physically present processors on the system; that validation occurs at application time.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | [`resolve_cpu_spec`](resolve_cpu_spec.md), [`parse_alias`](parse_alias.md), [`parse_mask`](parse_mask.md), [`read_config`](read_config.md) (indirectly) |
| Callees | [`mask_to_cpu_indices`](mask_to_cpu_indices.md) (for hex bitmask inputs) |
| API | `List` and `CONSUMER_CPUS` from [`collections.rs`](../collections.rs/README.md) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_mask | [parse_mask](parse_mask.md) |
| config module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*