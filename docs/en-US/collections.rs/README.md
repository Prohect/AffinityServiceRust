# collections module (AffinityServiceRust)

The `collections` module provides project-wide type aliases for high-performance collection types and capacity constants used for stack-allocated small vectors. It re-exports `FxHashMap` and `FxHashSet` from the `rustc_hash` crate (which use a fast, non-cryptographic hash) as `HashMap` and `HashSet`, and `SmallVec` from the `smallvec` crate as `List`. The module also defines a set of `usize` constants that specify inline capacities for `SmallVec` arrays throughout the application, tuned to typical workload sizes for process IDs, thread IDs, CPU sets, and pending operations.

## Type Aliases

| Type | Description |
|------|-------------|
| [HashMap](HashMap.md) | Alias for `FxHashMap<K, V>`, a hash map using the Fx (Firefox) non-cryptographic hash function for fast lookups. |
| [HashSet](HashSet.md) | Alias for `FxHashSet<V>`, a hash set using the Fx non-cryptographic hash function. |
| [List](List.md) | Alias for `SmallVec<E>`, a vector that stores elements inline up to a fixed capacity before spilling to the heap. |
| [list!](list.md) | Re-export of the `smallvec!` macro as `list!`, used to construct `List` instances with vec-like syntax. |

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| [PIDS](PIDS.md) | `256` | Inline capacity for `SmallVec` arrays holding process IDs. |
| [TIDS_FULL](TIDS_FULL.md) | `96` | Inline capacity for `SmallVec` arrays holding full thread ID sets. |
| [TIDS_CAPED](TIDS_CAPED.md) | `32` | Inline capacity for `SmallVec` arrays holding capped (limited) thread ID sets. |
| [CONSUMER_CPUS](CONSUMER_CPUS.md) | `32` | Inline capacity for `SmallVec` arrays holding CPU set IDs or CPU indices. |
| [PENDING](PENDING.md) | `16` | Inline capacity for `SmallVec` arrays holding pending operation entries. |

## See Also

| Link | Description |
|------|-------------|
| [winapi.rs module](../winapi.rs/README.md) | Primary consumer of `List` and `CONSUMER_CPUS` for CPU set operations. |
| [process.rs module](../process.rs/README.md) | Uses `HashMap` for PID-to-process mapping. |
| [logging.rs module](../logging.rs/README.md) | Uses `HashMap` and `HashSet` for failure tracking and deduplication. |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
