# format_cpu_indices function (config.rs)

Formats a slice of CPU indices into a compact, human-readable string representation using range notation where possible.

## Syntax

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## Parameters

`cpus`

A slice of CPU indices (`&[u32]`) to format. The indices do not need to be pre-sorted; the function sorts them internally before formatting.

## Return value

Returns a `String` containing the formatted CPU specification. Consecutive indices are collapsed into ranges using dash notation (`start-end`), and non-consecutive indices or ranges are separated by commas. Returns `"0"` if the input slice is empty.

## Remarks

This function is the display-side counterpart to [parse_cpu_spec](parse_cpu_spec.md). While `parse_cpu_spec` converts a string specification into a vector of CPU indices, `format_cpu_indices` converts a vector back into a compact string.

The formatting algorithm works as follows:

1. If the input slice is empty, return the string `"0"`.
2. Sort a copy of the indices in ascending order.
3. Walk through the sorted list, detecting runs of consecutive values.
4. For each run, emit `start-end` if the run spans more than one value, or just the single value otherwise.
5. Separate each segment with a comma.

### Examples

| Input | Output |
| --- | --- |
| `[]` | `"0"` |
| `[0, 1, 2, 3]` | `"0-3"` |
| `[0, 2, 4]` | `"0,2,4"` |
| `[0, 1, 2, 5, 6, 7]` | `"0-2,5-7"` |
| `[3, 1, 2, 0]` | `"0-3"` |
| `[0, 1, 2, 4, 8, 9, 10]` | `"0-2,4,8-10"` |

Note that the output uses commas as separators, whereas [parse_cpu_spec](parse_cpu_spec.md) accepts semicolons as separators. This is intentional — the comma-separated format is used for display and config output, while the semicolon-separated format is used for config input parsing.

This function is used by the logging and reporting subsystems to display CPU assignments in a readable way, and by [sort_and_group_config](sort_and_group_config.md) when generating grouped config output.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Line** | L134–L164 |
| **Visibility** | `pub` |
| **Related** | [parse_cpu_spec](parse_cpu_spec.md), [cpu_indices_to_mask](cpu_indices_to_mask.md), [mask_to_cpu_indices](mask_to_cpu_indices.md) |