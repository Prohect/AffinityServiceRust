# TIDS_CAPED constant (collections.rs)

Defines the inline capacity for `SmallVec` arrays that hold capped (limited) thread ID sets. This constant is used as the array size parameter in `List<[T; TIDS_CAPED]>` type annotations throughout the project, allowing up to 32 thread IDs to be stored on the stack before the vector spills to a heap allocation.

## Syntax

```rust
pub const TIDS_CAPED: usize = 32;
```

## Value

`32`

## Remarks

- This constant is used as the inline capacity for `SmallVec` (`List`) arrays that store subsets of thread IDs — typically when the application imposes a cap on the number of threads it processes for a given rule or operation. The name "CAPED" (rather than "CAPPED") reflects the source code's naming convention.

- The value of `32` is smaller than [`TIDS_FULL`](TIDS_FULL.md) (`96`), reflecting the expectation that capped thread sets are smaller than full thread enumerations. When fewer than 32 thread IDs are stored, the `SmallVec` avoids heap allocation entirely, keeping the data inline within the array on the stack.

- If more than 32 entries are pushed into a `List<[T; TIDS_CAPED]>`, the `SmallVec` transparently spills to a heap-allocated `Vec`, so correctness is preserved regardless of actual thread count.

- This constant is defined alongside the related capacity constants [`PIDS`](PIDS.md), [`TIDS_FULL`](TIDS_FULL.md), [`CONSUMER_CPUS`](CONSUMER_CPUS.md), and [`PENDING`](PENDING.md), all of which tune the inline storage capacity for different categories of IDs and operations.

### Comparison of capacity constants

| Constant | Value | Typical use |
|----------|-------|-------------|
| [`PIDS`](PIDS.md) | `256` | Process ID arrays |
| [`TIDS_FULL`](TIDS_FULL.md) | `96` | Full (uncapped) thread ID arrays |
| **TIDS_CAPED** | `32` | Capped/limited thread ID arrays |
| [`CONSUMER_CPUS`](CONSUMER_CPUS.md) | `32` | CPU set ID / CPU index arrays |
| [`PENDING`](PENDING.md) | `16` | Pending operation entries |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Type** | `pub const usize` |
| **Used by** | `apply.rs`, `scheduler.rs` — thread processing with capped iteration limits |
| **Dependencies** | None |
| **Platform** | Platform-independent |

## See Also

| Topic | Link |
|-------|------|
| TIDS_FULL constant | [TIDS_FULL](TIDS_FULL.md) |
| PIDS constant | [PIDS](PIDS.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING constant | [PENDING](PENDING.md) |
| List type alias | [List](List.md) |
| collections module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
