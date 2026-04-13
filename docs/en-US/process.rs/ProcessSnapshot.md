# ProcessSnapshot struct (process.rs)

Captures a point-in-time snapshot of all running processes and their threads on the system using a single `NtQuerySystemInformation` syscall.

## Syntax

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
```

## Members

`buffer`

A mutable reference to a caller-owned byte buffer that holds the raw `SYSTEM_PROCESS_INFORMATION` data returned by `NtQuerySystemInformation`. The snapshot borrows this buffer for its entire lifetime, and all [ProcessEntry](ProcessEntry.md) thread pointers reference memory within it. The buffer is cleared when the snapshot is dropped.

`pid_to_process`

A `&'a mut HashMap<u32, ProcessEntry>` — a mutable reference to the global [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) mapping each process identifier (PID) to its corresponding [ProcessEntry](ProcessEntry.md). Populated during [`take()`](#methods) by walking the linked list of `SYSTEM_PROCESS_INFORMATION` structures in the raw buffer. Previously an owned `HashMap`, now a reference to the shared global static to allow the snapshot data to persist across the lifetime of a loop iteration.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **take** | `pub fn take(buffer: &'a mut Vec<u8>, pid_to_process: &'a mut HashMap<u32, ProcessEntry>) -> Result<Self, i32>` | Captures a snapshot of all processes and threads. Returns `Err(NTSTATUS)` on failure. |
| **get_by_name** | `pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry>` | Finds all processes matching the given name (case-insensitive). |

### take

Captures a snapshot of all processes and threads via `NtQuerySystemInformation(SystemProcessInformation)`.

The function accepts two mutable references: a byte buffer (typically from [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md)) and a process map (typically from [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md)). Both are global statics reused across loop iterations.

The function dynamically grows the buffer if the initial capacity is insufficient. When `NtQuerySystemInformation` returns `STATUS_INFO_LENGTH_MISMATCH` (`0xC0000004`), the buffer is reallocated to the size hinted by `return_len` (8-byte aligned) and the call is retried in a loop.

After a successful syscall, the raw buffer is walked as a linked list of `SYSTEM_PROCESS_INFORMATION` structures. Each entry is converted to a [ProcessEntry](ProcessEntry.md) and inserted into `pid_to_process` keyed by PID. The map is cleared before repopulation.

**Performance:** This approach captures all process and thread information in a single kernel transition — O(P+T) where P is the number of processes and T the total number of threads. This is significantly faster than the `ToolHelp32` API which requires multiple syscalls (`CreateToolhelp32Snapshot` + `Process32First/Next` + `Thread32First/Next`).

### get_by_name

Returns a vector of references to all [ProcessEntry](ProcessEntry.md) instances whose lowercased name matches the given `name`. Comparison is case-insensitive because `ProcessEntry` stores names in lowercase.

## Remarks

`ProcessSnapshot` uses a lifetime parameter `'a` tied to both the backing buffer and the process map to ensure memory safety. The raw `SYSTEM_PROCESS_INFORMATION` structures contain `UNICODE_STRING` and thread array pointers that reference memory within the buffer. By borrowing both the buffer and the map with the same lifetime annotation, the Rust borrow checker guarantees that neither can be freed or reallocated while any `ProcessSnapshot` or `ProcessEntry` is alive.

The `Drop` implementation clears both `pid_to_process` and the backing buffer. This ensures that `ProcessEntry` objects (which hold raw pointers into the buffer) are destroyed before the buffer contents are zeroed, preventing dangling pointer access.

**Global statics:** The buffer and process map are stored in [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) and [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) respectively. These `Lazy<Mutex<...>>` statics are locked and passed into `take()` by the main loop. This design avoids repeated heap allocations while keeping the borrow checker aware of lifetimes.

**Buffer reuse:** The caller owns the buffer and can pass it to successive `take()` calls. The buffer capacity grows monotonically — once enlarged for a busy system, subsequent snapshots reuse the larger allocation without reallocation. This is an intentional design to minimize heap churn in the hot loop of the service.

**Thread data lifetime:** Thread information within each [ProcessEntry](ProcessEntry.md) is accessed via raw pointers (`threads_base_ptr`) into the snapshot buffer. These pointers are only dereferenced lazily when [`get_threads()`](ProcessEntry.md) is called, and are valid for the lifetime of the snapshot.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/process.rs` |
| **Source lines** | L6–L14 |
| **Called by** | [`main`](../main.rs/main.md) in `src/main.rs` (each loop iteration) |
| **Key dependencies** | `ntapi::ntexapi::NtQuerySystemInformation`, [ProcessEntry](ProcessEntry.md), [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md), [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) |
| **Windows API** | [NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) with `SystemProcessInformation` |

## See also

- [process.rs module overview](README.md)
- [ProcessEntry](ProcessEntry.md)
- [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md)
- [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md)
- [apply_config_process_level](../main.rs/apply_config_process_level.md) — consumes the snapshot for process-level settings
- [apply_config_thread_level](../main.rs/apply_config_thread_level.md) — consumes the snapshot for thread-level settings