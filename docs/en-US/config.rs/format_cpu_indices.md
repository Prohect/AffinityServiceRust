# format_cpu_indices function (config.rs)

Formats a slice of CPU indices into a compact, human-readable range string. Consecutive indices are collapsed into dash-separated ranges, and non-consecutive indices are comma-separated.

## Syntax

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## Parameters

`cpus: &[u32]`

A slice of CPU indices to format. The slice does not need to be pre-sorted; the function sorts a copy internally. Duplicate values are preserved in sorting but do not affect the output ranges.

## Return value

`String` — A compact representation of the CPU indices. Returns `"0"` if the input slice is empty. Otherwise, returns a comma-separated string where consecutive runs are collapsed into ranges.

**Examples of output:**

| Input | Output |
|-------|--------|
| `[]` | `"0"` |
| `[0, 1, 2, 3]` | `"0-3"` |
| `[0, 2, 4]` | `"0,2,4"` |
| `[0, 1, 2, 8, 9, 10, 16]` | `"0-2,8-10,16"` |
| `[5]` | `"5"` |

## Remarks

The function copies the input into a stack-allocated `List<[u32; CONSUMER_CPUS]>` (a `SmallVec`) and sorts it before formatting. This ensures correct range detection regardless of the input order.

### Range collapsing algorithm

The function iterates through the sorted list. For each starting index, it extends a contiguous run as long as the next value equals the current value plus one. When the run ends:

- If the start and end of the run are equal, the single value is appended.
- If the start and end differ, a `"start-end"` range string is appended.

Ranges and individual values are joined with commas.

### Empty-returns-zero convention

The string `"0"` is returned for an empty input because `0` in the AffinityServiceRust config format means "no change" or "not configured." This convention allows the output of `format_cpu_indices` to be directly inserted into config file lines without special-casing empty CPU sets.

### Relationship with parse_cpu_spec

`format_cpu_indices` is the inverse of [parse_cpu_spec](parse_cpu_spec.md) for range and individual formats. That is, `parse_cpu_spec(format_cpu_indices(cpus))` produces a sorted, deduplicated version of the original input. However, `format_cpu_indices` never emits hex-mask format output, even if the input was originally parsed from a hex string.

### Usage

This function is used throughout the codebase for:

- Generating human-readable log messages showing CPU assignments (affinity changes, CPU set changes, prime thread pins).
- Producing config file output in the [convert](convert.md) and [sort_and_group_config](sort_and_group_config.md) utilities.
- Debug display of CPU sets in diagnostic messages.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Callers** | `apply_affinity`, `apply_process_default_cpuset`, `apply_prime_threads_promote`, `apply_prime_threads_demote`, `reset_thread_ideal_processors`, `apply_ideal_processors`, logging functions |
| **Callees** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Inverse parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Bitmask to indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| Indices to bitmask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| Collection types | [List / CONSUMER_CPUS](../collections.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*