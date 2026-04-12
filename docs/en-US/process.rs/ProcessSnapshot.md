# ProcessSnapshot struct (process.rs)

Captures a point-in-time snapshot of all running processes and their threads on the system using a single `NtQuerySystemInformation` syscall.

## Syntax

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}
```

## Members

`buffer`

A mutable reference to a caller-owned byte buffer that holds the raw `SYSTEM_PROCESS_INFORMATION` data returned by `NtQuerySystemInformation`. The snapshot borrows this buffer for its entire lifetime, and all [ProcessEntry](ProcessEntry.md) thread pointers reference memory within it. The buffer is cleared when the snapshot is dropped.

`pid_to_process`

A `HashMap<u32, ProcessEntry>` mapping each process identifier (PID) to its corresponding [ProcessEntry](ProcessEntry.md). Populated during [`take()`](#methods) by walking the linked list of `SYSTEM_PROCESS_INFORMATION` structures in the raw buffer.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **take** | `pub fn take(buffer: &'a mut Vec<u8>) -> Result<Self, i32>` | Captures a snapshot of all processes and threads. Returns `Err(NTSTATUS)` on failure. |
| **get_by_name** | `pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry>` | Finds all processes matching the given name (case-insensitive). |

### take

Captures a snapshot of all processes and threads via `NtQuerySystemInformation(SystemProcessInformation)`.

The function dynamically grows the buffer if the initial capacity is insufficient. When `NtQuerySystemInformation` returns `STATUS_INFO_LENGTH_MISMATCH` (`0xC0000004`), the buffer is reallocated to the size hinted by `return_len` (8-byte aligned) and the call is retried in a loop.

After a successful syscall, the raw buffer is walked as a linked list of `SYSTEM_PROCESS_INFORMATION` structures. Each entry is converted to a [ProcessEntry](ProcessEntry.md) and inserted into `pid_to_process` keyed by PID.

**Performance:** This approach captures all process and thread information in a single kernel transition — O(P+T) where P is the number of processes and T the total number of threads. This is significantly faster than the `ToolHelp32` API which requires multiple syscalls (`CreateToolhelp32Snapshot` + `Process32First/Next` + `Thread32First/Next`).

### get_by_name

Returns a vector of references to all [ProcessEntry](ProcessEntry.md) instances whose lowercased name matches the given `name`. Comparison is case-insensitive because `ProcessEntry` stores names in lowercase.

## Remarks

`ProcessSnapshot` uses a lifetime parameter `'a` tied to the backing buffer to ensure memory safety. The raw `SYSTEM_PROCESS_INFORMATION` structures contain `UNICODE_STRING` and thread array pointers that reference memory within the buffer. By borrowing the buffer with a lifetime annotation, the Rust borrow checker guarantees that the buffer cannot be freed or reallocated while any `ProcessSnapshot` or `ProcessEntry` is alive.

The `Drop` implementation clears both `pid_to_process` and the backing buffer. This ensures that `ProcessEntry` objects (which hold raw pointers into the buffer) are destroyed before the buffer contents are zeroed, preventing dangling pointer access.

**Buffer reuse:** The caller owns the buffer and can pass it to successive `take()` calls. The buffer capacity grows monotonically — once enlarged for a busy system, subsequent snapshots reuse the larger allocation without reallocation. This is an intentional design to minimize heap churn in the hot loop of the service.

**Thread data lifetime:** Thread information within each [ProcessEntry](ProcessEntry.md) is accessed via raw pointers (`threads_base_ptr`) into the snapshot buffer. These pointers are only dereferenced lazily when [`get_threads()`](ProcessEntry.md) is called, and are valid for the lifetime of the snapshot.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/process.rs` |
| **Source lines** | L4–L72 |
| **Called by** | [`main`](../main.rs/main.md) in `src/main.rs` (each loop iteration) |
| **Key dependencies** | `ntapi::ntexapi::NtQuerySystemInformation`, [ProcessEntry](ProcessEntry.md) |
| **Windows API** | [NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) with `SystemProcessInformation` |

## See also

- [process.rs module overview](README.md)
- [ProcessEntry](ProcessEntry.md)
- [apply_config](../apply.rs/README.md) — consumes the snapshot each iteration