# process.rs Module (process.rs)

The `process` module provides a high-performance process and thread enumeration facility built on the native `NtQuerySystemInformation` syscall. It captures a consistent, point-in-time snapshot of all running processes and their threads in a single system call.

## Overview

This module contains two primary types and two shared statics:

- **[ProcessSnapshot](ProcessSnapshot.md)** — Captures and owns a snapshot of all system processes and threads via a single `NtQuerySystemInformation(SystemProcessInformation)` call.
- **[ProcessEntry](ProcessEntry.md)** — Represents a single process within the snapshot, providing access to process metadata and lazily-parsed thread information.
- **[SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md)** — Global buffer reused across loop iterations to avoid repeated heap allocations.
- **[PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md)** — Global process map populated by each snapshot, shared across the lifetime of a loop iteration.

The snapshot approach is fundamentally different from the Win32 ToolHelp32 API (`CreateToolhelp32Snapshot` / `Process32First` / `Process32Next`), which requires multiple syscalls and can produce inconsistent results when processes are created or destroyed during enumeration.

## Items

### Structs

| Name | Description |
| --- | --- |
| [ProcessSnapshot](ProcessSnapshot.md) | Lifetime-bound snapshot of all processes and threads from a single `NtQuerySystemInformation` call. |
| [ProcessEntry](ProcessEntry.md) | A single process entry with lazy thread parsing and efficient name lookup. |

### Statics

| Name | Description |
| --- | --- |
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | Global buffer for snapshot data, reused across iterations. |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | Global process map populated by each snapshot. |

## Architecture

### Memory Model

`ProcessSnapshot` borrows a caller-supplied `Vec<u8>` buffer and a caller-supplied `HashMap<u32, ProcessEntry>` (typically the global `PID_TO_PROCESS_MAP`) and fills them with raw `SYSTEM_PROCESS_INFORMATION` structures from the kernel. `ProcessEntry` objects are constructed from the buffer and store raw pointers back into it for deferred thread parsing. The lifetime parameter `'a` on `ProcessSnapshot` ensures that both the buffer and the map outlive all references.

When the `ProcessSnapshot` is dropped, the `pid_to_process` map and the backing buffer are both cleared, invalidating all raw pointers safely.

### Performance

| Aspect | ProcessSnapshot (NtQuerySystemInformation) | ToolHelp32 |
| --- | --- | --- |
| **Syscalls** | 1 (O(P+T) in one call) | O(P) + O(T) across multiple calls |
| **Consistency** | Atomic kernel snapshot | Races possible between calls |
| **Thread access** | Included in same buffer | Separate `Thread32First`/`Thread32Next` |
| **Buffer management** | Dynamic growth on `STATUS_INFO_LENGTH_MISMATCH` | Opaque, managed by OS |

### Typical Usage

```rust
let mut buffer = SNAPSHOT_BUFFER.lock().unwrap();
let mut pid_to_process = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buffer, &mut pid_to_process)?;

for (pid, entry) in snapshot.pid_to_process.iter() {
    println!("{}: {}", pid, entry.get_name());
}
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/process.rs` |
| **Crate dependencies** | `ntapi` (for `NtQuerySystemInformation`, `SYSTEM_PROCESS_INFORMATION`, `SYSTEM_THREAD_INFORMATION`) |
| **Called by** | [`main`](../main.rs/main.md) (each loop iteration) |
| **Key consumers** | [`apply_config_process_level`](../main.rs/apply_config_process_level.md), [`apply_config_thread_level`](../main.rs/apply_config_thread_level.md), [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_prime_threads`](../apply.rs/apply_prime_threads.md), [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) |

## See also

- [ProcessSnapshot](ProcessSnapshot.md)
- [ProcessEntry](ProcessEntry.md)
- [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md)
- [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — consumes thread data from snapshots for cycle tracking
- [apply.rs module overview](../apply.rs/README.md) — primary consumer of process snapshot data