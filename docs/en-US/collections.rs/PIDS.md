# PIDS constant (collections.rs)

Inline capacity constant for `SmallVec` arrays that hold process IDs. This value determines how many process IDs can be stored on the stack before the `SmallVec` spills to a heap allocation.

## Syntax

```rust
pub const PIDS: usize = 256;
```

## Value

`256`

## Remarks

- This constant is used as the inline capacity parameter for `List<[T; PIDS]>` (i.e., `SmallVec<[T; PIDS]>`) throughout the application wherever collections of process IDs are needed.

- The value of `256` is chosen to accommodate the typical number of matching processes on a desktop or workstation system without triggering a heap allocation. On systems with more than 256 matching PIDs, the `SmallVec` transparently spills to the heap with no behavioral change — only a performance cost for the additional allocation.

- Because `SmallVec` stores its inline elements directly within the struct, a `List<[u32; PIDS]>` occupies `256 * 4 = 1024` bytes on the stack. Callers should be mindful of stack frame sizes when using this capacity in deeply recursive functions or when multiple such collections are active simultaneously.

- This constant is defined at the module level and is available to all modules that import from `collections`.

### Comparison with other capacity constants

| Constant | Value | Typical use |
|----------|-------|-------------|
| **PIDS** | `256` | Process ID collections |
| [TIDS_FULL](TIDS_FULL.md) | `96` | Full thread ID sets per process |
| [TIDS_CAPED](TIDS_CAPED.md) | `32` | Capped (limited) thread ID sets |
| [CONSUMER_CPUS](CONSUMER_CPUS.md) | `32` | CPU set ID and CPU index arrays |
| [PENDING](PENDING.md) | `16` | Pending operation entries |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Type** | `usize` |
| **Used by** | `apply.rs`, `scheduler.rs` — for PID collection buffers |
| **Platform** | Platform-independent |

## See Also

| Topic | Link |
|-------|------|
| TIDS_FULL constant | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED constant | [TIDS_CAPED](TIDS_CAPED.md) |
| CONSUMER_CPUS constant | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING constant | [PENDING](PENDING.md) |
| List type alias | [List](List.md) |
| collections module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
