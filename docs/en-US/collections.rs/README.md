# collections module (AffinityServiceRust)

The `collections` module defines performance-oriented type aliases and pre-tuned capacity constants used throughout the project. By centralizing these definitions, every module shares the same high-performance hash map, hash set, and small-vector implementations without repeating crate-level imports.

## Type Aliases

| Name | Underlying Type | Description |
|------|----------------|-------------|
| `HashMap<K, V>` | `FxHashMap<K, V>` ([rustc-hash](https://crates.io/crates/rustc-hash)) | Drop-in replacement for `std::collections::HashMap` that uses the Fx hash algorithm. Significantly faster for small, integer, or pointer-sized keys common in process/thread ID lookups. Not cryptographically secure — acceptable because all keys originate from trusted local system data. |
| `HashSet<V>` | `FxHashSet<V>` ([rustc-hash](https://crates.io/crates/rustc-hash)) | Drop-in replacement for `std::collections::HashSet` backed by the same Fx hash algorithm. Used for deduplication of PIDs, TIDs, and error tracking sets. |
| `List<E>` | `SmallVec<E>` ([smallvec](https://crates.io/crates/smallvec)) | Stack-allocated vector that spills to the heap only when the element count exceeds the inline capacity encoded in the type parameter `E` (e.g., `[u32; 32]` stores up to 32 `u32` values on the stack). Eliminates heap allocation for the common case of small CPU index lists and thread ID vectors. |

## Macro Re-exports

| Name | Underlying Macro | Description |
|------|-----------------|-------------|
| `list!` | `smallvec::smallvec!` | Convenience macro for constructing `List<E>` instances with literal values, analogous to `vec![]`. |

## Constants

| Name | Type | Value | Description |
|------|------|-------|-------------|
| `PIDS` | `usize` | `256` | Inline capacity for process ID lists. Sized to cover the typical number of managed processes without heap allocation. Used as the type parameter in `List<[u32; PIDS]>`. |
| `TIDS_FULL` | `usize` | `96` | Inline capacity for full thread ID lists per process. Accommodates heavyweight applications (games, browsers) that may spawn many threads. |
| `TIDS_CAPED` | `usize` | `32` | Inline capacity for capped (filtered) thread ID lists. Used when only a subset of threads — such as prime thread candidates — needs to be tracked. |
| `CONSUMER_CPUS` | `usize` | `32` | Inline capacity for CPU index arrays in affinity, CPU set, and prime thread specifications. Covers consumer-grade processors up to 32 logical cores without heap allocation. |
| `PENDING` | `usize` | `16` | Inline capacity for pending operation queues such as deferred thread promotions or demotions within a single apply cycle. |

## Syntax

```rust
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

pub type HashMap<K, V> = FxHashMap<K, V>;
pub type HashSet<V> = FxHashSet<V>;
pub type List<E> = SmallVec<E>;
pub use smallvec::smallvec as list;

pub const PIDS: usize = 256;
pub const TIDS_FULL: usize = 96;
pub const TIDS_CAPED: usize = 32;
pub const CONSUMER_CPUS: usize = 32;
pub const PENDING: usize = 16;
```

## Remarks

- **Why `FxHashMap` over `std::HashMap`?** The default `std::HashMap` uses SipHash-1-3, which provides HashDoS resistance at the cost of throughput. AffinityServiceRust hashes only local, trusted data (PIDs, TIDs, process names) in a tight polling loop, so the faster Fx algorithm yields measurable latency savings with no security trade-off.
- **Why `SmallVec` over `Vec`?** Most affinity rules target fewer than 32 CPUs and most processes have fewer than 96 threads. `SmallVec` keeps these common cases entirely on the stack, avoiding allocator overhead in the hot path of the apply loop. The capacity constants above are tuned for consumer-grade hardware; workstation or server topologies with more than 32 logical processors will transparently spill to the heap.
- **Naming convention:** The constants use short, uppercase names (`PIDS`, `TIDS_FULL`) because they appear as generic array-size parameters in type signatures (e.g., `List<[u32; CONSUMER_CPUS]>`) and brevity improves readability at call sites.

## Requirements

| | |
|---|---|
| **Module** | `src/collections.rs` |
| **Crate dependencies** | [rustc-hash](https://crates.io/crates/rustc-hash), [smallvec](https://crates.io/crates/smallvec) |
| **Callers** | Every module in the project imports from `collections` |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Configuration parsing (primary consumer) | [config.rs](../config.rs/README.md) |
| Apply loop (hot-path consumer) | [apply.rs](../apply.rs/README.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Process snapshot data | [ProcessEntry](../process.rs/ProcessEntry.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*