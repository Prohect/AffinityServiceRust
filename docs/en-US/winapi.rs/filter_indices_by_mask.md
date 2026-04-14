# filter_indices_by_mask function (winapi.rs)

Filters a slice of logical CPU indices, retaining only those whose corresponding bit is set in the given affinity mask. This is used to intersect a user-specified CPU list with the effective affinity mask of a process, ensuring that only reachable processors are targeted for CPU set or ideal processor assignment.

## Syntax

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | Slice of zero-based logical processor indices to filter (e.g., `[0, 1, 4, 5, 6, 7]`). Values ≥ 64 are silently excluded because they cannot be represented in a `usize` bitmask on 64-bit platforms. |
| `affinity_mask` | `usize` | Bitmask where bit *N* being set means logical processor *N* is allowed. Typically obtained from `GetProcessAffinityMask` or constructed from a configuration CPU spec via [`cpu_indices_to_mask`](../config.rs/cpu_indices_to_mask.md). |

## Return value

A `Vec<u32>` containing the subset of `cpu_indices` whose corresponding bit is set in `affinity_mask`. The order of elements is preserved from the input slice.

**Examples:**

| `cpu_indices` | `affinity_mask` (binary) | Result |
|---------------|--------------------------|--------|
| `[0, 1, 2, 3]` | `0b0101` (5) | `[0, 2]` |
| `[4, 5, 6, 7]` | `0b1111_0000` (0xF0) | `[4, 5, 6, 7]` |
| `[0, 1, 2]` | `0` | `[]` |
| `[]` | `0xFF` | `[]` |
| `[64, 65]` | `usize::MAX` | `[]` |

## Remarks

The function uses iterator combinators (`filter` + `copied` + `collect`) for a concise, allocation-efficient implementation. Each index is tested with the expression `idx < 64 && ((1usize << idx) & affinity_mask) != 0`, which performs a bounds check before the shift to avoid undefined-width shifts.

### Relationship to CPU set functions

This function operates at the **affinity mask** level, not the CPU Set ID level. It is complementary to the CPU set translation functions:

- [cpusetids_from_indices](cpusetids_from_indices.md) translates indices to CPU Set IDs.
- [cpusetids_from_mask](cpusetids_from_mask.md) translates an affinity mask to CPU Set IDs.
- `filter_indices_by_mask` narrows an index list by an affinity mask **before** translation.

### 64-processor limit

The `idx < 64` guard reflects the single-group affinity mask model used by `GetProcessAffinityMask`. On systems with more than 64 logical processors (multiple processor groups), indices ≥ 64 cannot be represented in a single `usize` bitmask and are filtered out. Multi-group affinity is handled separately through CPU Set APIs, which use opaque IDs rather than bitmask positions.

### Use in the apply module

The [apply_affinity](../apply.rs/apply_affinity.md) and [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) functions call `filter_indices_by_mask` to intersect the configured CPU list with the process's current affinity mask, ensuring ideal processors are only assigned to CPUs the process can actually run on.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [apply_affinity](../apply.rs/apply_affinity.md), [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Dependencies** | None (pure computation) |

## See Also

| Topic | Link |
|-------|------|
| CPU indices → CPU Set IDs | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Affinity mask → CPU Set IDs | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set IDs → CPU indices | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU Set IDs → affinity mask | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU indices to bitmask utility | [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) |
| Affinity application logic | [apply_affinity](../apply.rs/apply_affinity.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd