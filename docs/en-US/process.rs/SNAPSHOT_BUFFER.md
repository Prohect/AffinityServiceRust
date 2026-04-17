# SNAPSHOT_BUFFER static (process.rs)

Shared backing buffer for `NtQuerySystemInformation` results. This static provides the raw byte storage that [`ProcessSnapshot::take`](ProcessSnapshot.md) fills with system process information data. It is lazily initialized with a small initial capacity and dynamically resized as needed during snapshot capture.

## Syntax

```rust
pub static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
```

## Type

`once_cell::sync::Lazy<std::sync::Mutex<Vec<u8>>>`

## Remarks

> **Warning:** Do not access this static directly. Use the [`ProcessSnapshot`](ProcessSnapshot.md) struct instead, which manages the buffer's lifecycle and ensures correctness.

### Initial capacity

The buffer is initialized with a capacity of 32 bytes — intentionally small. On the first call to [`ProcessSnapshot::take`](ProcessSnapshot.md), `NtQuerySystemInformation` will return `STATUS_INFO_LENGTH_MISMATCH`, prompting the snapshot logic to reallocate the buffer to the size reported by the API's `return_length` output parameter (rounded up to an 8-byte boundary). After the first successful snapshot, the buffer retains its enlarged capacity for subsequent calls, avoiding repeated reallocations.

### Lifecycle

1. **First access** — `Lazy` initializes the mutex with a 32-byte vector.
2. **Snapshot capture** — [`ProcessSnapshot::take`](ProcessSnapshot.md) locks the mutex, uses the buffer to call `NtQuerySystemInformation`, and may resize it if the current capacity is insufficient.
3. **Snapshot drop** — When the `ProcessSnapshot` is dropped, the buffer is cleared (`Vec::clear()`), releasing the data but retaining the allocated capacity for the next snapshot cycle.
4. **Process exit** — The buffer is never explicitly deallocated; it lives for the duration of the process as a `'static` resource.

### Thread safety

The buffer is protected by a `Mutex`. Only one thread can hold the lock at a time, ensuring that concurrent snapshot attempts are serialized. In practice, snapshots are taken from the main scheduling loop on a single thread, so contention is not expected.

### Relationship to PID_TO_PROCESS_MAP

This buffer and [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) are always used together. The `ProcessSnapshot::take` method requires mutable references to both, and the `ProcessSnapshot` struct holds references to both for the duration of its lifetime. The raw pointers inside `SYSTEM_PROCESS_INFORMATION` structures (stored in process entries) point into this buffer, so the buffer **must not** be modified or deallocated while any `ProcessEntry` references are live.

### Why a global static?

The buffer is stored as a global static rather than a local variable to enable buffer reuse across scheduling loop iterations. By retaining the allocated capacity between snapshots, the application avoids repeated large allocations (typically 1–4 MB) on every cycle.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `process.rs` |
| **Visibility** | `pub` (but should not be accessed directly; use `ProcessSnapshot`) |
| **Used by** | [`ProcessSnapshot::take`](ProcessSnapshot.md) |
| **Dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex` |
| **Win32 API** | `NtQuerySystemInformation` (via `ntapi` crate, called by `ProcessSnapshot::take`) |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| ProcessSnapshot struct | [ProcessSnapshot](ProcessSnapshot.md) |
| PID_TO_PROCESS_MAP static | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| ProcessEntry struct | [ProcessEntry](ProcessEntry.md) |
| process module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
