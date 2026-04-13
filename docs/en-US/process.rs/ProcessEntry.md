# ProcessEntry struct (process.rs)

Represents a single process from a system snapshot, wrapping the native `SYSTEM_PROCESS_INFORMATION` structure with lazy thread parsing and cached name lookup.

`ProcessEntry` stores a raw pointer to the thread array from the snapshot buffer and defers parsing into a `HashMap<u32, SYSTEM_THREAD_INFORMATION>` until `get_threads` or `get_thread` is first called. The process image name is parsed and lowercased eagerly at construction time for efficient matching during configuration lookups.

## Syntax

```rust
#[derive(Clone)]
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | The raw NT process information structure. Public for direct field access (e.g., `UniqueProcessId`, `NumberOfThreads`, `WorkingSetSize`). |
| `threads` | `HashMap<u32, SYSTEM_THREAD_INFORMATION>` | Lazily-populated map from thread ID (TID) to thread information. Empty until `get_threads` is called. Private. |
| `threads_base_ptr` | `usize` | Base address of the `Threads` flexible array member from the snapshot buffer, stored as `usize` to satisfy `Send`. Private. |
| `name` | `String` | Lowercased process image name (e.g., `"explorer.exe"`), parsed from the `ImageName` `UNICODE_STRING` at construction. Private. |

## Methods

### new

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION,
) -> Self
```

Constructs a `ProcessEntry` from a raw process information structure and a pointer to its thread array.

The image name is decoded from `process.ImageName` (UTF-16LE `UNICODE_STRING`) and lowercased immediately. If the image name buffer is null or the length is zero, the name is set to an empty string. The thread map is left empty; threads are parsed lazily on first access via `get_threads`.

| Parameter | Description |
|-----------|-------------|
| `process` | A copy of the `SYSTEM_PROCESS_INFORMATION` structure for this process. |
| `threads_base_ptr` | Pointer to the first element of the `Threads` flexible array member. Stored as `usize` internally. Must remain valid for the lifetime of the owning `ProcessSnapshot`. |

**Return value:** A new `ProcessEntry` with an eagerly-parsed name and a deferred thread map.

### get_threads

```rust
pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

Returns the thread information map, lazily populating it from the raw thread array pointer on first call.

If the internal thread map length does not match `process.NumberOfThreads`, the map is cleared and repopulated by iterating over the raw `SYSTEM_THREAD_INFORMATION` array. Each thread is keyed by its TID (`ClientId.UniqueThread`). On subsequent calls with no change in thread count, the cached map is returned directly.

**Return value:** A reference to the `HashMap<u32, SYSTEM_THREAD_INFORMATION>` keyed by thread ID.

> [!IMPORTANT]
> The raw pointer dereference is safe only while the parent `ProcessSnapshot` (and its backing buffer) is alive. Calling this method after the snapshot is dropped is undefined behavior.

### get_thread

```rust
pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION>
```

Returns thread information for a specific TID, or `None` if the thread does not belong to this process.

Internally calls `get_threads` to ensure the map is populated, then performs a hash lookup.

| Parameter | Description |
|-----------|-------------|
| `tid` | The thread ID to look up. |

**Return value:** `Some(&SYSTEM_THREAD_INFORMATION)` if the thread exists, `None` otherwise.

### get_name

```rust
pub fn get_name(&self) -> &str
```

Returns the lowercased process image name (e.g., `"chrome.exe"`).

This value is computed once during `new` and cached. Returns an empty string for the System Idle Process (PID 0) or any process whose `ImageName` buffer was null.

**Return value:** A string slice referencing the cached lowercase name.

### get_name_original_case

```rust
pub fn get_name_original_case(&self) -> String
```

Returns the process image name preserving original casing from the snapshot buffer.

Unlike `get_name`, this method re-reads the `ImageName` `UNICODE_STRING` from the `process` field on each call without lowercasing. Useful for display or logging where the original casing matters.

**Return value:** A new `String` with the original-case image name, or an empty string if the buffer is null.

> [!NOTE]
> This method dereferences the `ImageName.Buffer` pointer, which points into the snapshot buffer. It is safe only while the parent `ProcessSnapshot` is alive.

### pid

```rust
pub fn pid(&self) -> u32
```

Returns the process ID.

Extracts `UniqueProcessId` from the underlying `SYSTEM_PROCESS_INFORMATION` structure, casting through `usize` to `u32`.

**Return value:** The PID as a `u32`.

### thread_count

```rust
pub fn thread_count(&self) -> u32
```

Returns the number of threads reported by the operating system for this process.

This reads `NumberOfThreads` from the underlying `SYSTEM_PROCESS_INFORMATION` and does **not** require the thread map to be populated.

**Return value:** The thread count as a `u32`.

## Remarks

### Lazy thread parsing

The primary design goal of `ProcessEntry` is to avoid parsing thread arrays for processes that are not targeted by any configuration rule. In a typical system snapshot with hundreds of processes and thousands of threads, only a handful of processes match user-defined rules. Deferring the `O(n)` thread array walk to `get_threads` keeps per-snapshot overhead proportional to the number of *matched* processes.

### Safety and Send

`ProcessEntry` stores the thread array pointer as a `usize` rather than a raw `*const` to allow the type to implement `Send`. An `unsafe impl Send for ProcessEntry` is provided with the contract that instances are only accessed through a `Mutex` (via `PID_TO_PROCESS_MAP`) and that the snapshot buffer outlives all references.

### Cloning

`ProcessEntry` derives `Clone`. Cloned instances share the same `threads_base_ptr` value, meaning clones are also only valid within the snapshot buffer lifetime. The `threads` `HashMap` is deep-cloned, so a clone that has already parsed threads does not need to re-parse.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `process.rs` |
| Constructed by | [`ProcessSnapshot::take`](ProcessSnapshot.md) |
| Used by | [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_prime_threads`](../apply.rs/apply_prime_threads.md), [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md), [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| NT API | `SYSTEM_PROCESS_INFORMATION`, `SYSTEM_THREAD_INFORMATION` (ntapi) |
| Privileges | None (data is read from a snapshot already captured with appropriate privilege) |

## See Also

| Link | Description |
|------|-------------|
| [ProcessSnapshot](ProcessSnapshot.md) | RAII wrapper that captures the snapshot and owns the buffer lifetime. |
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | Global buffer backing the snapshot data. |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | Global map that stores `ProcessEntry` instances by PID. |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Consumes thread cycle data from `ProcessEntry` for prime thread selection. |