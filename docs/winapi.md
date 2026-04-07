# WinAPI Module Documentation

Windows API wrappers and utility functions.

## Overview

This module provides safe wrappers around Windows APIs:
- Process/thread handle management
- Privilege management
- CPU Set operations
- Module enumeration and address resolution
- UAC elevation

## Called By

- `main.rs` - UAC elevation, timer resolution, privilege enabling
- `apply.rs` - Process/thread handles, CPU Sets, ideal processors
- `scheduler.rs` - Module resolution for thread tracking
- `logging.rs` - Error tracking

## NTDLL Imports

Raw NT API functions imported from ntdll.dll:

```rust
#[link(name = "ntdll")]
unsafe extern "system" {
    pub fn NtQueryInformationProcess(...)
    pub fn NtQueryInformationThread(...)
    pub fn NtSetInformationProcess(...)
    pub fn NtSetTimerResolution(...)
}
```

**Called By:**
- `apply_io_priority()` - Process I/O priority query/set
- `get_thread_start_address()` - Thread start address query
- `main.rs` - Timer resolution setting

## Data Structures

### ProcessHandle

Safe handle container with automatic cleanup.

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,      // PROCESS_QUERY_LIMITED_INFORMATION
    pub r_handle: Option<HANDLE>,      // PROCESS_QUERY_INFORMATION (optional)
    pub w_limited_handle: HANDLE,      // PROCESS_SET_LIMITED_INFORMATION
    pub w_handle: Option<HANDLE>,      // PROCESS_SET_INFORMATION (optional)
}
```

**Drop Implementation:** Closes all valid handles automatically.

**Note:** Limited handles are always present. Full handles may be `None` for protected processes or without elevation.

### CpuSetData

CPU Set ID to logical processor mapping.

```rust
pub struct CpuSetData {
    id: u32,                      // CPU Set ID (Windows internal)
    logical_processor_index: u8,  // Logical processor number (0, 1, 2...)
}
```

## Handle Management

### get_process_handle

Opens a process handle with appropriate access rights.

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

**Access Rights Requested:**
1. `PROCESS_QUERY_LIMITED_INFORMATION` (always)
2. `PROCESS_SET_LIMITED_INFORMATION` (always)
3. `PROCESS_QUERY_INFORMATION` (best effort)
4. `PROCESS_SET_INFORMATION` (best effort)

**Error Mapping:** Internal error codes for `is_new_error()`:
- `0` - `PROCESS_QUERY_LIMITED_INFORMATION` failed
- `1` - `PROCESS_SET_LIMITED_INFORMATION` failed
- `2` - `PROCESS_QUERY_INFORMATION` failed (warning)
- `3` - `PROCESS_SET_INFORMATION` failed (warning)

**Called By:** `apply_config()` in `main.rs`

## CPU Set Operations

### get_cpu_set_information

Returns global CPU Set information.

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

**Lazy Initialization:** First call queries system via `GetSystemCpuSetInformation`.

**Caching:** Results cached for process lifetime.

### cpusetids_from_indices

Converts logical CPU indices to CPU Set IDs.

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
```

**Example:**
```rust
cpusetids_from_indices(&[0, 1, 2])  // → [cpu_set_id_0, cpu_set_id_1, cpu_set_id_2]
```

**Called By:**
- `apply_process_default_cpuset()` - Process CPU Sets
- `apply_prime_threads_promote()` - Thread CPU Sets

### indices_from_cpusetids

Converts CPU Set IDs back to logical indices.

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
```

**Called By:** `apply.rs` - Logging CPU Set changes

### filter_indices_by_mask

Filters CPU indices to only those allowed by affinity mask.

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
```

**Purpose:** Ensures prime CPU assignments respect process affinity.

**Example:**
```rust
filter_indices_by_mask(&[0, 1, 2, 3], 0x05)  // → [0, 2] (bits 0 and 2 set)
```

## Privilege Management

### is_running_as_admin

Checks if current process has elevated token.

```rust
pub fn is_running_as_admin() -> bool
```

**Method:** Opens process token and queries `TokenElevation`.

### request_uac_elevation

Restarts process with administrator privileges.

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

**Mechanism:** Spawns PowerShell with `Start-Process -Verb RunAs`.

**Arguments:**
- `console` - Whether console output was requested (for warning message)

**Side Effect:** Current process exits after spawning elevated child.

**Called By:** `main.rs` on startup if not admin and `-noUAC` not set

### enable_debug_privilege

Enables `SeDebugPrivilege` for current process.

```rust
pub fn enable_debug_privilege()
```

**Purpose:** Allows access to protected processes for reading thread start addresses.

**Steps:**
1. Open process token with `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY`
2. Lookup `SE_DEBUG_NAME` LUID
3. Adjust token privileges to enable

**Called By:** `main.rs` on startup (unless `-noDebugPriv`)

### enable_inc_base_priority_privilege

Enables `SeIncreaseBasePriorityPrivilege`.

```rust
pub fn enable_inc_base_priority_privilege()
```

**Purpose:** Required for I/O Priority "high" setting.

**Called By:** `main.rs` on startup (unless `-noIncBasePriority`)

## Affinity Utilities

### is_affinity_unset

Checks if process has default affinity (all system CPUs).

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

**Returns:** `true` if `current_mask == system_mask`

**Called By:** `main.rs` `-find` mode to identify unconfigured processes

**Error Handling:** Logs access denied errors to `.find.log` once per process name.

## Thread Operations

### get_thread_start_address

Queries thread start address via NT API.

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

**Returns:** Start address or 0 on failure

**Info Class:** 9 (`ThreadQuerySetWin32StartAddress`)

**Called By:** `prefetch_all_thread_cycles()` for module identification

### set_thread_ideal_processor_ex

Sets preferred processor for thread.

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8
) -> Result<PROCESSOR_NUMBER, Error>
```

**Returns:** Previous ideal processor

**Called By:**
- `reset_thread_ideal_processors()` - After affinity change
- `apply_ideal_processors()` - Ideal processor assignment

### get_thread_ideal_processor_ex

Gets current ideal processor.

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

**Called By:** `apply_ideal_processors()` for lazy set optimization

## Module Resolution

### resolve_address_to_module

Maps memory address to module name with offset.

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

**Returns:**
- `"module.dll+0xABC"` - If address within known module
- `"0x7FF12345"` - If not in any module
- `"0x0"` - If address is 0

**Caching:** Module list cached per process in `MODULE_CACHE`.

**Called By:**
- `apply_prime_threads_promote()` - For change logging
- `apply_prime_threads_demote()` - For change logging
- `scheduler.rs` - For thread tracking report

### drop_module_cache

Clears module cache for a process.

```rust
pub fn drop_module_cache(pid: u32)
```

**Called By:**
- `main.rs` - Before prime scheduling each loop
- `scheduler.rs` - When process exits

## Process Management

### terminate_child_processes

Terminates child processes of current process.

```rust
pub fn terminate_child_processes()
```

**Purpose:** Cleanup orphaned console host processes after UAC elevation.

**Called By:** `main.rs` on startup

**Targets:** Any process with `th32ParentProcessID == current_pid`

## Static Data

### CPU_SET_INFORMATION

Lazy-initialized global CPU Set data.

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>>
```

Populated on first call to `get_cpu_set_information()`.

### MODULE_CACHE

Per-process module enumeration cache.

```rust
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>
```

Mapping: `pid → [(base_address, size, module_name), ...]`

## Dependencies

- `crate::error_codes` - Error code translation
- `crate::log` - Logging macro
- `crate::logging` - Error tracking
- `once_cell` - Lazy static initialization
- `windows` - Win32 API

## Safety Notes

This module contains extensive `unsafe` code for Windows API interop. Key invariants:

1. **Handle Validity:** All `HANDLE` values checked with `is_invalid()` before use
2. **Pointer Safety:** Module enumeration uses proper buffer sizing
3. **Lifetime Management:** `ProcessHandle` Drop impl ensures handle closure
4. **Thread Safety:** Static caches protected by `Mutex`
