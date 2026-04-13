# SNAPSHOT_BUFFER static (process.rs)

Global buffer used by [`ProcessSnapshot::take`](ProcessSnapshot.md#processsnapshottake) to store the raw byte output of `NtQuerySystemInformation(SystemProcessInformation, ...)`.

## Syntax

```rust
pub static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| Inner value | `Vec<u8>` | Byte buffer that receives the variable-length array of `SYSTEM_PROCESS_INFORMATION` structures returned by the kernel. Initialized to 32 bytes; grown automatically by `ProcessSnapshot::take` when the kernel returns `STATUS_INFO_LENGTH_MISMATCH`. |

## Remarks

> **Important:** Do not lock or access `SNAPSHOT_BUFFER` directly. Always go through the [`ProcessSnapshot`](ProcessSnapshot.md) RAII wrapper, which locks both `SNAPSHOT_BUFFER` and [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) together and ensures they stay consistent.

The buffer is allocated once via `once_cell::sync::Lazy` and reused across successive snapshots to amortize allocation cost. `ProcessSnapshot::take` may reallocate the buffer when the system's process list grows beyond the current capacity. When the `ProcessSnapshot` is dropped, the buffer is cleared (length set to zero) but the underlying allocation is retained for the next snapshot.

### Lifetime and thread safety

`SNAPSHOT_BUFFER` is wrapped in a `Mutex` to guarantee exclusive access. Because the raw `SYSTEM_PROCESS_INFORMATION` structures inside the buffer contain pointers (e.g., `ImageName.Buffer`) that are only valid while the buffer contents are unchanged, the `ProcessSnapshot` borrow keeps the mutex guard alive for the duration of the snapshot's use.

### Growth strategy

When `NtQuerySystemInformation` returns `STATUS_INFO_LENGTH_MISMATCH` (`0xC0000004`):

1. If the kernel reported a required length (`return_len > 0`), the buffer is resized to that length rounded up to an 8-byte boundary.
2. Otherwise, the buffer capacity is doubled.

The call is then retried in a loop until it succeeds or returns a different NTSTATUS error.

## Requirements

| | |
|---|---|
| **Module** | `process.rs` |
| **Crate dependencies** | `once_cell`, `ntapi` |
| **Synchronization** | `std::sync::Mutex` — lock before access |
| **Privileges** | None beyond those required by the calling [`ProcessSnapshot::take`](ProcessSnapshot.md#processsnapshottake) |

## See Also

| Topic | Link |
|-------|------|
| ProcessSnapshot struct | [ProcessSnapshot](ProcessSnapshot.md) |
| PID_TO_PROCESS_MAP static | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| ProcessEntry struct | [ProcessEntry](ProcessEntry.md) |
| NtQuerySystemInformation | [Microsoft Learn — NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) |