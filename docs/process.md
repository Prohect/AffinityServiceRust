# Process Module Documentation

Process and thread enumeration via NtQuerySystemInformation.

## Overview

This module provides efficient process snapshotting using the native Windows API:
- Captures all processes and threads in single syscall
- Lazy thread parsing for memory efficiency
- Safe lifetime management of raw pointers

## Called By

- [main.rs](main.md#main-loop) - Main loop process enumeration
- [apply.rs](apply.md) - Thread iteration for affinity/ideal processor operations

## Data Structures

### ProcessSnapshot

Captures system-wide process information.

```rust
pub struct ProcessSnapshot {
    buffer: Vec<u8>,                              // Raw SYSTEM_PROCESS_INFORMATION data
    pub pid_to_process: HashMap<u32, ProcessEntry>, // Parsed process entries
}
```

**Fields:**
- `pid_to_process`: HashMap of [`ProcessEntry`](#processentry) indexed by PID

**Lifetime:** The `buffer` must outlive any references in `ProcessEntry` (threads pointer).

**Drop Implementation:** Clears both collections to release memory.

### ProcessEntry

Single process information with lazy thread parsing.

```rust
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,      // Native system structure
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>, // Parsed threads (lazy)
    threads_base_ptr: usize,                      // Raw pointer to thread array
    name: String,                                 // Process name (lowercased)
}
```

## Methods

### ProcessSnapshot::take

Captures a snapshot of all processes and threads.

```rust
pub fn take() -> Result<Self, i32>
```

**Algorithm:**
1. Start with 1KB buffer
2. Call `NtQuerySystemInformation(SystemProcessInformation, ...)`
3. If `STATUS_INFO_LENGTH_MISMATCH`, grow buffer and retry
4. Parse linked list of `SYSTEM_PROCESS_INFORMATION` structures
5. Create `ProcessEntry` for each process

**Error:** Returns negative NTSTATUS on failure.

**Example:**
```rust
match ProcessSnapshot::take() {
    Ok(processes) => {
        for (pid, entry) in &processes.pid_to_process {
            println!("{}: {}", pid, entry.get_name());
        }
    }
    Err(status) => eprintln!("Failed: 0x{:08X}", status),
}
```

### ProcessSnapshot::get_by_name

Find processes by name (case-insensitive).

```rust
pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry>
```

**Note:** Returns references bound to snapshot lifetime.

### ProcessEntry::new

Creates ProcessEntry from raw system data.

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION
) -> Self
```

**Safety:** `threads_base_ptr` must remain valid (points into snapshot buffer).

### ProcessEntry::get_threads

Returns thread information map, lazily populating on first call.

```rust
#[inline]
pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

**Lazy Loading:**
- First call: Parses thread array from raw pointer into HashMap
- Subsequent calls: Returns cached HashMap

**Thread Structure:**
```rust
SYSTEM_THREAD_INFORMATION {
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    CreateTime: LARGE_INTEGER,
    WaitTime: ULONG,
    StartAddress: PVOID,
    ClientId: CLIENT_ID { UniqueProcess, UniqueThread },
    Priority: KPRIORITY,
    BasePriority: LONG,
    ContextSwitches: ULONG,
    ThreadState: THREAD_STATE,
    WaitReason: KWAIT_REASON,
    // ... additional fields
}
```

### ProcessEntry::get_thread

Get single thread by TID.

```rust
#[inline]
pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION>
```

### ProcessEntry::get_name

Get process name (lowercase).

```rust
#[inline]
pub fn get_name(&self) -> &str
```

### ProcessEntry::get_name_original_case

Get process name (original case from system).

```rust
#[inline]
pub fn get_name_original_case(&self) -> String
```

### ProcessEntry::pid

Get process ID.

```rust
#[inline]
pub fn pid(&self) -> u32
```

### ProcessEntry::thread_count

Get number of threads.

```rust
#[inline]
pub fn thread_count(&self) -> u32
```

## SYSTEM_PROCESS_INFORMATION Structure

Key fields used:

| Field | Description |
|-------|-------------|
| `NextEntryOffset` | Offset to next process (0 = last) |
| `NumberOfThreads` | Thread count |
| `ImageName` | UNICODE_STRING with process name |
| `UniqueProcessId` | Process ID |
| `Threads[]` | Inline array of thread info structures |

## Safety Considerations

### Raw Pointer Handling

The module uses unsafe code for:
1. Calling `NtQuerySystemInformation`
2. Dereferencing `SYSTEM_PROCESS_INFORMATION` pointers
3. Iterating thread arrays

**Invariants Maintained:**
- Buffer is sized correctly before parsing
- Pointers are valid while snapshot exists
- Null checks before dereferencing

### Memory Layout

The Windows API returns a linked list in a contiguous buffer:
```
[Process1][Gap][Process2][Gap][Process3][Gap]...
     ↓
 [Threads1...]
```

The `Threads` array is inline within each process structure.

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `take()` | O(P + T) | Single syscall, P=processes, T=threads |
| `get_threads()` | O(T_p) | Lazy, T_p=threads in process |
| `pid()` | O(1) | Direct field access |
| `get_name()` | O(1) | Cached string reference |

## Comparison with ToolHelp

| Feature | ProcessSnapshot | ToolHelp32 |
|---------|-----------------|------------|
| Syscalls | 1 | 1 + N (processes) + M (threads) |
| Atomcity | Single snapshot | Race conditions possible |
| Memory | Buffer size (~1MB) | Minimal |
| Speed | Faster | Slower for many threads |
| Info | Full SYSTEM_THREAD_INFORMATION | Limited |

## Dependencies

- `ntapi::ntexapi` - `NtQuerySystemInformation`, structures
- `std::collections::HashMap` - Process/thread lookup
- `std::slice` - UNICODE_STRING parsing

## Platform Requirements

- Windows XP or later
- No special privileges required for query
- Returns limited info for some system processes without elevation
