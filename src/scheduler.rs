//! Prime Thread Scheduler for dynamic thread-to-core assignment.
//!
//! This module implements a scheduler that dynamically assigns the most
//! CPU-intensive threads to preferred "prime" cores on hybrid CPUs.

use crate::config::ConfigConstants;
use std::collections::HashMap;
use windows::Win32::Foundation::{CloseHandle, HANDLE};

/// Dynamically assigns the most CPU-intensive threads to preferred "prime" cores.
///
/// On hybrid CPUs (Intel 12th gen+), this pins hot threads to P-cores while letting
/// less active threads float to E-cores.
///
/// # Algorithm
/// 1. Sort threads by CPU time delta to find candidates
/// 2. Query CPU cycles (more accurate than time on hybrid CPUs)
/// 3. Apply hysteresis to prevent thrashing:
///    - Existing prime threads stay if they exceed `keep_threshold`
///    - New threads promote only if they exceed `entry_threshold` for 2+ intervals
/// 4. Use `SetThreadSelectedCpuSets` to pin/unpin threads
///
/// PIDs and TIDs can be reused by Windows after a process exits, so we must
/// clear stats when a process dies to avoid applying stale data.
pub struct PrimeThreadScheduler {
    /// Maps PID -> ProcessStats. Cleared when process exits (PIDs can be reused by OS).
    pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}

impl PrimeThreadScheduler {
    /// Creates a new scheduler with the given constants.
    pub fn new(constants: ConfigConstants) -> Self {
        Self {
            pid_to_process_stats: HashMap::new(),
            constants,
        }
    }

    /// Resets the alive flag for all tracked processes.
    /// Called at the start of each loop iteration.
    pub fn reset_alive(&mut self) {
        self.pid_to_process_stats.values_mut().for_each(|stats| stats.alive = false);
    }

    /// Marks a process as alive for this iteration.
    pub fn set_alive(&mut self, pid: u32) {
        self.pid_to_process_stats.entry(pid).or_insert_with(ProcessStats::new).alive = true;
    }

    /// Gets or creates thread stats for the given PID/TID pair.
    #[inline]
    pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats {
        self.pid_to_process_stats
            .entry(pid)
            .or_insert_with(ProcessStats::new)
            .tid_to_thread_stats
            .entry(tid)
            .or_insert_with(ThreadStats::new)
    }

    /// Closes thread handles and removes stats for dead processes.
    pub fn close_dead_process_handles(&mut self) {
        self.pid_to_process_stats.retain(|_, process_stats| {
            if !process_stats.alive {
                for stats in process_stats.tid_to_thread_stats.values() {
                    if let Some(handle) = stats.handle {
                        unsafe {
                            let _ = CloseHandle(handle);
                        }
                    }
                }
            }
            process_stats.alive
        });
    }
}

/// Per-process state for the PrimeThreadScheduler.
pub struct ProcessStats {
    /// Set to false at loop start, true when process is seen. Dead processes are cleaned up.
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
}

impl ProcessStats {
    pub fn new() -> Self {
        Self {
            alive: true,
            tid_to_thread_stats: HashMap::new(),
        }
    }
}

impl Default for ProcessStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-thread state for the Prime Thread Scheduler.
pub struct ThreadStats {
    /// KernelTime + UserTime from last snapshot, used to calculate delta.
    pub last_total_time: i64,
    /// CPU cycles from last snapshot, used for accurate activity measurement.
    pub last_cycles: u64,
    /// Cached thread handle to avoid repeated OpenThread calls.
    pub handle: Option<HANDLE>,
    /// Current CPU set IDs assigned to this thread. Empty = not pinned (inherits from process).
    pub cpu_set_ids: Vec<u32>,
    /// Consecutive intervals this thread exceeded entry_threshold. Must reach 2 to be promoted.
    pub active_streak: u8,
}

impl ThreadStats {
    pub fn new() -> Self {
        Self {
            last_total_time: 0,
            last_cycles: 0,
            handle: None,
            cpu_set_ids: vec![],
            active_streak: 0,
        }
    }
}

impl Default for ThreadStats {
    fn default() -> Self {
        Self::new()
    }
}
