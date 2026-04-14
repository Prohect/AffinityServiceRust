# format_cpu_indices function (config.rs)

Formats a slice of CPU indices into a compact, human-readable range string. Consecutive indices are collapsed into `start-end` ranges and non-consecutive indices are separated by commas. This function is used throughout the service for log messages, config file generation, and diagnostic output.

## Syntax

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpus` | `&[u32]` | Slice of CPU indices to format. Does not need to be sorted or deduplicated — the function creates a sorted copy internally. May be empty. |

## Return value

Returns a `String` containing the formatted CPU index representation.

| Input | Output |
|-------|--------|
| `&[]` | `"0"` |
| `&[0, 1, 2, 3]` | `"0-3"` |
| `&[0, 2, 4]` | `"0,2,4"` |
| `&[0, 1, 2, 5, 6, 7, 12]` | `"0-2,5-7,12"` |
| `&[3]` | `"3"` |

An empty slice produces the string `"0"`, which in the configuration file format represents "no CPUs" / "no change".

## Remarks

### Algorithm

1. If the input slice is empty, return `"0"` immediately.
2. Create a sorted copy of the input.
3. Walk the sorted list, extending the current range while indices are consecutive (`sorted[i+1] == sorted[i] + 1`).
4. When a gap is found, emit the accumulated range:
   - Single index → `"N"`
   - Two or more consecutive indices → `"start-end"`
5. Separate ranges with commas (no spaces).

### Inverse relationship with parse_cpu_spec

`format_cpu_indices` is the display counterpart of [parse_cpu_spec](parse_cpu_spec.md). The round-trip `format_cpu_indices(parse_cpu_spec(s))` produces a normalized representation of the original spec string, with duplicates removed and indices sorted. Note that hex-mask inputs (e.g., `"0xFF"`) are normalized to range notation (e.g., `"0-7"`).

### Empty-means-zero convention

The config file format uses `"0"` to mean "no CPU specification" (i.e., do not change affinity/cpuset). Returning `"0"` for an empty slice preserves this convention so that generated config files are syntactically valid and semantically correct when re-parsed.

### Output in generated files

This function is called by [convert](convert.md) and [sort_and_group_config](sort_and_group_config.md) when writing CPU specifications into output config files. The compact range notation keeps generated files concise, especially for systems with many cores (e.g., `"0-63,128-191"` instead of listing 128 individual indices).

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [convert](convert.md), [sort_and_group_config](sort_and_group_config.md), apply module log formatting |
| **Dependencies** | None (pure function) |

## See Also

| Topic | Link |
|-------|------|
| CPU spec string parser (inverse) | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU indices to bitmask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| Bitmask to CPU indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd