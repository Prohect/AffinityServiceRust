use crate::{
    config::ConfigConstants,
    logging::log_message,
    priority::ThreadPriority,
    winapi::{drop_module_cache, resolve_address_to_module},
};

use ntapi::ntexapi::SYSTEM_THREAD_INFORMATION;
use std::{cmp::Reverse, collections::HashMap};
use windows::Win32::Foundation::{CloseHandle, HANDLE};

#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}

impl PrimeThreadScheduler {
    pub fn new(constants: ConfigConstants) -> Self {
        Self {
            pid_to_process_stats: HashMap::new(),
            constants,
        }
    }

    pub fn reset_alive(&mut self) {
        self.pid_to_process_stats.values_mut().for_each(|stats| stats.alive = false);
    }

    pub fn set_alive(&mut self, pid: u32) {
        self.pid_to_process_stats.entry(pid).or_insert(ProcessStats::new(pid)).alive = true;
    }

    pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String) {
        let stats = self.pid_to_process_stats.entry(pid).or_insert(ProcessStats::new(pid));
        stats.track_top_x_threads = track_top_x_threads;
        stats.process_name = process_name;
    }

    #[inline]
    pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats {
        self.pid_to_process_stats
            .entry(pid)
            .or_insert(ProcessStats::new(pid))
            .tid_to_thread_stats
            .entry(tid)
            .or_insert(ThreadStats::new(pid))
    }

    /// Updates active streak counters for hysteresis-based thread selection.
    ///
    /// Threads accumulate "active streak" when their cycle count exceeds the entry threshold.
    /// This prevents briefly-active threads from being promoted to prime status.
    /// Streak is reset when cycles drop below the keep threshold.
    pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)]) {
        let max_cycles = tid_with_delta_cycles.iter().map(|&(_, c)| c).max().unwrap_or(0);
        let entry_min = (max_cycles as f64 * self.constants.entry_threshold) as u64;
        let keep_min = (max_cycles as f64 * self.constants.keep_threshold) as u64;
        for &(tid, delta) in tid_with_delta_cycles {
            let streak = &mut self.get_thread_stats(pid, tid).active_streak;
            if *streak > 0 {
                if delta < keep_min {
                    *streak = 0;
                    continue;
                }
                *streak = (*streak + 1).min(254);
            } else if delta >= entry_min {
                *streak = 1;
            }
        }
    }

    /// Selects top threads using hysteresis to prevent promotion/demotion thrashing.
    ///
    /// First pass: keeps currently-assigned threads if they still qualify (cycles >= keep_threshold).
    /// Second pass: fills remaining slots with new threads that have sufficient active streak.
    ///
    /// The `is_currently_assigned` callback checks if a thread already has the resource being assigned.
    pub fn select_top_threads_with_hysteresis(
        &mut self,
        pid: u32,

        tid_with_delta_cycles: &mut [(u32, u64, bool)],
        slot_count: usize,
        is_currently_assigned: fn(&ThreadStats) -> bool,
    ) {
        tid_with_delta_cycles.sort_unstable_by_key(|&(_, delta, _)| Reverse(delta));
        let max_cycles = tid_with_delta_cycles.first().map(|&(_, c, _)| c).unwrap_or(0u64);
        let entry_min = (max_cycles as f64 * self.constants.entry_threshold) as u64;
        let keep_min = (max_cycles as f64 * self.constants.keep_threshold) as u64;
        let mut slots_used = 0usize;

        // First pass: retain currently-assigned threads that still qualify
        // This prevents threads from losing prime status due to minor fluctuations
        for (tid, delta, is_prime) in tid_with_delta_cycles.iter_mut() {
            if slots_used >= slot_count {
                continue;
            }
            // Keep threshold is higher than entry threshold (hysteresis)
            if is_currently_assigned(self.get_thread_stats(pid, *tid)) && *delta >= keep_min {
                *is_prime = true;
                slots_used += 1;
            }
        }

        // Second pass: fill remaining slots with new qualifying threads
        // Requires active_streak to prevent promotion of briefly-active threads
        for (tid, delta, is_prime) in tid_with_delta_cycles.iter_mut() {
            if slots_used >= slot_count {
                break;
            }
            if *tid == 0 || *is_prime {
                continue;
            }
            if *delta >= entry_min && self.get_thread_stats(pid, *tid).active_streak >= self.constants.min_active_streak {
                *is_prime = true;
                slots_used += 1;
            }
        }
    }

    /// Cleans up resources for processes that no longer exist.
    ///
    /// Closes thread handles, clears module cache, and optionally logs
    /// top N threads by cycles for debugging/analysis purposes.
    pub fn close_dead_process_handles(&mut self) {
        self.pid_to_process_stats.retain(|pid, process_stats| {
            if !process_stats.alive {
                if process_stats.track_top_x_threads != 0 {
                    let x = process_stats.track_top_x_threads.unsigned_abs() as usize;
                    let mut threads: Vec<(&u32, &ThreadStats)> = process_stats.tid_to_thread_stats.iter().collect();
                    threads.sort_by(|a, b| b.1.last_cycles.cmp(&a.1.last_cycles));

                    let top_x = threads.into_iter().take(x);
                    let mut report = format!(
                        "Process {} ({}) exited. Top {} threads by CPU cycles:\n",
                        process_stats.process_name, pid, x
                    );
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

                drop_module_cache(*pid);
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

#[derive(Debug)]
pub struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    #[allow(dead_code)]
    pub process_id: u32,
}

impl ProcessStats {
    pub fn new(process_id: u32) -> Self {
        Self {
            alive: true,
            tid_to_thread_stats: HashMap::new(),
            track_top_x_threads: 0,
            process_name: String::new(),
            process_id,
        }
    }
}

impl Default for ProcessStats {
    fn default() -> Self {
        Self::new(0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdealProcessorState {
    pub current_group: u16,

    pub current_number: u8,

    pub previous_group: u16,

    pub previous_number: u8,

    pub is_assigned: bool,
}

impl IdealProcessorState {
    pub fn new() -> Self {
        Self {
            current_group: 0,
            current_number: 0,
            previous_group: 0,
            previous_number: 0,
            is_assigned: false,
        }
    }
}

impl Default for IdealProcessorState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ThreadStats {
    pub last_total_time: i64,

    pub cached_total_time: i64,

    pub last_cycles: u64,

    pub cached_cycles: u64,

    pub handle: Option<HANDLE>,

    pub pinned_cpu_set_ids: Vec<u32>,

    pub active_streak: u8,

    pub start_address: usize,

    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,

    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}

impl std::fmt::Debug for ThreadStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThreadStats")
            .field("last_total_time", &self.last_total_time)
            .field("cached_total_time", &self.cached_total_time)
            .field("last_cycles", &self.last_cycles)
            .field("cached_cycles", &self.cached_cycles)
            .field("handle", &self.handle)
            .field("pinned_cpu_set_ids", &self.pinned_cpu_set_ids)
            .field("active_streak", &self.active_streak)
            .field("start_address", &resolve_address_to_module(self.process_id, self.start_address))
            .field("original_priority", &self.original_priority)
            .field("ideal_processor", &self.ideal_processor)
            .finish()
    }
}

impl ThreadStats {
    pub fn new(process_id: u32) -> Self {
        Self {
            last_total_time: 0,
            cached_total_time: 0,
            last_cycles: 0,
            cached_cycles: 0,
            handle: None,
            pinned_cpu_set_ids: vec![],
            active_streak: 0,
            start_address: 0,
            original_priority: None,
            last_system_thread_info: None,
            ideal_processor: IdealProcessorState::new(),
            process_id,
        }
    }
}

impl Default for ThreadStats {
    fn default() -> Self {
        Self::new(0)
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
