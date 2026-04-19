# ProcessSnapshot struct (process.rs)

RAII guard that captures a point-in-time snapshot of all running processes and their threads using `NtQuerySystemInformation(SystemProcessInformation)`. The struct holds mutable references to a shared buffer and process map, both of which are cleared automatically when the snapshot is dropped.

## Syntax

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `buffer` | `&'a mut Vec<u8>` | Mutable reference to the raw byte buffer that holds the `SYSTEM_PROCESS_INFORMATION` linked list returned by `NtQuerySystemInformation`. Reused across iterations via [SNAPSHOT_BUFFER](README.md). |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | Mutable reference to the process map populated during `take()`. Keyed by PID, values are [ProcessEntry](ProcessEntry.md) structs. Public so callers can iterate and query processes. |

## Methods

### take

```rust
pub fn take(
    buffer: &'a mut Vec<u8>,
    pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
) -> Result<Self, i32>
```

Captures a snapshot of all processes and threads on the system by calling `NtQuerySystemInformation(SystemProcessInformation)`.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `buffer` | `&'a mut Vec<u8>` | Mutable reference to the byte buffer used to receive the system information. The buffer is grown dynamically if it is too small. Typically sourced from `SNAPSHOT_BUFFER`. |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | Mutable reference to the HashMap that will be populated with process entries. Cleared at the start of each call. Typically sourced from `PID_TO_PROCESS_MAP`. |

**Return value**

`Result<ProcessSnapshot<'a>, i32>` — On success, returns the RAII snapshot guard. On failure, returns the NTSTATUS error code as `i32`.

**Remarks**

- The function calls `NtQuerySystemInformation` in a retry loop. If the call returns `STATUS_INFO_LENGTH_MISMATCH` (`0xC0000004`), the buffer is reallocated to the size indicated by the `ReturnLength` out-parameter (rounded up to an 8-byte boundary), or doubled if `ReturnLength` is zero.
- After a successful call, the raw buffer is walked as a linked list of `SYSTEM_PROCESS_INFORMATION` structures. Each entry's `NextEntryOffset` field points to the next entry; a zero offset indicates the last entry.
- For each process, a [ProcessEntry](ProcessEntry.md) is constructed via `ProcessEntry::new()`, which extracts the lowercase process name and stores the base pointer to the thread array.
- The buffer and map are **cleared on drop**, so all [ProcessEntry](ProcessEntry.md) references derived from `pid_to_process` become invalid after the snapshot is dropped.

### Drop

```rust
impl<'a> Drop for ProcessSnapshot<'a> {
    fn drop(&mut self);
}
```

Clears both `pid_to_process` and `buffer` when the snapshot goes out of scope. This ensures that stale raw pointers stored in [ProcessEntry](ProcessEntry.md) (the `threads_base_ptr` field) are never dereferenced after the underlying buffer has been reused.

## Remarks

`ProcessSnapshot` follows the RAII pattern to tie the lifetime of parsed process data to the lifetime of the raw buffer. Because `SYSTEM_PROCESS_INFORMATION.Threads` is a variable-length array appended to each structure, the thread data is only valid while the original buffer is alive. The `Drop` implementation enforces this invariant.

### Typical usage

```rust
let mut buffer = SNAPSHOT_BUFFER.lock().unwrap();
let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buffer, &mut map)?;
// Use snapshot.pid_to_process to iterate processes
// snapshot is dropped here, clearing buffer and map
```

### Buffer growth strategy

| Condition | Action |
|-----------|--------|
| `ReturnLength > 0` | Allocate `((ReturnLength / 8) + 1) * 8` bytes (8-byte aligned ceiling) |
| `ReturnLength == 0` | Double the current buffer capacity |

The initial buffer size is 32 bytes (from `SNAPSHOT_BUFFER`), which will always trigger at least one resize on the first call. Subsequent calls reuse the capacity from the previous successful call, so resizes become rare after the first iteration.

## Requirements

| | |
|---|---|
| **Module** | `src/process.rs` |
| **Callers** | Main polling loop in `src/main.rs` |
| **Callees** | `NtQuerySystemInformation` (ntdll), [ProcessEntry::new](ProcessEntry.md) |
| **NT API** | [NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) with `SystemProcessInformation` (class 5) |
| **Privileges** | None required for basic enumeration; [SeDebugPrivilege](../winapi.rs/enable_debug_privilege.md) extends visibility to protected processes |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [process.rs](README.md) |
| Process data container | [ProcessEntry](ProcessEntry.md) |
| Thread handle management | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*