# ProcessSnapshot struct (process.rs)

The `ProcessSnapshot` struct captures a point-in-time snapshot of all running processes and their threads on the system by calling `NtQuerySystemInformation` with `SystemProcessInformation`. It holds mutable references to a shared buffer and a PID-to-process lookup map, both of which are automatically cleared when the snapshot is dropped (RAII semantics). This ensures that stale process data does not persist between scheduling cycles.

## Syntax

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `buffer` | `&'a mut Vec<u8>` | Mutable reference to the backing byte buffer that stores the raw `SYSTEM_PROCESS_INFORMATION` structures returned by `NtQuerySystemInformation`. This field is private. |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | Mutable reference to a PID-keyed map of parsed [`ProcessEntry`](ProcessEntry.md) objects. Public, allowing callers to look up process information by PID after taking a snapshot. |

## Methods

### `ProcessSnapshot::take`

```rust
pub fn take(
    buffer: &'a mut Vec<u8>,
    pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
) -> Result<Self, i32>
```

Captures a new process snapshot. This is the only way to construct a `ProcessSnapshot`.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `buffer` | `&'a mut Vec<u8>` | A mutable reference to the reusable byte buffer. Typically obtained by locking the [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) static. The buffer is dynamically resized if `NtQuerySystemInformation` returns `STATUS_INFO_LENGTH_MISMATCH`. |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | A mutable reference to the reusable PID-to-process map. Typically obtained by locking the [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) static. Cleared at the start of each call before being repopulated. |

#### Return value

| Outcome | Description |
|---------|-------------|
| `Ok(ProcessSnapshot)` | A successfully constructed snapshot. The `pid_to_process` map is populated with all running processes. |
| `Err(i32)` | An `NTSTATUS` error code from `NtQuerySystemInformation` (other than `STATUS_INFO_LENGTH_MISMATCH`, which is handled internally by retrying with a larger buffer). |

## Remarks

### Snapshot algorithm

1. **Query with retry loop.** Calls `NtQuerySystemInformation(SystemProcessInformation, ...)` in a loop. If the call returns `STATUS_INFO_LENGTH_MISMATCH` (`-1073741820` / `0xC0000004`), the buffer is reallocated to the size indicated by `return_len` (rounded up to an 8-byte boundary), or doubled if `return_len` is zero. The loop retries until a non-mismatch status is received.

2. **Truncate buffer.** On success, the buffer is truncated to the actual number of bytes returned (`return_len`), reclaiming unused capacity.

3. **Parse linked list.** The raw buffer contains a linked list of `SYSTEM_PROCESS_INFORMATION` structures connected via `NextEntryOffset`. The function walks this list, constructing a [`ProcessEntry`](ProcessEntry.md) for each process and inserting it into `pid_to_process` keyed by `UniqueProcessId`.

4. **Return the snapshot.** The populated `ProcessSnapshot` is returned, borrowing both the buffer and the map. The buffer must remain valid for the lifetime of the snapshot because `ProcessEntry` objects contain pointers into the buffer's memory (for thread information arrays and process name strings).

### Drop behavior

When the `ProcessSnapshot` is dropped:
- `pid_to_process.clear()` is called, removing all parsed process entries.
- `buffer.clear()` is called, zeroing the buffer length (but not deallocating its capacity, so the allocation can be reused on the next snapshot).

This cleanup is critical because [`ProcessEntry`](ProcessEntry.md) objects store raw pointers (`threads_base_ptr`) into the buffer. Clearing both the map and the buffer simultaneously prevents dangling pointer access.

### Buffer sizing strategy

The initial buffer is small (32 bytes, as defined in [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md)). On the first call, `NtQuerySystemInformation` will almost certainly return `STATUS_INFO_LENGTH_MISMATCH`, causing a resize. The new size is calculated as `((return_len / 8) + 1) * 8` to align to an 8-byte boundary. On subsequent calls, the buffer retains its previous capacity (via `Vec::capacity()`), so resizing is only needed if the system's process count has grown significantly.

### Unsafe code

The entire body of `take` is wrapped in `unsafe` because:
- `NtQuerySystemInformation` is an FFI call to `ntdll.dll`.
- The raw buffer is reinterpreted as `SYSTEM_PROCESS_INFORMATION` pointers.
- Thread information arrays are accessed via raw pointer arithmetic from `SYSTEM_PROCESS_INFORMATION.Threads`.

### Typical usage pattern

```text
let mut buf = SNAPSHOT_BUFFER.lock().unwrap();
let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buf, &mut map)?;

for (pid, entry) in snapshot.pid_to_process.iter() {
    // Process each entry...
}
// snapshot is dropped here, clearing both buf and map
```

### Platform notes

- **Windows only.** Relies on the NT-native `NtQuerySystemInformation` API from `ntdll.dll` (imported via the `ntapi` crate).
- The `SYSTEM_PROCESS_INFORMATION` and `SYSTEM_THREAD_INFORMATION` types come from the `ntapi::ntexapi` module.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `process.rs` |
| **Callers** | `scheduler.rs` — main scheduling loop; `apply.rs` — rule application |
| **Callees** | `NtQuerySystemInformation` (ntdll, via `ntapi`), [`ProcessEntry::new`](ProcessEntry.md) |
| **Statics** | [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md), [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) |
| **Win32 API** | `NtQuerySystemInformation` with `SystemProcessInformation` |
| **Privileges** | None explicitly required, but `SeDebugPrivilege` enables enumeration of protected processes. |

## See Also

| Topic | Link |
|-------|------|
| ProcessEntry struct | [ProcessEntry](ProcessEntry.md) |
| SNAPSHOT_BUFFER static | [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) |
| PID_TO_PROCESS_MAP static | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| collections module | [collections.rs](../collections.rs/README.md) |
| winapi module | [winapi.rs](../winapi.rs/README.md) |
| event_trace module | [event_trace.rs](../event_trace.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
