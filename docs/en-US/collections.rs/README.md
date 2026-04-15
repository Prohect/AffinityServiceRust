# collections module (AffinityServiceRust)

The `collections` module provides project-wide type aliases for high-performance collection types and capacity constants used for stack-allocated small vectors. It re-exports `FxHashMap` and `FxHashSet` from the `rustc_hash` crate (which use a fast, non-cryptographic hash) as `HashMap` and `HashSet`, and `SmallVec` from the `smallvec` crate as `List`. The module also defines a set of `usize` constants that specify inline capacities for `SmallVec` arrays throughout the application, tuned to typical workload sizes for process IDs, thread IDs, CPU sets, and pending operations.

## Type Aliases

| Type | Description |
|------|-------------|
| [HashMap](HashMap.md) | Alias for `FxHashMap<K, V>`, a hash map using the Fx (Firefox) non-cryptographic hash function for fast lookups. |
| [HashSet](HashSet.md) | Alias for `FxHashSet<V>`, a hash set using the Fx non-cryptographic hash function. |
| [List](List.md) | Alias for `SmallVec<E>`, a vector that stores elements inline up to a fixed capacity before spilling to the heap. |

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| [PIDS](PIDS.md) | `512` | Inline capacity for `SmallVec` arrays holding process IDs. |
| [TIDS_FULL](TIDS_FULL.md) | `128` | Inline capacity for `SmallVec` arrays holding full thread ID sets. |
| [TIDS_CAPED](TIDS_CAPED.md) | `64` | Inline capacity for `SmallVec` arrays holding capped (limited) thread ID sets. |
| [CONSUMER_CPUS](CONSUMER_CPUS.md) | `32` | Inline capacity for `SmallVec` arrays holding CPU set IDs or CPU indices. |
| [PENDING](PENDING.md) | `16` | Inline capacity for `SmallVec` arrays holding pending operation entries. |

## See Also

| Link | Description |
|------|-------------|
| [winapi.rs module](../winapi.rs/README.md) | Primary consumer of `List` and `CONSUMER_CPUS` for CPU set operations. |
| [process.rs module](../process.rs/README.md) | Uses `HashMap` for PID-to-process mapping. |
| [logging.rs module](../logging.rs/README.md) | Uses `HashMap` and `HashSet` for failure tracking and deduplication. |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
