# ProcessEntry struct (process.rs)

Represents a single process within a [ProcessSnapshot](ProcessSnapshot.md), wrapping the native `SYSTEM_PROCESS_INFORMATION` structure with convenience accessors and lazy thread parsing.

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

`process`

The raw `SYSTEM_PROCESS_INFORMATION` structure as returned by `NtQuerySystemInformation`. This is a public field providing direct access to all native process fields such as `UniqueProcessId`, `NumberOfThreads`, `WorkingSetSize`, `HandleCount`, and timing information (`KernelTime`, `UserTime`, `CreateTime`).

`threads`

Private `HashMap<u32, SYSTEM_THREAD_INFORMATION>` mapping thread IDs to their native thread information structures. This map is lazily populated on the first call to [`get_threads()`](#methods) by reading from the raw pointer stored in `threads_base_ptr`. Subsequent calls return the cached map unless the thread count has changed.

`threads_base_ptr`

Private `usize` storing the raw pointer (cast to integer) to the start of the `SYSTEM_THREAD_INFORMATION` array that immediately follows the `SYSTEM_PROCESS_INFORMATION` structure in the snapshot buffer. This pointer remains valid for the lifetime `'a` of the owning [ProcessSnapshot](ProcessSnapshot.md).

`name`

Private `String` containing the lowercase process image name (e.g., `"notepad.exe"`). Extracted from the `ImageName` wide-string field during construction and stored for efficient repeated lookups.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **new** | `pub fn new(process: SYSTEM_PROCESS_INFORMATION, threads_base_ptr: *const SYSTEM_THREAD_INFORMATION) -> Self` | Constructs a new `ProcessEntry`, extracting the lowercase process name from the `ImageName` wide-string buffer. |
| **get_threads** | `pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION>` | Returns the thread map, lazily parsing the raw thread array on first call. Repopulates if thread count has changed. |
| **get_thread** | `pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION>` | Returns thread information for a specific thread ID, or `None` if not found. Triggers lazy parsing via `get_threads()`. |
| **get_name** | `pub fn get_name(&self) -> &str` | Returns the lowercase process image name (e.g., `"explorer.exe"`). |
| **get_name_original_case** | `pub fn get_name_original_case(&self) -> String` | Returns the process image name in its original case by re-reading the `ImageName` wide-string buffer. Requires the snapshot buffer to still be alive. |
| **pid** | `pub fn pid(&self) -> u32` | Returns the process identifier extracted from `process.UniqueProcessId`. |
| **thread_count** | `pub fn thread_count(&self) -> u32` | Returns the number of threads reported by `process.NumberOfThreads`. |

## Remarks

### Lazy thread parsing

Thread information is not parsed during construction. Instead, the `threads_base_ptr` raw pointer is stored and threads are only parsed into the `HashMap` when [`get_threads()`](#methods) or [`get_thread()`](#methods) is first called. This avoids the cost of parsing thread arrays for processes that are never inspected at the thread level.

If `get_threads()` is called again and `process.NumberOfThreads` no longer matches the cached map size, the map is cleared and repopulated. This handles the edge case where the struct is reused across snapshots, though in practice each `ProcessEntry` is created fresh per snapshot.

### Memory safety

The `threads_base_ptr` field is a raw pointer into the snapshot buffer owned by [ProcessSnapshot](ProcessSnapshot.md). It is safe to dereference only while the parent `ProcessSnapshot` is alive. The lifetime parameter `'a` on `ProcessSnapshot` ensures the backing buffer cannot be freed while entries are in use. The `ProcessEntry` itself is `Clone`, but cloning copies the pointer value — the clone is only valid while the original snapshot buffer exists.

### Name handling

The process name is extracted once during `new()` and lowercased for case-insensitive matching throughout the application. The original-case name can be recovered via `get_name_original_case()`, which re-reads from the wide-string pointer in the `SYSTEM_PROCESS_INFORMATION` structure. For the System Idle Process (PID 0), the image name is empty.

### Usage in the apply pipeline

`ProcessEntry` is the primary process representation passed through the apply pipeline. Functions like [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_prime_threads`](../apply.rs/apply_prime_threads.md), and [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) accept `&mut ProcessEntry` to access thread information for per-thread operations.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/process.rs` |
| **Source lines** | L74–L158 |
| **Created by** | [`ProcessSnapshot::take`](ProcessSnapshot.md) |
| **Consumed by** | [`apply_config`](../main.rs/apply_config.md), [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_prime_threads`](../apply.rs/apply_prime_threads.md), [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) |
| **Windows types** | `SYSTEM_PROCESS_INFORMATION`, `SYSTEM_THREAD_INFORMATION` (from `ntapi`) |

## See also

- [ProcessSnapshot](ProcessSnapshot.md)
- [process.rs module overview](README.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)
- [apply_config](../main.rs/apply_config.md)