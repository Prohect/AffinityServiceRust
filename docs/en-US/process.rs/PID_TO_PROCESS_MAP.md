# PID_TO_PROCESS_MAP static (process.rs)

Global shared PID-to-process lookup map that is populated by [`ProcessSnapshot::take`](ProcessSnapshot.md) during each snapshot capture. This static holds parsed [`ProcessEntry`](ProcessEntry.md) objects keyed by process ID, enabling efficient O(1) lookups of process information by PID.

## Syntax

```rust
pub static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));
```

## Type

`once_cell::sync::Lazy<std::sync::Mutex<HashMap<u32, ProcessEntry>>>`

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Key | `u32` | The process identifier (PID). |
| Value | [`ProcessEntry`](ProcessEntry.md) | The parsed process information including name, thread count, and raw `SYSTEM_PROCESS_INFORMATION` data. |

## Remarks

> **DO NOT DIRECTLY ACCESS** — Use the [`ProcessSnapshot`](ProcessSnapshot.md) struct instead.

This static is intended to be accessed exclusively through the [`ProcessSnapshot`](ProcessSnapshot.md) RAII wrapper, which manages its lifecycle (population and cleanup). Direct access bypasses the snapshot's safety guarantees and may result in reading stale or partially-populated data.

### Lifecycle

1. **Initialization.** The map is lazily initialized as an empty `HashMap` on first access via `once_cell::sync::Lazy`.
2. **Population.** [`ProcessSnapshot::take`](ProcessSnapshot.md) clears the map and re-populates it by parsing the raw `SYSTEM_PROCESS_INFORMATION` structures from the [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md). Each process is inserted with its PID as the key.
3. **Cleanup.** When the `ProcessSnapshot` is dropped, the map is cleared via `pid_to_process.clear()`, ensuring no stale entries persist between snapshot cycles.

### Thread safety

The map is protected by a `std::sync::Mutex`. Callers must acquire the lock before reading or writing. In practice, the lock is acquired once per scheduling cycle when taking a new snapshot, and the resulting `MutexGuard` is held for the duration of the snapshot's lifetime via the `ProcessSnapshot` struct's borrowed reference.

### Data validity

The [`ProcessEntry`](ProcessEntry.md) values stored in this map contain raw pointers (`threads_base_ptr`) that point into the [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md). These pointers are only valid for the lifetime of the snapshot buffer's current contents. After the `ProcessSnapshot` is dropped and the buffer is cleared, any retained `ProcessEntry` references would contain dangling pointers. The RAII design of `ProcessSnapshot` prevents this by clearing both the map and the buffer on drop.

### HashMap type

The `HashMap` used here is the project's custom alias for `FxHashMap` from the [`collections`](../collections.rs/README.md) module, which uses a fast non-cryptographic hash function (Fx hash) optimized for integer keys like PIDs.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `process.rs` |
| **Type** | `Lazy<Mutex<HashMap<u32, ProcessEntry>>>` |
| **Visibility** | `pub` (but should only be accessed via [`ProcessSnapshot`](ProcessSnapshot.md)) |
| **Populated by** | [`ProcessSnapshot::take`](ProcessSnapshot.md) |
| **Cleared by** | `ProcessSnapshot::drop` |
| **Dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex`, [`HashMap`](../collections.rs/HashMap.md), [`ProcessEntry`](ProcessEntry.md) |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| ProcessSnapshot struct | [ProcessSnapshot](ProcessSnapshot.md) |
| ProcessEntry struct | [ProcessEntry](ProcessEntry.md) |
| SNAPSHOT_BUFFER static | [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) |
| collections module | [collections.rs](../collections.rs/README.md) |
| HashMap type alias | [HashMap](../collections.rs/HashMap.md) |
| process module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
