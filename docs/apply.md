# Apply Module Documentation

Applies configuration settings to target processes.

## Overview

This module implements the core logic for applying process configurations:
- Process priority class
- CPU affinity (hard mask)
- CPU Sets (soft preference)
- I/O priority
- Memory priority
- Prime thread scheduling
- Ideal processor assignment

## Called By

- `apply_config()` in [main.rs](main.md) - Main orchestration function
- Internal cross-calls between apply functions

## Data Structures

### ApplyConfigResult

Collects changes and errors during config application.

```rust
pub struct ApplyConfigResult {
    pub changes: Vec<String>,   // Human-readable change descriptions
    pub errors: Vec<String>,    // Error messages with context
}
```

**Methods:**
- `new()` - Create empty result
- `add_change(change: String)` - Add change message (format: `"$operation details"`)
- `add_error(error: String)` - Add error message (format: `"$fn_name: [$operation][$error] details"`)
- `is_empty() -> bool` - Check if no changes or errors

## Apply Functions

### apply_config (in main.rs)

Orchestrates applying all configuration settings to a target process.

**Called By:** [main.rs](main.md#main-loop) main loop

**Flow:**
1. Get process handle
2. Apply priority
3. Apply affinity (captures current_mask for filtering)
4. Apply CPU Sets
5. Apply I/O priority
6. Apply memory priority
7. If prime/ideal/tracking enabled:
   - Drop module cache
   - Set alive in scheduler
   - Prefetch thread cycles
   - Apply prime threads
   - Apply ideal processors
   - Update thread stats

### apply_priority

Sets process priority class.

```rust
pub fn apply_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

**Windows API:** `GetPriorityClass`, `SetPriorityClass`

**Change Logged:** `"Priority: {old} -> {new}"`

### apply_affinity

Sets hard CPU affinity mask.

```rust
pub fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    current_mask: &mut usize,  // Output: filled with current mask
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
)
```

**Side Effect:** `current_mask` is filled with the process's current affinity mask

**Windows API:** `GetProcessAffinityMask`, `SetProcessAffinityMask`

**Change Logged:** `"Affinity: {old:#X} -> {new:#X}"`

**Post-Action:** If affinity changed, calls `reset_thread_ideal_processors()`

### reset_thread_ideal_processors

Resets ideal processors after affinity change.

```rust
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
)
```

**Purpose:** When process affinity changes, Windows may clamp thread ideal processors. This redistributes threads across new affinity CPUs.

**Algorithm:**
1. Sort threads by total CPU time (kernel + user)
2. Assign ideal processors round-robin across affinity CPUs
3. Apply random shift to avoid clumping
4. Skip assignment if already on target CPU (lazy set)

**Windows API:** `OpenThread`, `SetThreadIdealProcessorEx`

### apply_process_default_cpuset

Sets soft CPU preference via CPU Sets.

```rust
pub fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

**Windows API:** `GetProcessDefaultCpuSets`, `SetProcessDefaultCpuSets`

**Note:** Query may fail with error 122 (INSUFFICIENT_BUFFER) initially - this is expected.

### apply_io_priority

Sets I/O priority.

```rust
pub fn apply_io_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

**Windows API:** `NtQueryInformationProcess`, `NtSetInformationProcess` (class 33)

**Privilege:** I/O Priority "high" requires `SeIncreaseBasePriorityPrivilege` + admin

**Change Logged:** `"IO Priority: {old} -> {new}"`

### apply_memory_priority

Sets memory page priority.

```rust
pub fn apply_memory_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

**Windows API:** `GetProcessInformation`, `SetProcessInformation` (`ProcessMemoryPriority`)

**Change Logged:** `"Memory Priority: {old} -> {new}"`

## Prime Thread Scheduling

### prefetch_all_thread_cycles

Prefetches thread cycle counts for prime thread selection.

```rust
pub fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

**Algorithm:**
1. Get threads sorted by CPU time delta
2. Keep top N threads (N = logical CPU count × 2)
3. Open handles to threads without cached handles
4. Query `QueryThreadCycleTime` for each
5. Calculate cycle deltas
6. Update active streaks in scheduler

**Optimization:** Only opens handles for threads likely to be selected (top by CPU time).

**Windows API:** `OpenThread`, `QueryThreadCycleTime`

### apply_prime_threads

Main prime thread scheduling orchestration.

```rust
pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
)
```

**Algorithm:**
1. Set tracking info if enabled
2. Sort threads by CPU time delta
3. Select candidate pool (4× prime slots or CPU count)
4. Include previously-pinned threads for demotion check
5. Get cycle deltas for candidates
6. **Select:** Apply hysteresis to choose prime threads
7. **Promote:** Assign CPU sets and boost priority
8. **Demote:** Remove CPU sets and restore priority
9. Cleanup handles for exited threads

**Windows API:** `SetThreadSelectedCpuSets`, `SetThreadPriority`

### apply_prime_threads_select

Selects top threads using hysteresis.

```rust
pub fn apply_prime_threads_select(
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    prime_count: usize,
)
```

Calls `select_top_threads_with_hysteresis()` with `|ts| !ts.pinned_cpu_set_ids.is_empty()` as the "currently assigned" check.

### apply_prime_threads_promote

Promotes selected threads to prime status.

```rust
pub fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
)
```

**For each selected thread:**
1. Resolve start address to module name
2. Match against prefix rules
3. Filter CPUs by current affinity mask (if set)
4. Apply `SetThreadSelectedCpuSets`
5. Boost thread priority (explicit or auto)

**Change Logged:**
- `"Thread {tid} -> (promoted, [{cpus}], cycles={cycles}, start={module})"`
- `"Thread {tid} -> ({action}: {old} -> {new})"` (priority)

### apply_prime_threads_demote

Demotes threads that no longer qualify.

```rust
pub fn apply_prime_threads_demote(
    process: &mut ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
)
```

**For each thread not in selected set but with pinned CPUs:**
1. Clear CPU Set assignment (`SetThreadSelectedCpuSets` with empty)
2. Restore original thread priority
3. **Always clear** `pinned_cpu_set_ids` to prevent retry loops

**Change Logged:** `"Thread {tid} -> (demoted, start={module})"`

## Ideal Processor Assignment

### apply_ideal_processors

Assigns ideal processors to threads based on start module.

```rust
pub fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
)
```

**Algorithm per rule:**
1. Filter threads by module prefix (if specified)
2. Sort by cycle count
3. Select top N using hysteresis (same as prime scheduling)
4. For selected threads:
   - If not already on free-pool CPU, set ideal processor
   - Track assignment in `IdealProcessorState`
5. For demoted threads:
   - Restore original ideal processor

**Lazy Set Optimization:** If thread's current ideal processor is in the free pool, skip syscall.

**Windows API:** `SetThreadIdealProcessorEx`

**Change Logged:** Includes `start=module+offset` (e.g., `start=cs2.exe+0xEA60`)

## Helper Functions

### get_handles

Extracts read and write handles from ProcessHandle.

```rust
#[inline(always)]
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>)
```

Prefers full handles over limited handles.

### log_error_if_new

Logs error only if new for this pid/operation combination.

```rust
#[inline(always)]
fn log_error_if_new(
    pid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
)
```

Uses `logging::is_new_error()` for deduplication.

## Error Handling

All functions use `log_error_if_new()` to prevent log spam:
- Access denied errors logged once per (pid, process_name, operation)
- Invalid handle errors tracked separately
- Error map purged of dead PIDs each loop

## Dependencies

- `crate::config` - Config structures and CPU utilities
- `crate::error_codes` - Error code translation
- `crate::logging` - Error tracking and logging
- `crate::priority` - Priority enums
- `crate::process` - Process and thread enumeration
- `crate::scheduler` - Prime thread scheduler
- `crate::winapi` - Windows API wrappers
- `rand` - Random shift for ideal processor reset
- `windows` - Win32 API
