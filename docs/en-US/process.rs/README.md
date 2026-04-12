# process.rs Module (process.rs)

The `process` module provides a high-performance process and thread enumeration facility built on the native `NtQuerySystemInformation` syscall. It captures a consistent, point-in-time snapshot of all running processes and their threads in a single system call.

## Overview

This module contains two primary types:

- **[ProcessSnapshot](ProcessSnapshot.md)** — Captures and owns a snapshot of all system processes and threads via a single `NtQuerySystemInformation(SystemProcessInformation)` call.
- **[ProcessEntry](ProcessEntry.md)** — Represents a single process within the snapshot, providing access to process metadata and lazily-parsed thread information.

The snapshot approach is fundamentally different from the Win32 ToolHelp32 API (`CreateToolhelp32Snapshot` / `Process32First` / `Process32Next`), which requires multiple syscalls and can produce inconsistent results when processes are created or destroyed during enumeration.

## Items

### Structs

| Name | Description |
| --- | --- |
| [ProcessSnapshot](ProcessSnapshot.md) | Lifetime-bound snapshot of all processes and threads from a single `NtQuerySystemInformation` call. |
| [ProcessEntry](ProcessEntry.md) | A single process entry with lazy thread parsing and efficient name lookup. |

## Architecture

### Memory Model

`ProcessSnapshot` borrows a caller-supplied `Vec<u8>` buffer and fills it with raw `SYSTEM_PROCESS_INFORMATION` structures from the kernel. `ProcessEntry` objects are constructed from this buffer and store raw pointers back into it for deferred thread parsing. The lifetime parameter `'a` on `ProcessSnapshot` ensures that the buffer outlives all references.

When the `ProcessSnapshot` is dropped, the `pid_to_process` map and the buffer are both cleared, invalidating all raw pointers safely.

### Performance

| Aspect | ProcessSnapshot (NtQuerySystemInformation) | ToolHelp32 |
| --- | --- | --- |
| **Syscalls** | 1 (O(P+T) in one call) | O(P) + O(T) across multiple calls |
| **Consistency** | Atomic kernel snapshot | Races possible between calls |
| **Thread access** | Included in same buffer | Separate `Thread32First`/`Thread32Next` |
| **Buffer management** | Dynamic growth on `STATUS_INFO_LENGTH_MISMATCH` | Opaque, managed by OS |

### Typical Usage

```rust
let mut buffer = Vec::with_capacity(4 * 1024 * 1024);
let snapshot = ProcessSnapshot::take(&mut buffer)?;

for (pid, entry) in &snapshot.pid_to_process {
    println!("{}: {}", pid, entry.get_name());
}
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/process.rs` |
| **Crate dependencies** | `ntapi` (for `NtQuerySystemInformation`, `SYSTEM_PROCESS_INFORMATION`, `SYSTEM_THREAD_INFORMATION`) |
| **Called by** | [`main`](../main.rs/main.md) (each loop iteration) |
| **Key consumers** | [`apply_config`](../main.rs/apply_config.md), [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_prime_threads`](../apply.rs/apply_prime_threads.md), [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) |

## See also

- [ProcessSnapshot](ProcessSnapshot.md)
- [ProcessEntry](ProcessEntry.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — consumes thread data from snapshots for cycle tracking
- [apply.rs module overview](../apply.rs/README.md) — primary consumer of process snapshot data