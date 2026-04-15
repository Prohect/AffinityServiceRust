# CONSUMER_CPUS constant (collections.rs)

Specifies the inline capacity for `SmallVec` arrays holding CPU set IDs or CPU indices. This constant is used as the type-level array size parameter in `List<[u32; CONSUMER_CPUS]>` throughout the CPU set conversion and filtering functions in the [`winapi`](../winapi.rs/README.md) module.

## Syntax

```rust
pub const CONSUMER_CPUS: usize = 32;
```

## Value

`32`

## Remarks

- This constant defines the number of elements that a `SmallVec<[u32; CONSUMER_CPUS]>` (aliased as [`List`](List.md)) can store inline on the stack before spilling to a heap allocation. With a value of 32, each inline `List<[u32; CONSUMER_CPUS]>` occupies `32 × 4 = 128` bytes on the stack (plus `SmallVec` overhead).

- The value of 32 is chosen to accommodate the typical number of logical processors on consumer-grade CPUs (e.g., 16-core / 32-thread processors). For systems with more than 32 logical processors, the `SmallVec` transparently spills to the heap with no correctness impact — only a minor performance cost from the allocation.

- This constant is used as the inline capacity in the return types of the following functions in [`winapi.rs`](../winapi.rs/README.md):
  - [`cpusetids_from_indices`](../winapi.rs/cpusetids_from_indices.md)
  - [`cpusetids_from_mask`](../winapi.rs/cpusetids_from_mask.md)
  - [`indices_from_cpusetids`](../winapi.rs/indices_from_cpusetids.md)
  - [`mask_from_cpusetids`](../winapi.rs/mask_from_cpusetids.md)
  - [`filter_indices_by_mask`](../winapi.rs/filter_indices_by_mask.md)

- Changing this value affects the stack footprint of every `List<[u32; CONSUMER_CPUS]>` variable and return value. Increasing it reduces heap allocations on high-core-count systems; decreasing it reduces stack usage for applications targeting systems with fewer cores.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Type** | `usize` |
| **Used by** | [`winapi.rs`](../winapi.rs/README.md) CPU set conversion functions, `apply.rs` |
| **Dependencies** | None |
| **Platform** | Platform-independent |

## See Also

| Topic | Link |
|-------|------|
| List type alias | [List](List.md) |
| PIDS constant | [PIDS](PIDS.md) |
| TIDS_FULL constant | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED constant | [TIDS_CAPED](TIDS_CAPED.md) |
| PENDING constant | [PENDING](PENDING.md) |
| cpusetids_from_indices | [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md) |
| collections module overview | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
