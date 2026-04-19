# ProcessEntry struct (process.rs)

Represents a single process from a system process snapshot. Wraps the native `SYSTEM_PROCESS_INFORMATION` structure together with a cached lowercase process name and a raw pointer to the thread array. Implements `Clone` and `Send` (via an explicit unsafe impl, justified by the Mutex-guarded access pattern).

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

| Member | Type | Description |
|--------|------|-------------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | The raw Windows kernel structure containing process metrics: PID, thread count, working set size, timing information, handle count, and the `ImageName` UNICODE_STRING. |
| `threads_base_ptr` | `usize` | Base address of the `SYSTEM_THREAD_INFORMATION` array that immediately follows the process structure in the snapshot buffer. Stored as `usize` rather than a raw pointer to allow `Clone` derivation. Only valid while the parent [ProcessSnapshot](ProcessSnapshot.md) is alive. |
| `name` | `String` | Lowercase copy of the process image name, extracted from `process.ImageName` at construction time. Used for case-insensitive configuration matching throughout the service. |

## Methods

### new

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION,
) -> Self
```

Constructs a `ProcessEntry` from a native process information structure and a pointer to its thread array.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | Copied process information structure from the snapshot buffer. |
| `threads_base_ptr` | `*const SYSTEM_THREAD_INFORMATION` | Pointer to the first element of the thread array embedded in the snapshot buffer. Cast to `usize` for storage. |

**Remarks**

- Extracts the process image name from the `ImageName` UNICODE_STRING field, converting UTF-16 to a Rust `String` via `String::from_utf16_lossy`.
- The name is immediately lowercased with `.to_lowercase()` for case-insensitive matching.
- If `ImageName.Length` is zero or the buffer pointer is null (as is the case for the System Idle Process, PID 0), an empty string is stored.

---

### get_threads

```rust
pub fn get_threads(&self) -> HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

Parses the raw thread array from the snapshot buffer into a `HashMap` keyed by thread ID (TID).

**Return value**

`HashMap<u32, SYSTEM_THREAD_INFORMATION>` — A map from each thread's `ClientId.UniqueThread` (cast to `u32`) to its full `SYSTEM_THREAD_INFORMATION` structure.

**Remarks**

- Iterates over `process.NumberOfThreads` entries starting at `threads_base_ptr`.
- The returned map is freshly constructed on each call; results are not cached within the `ProcessEntry` itself.
- If `threads_base_ptr` is null (stored as `0usize`), returns an empty map immediately.
- **Safety:** The pointer arithmetic is valid only while the parent [ProcessSnapshot](ProcessSnapshot.md) is alive and its buffer has not been cleared. Calling this method after the snapshot is dropped is undefined behavior.

---

### get_name

```rust
#[inline]
pub fn get_name(&self) -> &str
```

Returns the cached lowercase process image name.

**Return value**

`&str` — A borrowed reference to the lowercase name string. Returns `""` for the System Idle Process (PID 0).

---

### get_name_original_case

```rust
#[inline]
pub fn get_name_original_case(&self) -> String
```

Re-reads the process image name from the `UNICODE_STRING` in its original case.

**Return value**

`String` — The process name preserving the original casing from the kernel, e.g. `"svchost.exe"` or `"MsMpEng.exe"`. Returns an empty string if the image name buffer is null or zero-length.

**Remarks**

- Unlike [get_name](#get_name), this method performs a fresh UTF-16 conversion every call.
- Reads directly from the `process.ImageName.Buffer` pointer, so the same lifetime constraint as [get_threads](#get_threads) applies.
- Currently marked `#[allow(dead_code)]`; available for diagnostic/logging scenarios.

---

### pid

```rust
#[inline]
pub fn pid(&self) -> u32
```

Returns the process identifier.

**Return value**

`u32` — The `UniqueProcessId` field from the native structure, cast through `usize` to `u32`.

---

### thread_count

```rust
#[inline]
pub fn thread_count(&self) -> u32
```

Returns the number of threads in this process.

**Return value**

`u32` — The `NumberOfThreads` field from the native `SYSTEM_PROCESS_INFORMATION` structure.

## Remarks

### Send safety

`ProcessEntry` contains a `SYSTEM_PROCESS_INFORMATION` value which includes raw pointers (e.g., `ImageName.Buffer`). The explicit `unsafe impl Send for ProcessEntry` is justified by the access pattern: all `ProcessEntry` instances live inside `PID_TO_PROCESS_MAP`, which is protected by a `Mutex`. No `ProcessEntry` is ever sent across threads without the mutex guard.

### Lifetime constraints

The `threads_base_ptr` and `process.ImageName.Buffer` pointers refer into the snapshot buffer owned by [ProcessSnapshot](ProcessSnapshot.md). All access through these pointers must occur while the snapshot is alive. The `name` field is a safe owned `String` copy and has no such constraint.

### Clone behavior

Cloning a `ProcessEntry` copies the `SYSTEM_PROCESS_INFORMATION` structure by value (including its embedded raw pointers) and clones the `name` string. The cloned entry shares the same pointer-based fields and is subject to the same lifetime constraints as the original.

## Requirements

| | |
|---|---|
| **Module** | `src/process.rs` |
| **Callers** | Main polling loop, [apply_prime_threads](../apply.rs/apply_prime_threads.md), any code that iterates processes from [ProcessSnapshot](ProcessSnapshot.md) |
| **Dependencies** | `ntapi::ntexapi::SYSTEM_PROCESS_INFORMATION`, `ntapi::ntexapi::SYSTEM_THREAD_INFORMATION` |
| **Privileges** | None (operates on already-captured snapshot data) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [process.rs](README.md) |
| Snapshot RAII wrapper | [ProcessSnapshot](ProcessSnapshot.md) |
| Thread-level stats tracking | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Process handle management | [ProcessHandle](../winapi.rs/ProcessHandle.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*