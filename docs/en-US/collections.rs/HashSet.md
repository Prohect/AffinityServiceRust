# HashSet type alias (collections.rs)

Type alias for `FxHashSet<V>` from the `rustc_hash` crate, providing a high-performance hash set that uses the Fx (Firefox) non-cryptographic hash function. This alias is used throughout AffinityServiceRust wherever a `HashSet` is needed, ensuring consistent use of the faster hash implementation instead of the standard library's `SipHash`-based `HashSet`.

## Syntax

```rust
pub type HashSet<V> = FxHashSet<V>;
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `V` | Generic | The type of value stored in the set. Must implement `Eq` and `Hash`. |

## Remarks

- `FxHashSet` is a type alias in the `rustc_hash` crate defined as `HashSet<V, FxBuildHasher>` — it is a standard `HashSet` with a custom hasher. All standard `HashSet` methods (`insert`, `contains`, `remove`, `iter`, etc.) are available.

- The Fx hash function is **not cryptographically secure**. It is optimized for speed on small keys (integers, short strings) rather than resistance to hash-flooding attacks. This is appropriate for AffinityServiceRust because the data being hashed (process names, PIDs, operation keys) is not adversarially controlled.

- Performance characteristics:
  - Integer keys (`u32`, `usize`): Fx hash is significantly faster than `SipHash` because it avoids the multi-round mixing that `SipHash` performs.
  - String keys: Fx hash is faster for short strings (< ~64 bytes) and comparable for longer strings.
  - Memory layout is identical to `std::collections::HashSet` — only the hash function differs.

- The alias is defined in the [`collections`](../collections.rs/README.md) module and re-exported for use across the entire project. Callers use `HashSet<V>` without needing to import `rustc_hash` directly.

### Usage in the project

| Module | Usage |
|--------|-------|
| `logging.rs` | [`FINDS_SET`](../logging.rs/statics.md#finds_set) — deduplication of discovered process names. |
| `logging.rs` | [`FINDS_FAIL_SET`](../logging.rs/statics.md#finds_fail_set) — tracking access-denied process names in find mode. |

### Comparison with standard library

| Property | `std::collections::HashSet` | `collections::HashSet` (this alias) |
|----------|-----------------------------|--------------------------------------|
| Hash function | `SipHash-1-3` | `FxHash` |
| DoS-resistant | Yes | No |
| Speed (integer keys) | Moderate | Fast |
| Speed (string keys) | Moderate | Fast (short strings) |
| Crate | `std` | `rustc_hash` |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Crate** | `rustc_hash` (`FxHashSet`) |
| **Traits required on `V`** | `Eq + Hash` |
| **Platform** | Cross-platform |

## See Also

| Topic | Link |
|-------|------|
| HashMap type alias | [HashMap](HashMap.md) |
| List type alias | [List](List.md) |
| collections module overview | [README](README.md) |
| logging statics (FINDS_SET, FINDS_FAIL_SET) | [statics](../logging.rs/statics.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
