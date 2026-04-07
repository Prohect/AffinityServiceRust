# Logging Module Documentation

Logging infrastructure and error deduplication.

## Overview

This module provides:
- Dual logging (console vs file)
- Date-stamped log files
- Error deduplication to prevent log spam
- Separate `.find.log` for process discovery

## Called By

- All modules via `log!` macro
- [main.rs](main.md) - General logging
- [apply.rs](apply.md) - Error logging with deduplication
- [winapi.rs](winapi.md) - Find mode logging

## Macros

### log!

Primary logging macro.

```rust
macro_rules! log {
    ($($arg:tt)*) => { ... }
}
```

**Usage:**
```rust
log!("Process {} started", pid);
log!("Error: {}", e);
```

**Output Format:** `[HH:MM:SS] message`

**Destination:**
- Console if `use_console() == true`
- File `logs/YYYYMMDD.log` otherwise

**Dust Bin Mode:** Messages suppressed if `DUST_BIN_MODE` is true.

## Static State

### LOCALTIME_BUFFER

Shared timestamp for consistent time display.

```rust
pub static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>>
```

**Updated:** Each loop iteration in [main.rs](main.md#main-loop)

**Purpose:** Ensures all log entries in same loop share identical timestamp

### FINDS_SET

Deduplication set for `-find` mode.

```rust
static FINDS_SET: Lazy<Mutex<HashSet<String>>>
```

**Purpose:** Prevents logging same process name multiple times per session.

### FINDS_FAIL_SET

Tracks processes that failed with ACCESS_DENIED in `-find` mode.

```rust
pub static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>>
```

**Used By:** `is_affinity_unset()` - Skips retrying known-failed processes

### PID_MAP_FAIL_ENTRY_SET

Error deduplication map.

```rust
static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>
```

Structure: `pid → { (pid, name, operation, error_code) → alive }`

### DUST_BIN_MODE

Prevents logging before UAC elevation.

```rust
pub static DUST_BIN_MODE: Lazy<Mutex<bool>>
```

**Purpose:** Avoids duplicate logs when process restarts with elevation.

**Set By:** [main.rs](main.md) initially, cleared after elevation.

### USE_CONSOLE

Output mode flag.

```rust
static USE_CONSOLE: Lazy<Mutex<bool>>
```

**Set By:** `-console` CLI flag or `-validate` mode

### LOG_FILE / FIND_LOG_FILE

Lazy-initialized log file handles.

```rust
static LOG_FILE: Lazy<Mutex<File>>
static FIND_LOG_FILE: Lazy<Mutex<File>>
```

**Paths:**
- `logs/YYYYMMDD.log` - General log
- `logs/YYYYMMDD.find.log` - Find mode log

## Enums

### Operation

Operations tracked for error deduplication.

```rust
pub enum Operation {
    OpenProcess2processQueryLimitedInformation,
    OpenProcess2processSetLimitedInformation,
    OpenProcess2processQueryInformation,
    OpenProcess2processSetInformation,
    OpenThread,
    SetPriorityClass,
    GetProcessAffinityMask,
    SetProcessAffinityMask,
    GetProcessDefaultCpuSets,
    SetProcessDefaultCpuSets,
    QueryThreadCycleTime,
    SetThreadSelectedCpuSets,
    SetThreadPriority,
    NtQueryInformationProcess2ProcessInformationIOPriority,
    NtSetInformationProcess2ProcessInformationIOPriority,
    GetProcessInformation2ProcessMemoryPriority,
    SetProcessInformation2ProcessMemoryPriority,
    SetThreadIdealProcessorEx,
    GetThreadIdealProcessorEx,
    InvalidHandle,
}
```

## Error Deduplication

### is_new_error

Checks if this error hasn't been logged before.

```rust
pub fn is_new_error(
    pid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32
) -> bool
```

**Returns:** `true` if first time seeing this (pid, name, operation, code) combination

**Algorithm:**
1. Create `ApplyFailEntry` key
2. Check if entry exists in map for this PID
3. If entry exists with different process_name, clear map (PID reuse)
4. If entry exists → return `false`
5. If new → insert with `alive=true` → return `true`

**Example Usage:**
```rust
if is_new_error(pid, name, Operation::SetPriorityClass, error_code) {
    log!("Failed to set priority: {}", error_code);
}
```

### purge_fail_map

Removes stale entries from error tracking.

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, String)])
```

**Algorithm:**
1. Mark all entries as dead (`alive = false`)
2. Mark currently-running processes as alive
3. Remove all dead entries

**Called By:** [main.rs](main.md#main-loop) each loop iteration after process enumeration

**Purpose:** Prevents unbounded growth and handles PID reuse

## Logging Functions

### log_message

Primary logging function (used by `log!` macro).

```rust
pub fn log_message(args: &str)
```

**Checks:**
- Returns early if `DUST_BIN_MODE` is true
- Uses `LOCALTIME_BUFFER` for timestamp
- Routes to console or file based on `USE_CONSOLE`

### log_pure_message

Logs without timestamp.

```rust
pub fn log_pure_message(args: &str)
```

**Used For:** Continuation lines in multi-line log entries.

### log_to_find

Logs to find log file.

```rust
pub fn log_to_find(msg: &str)
```

**Output:** `[HH:MM:SS] message` to `.find.log`

**Called By:** Error functions when logging access denied for `-find` mode

### log_process_find

Logs discovered process from `-find` mode (deduplicated).

```rust
pub fn log_process_find(process_name: &str)
```

**Deduplication:** Uses `FINDS_SET` to log each process once per session.

**Output:** `[HH:MM:SS] find process.exe`

**Called By:** [main.rs](main.md#-find-flag) `-find` mode

## Accessors

### use_console

Returns reference to console flag.

```rust
pub fn use_console() -> &'static Mutex<bool>
```

### logger / find_logger

Returns reference to log file handles.

```rust
pub fn logger() -> &'static Mutex<File>
pub fn find_logger() -> &'static Mutex<File>
```

## File Paths

### get_log_path

Generates log file path based on current date.

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

**Format:** `logs/YYYYMMDD{suffix}.log`

**Examples:**
- `logs/20240115.log`
- `logs/20240115.find.log`

**Directory Creation:** Creates `logs/` if not exists.

## Error Entry Structure

### ApplyFailEntry

Key for error deduplication map.

```rust
struct ApplyFailEntry {
    pid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

**Equality:** All fields must match for entries to be considered equal.

**Hashing:** Derived from all fields.

## Usage Patterns

### Standard Logging

```rust
log!("Starting service with interval: {}ms", interval);
```

### Error Logging with Deduplication

```rust
let error_code = unsafe { GetLastError().0 };
if is_new_error(pid, name, Operation::SetAffinityMask, error_code) {
    log_to_find(&format!("Failed: {}", error_code));
}
```

### Multi-line Output

```rust
log_message(&format!("Process {}:", pid));
for line in details {
    log_pure_message(&format!("  {}", line));
}
```

### Find Mode Logging

```rust
// In main loop
if !in_configs && !blacklist.contains(&name) {
    log_process_find(&name);
}
```

## Dependencies

- `chrono` - Date/time handling
- `once_cell` - Lazy initialization
- `std::collections` - HashMap/HashSet for deduplication
- `std::fs` - File operations
- `std::io` - Write trait

## Thread Safety

All public functions are thread-safe via `Mutex` protection:
- `LOCALTIME_BUFFER` - Mutex<DateTime>
- `FINDS_SET` - Mutex<HashSet>
- `FINDS_FAIL_SET` - Mutex<HashSet>
- `PID_MAP_FAIL_ENTRY_SET` - Mutex<HashMap>
- `USE_CONSOLE` - Mutex<bool>
- `DUST_BIN_MODE` - Mutex<bool>
- `LOG_FILE` - Mutex<File>
- `FIND_LOG_FILE` - Mutex<File>

## File Rotation

Log files are date-based:
- New file created each day
- No automatic cleanup
- Files accumulate in `logs/` directory

## Performance Considerations

- `log_message()` acquires multiple locks - avoid in hot loops
- `is_new_error()` uses HashMap - O(1) lookup
- `purge_fail_map()` runs once per loop - O(n) on error count
- File writes are buffered by OS
