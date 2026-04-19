# filter_indices_by_mask function (winapi.rs)

Filters a list of logical CPU indices to only those whose corresponding bit is set in the given affinity mask. Used to intersect a configured CPU index list with the actual process affinity, ensuring that prime thread pinning and CPU set operations only target processors the process is allowed to use.

## Syntax

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | Slice of logical CPU indices (0-based) to filter. Each value represents a logical processor number (e.g., `0`, `1`, `5`, `12`). |
| `affinity_mask` | `usize` | A bitmask where bit *N* being set indicates that logical processor *N* is permitted. Typically obtained from `GetProcessAffinityMask` or constructed from configuration. |

## Return value

`List<[u32; CONSUMER_CPUS]>` — A stack-allocated list containing only those indices from `cpu_indices` whose corresponding bit is set in `affinity_mask`. The order of elements matches the input order. Returns an empty list if no indices pass the filter.

## Remarks

The filtering condition for each index is:

```
idx < 64 && ((1usize << idx) & affinity_mask) != 0
```

- **64-bit limit:** Indices ≥ 64 are silently excluded because a `usize` on 64-bit Windows can only represent processors 0–63 via bit positions. Systems with more than 64 logical processors use processor groups, which are handled separately by the CPU Sets API.
- **No deduplication:** If the input `cpu_indices` contains duplicate values, the output will also contain duplicates for any that pass the filter.
- **No sorting:** The output preserves the original input ordering. If sorted output is needed, the caller must sort the result separately.
- The function uses iterator chaining (`filter` + `copied` + `collect`) and performs no heap allocation — the result is collected into a stack-backed `List<[u32; CONSUMER_CPUS]>`.

### Typical usage

This function is called by [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) to ensure that prime thread CPU indices are restricted to only those processors within the process's current affinity mask. For example, if the configuration specifies prime CPUs `[4, 5, 6, 7]` but the process has an affinity mask of `0x3F` (CPUs 0–5), the function returns `[4, 5]`.

### Example

| `cpu_indices` | `affinity_mask` | Result |
|---------------|-----------------|--------|
| `[0, 2, 4]` | `0x15` (bits 0, 2, 4) | `[0, 2, 4]` |
| `[0, 2, 4]` | `0x05` (bits 0, 2) | `[0, 2]` |
| `[0, 1, 2]` | `0x00` | `[]` (empty) |
| `[64, 65]` | `0xFFFFFFFFFFFFFFFF` | `[]` (indices ≥ 64 excluded) |

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), other apply functions that need to intersect configured indices with process affinity |
| **Callees** | None (pure computation) |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Index → CPU Set ID conversion | [cpusetids_from_indices](cpusetids_from_indices.md) |
| Mask → CPU Set ID conversion | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set ID → index conversion | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU Set ID → mask conversion | [mask_from_cpusetids](mask_from_cpusetids.md) |
| Prime thread promotion | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*