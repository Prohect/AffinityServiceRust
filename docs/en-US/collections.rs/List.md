# List type alias (collections.rs)

Type alias for `SmallVec<E>` from the `smallvec` crate. `List` is a drop-in replacement for `Vec` that stores a fixed number of elements inline (on the stack) before spilling to a heap allocation. This hybrid strategy eliminates heap allocation overhead for small, common-case collections while retaining the flexibility of a growable vector for larger workloads.

## Syntax

```rust
pub type List<E> = SmallVec<E>;
```

## Parameters

| Type Parameter | Description |
|----------------|-------------|
| `E` | The array type that defines both the element type and the inline capacity. For example, `[u32; 32]` means the `List` stores `u32` elements with an inline capacity of 32. |

## Remarks

- `SmallVec<E>` is parameterized by an **array type**, not a scalar element type and a separate capacity constant. The array's length determines the inline capacity, and the array's element type determines the stored element type. For example, `List<[u32; CONSUMER_CPUS]>` stores up to `CONSUMER_CPUS` (32) `u32` values on the stack.

- When the number of elements exceeds the inline capacity, `SmallVec` transparently allocates heap memory and moves the elements, behaving identically to a `Vec`. The transition is invisible to callers — the API is a superset of `Vec`'s API.

- The `list!` macro (re-exported as `pub use smallvec::smallvec as list`) provides a convenient `vec!`-like syntax for constructing `List` instances with initial values:

  ```text
  let cpus: List<[u32; CONSUMER_CPUS]> = list![0, 1, 2, 3];
  ```

- Throughout AffinityServiceRust, `List` is used with the following inline capacity constants defined in the same module:

  | Usage | Array Type | Inline Capacity |
  |-------|-----------|-----------------|
  | Process ID collections | `[u32; PIDS]` | 512 |
  | Full thread ID sets | `[u32; TIDS_FULL]` | 128 |
  | Capped thread ID sets | `[u32; TIDS_CAPED]` | 64 |
  | CPU set IDs / indices | `[u32; CONSUMER_CPUS]` | 32 |
  | Pending operation entries | Various | 16 (`PENDING`) |

- The inline capacity values are tuned to cover the common case without heap allocation. For the vast majority of consumer and server systems, the number of CPU cores fits within 32, thread counts per process fit within 128, and monitored processes fit within 512.

### Performance characteristics

| Operation | Small (≤ inline capacity) | Large (> inline capacity) |
|-----------|--------------------------|--------------------------|
| Push | O(1) amortized, no alloc | O(1) amortized, heap alloc on spill |
| Index access | O(1), stack-local | O(1), heap pointer dereference |
| Iteration | Cache-friendly (contiguous stack memory) | Cache-friendly (contiguous heap memory) |
| Drop | No deallocation needed | Heap deallocation |

### Relationship to `Vec`

`SmallVec` implements `Deref<Target = [T]>`, so it can be used anywhere a slice `&[T]` is expected. It also implements most of the same traits as `Vec`, including `IntoIterator`, `Extend`, `FromIterator`, `Index`, `Clone`, `Debug`, and `PartialEq`.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Underlying type** | `smallvec::SmallVec<E>` |
| **Crate dependency** | [`smallvec`](https://crates.io/crates/smallvec) |
| **Platform** | Cross-platform |

## See Also

| Topic | Link |
|-------|------|
| HashMap type alias | [HashMap](HashMap.md) |
| HashSet type alias | [HashSet](HashSet.md) |
| PIDS constant | [PIDS](PIDS.md) |
| TIDS_FULL constant | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED constant | [TIDS_CAPED](TIDS_CAPED.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING constant | [PENDING](PENDING.md) |
| collections module overview | [README](README.md) |
| winapi module (primary consumer) | [winapi.rs](../winapi.rs/README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
