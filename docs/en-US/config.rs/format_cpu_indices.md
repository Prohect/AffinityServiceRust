# format_cpu_indices function (config.rs)

Formats a slice of CPU indices into a compact, human-readable string representation using range notation where consecutive indices allow it. This is the inverse display operation of [`parse_cpu_spec`](parse_cpu_spec.md) and is used throughout the application for logging and diagnostics.

## Syntax

```AffinityServiceRust/src/config.rs#L129-159
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpus` | `&[u32]` | A slice of CPU index values to format. The slice does not need to be sorted or deduplicated — the function creates a sorted copy internally before formatting. |

## Return value

Type: `String`

A compact string representation of the CPU indices. Consecutive indices are collapsed into ranges using dash notation, and non-consecutive values are separated by commas.

| Input | Output |
|-------|--------|
| `[]` (empty) | `"0"` |
| `[0, 1, 2, 3]` | `"0-3"` |
| `[0, 2, 4]` | `"0,2,4"` |
| `[0, 1, 2, 5, 6, 7]` | `"0-2,5-7"` |
| `[3]` | `"3"` |
| `[0, 1, 2, 8]` | `"0-2,8"` |

## Remarks

### Sorting

The function copies the input into a local `List<[u32; CONSUMER_CPUS]>`, sorts it, and then iterates over the sorted list to detect consecutive runs. The original input slice is not modified.

### Empty-input convention

An empty slice returns the string `"0"`, which is the configuration file convention for "no change" or "unset." This matches the behavior of [`parse_cpu_spec`](parse_cpu_spec.md), which treats both an empty string and the literal `"0"` as an empty CPU list.

### Algorithm

The formatting algorithm performs a single linear scan over the sorted list:

1. Initialize `start` and `end` to the first element.
2. While the next element equals `end + 1`, extend the current range by advancing `end`.
3. When a gap is detected (or the list is exhausted), emit the accumulated range:
   - If `start == end`, emit a single number (e.g., `"3"`).
   - If `start < end`, emit a range (e.g., `"0-3"`).
4. Separate emitted ranges with commas.

### Relationship to parse_cpu_spec

`format_cpu_indices` and [`parse_cpu_spec`](parse_cpu_spec.md) form a round-trip pair for the range subset of CPU specifications. Given sorted, deduplicated input, `parse_cpu_spec(format_cpu_indices(cpus))` reproduces the original list. However, hex-mask specifications (`0xFF`) are not reproduced by `format_cpu_indices` — ranges are always used.

### Usage

This function is called in logging and diagnostic output to display CPU sets in a readable form — for example, when printing which CPUs a rule applies to or when generating auto-grouped configuration files via [`sort_and_group_config`](sort_and_group_config.md).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Callers | Logging utilities, [`sort_and_group_config`](sort_and_group_config.md), diagnostic output |
| Callees | `List::sort` |
| Dependencies | `List`, `CONSUMER_CPUS` from [`collections.rs`](../collections.rs/README.md) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| config module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*