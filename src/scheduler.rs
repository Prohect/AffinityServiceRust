//! Prime Thread Scheduler for dynamic thread-to-core assignment.
//!
//! This module implements a scheduler that dynamically assigns the most
//! CPU-intensive threads to preferred "prime" cores on hybrid CPUs.

use crate::{
    config::{ConfigConstants, ProcessConfig},
    logging::{log_message, log_pure_message},
    priority::ThreadPriority,
    winapi::{clear_module_cache, get_cpu_set_information, resolve_address_to_module},
};
use ntapi::ntexapi::SYSTEM_THREAD_INFORMATION;
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
    pub fn set_alive(&mut self, pid: u32, name: &str) {
        let stats = self.pid_to_process_stats.entry(pid).or_insert_with(ProcessStats::new);
        stats.alive = true;
        if stats.process_name.is_empty() {
            stats.process_name = name.to_string();
        }
    }

    /// Gets or creates thread stats for the given PID/TID pair.
    #[inline]
    pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats {
        let process_stats = self.pid_to_process_stats.entry(pid).or_insert_with(ProcessStats::new);
        let is_new = !process_stats.tid_to_thread_stats.contains_key(&tid);
        let stats = process_stats.tid_to_thread_stats.entry(tid).or_insert_with(ThreadStats::new);
        if is_new {
            process_stats.total_threads_tracked += 1;
        }
        stats.seen = true;
        stats
    }

    /// Closes thread handles and removes stats for dead processes.
    pub fn close_dead_process_handles(&mut self, configs: &HashMap<String, ProcessConfig>) {
        self.pid_to_process_stats.retain(|pid, process_stats| {
            if !process_stats.alive {
                // Generate post-mortem report if monitoring was enabled
                if let Some(config) = configs.get(&process_stats.process_name) {
                    if config.prime_threads_monitor {
                        Self::report_post_mortem(*pid, process_stats, config);
                    }
                }

                clear_module_cache(*pid);
                for stats in process_stats.tid_to_thread_stats.values() {
                    if let Some(handle) = stats.handle {
                        unsafe {
                            let _ = CloseHandle(handle);
                        }
                    }
                }
            } else {
                // Cleanup dead threads within an alive process
                let mut dead_history = Vec::new();
                process_stats.tid_to_thread_stats.retain(|tid, thread_stats| {
                    if !thread_stats.seen {
                        // Thread died, record in history if it's a top thread
                        if let Some(info) = thread_stats.last_thread_info {
                            dead_history.push(ThreadHistory {
                                tid: *tid,
                                info,
                                total_cycles: thread_stats.total_cycles,
                                module_name: thread_stats.module_name.clone(),
                            });
                        }
                        if let Some(handle) = thread_stats.handle {
                            unsafe {
                                let _ = CloseHandle(handle);
                            }
                        }
                        false
                    } else {
                        thread_stats.seen = false; // Reset for next loop
                        true
                    }
                });

                if !dead_history.is_empty() {
                    if let Some(config) = configs.get(&process_stats.process_name) {
                        if config.prime_threads_monitor {
                            process_stats.top_threads_history.extend(dead_history);
                            process_stats.top_threads_history.sort_by(|a, b| b.total_cycles.cmp(&a.total_cycles));
                            let top_x = config.prime_threads_top_x.unwrap_or_else(|| get_cpu_set_information().lock().unwrap().len() * 2);
                            process_stats.top_threads_history.truncate(top_x);
                        }
                    }
                }
            }
            process_stats.alive
        });
    }

    fn report_post_mortem(pid: u32, process_stats: &mut ProcessStats, config: &ProcessConfig) {
        // Merge remaining alive threads into history for final ranking
        for (tid, thread_stats) in &process_stats.tid_to_thread_stats {
            if let Some(info) = thread_stats.last_thread_info {
                process_stats.top_threads_history.push(ThreadHistory {
                    tid: *tid,
                    info,
                    total_cycles: thread_stats.total_cycles,
                    module_name: thread_stats.module_name.clone(),
                });
            }
        }

        process_stats.top_threads_history.sort_by(|a, b| b.total_cycles.cmp(&a.total_cycles));
        let top_x = config.prime_threads_top_x.unwrap_or_else(|| get_cpu_set_information().lock().unwrap().len() * 2);
        process_stats.top_threads_history.truncate(top_x);

        if process_stats.top_threads_history.is_empty() {
            return;
        }

        log_message(&format!("=== Post-Mortem Report: {} (PID: {}) ===", process_stats.process_name, pid));
        log_pure_message(&format!("Total Threads Tracked: {}", process_stats.total_threads_tracked));
        log_pure_message(&format!("Top {} Threads by CPU Cycles:", process_stats.top_threads_history.len()));

        for (i, history) in process_stats.top_threads_history.iter().enumerate() {
            let state_str = match history.info.ThreadState {
                2 => "Running",
                5 => "Waiting",
                _ => "Other",
            };

            log_pure_message(&format!(
                "{:2}. TID: {:>5} | Cycles: {:>10.2}B | Switches: {:>8} | Start: {}",
                i + 1,
                history.tid,
                history.total_cycles as f64 / 1_000_000_000.0,
                history.info.ContextSwitches,
                history.module_name
            ));
            log_pure_message(&format!(
                "    [State: {} | Priority: {} (Base: {})]",
                state_str, history.info.Priority, history.info.BasePriority
            ));
        }
    }
}

/// A record of a thread's performance for post-mortem reporting.
pub struct ThreadHistory {
    pub tid: u32,
    pub info: SYSTEM_THREAD_INFORMATION,
    pub total_cycles: u64,
    pub module_name: String,
}

/// Per-process state for the PrimeThreadScheduler.
pub struct ProcessStats {
    /// Set to false at loop start, true when process is seen. Dead processes are cleaned up.
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    /// History of threads that have died.
    pub top_threads_history: Vec<ThreadHistory>,
    /// Total number of unique threads seen for this process.
    pub total_threads_tracked: usize,
    /// Process name for reporting.
    pub process_name: String,
}

impl ProcessStats {
    pub fn new() -> Self {
        Self {
            alive: true,
            tid_to_thread_stats: HashMap::new(),
            top_threads_history: Vec::new(),
            total_threads_tracked: 0,
            process_name: String::new(),
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
    /// Cached start address of the thread.
    pub start_address: usize,
    /// Original thread priority before promotion. None if not promoted.
    pub original_priority: Option<ThreadPriority>,
    /// Total CPU cycles consumed by this thread.
    pub total_cycles: u64,
    /// Last seen thread information from NT snapshot.
    pub last_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    /// Resolved module name and offset.
    pub module_name: String,
    /// Whether this thread was seen in the current iteration.
    pub seen: bool,
}

impl ThreadStats {
    pub fn new() -> Self {
        Self {
            last_total_time: 0,
            last_cycles: 0,
            handle: None,
            cpu_set_ids: vec![],
            active_streak: 0,
            start_address: 0,
            original_priority: None,
            total_cycles: 0,
            last_thread_info: None,
            module_name: String::new(),
            seen: true,
        }
    }

    pub fn update_from_info(&mut self, info: &SYSTEM_THREAD_INFORMATION, cycles: u64, pid: u32) {
        self.last_thread_info = Some(*info);
        self.total_cycles = cycles;
        if self.start_address == 0 {
            self.start_address = info.StartAddress as usize;
            self.module_name = resolve_address_to_module(pid, self.start_address);
        }
    }
}

impl Default for ThreadStats {
    fn default() -> Self {
        Self::new()
    }
}
