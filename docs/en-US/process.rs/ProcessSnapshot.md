# ProcessSnapshot struct (process.rs)

Provides an RAII wrapper around a system-wide process snapshot obtained via `NtQuerySystemInformation`. When the `ProcessSnapshot` is dropped, the backing buffer and parsed process map are cleared, ensuring no stale pointers to kernel-returned data persist beyond the snapshot's useful lifetime.

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
| `buffer` | `&'a mut Vec<u8>` | Mutable reference to the raw byte buffer that backs the snapshot. Filled by `NtQuerySystemInformation` with `SystemProcessInformation`. The buffer is grown automatically on `STATUS_INFO_LENGTH_MISMATCH` and truncated to the actual return length on success. Private — callers interact through the public API. |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | Mutable reference to a PID-keyed map populated during `take`. Each value is a [`ProcessEntry`](ProcessEntry.md) that wraps a `SYSTEM_PROCESS_INFORMATION` structure and provides lazy thread enumeration. Public so that the main loop can iterate, look up, and mutate entries by PID. |

## Lifetime parameter `'a`

Both the buffer and the map are borrowed from caller-owned statics ([`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) and [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md)). The lifetime `'a` ties the `ProcessSnapshot` to those `MutexGuard` borrows, guaranteeing that the raw pointers stored inside each [`ProcessEntry`](ProcessEntry.md) remain valid for as long as the snapshot exists.

## Methods

| Method | Description |
|--------|-------------|
| [`take`](#take) | Captures a new process snapshot by calling `NtQuerySystemInformation`. |
| [`get_by_name`](#get_by_name) | Returns all process entries whose image name matches the given string. |
| [`drop`](#drop-impl) | Clears the process map and buffer when the snapshot goes out of scope. |

---

### take

Captures a snapshot of every running process and its threads.

#### Syntax

```rust
pub fn take(
    buffer: &'a mut Vec<u8>,
    pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
) -> Result<Self, i32>
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `buffer` | `&'a mut Vec<u8>` | Reusable byte buffer. On the first call this is typically 32 bytes; the function grows it as needed. The capacity is preserved across calls so that subsequent snapshots rarely need reallocation. |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | Map to populate. Cleared before parsing begins. Keyed by PID (`UniqueProcessId` cast to `u32`). |

#### Return value

| Value | Description |
|-------|-------------|
| `Ok(ProcessSnapshot)` | A valid snapshot. The caller owns the RAII guard; dropping it clears the data. |
| `Err(i32)` | The NTSTATUS error code returned by `NtQuerySystemInformation` (any negative value other than `STATUS_INFO_LENGTH_MISMATCH`, which is handled internally). |

#### Remarks

1. **Buffer growth strategy** — When `NtQuerySystemInformation` returns `STATUS_INFO_LENGTH_MISMATCH` (`0xC0000004`), the function retries with a larger buffer. If the kernel provided a `return_len`, the new size is `((return_len / 8) + 1) * 8` (8-byte aligned); otherwise the buffer doubles in size. This loop continues until the call succeeds or returns a different error.

2. **Linked-list traversal** — `SYSTEM_PROCESS_INFORMATION` entries form an in-memory linked list via `NextEntryOffset`. The function walks this list from offset 0, constructing a [`ProcessEntry`](ProcessEntry.md) for each node, until `NextEntryOffset == 0`.

3. **Thread pointers** — Each `SYSTEM_PROCESS_INFORMATION` is immediately followed by its thread array (`Threads` flexible array member). The base pointer to this array is captured by [`ProcessEntry::new`](ProcessEntry.md) so that `get_threads` can lazily parse it later. These pointers are only valid while `buffer` is alive — the `Drop` implementation ensures cleanup.

4. **Safety** — The function body is wrapped in `unsafe` because it dereferences raw pointers returned by the kernel. Correctness depends on `NtQuerySystemInformation` writing valid `SYSTEM_PROCESS_INFORMATION` structures and the buffer remaining pinned for the lifetime of the snapshot.

5. **Typical call pattern**:
   ```rust
   let mut buf = SNAPSHOT_BUFFER.lock().unwrap();
   let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
   let snapshot = ProcessSnapshot::take(&mut buf, &mut map)?;
   // Use snapshot.pid_to_process …
   // snapshot dropped here — buffer and map are cleared
   ```

---

### get_by_name

Returns references to all process entries whose lowercased image name matches the given string.

#### Syntax

```rust
pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry>
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `String` | The process image name to search for (e.g., `"explorer.exe"`). Compared against the lowercased name stored in each [`ProcessEntry`](ProcessEntry.md). |

#### Return value

A `Vec<&ProcessEntry>` containing zero or more references to matching entries. Multiple entries can match when several processes share the same image name.

#### Remarks

- This method is marked `#[allow(dead_code)]` in the source and is primarily available for diagnostic or interactive use.
- Comparison uses the pre-lowercased name cached by [`ProcessEntry::new`](ProcessEntry.md), so the `name` argument should also be lowercase for a match.

---

### Drop impl

Clears all snapshot data when the `ProcessSnapshot` goes out of scope.

#### Syntax

```rust
impl<'a> Drop for ProcessSnapshot<'a> {
    fn drop(&mut self);
}
```

#### Remarks

The `Drop` implementation calls `self.pid_to_process.clear()` then `self.buffer.clear()`. This is critical because [`ProcessEntry`](ProcessEntry.md) objects hold raw `threads_base_ptr` pointers into the buffer. Clearing the map first ensures no `ProcessEntry` outlives the buffer memory it references.

Note that `Vec::clear` sets the length to zero but retains allocated capacity, so the buffer memory is reused by the next `take` call without reallocation (unless the system has grown).

## Requirements

| | |
|---|---|
| **Module** | `process.rs` |
| **Callers** | [`main.rs`](../main.rs/README.md) main loop, [`apply.rs`](../apply.rs/README.md) apply functions |
| **Callees** | `NtQuerySystemInformation` (ntdll), [`ProcessEntry::new`](ProcessEntry.md) |
| **API** | `ntapi::ntexapi::NtQuerySystemInformation` with `SystemProcessInformation` information class |
| **Privileges** | `SeDebugPrivilege` recommended for full system visibility; without it, some process entries may be omitted by the kernel |

## See Also

| Link | Description |
|------|-------------|
| [`ProcessEntry`](ProcessEntry.md) | Individual process wrapper with lazy thread parsing |
| [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) | Global buffer backing the snapshot |
| [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) | Global PID-to-ProcessEntry map |
| [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) | Consumes thread data from snapshots for prime-thread scheduling decisions |
| [NtQuerySystemInformation (MSDN)](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) | Underlying Windows API |