# filter_indices_by_mask function (winapi.rs)

Filters a slice of logical CPU indices to only those that are permitted by a given affinity bitmask. This is used to ensure that CPU set assignments do not include processors that fall outside the process's current affinity mask.

## Syntax

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | A slice of zero-based logical processor indices to filter. |
| `affinity_mask` | `usize` | A bitmask where each set bit represents an allowed logical processor (bit 0 = CPU 0, bit 1 = CPU 1, etc.). |

## Return value

Returns a `List<[u32; CONSUMER_CPUS]>` (`SmallVec` with inline capacity of 32) containing only those indices from `cpu_indices` whose corresponding bit is set in `affinity_mask`. The order of the input indices is preserved.

## Remarks

- The function performs a straightforward bitwise check: for each index in `cpu_indices`, it verifies that the index is less than 64 and that `(1usize << idx) & affinity_mask` is nonzero. Indices ≥ 64 are silently excluded because `usize` on 64-bit Windows is 64 bits wide.

- This function operates purely on indices and bitmasks — it does **not** consult the [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) cache or call any Windows API. It is a CPU-local computation with no locking or side effects.

- The returned `SmallVec` uses an inline capacity of `CONSUMER_CPUS` (32). If the filtered result exceeds 32 entries, the vector spills to a heap allocation.

- Typical use case: when a configuration rule specifies desired CPU indices but the target process has a restricted affinity mask (e.g., set by a parent process or job object), this function intersects the two sets so that only valid CPUs are assigned.

### Algorithm

1. Iterate over each `idx` in `cpu_indices`.
2. For each `idx`, check:
   - `idx < 64` (fits within the bitmask width), **and**
   - `(1usize << idx) & affinity_mask != 0` (the corresponding bit is set).
3. Collect passing indices into the result list via `Iterator::filter` and `Iterator::copied`.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` — used during rule application to restrict CPU indices to the process's allowed affinity mask before converting to CPU Set IDs. |
| **Callees** | None (pure computation). |
| **Types** | `List<[u32; CONSUMER_CPUS]>` from [collections](../collections.rs/README.md) |
| **Platform** | Platform-independent logic, but only meaningful on Windows where affinity masks are used. |

## See Also

| Topic | Link |
|-------|------|
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| is_affinity_unset | [is_affinity_unset](is_affinity_unset.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](../collections.rs/CONSUMER_CPUS.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
