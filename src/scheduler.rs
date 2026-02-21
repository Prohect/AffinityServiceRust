//! Prime Thread Scheduler for dynamic thread-to-core assignment.
//!
//! This module implements a scheduler that dynamically assigns the most
//! CPU-intensive threads to preferred "prime" cores on hybrid CPUs.

use crate::{
    config::ConfigConstants,
    logging::log_message,
    priority::ThreadPriority,
    winapi::{clear_module_cache, resolve_address_to_module},
};
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
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
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

    pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String) {
        let stats = self.pid_to_process_stats.entry(pid).or_insert_with(ProcessStats::new);
        stats.track_top_x_threads = track_top_x_threads;
        stats.process_name = process_name;
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
        self.pid_to_process_stats.retain(|pid, process_stats| {
            if !process_stats.alive {
                if process_stats.track_top_x_threads != 0 {
                    let x = process_stats.track_top_x_threads.abs() as usize;
                    let mut threads: Vec<(&u32, &ThreadStats)> = process_stats.tid_to_thread_stats.iter().collect();
                    threads.sort_by(|a, b| b.1.last_cycles.cmp(&a.1.last_cycles));

                    let top_x = threads.into_iter().take(x);
                    let mut report = format!("Process {} ({}) exited. Top {} threads by CPU cycles:\n", process_stats.process_name, pid, x);
                    for (i, (tid, stats)) in top_x.enumerate() {
                        let module_name = resolve_address_to_module(*pid, stats.start_address);
                        report.push_str(&format!(
                            "  [{}] TID: {} | Cycles: {} | StartAddress: 0x{:X} ({})\n",
                            i + 1,
                            tid,
                            stats.last_cycles,
                            stats.start_address,
                            module_name
                        ));
                        if let Some(info) = &stats.last_system_thread_info {
                            let kernel_time = unsafe { *info.KernelTime.QuadPart() };
                            let user_time = unsafe { *info.UserTime.QuadPart() };
                            let create_time = unsafe { *info.CreateTime.QuadPart() };
                            report.push_str(&format!("    KernelTime: {}\n", format_100ns(kernel_time)));
                            report.push_str(&format!("    UserTime: {}\n", format_100ns(user_time)));
                            report.push_str(&format!("    CreateTime: {}\n", format_filetime(create_time)));
                            report.push_str(&format!("    WaitTime: {}\n", info.WaitTime));
                            report.push_str(&format!(
                                "    ClientId: PID {:?}, TID {:?}\n",
                                info.ClientId.UniqueProcess, info.ClientId.UniqueThread
                            ));
                            report.push_str(&format!("    Priority: {}\n", info.Priority));
                            report.push_str(&format!("    BasePriority: {}\n", info.BasePriority));
                            report.push_str(&format!("    ContextSwitches: {}\n", info.ContextSwitches));
                            report.push_str(&format!("    ThreadState: {}\n", info.ThreadState));
                            report.push_str(&format!("    WaitReason: {}\n", info.WaitReason));
                        }
                    }
                    log_message(&report);
                }

                clear_module_cache(*pid);
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
    pub track_top_x_threads: i32,
    pub process_name: String,
}

impl ProcessStats {
    pub fn new() -> Self {
        Self {
            alive: true,
            tid_to_thread_stats: HashMap::new(),
            track_top_x_threads: 0,
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
    pub last_system_thread_info: Option<ntapi::ntexapi::SYSTEM_THREAD_INFORMATION>,
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
            last_system_thread_info: None,
        }
    }
}

impl Default for ThreadStats {
    fn default() -> Self {
        Self::new()
    }
}
fn format_100ns(time: i64) -> String {
    let seconds = time / 10_000_000;
    let ms = (time % 10_000_000) / 10_000;
    format!("{}.{:03} s", seconds, ms)
}

fn format_filetime(time: i64) -> String {
    let unix_time = time / 10_000_000 - 11644473600;
    if let Some(dt) = chrono::DateTime::from_timestamp(unix_time, ((time % 10_000_000) * 100) as u32) {
        dt.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    } else {
        time.to_string()
    }
}
