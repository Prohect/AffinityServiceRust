# Scheduler Module Documentation

Prime thread scheduler with hysteresis-based promotion/demotion.

## Overview

The `PrimeThreadScheduler` manages dynamic thread-to-CPU assignment for CPU-intensive threads. It uses:
- CPU cycle tracking per thread
- Hysteresis to prevent thrashing (entry vs keep thresholds)
- Active streak counting to filter briefly-active threads
- CPU Sets for fine-grained thread placement

## Called By

- `apply_prime_threads()` in [apply.rs](apply.md#prime-thread-scheduling) - Main scheduling entry point
- `apply_ideal_processors()` in [apply.rs](apply.md#ideal-processor-assignment) - Ideal processor assignment
- `prefetch_all_thread_cycles()` in [apply.rs](apply.md#prefetch_all_thread_cycles) - Cycle data collection
- [main.rs](main.md) - Process lifecycle management

## Data Structures

### PrimeThreadScheduler

```rust
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

**Fields:**
- `pid_to_process_stats` - Per-process thread statistics
- `constants` - Hysteresis thresholds from config

### ProcessStats

Per-process tracking data.

```rust
pub struct ProcessStats {
    pub alive: bool,                          // Process still running
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,             // Tracking mode (0=off, >0=track+prime, <0=track only)
    pub process_name: String,
    pub process_id: u32,
}
```

### ThreadStats

Per-thread tracking data.

```rust
pub struct ThreadStats {
    pub last_total_time: i64,                 // Previous total CPU time
    pub cached_total_time: i64,               // Current total CPU time (from system info)
    pub last_cycles: u64,                     // Previous cycle count
    pub cached_cycles: u64,                   // Current cycle count (from QueryThreadCycleTime)
    pub handle: Option<HANDLE>,               // Thread handle (cached)
    pub pinned_cpu_set_ids: Vec<u32>,         // Currently assigned CPU Set IDs
    pub active_streak: u8,                    // Consecutive active intervals
    pub start_address: usize,                 // Thread start address (for module identification)
    pub original_priority: Option<ThreadPriority>, // Priority before boost
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>, // Full thread info (tracking mode)
    pub ideal_processor: IdealProcessorState, // Ideal processor assignment state
    pub process_id: u32,
}
```

### IdealProcessorState

Tracks ideal processor assignments for hysteresis.

```rust
pub struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
```

## Hysteresis Algorithm

The scheduler uses two-pass hysteresis to prevent thread promotion/demotion thrashing:

### Thresholds

- `entry_threshold` (default 0.42) - Minimum % of max cycles to become candidate
- `keep_threshold` (default 0.69) - Minimum % of max cycles to stay prime (higher than entry)
- `min_active_streak` (default 2) - Consecutive active intervals before promotion

### Two-Pass Selection

**Pass 1 (Keep):** Currently-assigned threads stay assigned if cycles >= keep_threshold% of max.

**Pass 2 (Promote):** New threads are promoted if:
- Cycles >= entry_threshold% of max
- Active streak >= min_active_streak

This creates a "dead zone" between entry and keep thresholds where:
- Non-prime threads won't be promoted
- Prime threads won't be demoted
- Prevents oscillation when load is near threshold

## Methods

### new

Creates scheduler with given constants.

```rust
pub fn new(constants: ConfigConstants) -> Self
```

### reset_alive / set_alive

Lifecycle tracking for process existence.

```rust
pub fn reset_alive(&mut self)  // Mark all as dead (start of loop)
pub fn set_alive(&mut self, pid: u32)  // Mark specific process alive
```

### set_tracking_info

Configure thread tracking for a process.

```rust
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

**Parameters:**
- `track_top_x_threads` - Number of top threads to track:
  - `0` - No tracking
  - `>0` - Track and log top N on process exit
  - `<0` - Same but without prime scheduling (monitor only)

### get_thread_stats

Get or create thread statistics entry.

```rust
#[inline]
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

### update_active_streaks

Updates active streak counters based on cycle deltas.

```rust
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

**Algorithm:**
1. Find max cycles across all threads
2. Calculate entry_min = max * entry_threshold
3. Calculate keep_min = max * keep_threshold
4. For each thread:
   - If already has streak (>0) and cycles < keep_min → reset streak
   - If no streak and cycles >= entry_min → set streak to 1
   - If has streak and cycles >= keep_min → increment streak (capped at 254)

### select_top_threads_with_hysteresis

Core hysteresis selection algorithm.

```rust
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],  // (tid, cycles, is_selected)
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
) -> usize
```

**Returns:** Number of threads selected

**Parameters:**
- `tid_with_delta_cycles` - Thread data, third element is output flag
- `slot_count` - Maximum threads to select
- `is_currently_assigned` - Callback to check if thread already has resource

**Example Usage:**
```rust
// For prime scheduling
select_top_threads_with_hysteresis(pid, &mut candidates, prime_count, |ts| {
    !ts.pinned_cpu_set_ids.is_empty()  // Currently has CPU set assigned
});

// For ideal processor
select_top_threads_with_hysteresis(pid, &mut candidates, cpu_count, |ts| {
    ts.ideal_processor.is_assigned  // Currently has ideal processor assigned
});
```

### close_dead_process_handles

Cleanup for exited processes.

```rust
pub fn close_dead_process_handles(&mut self)
```

**Side Effects:**
- Logs top N threads if tracking enabled
- Clears module cache for process
- Closes all cached thread handles
- Removes process stats from map

**Tracking Output Format:**
```
Process name.exe (PID) exited. Top X threads by CPU cycles:
  [1] TID: 1234 | Cycles: 1234567890 | StartAddress: 0x7FF12345 (module.dll+0xABC)
    KernelTime: 1.234 s
    UserTime: 5.678 s
    CreateTime: 2024-01-15 10:30:45.123
    WaitTime: 0
    ClientId: PID 1234, TID 1234
    Priority: 8
    BasePriority: 8
    ContextSwitches: 12345
    ThreadState: 5
    WaitReason: 0
```

## Thread Tracking Output

When a tracked process exits, detailed statistics are logged:

| Field | Description |
|-------|-------------|
| TID | Thread ID |
| Cycles | Total CPU cycles consumed |
| StartAddress | Resolved to `module.dll+offset` format |
| KernelTime | Time in kernel mode |
| UserTime | Time in user mode |
| CreateTime | Thread creation timestamp |
| WaitTime | Wait time |
| ClientId | PID and TID |
| Priority | Current thread priority |
| BasePriority | Base priority level |
| ContextSwitches | Number of context switches |
| ThreadState | Current state |
| WaitReason | Wait reason if waiting |

## Dependencies

- `crate::config::ConfigConstants` - Threshold values
- `crate::logging::log_message` - Output
- `crate::priority::ThreadPriority` - Priority enums
- `crate::winapi` - Module resolution
- `chrono` - Time formatting
- `ntapi` - System thread information
- `windows` - Win32 API

## Algorithm Details

### Why Hysteresis?

Without hysteresis, threads near the threshold would constantly flip between prime and non-prime status, causing:
- Excessive CPU Set assignments
- Cache thrashing
- Log spam

The gap between entry (42%) and keep (69%) thresholds ensures:
- Threads must clearly exceed threshold to enter
- Threads must clearly drop below to exit
- Brief fluctuations don't cause changes

### Active Streak

The streak counter prevents promotion of threads that are only briefly active:
- Thread must be above entry threshold for N consecutive intervals
- Default N=2 means at least 2 * interval_ms of sustained activity
- Streak resets immediately when below keep threshold
