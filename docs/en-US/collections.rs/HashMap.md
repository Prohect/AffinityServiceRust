# HashMap type alias (collections.rs)

Type alias for `FxHashMap<K, V>` from the `rustc_hash` crate. This provides a drop-in replacement for the standard library's `HashMap` that uses the Fx (Firefox) non-cryptographic hash function, optimized for speed over hash-flooding resistance. It is used throughout AffinityServiceRust for all hash map needs, including PID-to-process lookups, failure tracking maps, and module caches.

## Syntax

```rust
pub type HashMap<K, V> = FxHashMap<K, V>;
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `K` | generic | The key type. Must implement `Eq` and `Hash`. |
| `V` | generic | The value type. |

## Remarks

- `FxHashMap` uses the Fx hash algorithm, originally developed for the Firefox browser and adopted by the Rust compiler (`rustc`) for internal use. It is significantly faster than the default `SipHash` used by `std::collections::HashMap` for small, integer-like keys (such as `u32` PIDs and TIDs), at the cost of not providing protection against hash-flooding denial-of-service attacks.

- Because AffinityServiceRust operates on trusted, locally-generated data (process IDs, thread IDs, CPU set IDs), the lack of hash-flooding resistance is not a security concern. The performance benefit is meaningful in the hot paths of the scheduling loop where maps are created, populated, and queried on every cycle.

- This alias allows the entire project to switch hash map implementations by changing a single line in `collections.rs`, without modifying any call sites.

- The alias is re-exported at module level, so consumers import it as `crate::collections::HashMap`.

- The `HashMap::default()` call (used throughout the codebase) creates an empty map with the default hasher, equivalent to `FxHashMap::default()`.

### Common uses in the project

| Module | Key type | Value type | Purpose |
|--------|----------|------------|---------|
| `process.rs` | `u32` (PID) | `ProcessEntry` | PID-to-process lookup map in `PID_TO_PROCESS_MAP` |
| `process.rs` | `u32` (TID) | `SYSTEM_THREAD_INFORMATION` | Thread map returned by `ProcessEntry::get_threads()` |
| `winapi.rs` | `u32` (PID) | `Vec<(usize, usize, String)>` | `MODULE_CACHE` per-process module list |
| `logging.rs` | `u32` (PID) | `HashMap<ApplyFailEntry, bool>` | `PID_MAP_FAIL_ENTRY_SET` failure tracking |
| `logging.rs` | `ApplyFailEntry` | `bool` | Inner failure-entry map with alive flags |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `collections.rs` |
| **Crate dependency** | `rustc_hash` (provides `FxHashMap`) |
| **Standard library equivalent** | `std::collections::HashMap` |
| **Platform** | Cross-platform |

## See Also

| Topic | Link |
|-------|------|
| HashSet type alias | [HashSet](HashSet.md) |
| List type alias | [List](List.md) |
| collections module overview | [README](README.md) |
| process module (primary consumer) | [process.rs](../process.rs/README.md) |
| logging module (failure tracking) | [logging.rs](../logging.rs/README.md) |
| winapi module (module cache) | [winapi.rs](../winapi.rs/README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
