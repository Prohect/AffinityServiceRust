# TIDS_FULL constant (collections.rs)

Specifies the inline capacity for `SmallVec` arrays that hold full (uncapped) thread ID sets. This constant is used as the type-level array size parameter for `List<[T; TIDS_FULL]>` throughout the application, allowing up to 96 thread IDs to be stored on the stack before the vector spills to a heap allocation.

## Syntax

```rust
pub const TIDS_FULL: usize = 96;
```

## Value

`96`

## Remarks

- This constant determines the inline (stack-allocated) capacity of `SmallVec` arrays used to hold thread ID collections where no cap is applied. When the number of thread IDs in a collection exceeds 96, the `SmallVec` automatically spills to a heap-allocated `Vec`, so correctness is not affected — only performance characteristics change.

- The value of 96 is chosen to cover the vast majority of real-world processes. Most applications have fewer than 96 threads, so the stack-allocated fast path handles the common case without any heap allocation overhead.

- For scenarios where a smaller thread ID set is expected or where stack space is at a premium, the companion constant [`TIDS_CAPED`](TIDS_CAPED.md) (value `32`) provides a reduced inline capacity.

- The `SmallVec` type is re-exported as [`List`](List.md) from the [collections](README.md) module, and the `TIDS_FULL` constant is used as a generic array-size parameter: `List<[u32; TIDS_FULL]>`.

### Capacity constant family

| Constant | Value | Typical use |
|----------|-------|-------------|
| [`PIDS`](PIDS.md) | `256` | Process ID collections |
| **TIDS_FULL** | `96` | Full (uncapped) thread ID collections |
| [`TIDS_CAPED`](TIDS_CAPED.md) | `32` | Capped (limited) thread ID collections |
| [`CONSUMER_CPUS`](CONSUMER_CPUS.md) | `32` | CPU set ID and CPU index collections |
| [`PENDING`](PENDING.md) | `16` | Pending operation entry collections |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Type** | `usize` (compile-time constant) |
| **Used by** | `apply.rs`, `scheduler.rs` — thread enumeration and processing |
| **Platform** | Platform-independent |

## See Also

| Topic | Link |
|-------|------|
| TIDS_CAPED constant | [TIDS_CAPED](TIDS_CAPED.md) |
| PIDS constant | [PIDS](PIDS.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING constant | [PENDING](PENDING.md) |
| List type alias | [List](List.md) |
| collections module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
