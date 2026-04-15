# PENDING constant (collections.rs)

Inline capacity constant for `SmallVec` arrays that hold pending operation entries. The value `16` specifies the number of elements that can be stored on the stack before the `SmallVec` spills to a heap allocation.

## Syntax

```rust
pub const PENDING: usize = 16;
```

## Value

`16`

## Remarks

- This constant is used as the inline capacity parameter for `List<[T; PENDING]>` (i.e., `SmallVec<[T; PENDING]>`) arrays throughout the application where pending or queued items need to be collected temporarily.

- The value of `16` is chosen as a practical default for short-lived collections of pending work items — such as processes or threads awaiting rule application in a single scheduling cycle. Most pending queues in normal workloads are expected to contain fewer than 16 entries, allowing them to remain entirely stack-allocated.

- If more than 16 elements are pushed into a `SmallVec<[T; PENDING]>`, the vector transparently spills to the heap. This spill is invisible to callers but incurs a one-time allocation cost. The stack-to-heap transition is handled internally by the `smallvec` crate.

- This is the smallest of the five capacity constants defined in the [`collections`](README.md) module, reflecting the expectation that pending-operation buffers are typically short.

### Capacity constants comparison

| Constant | Value | Typical use |
|----------|-------|-------------|
| [`PIDS`](PIDS.md) | `512` | Process ID lists |
| [`TIDS_FULL`](TIDS_FULL.md) | `128` | Full thread ID sets |
| [`TIDS_CAPED`](TIDS_CAPED.md) | `64` | Capped thread ID sets |
| [`CONSUMER_CPUS`](CONSUMER_CPUS.md) | `32` | CPU set IDs / CPU indices |
| **PENDING** | **16** | **Pending operation entries** |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Type** | `usize` |
| **Used by** | Various modules for `SmallVec` inline capacity in pending-work buffers |
| **Dependencies** | None — compile-time constant |

## See Also

| Topic | Link |
|-------|------|
| PIDS constant | [PIDS](PIDS.md) |
| TIDS_FULL constant | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED constant | [TIDS_CAPED](TIDS_CAPED.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| List type alias | [List](List.md) |
| collections module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
