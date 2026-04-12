# filter_indices_by_mask function (winapi.rs)

Filters a list of logical processor indices to only those whose corresponding bit is set in the given affinity mask.

## Syntax

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
```

## Parameters

`cpu_indices`

A slice of zero-based logical processor indices to filter.

`affinity_mask`

A bitmask where each set bit represents an allowed logical processor. Bit position `n` corresponds to logical processor index `n`.

## Return value

Returns a `Vec<u32>` containing only those indices from `cpu_indices` whose corresponding bit is set in `affinity_mask`. The order of elements is preserved from the input slice.

## Remarks

This function performs a simple bitwise intersection: for each index in `cpu_indices`, it checks whether `affinity_mask & (1 << index)` is non-zero, and includes the index in the result only if it is.

This is used when a process has both a configured CPU list and an existing affinity mask constraint. For example, if a configuration specifies CPUs `[0, 2, 4, 6]` but the process already has an affinity mask of `0b00110101` (CPUs 0, 2, 4, 5), the function returns `[0, 2, 4]` — the intersection of the configured set with the currently allowed set.

The function does not modify the global [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) or perform any Windows API calls. It is a pure computational helper.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L428–L435 |
| **Called by** | [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md) |

## See also

- [cpusetids_from_indices](cpusetids_from_indices.md)
- [cpusetids_from_mask](cpusetids_from_mask.md)
- [mask_from_cpusetids](mask_from_cpusetids.md)
- [winapi.rs module overview](README.md)