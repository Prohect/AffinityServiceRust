# ProcessEntry struct (process.rs)

Represents a single process from a system snapshot, wrapping the native `SYSTEM_PROCESS_INFORMATION` structure with a cached lowercase process name and lazy thread enumeration. `ProcessEntry` is the per-process data unit stored in the PID-keyed lookup map built by [`ProcessSnapshot::take`](ProcessSnapshot.md).

## Syntax

```rust
#[derive(Clone)]
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: usize,
    name: String,
}
```

## Members

| Field | Type | Visibility | Description |
|-------|------|------------|-------------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | `pub` | The raw NT process information structure as returned by `NtQuerySystemInformation`. Contains fields such as `UniqueProcessId`, `NumberOfThreads`, `ImageName`, and various resource counters. |
| `threads_base_ptr` | `usize` | Private | The base address (stored as `usize`) of the thread information array (`SYSTEM_THREAD_INFORMATION[]`) that immediately follows the process entry in the snapshot buffer. Stored as a numeric value rather than a raw pointer to satisfy `Clone` and `Send` requirements. |
| `name` | `String` | Private | The process image name in **lowercase**, decoded from the UTF-16 `ImageName` field during construction. Empty string for the System Idle Process (PID 0), which has a null `ImageName.Buffer`. |

## Methods

### `new`

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION,
) -> Self
```

Constructs a new `ProcessEntry` from a raw `SYSTEM_PROCESS_INFORMATION` structure and a pointer to its thread array. The process image name is decoded from `ImageName.Buffer` (UTF-16) into a lowercase `String` during construction. If `ImageName.Length` is zero or `ImageName.Buffer` is null, the name is set to an empty string.

### `get_threads`

```rust
pub fn get_threads(&self) -> HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

Builds and returns a `HashMap` mapping thread IDs (`u32`) to their corresponding `SYSTEM_THREAD_INFORMATION` structures. The raw thread array from `SYSTEM_PROCESS_INFORMATION` is parsed by iterating over `NumberOfThreads` entries starting at `threads_base_ptr`. Each thread's `ClientId.UniqueThread` is used as the map key.

Returns an empty map if the `threads_base_ptr` is null.

> **Note:** This method reconstructs the map on every call. It does not cache the result internally.

### `get_name`

```rust
pub fn get_name(&self) -> &str
```

Returns a reference to the cached lowercase process name. This is an `#[inline]` accessor with zero allocation overhead.

### `get_name_original_case`

```rust
pub fn get_name_original_case(&self) -> String
```

Re-reads the process image name from the raw `ImageName` UTF-16 buffer **without** lowercasing, returning the original-case name as a new `String`. This method performs an unsafe read of the `ImageName.Buffer` pointer, which is only valid while the parent [`ProcessSnapshot`](ProcessSnapshot.md) is alive and the backing buffer has not been cleared.

Marked `#[allow(dead_code)]` — reserved for diagnostic or display use cases where the original casing matters.

### `pid`

```rust
pub fn pid(&self) -> u32
```

Returns the process identifier extracted from `process.UniqueProcessId`, cast through `usize` to `u32`. This is an `#[inline]` accessor.

### `thread_count`

```rust
pub fn thread_count(&self) -> u32
```

Returns the number of threads in the process from `process.NumberOfThreads`. This is an `#[inline]` accessor.

## Remarks

### Send safety

`ProcessEntry` has an explicit `unsafe impl Send for ProcessEntry` declaration. This is safe under the following invariants:

- `ProcessEntry` instances are only accessed through `Mutex`-protected containers (`PID_TO_PROCESS_MAP`), ensuring single-threaded access at any given time.
- The raw pointers inside `SYSTEM_PROCESS_INFORMATION` (such as `ImageName.Buffer`) point into the snapshot buffer owned by [`ProcessSnapshot`](ProcessSnapshot.md). These pointers are only valid for the lifetime of that buffer and must not be dereferenced after the snapshot is dropped.

### Lifetime coupling

The `threads_base_ptr` and `process.ImageName.Buffer` point into the [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) owned by the parent [`ProcessSnapshot`](ProcessSnapshot.md). Once the snapshot is dropped (which clears the buffer), these pointers become dangling. Methods that dereference these pointers (`get_threads`, `get_name_original_case`) are only safe to call while the snapshot is alive.

The `name` field (lowercase `String`) is an owned copy, so `get_name()` is always safe to call regardless of snapshot lifetime.

### Clone behavior

`ProcessEntry` derives `Clone`. Cloned instances share the same `threads_base_ptr` value (a numeric address) and the same `SYSTEM_PROCESS_INFORMATION` content. Cloned entries have the same lifetime constraints as the original regarding pointer validity.

### Name normalization

Process names are stored in lowercase to enable case-insensitive matching against configuration rules. The conversion uses `String::to_lowercase()` on the UTF-16-decoded name, following Rust's Unicode lowercasing rules (which match Windows case-insensitive file name comparison for ASCII process names).

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `process.rs` |
| **Created by** | `ProcessEntry::new` (called from [`ProcessSnapshot::take`](ProcessSnapshot.md)) |
| **Stored in** | [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) |
| **Dependencies** | `ntapi::ntexapi::{SYSTEM_PROCESS_INFORMATION, SYSTEM_THREAD_INFORMATION}`, [`HashMap`](../collections.rs/HashMap.md) |
| **Platform** | Windows only |

## See Also

| Topic | Link |
|-------|------|
| ProcessSnapshot struct | [ProcessSnapshot](ProcessSnapshot.md) |
| SNAPSHOT_BUFFER static | [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) |
| PID_TO_PROCESS_MAP static | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| HashMap type alias | [HashMap](../collections.rs/HashMap.md) |
| winapi module | [winapi.rs](../winapi.rs/README.md) |
| process module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
