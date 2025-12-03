use chrono::{DateTime, Datelike, Local};
use ntapi::ntexapi::{NtQuerySystemInformation, SYSTEM_PROCESS_INFORMATION, SYSTEM_THREAD_INFORMATION, SystemProcessInformation};
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
    mem::size_of,
    path::{Path, PathBuf},
    process::Command,
    slice,
    sync::Mutex,
    thread,
    time::Duration,
};
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE, LUID, NTSTATUS},
    Security::{
        AdjustTokenPrivileges, GetTokenInformation, LookupPrivilegeValueW, SE_DEBUG_NAME, TOKEN_ADJUST_PRIVILEGES, TOKEN_ELEVATION, TOKEN_PRIVILEGES, TOKEN_QUERY,
        TokenElevation,
    },
    System::{
        Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS},
        SystemInformation::{GetSystemCpuSetInformation, SYSTEM_CPU_SET_INFORMATION},
        Threading::{
            ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, GetCurrentProcess, GetPriorityClass, GetProcessAffinityMask, GetProcessDefaultCpuSets,
            HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, MEMORY_PRIORITY, MEMORY_PRIORITY_BELOW_NORMAL, MEMORY_PRIORITY_LOW, MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_NORMAL,
            MEMORY_PRIORITY_VERY_LOW, NORMAL_PRIORITY_CLASS, OpenProcess, OpenProcessToken, OpenThread, PROCESS_CREATION_FLAGS, PROCESS_QUERY_INFORMATION,
            PROCESS_SET_INFORMATION, ProcessMemoryPriority, REALTIME_PRIORITY_CLASS, SetPriorityClass, SetProcessAffinityMask, SetProcessDefaultCpuSets,
            SetProcessInformation, SetThreadSelectedCpuSets, THREAD_QUERY_INFORMATION, THREAD_SET_LIMITED_INFORMATION,
        },
        WindowsProgramming::QueryThreadCycleTime,
    },
};

#[link(name = "ntdll")]
unsafe extern "system" {
    fn NtQueryInformationProcess(h_prc: HANDLE, process_information_class: u32, p_out: *mut std::ffi::c_void, out_length: u32, return_length: *mut u32) -> NTSTATUS;
    fn NtSetInformationProcess(
        process_handle: HANDLE,
        process_information_class: u32,
        process_information: *const std::ffi::c_void,
        process_information_length: u32,
    ) -> NTSTATUS;
    fn NtSetTimerResolution(desired_resolution: u32, set_resolution: bool, p_current_resolution: *mut std::ffi::c_void) -> NTSTATUS;
}

const PROCESS_INFORMATION_IO_PRIORITY: u32 = 33;

fn get_log_path(suffix: &str) -> PathBuf {
    let year = LOCALTIME_BUFFER.lock().unwrap().year();
    let month = LOCALTIME_BUFFER.lock().unwrap().month();
    let day = LOCALTIME_BUFFER.lock().unwrap().day();
    let log_dir = PathBuf::from("logs");
    if !log_dir.exists() {
        let _ = fs::create_dir_all(&log_dir);
    }
    log_dir.join(format!("{:04}{:02}{:02}{}.log", year, month, day, suffix))
}
fn logger() -> &'static Mutex<File> {
    static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
    &LOG_FILE
}
fn find_logger() -> &'static Mutex<File> {
    static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
    &FIND_LOG_FILE
}
fn get_cpu_set_information() -> &'static Mutex<Vec<SYSTEM_CPU_SET_INFORMATION>> {
    static CPU_SET_INFORMATION: Lazy<Mutex<Vec<SYSTEM_CPU_SET_INFORMATION>>> = Lazy::new(|| Mutex::new(init_cpu_set_information()));
    &CPU_SET_INFORMATION
}
static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
fn use_console() -> &'static Mutex<bool> {
    static CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
    &CONSOLE
}

#[derive(Debug, Clone, Copy)]
pub struct ConfigConstants {
    pub hysteresis_ratio: f64,
    pub absolute_keep_ratio: f64,
    pub entry_threshold_ratio: f64,
}

impl Default for ConfigConstants {
    fn default() -> Self {
        Self {
            hysteresis_ratio: 1.2,
            absolute_keep_ratio: 0.7,
            entry_threshold_ratio: 0.42,
        }
    }
}
#[derive(Debug, Clone)]
struct ProcessConfig {
    name: String,
    priority: ProcessPriority,
    affinity_mask: usize,
    cpu_set_mask: usize,
    prime_cpu_mask: usize,
    io_priority: IOPriority,
    memory_priority: MemoryPriority,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}
impl ProcessPriority {
    const TABLE: &'static [(Self, &'static str, Option<PROCESS_CREATION_FLAGS>)] = &[
        (Self::None, "none", None),
        (Self::Idle, "idle", Some(IDLE_PRIORITY_CLASS)),
        (Self::BelowNormal, "below normal", Some(BELOW_NORMAL_PRIORITY_CLASS)),
        (Self::Normal, "normal", Some(NORMAL_PRIORITY_CLASS)),
        (Self::AboveNormal, "above normal", Some(ABOVE_NORMAL_PRIORITY_CLASS)),
        (Self::High, "high", Some(HIGH_PRIORITY_CLASS)),
        (Self::Realtime, "real time", Some(REALTIME_PRIORITY_CLASS)),
    ];
    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("fail as str")
    }
    pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).and_then(|(_, _, val)| *val)
    }
    pub fn from_str(s: &str) -> Self {
        Self::TABLE
            .iter()
            .find(|(_, name, _)| s.to_lowercase() == *name)
            .map(|(v, _, _)| *v)
            .unwrap_or(Self::None)
    }
}

struct ThreadStats {
    last_total_time: i64,
    last_cycles: u64,
    handle: Option<HANDLE>, // Only present if thread is a VIP (selected)
    cpu_set_id: Option<u32>,
}

impl ThreadStats {
    pub fn new() -> Self {
        Self {
            last_total_time: 0,
            last_cycles: 0,
            handle: None,
            cpu_set_id: None,
        }
    }
}

struct ProcessStats {
    alive: bool,
    tid_to_thread_stats: HashMap<u32, ThreadStats>,
}

impl ProcessStats {
    pub fn new() -> Self {
        Self {
            alive: true,
            tid_to_thread_stats: HashMap::new(),
        }
    }
}

struct PrimeCoreScheduler {
    // clear threadStats once process exit as pid may be reused by OS
    pid_to_process_stats: HashMap<u32, ProcessStats>,
    constants: ConfigConstants,
}

impl PrimeCoreScheduler {
    fn new(constants: ConfigConstants) -> Self {
        Self {
            pid_to_process_stats: HashMap::new(),
            constants,
        }
    }

    fn reset_alive(&mut self) {
        // set alive to false every loop start
        self.pid_to_process_stats.values_mut().for_each(|stats| stats.alive = false);
    }

    fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats {
        let process_stats = self.pid_to_process_stats.entry(pid).or_insert_with(ProcessStats::new);
        process_stats.alive = true;
        process_stats.tid_to_thread_stats.entry(tid).or_insert_with(ThreadStats::new)
    }

    fn close_dead_process_handles(&mut self) {
        self.pid_to_process_stats.retain(|_, process_stats| {
            if !process_stats.alive {
                for stats in process_stats.tid_to_thread_stats.values() {
                    if let Some(handle) = stats.handle {
                        unsafe {
                            let _ = CloseHandle(handle);
                        }
                    }
                }
                false
            } else {
                true
            }
        });
    }
}
#[derive(Clone)]
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
}
impl ProcessEntry {
    pub fn new(process: SYSTEM_PROCESS_INFORMATION, threads_base_ptr: *const SYSTEM_THREAD_INFORMATION) -> Self {
        ProcessEntry {
            process,
            threads: HashMap::new(),
            threads_base_ptr: threads_base_ptr as usize,
        }
    }
    pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION> {
        if self.process.NumberOfThreads as usize != self.threads.len() {
            unsafe {
                let threads_ptr = self.threads_base_ptr as *const SYSTEM_THREAD_INFORMATION;
                if threads_ptr.is_null() {
                    return &self.threads;
                }
                for i in 0..self.process.NumberOfThreads as usize {
                    let thread = *threads_ptr.add(i);
                    self.threads.insert(thread.ClientId.UniqueThread as u32, thread);
                }
            }
        }
        &self.threads
    }

    pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION> {
        self.get_threads();
        self.threads.get(&tid)
    }
    pub fn get_name(&self) -> String {
        unsafe {
            if self.process.ImageName.Length > 0 && !self.process.ImageName.Buffer.is_null() {
                let wchar_count = (self.process.ImageName.Length / 2) as usize;
                let wide_slice = slice::from_raw_parts(self.process.ImageName.Buffer, wchar_count);
                String::from_utf16_lossy(wide_slice).to_lowercase()
            } else {
                String::new()
            }
        }
    }
    pub fn get_name_original_case(&self) -> String {
        unsafe {
            if self.process.ImageName.Length > 0 && !self.process.ImageName.Buffer.is_null() {
                let wchar_count = (self.process.ImageName.Length / 2) as usize;
                let wide_slice = slice::from_raw_parts(self.process.ImageName.Buffer, wchar_count);
                String::from_utf16_lossy(wide_slice)
            } else {
                String::new()
            }
        }
    }
    #[inline]
    pub fn pid(&self) -> u32 {
        self.process.UniqueProcessId as usize as u32
    }
    #[inline]
    pub fn ppid(&self) -> u32 {
        self.process.InheritedFromUniqueProcessId as usize as u32
    }
    #[inline]
    pub fn session_id(&self) -> u32 {
        self.process.SessionId
    }
    #[inline]
    pub fn unique_process_key(&self) -> usize {
        self.process.UniqueProcessKey
    }
    #[inline]
    pub fn thread_count(&self) -> u32 {
        self.process.NumberOfThreads
    }
    #[inline]
    pub fn thread_count_high_watermark(&self) -> u32 {
        self.process.NumberOfThreadsHighWatermark
    }
    #[inline]
    pub fn handle_count(&self) -> u32 {
        self.process.HandleCount
    }
    #[inline]
    pub fn base_priority(&self) -> i32 {
        self.process.BasePriority
    }
    /// (100ns ticks since 1601-01-01)
    #[inline]
    pub fn create_time(&self) -> i64 {
        unsafe { *self.process.CreateTime.QuadPart() }
    }
    #[inline]
    pub fn user_time(&self) -> i64 {
        unsafe { *self.process.UserTime.QuadPart() }
    }
    #[inline]
    pub fn kernel_time(&self) -> i64 {
        unsafe { *self.process.KernelTime.QuadPart() }
    }
    #[inline]
    pub fn total_time(&self) -> i64 {
        self.user_time() + self.kernel_time()
    }
    #[inline]
    pub fn user_time_secs(&self) -> f64 {
        self.user_time() as f64 / 10_000_000.0
    }
    #[inline]
    pub fn kernel_time_secs(&self) -> f64 {
        self.kernel_time() as f64 / 10_000_000.0
    }
    #[inline]
    pub fn total_time_secs(&self) -> f64 {
        self.total_time() as f64 / 10_000_000.0
    }
    #[inline]
    pub fn cycle_time(&self) -> u64 {
        self.process.CycleTime
    }
    #[inline]
    pub fn working_set_size(&self) -> usize {
        self.process.WorkingSetSize
    }
    #[inline]
    pub fn working_set_size_kb(&self) -> usize {
        self.process.WorkingSetSize / 1024
    }
    #[inline]
    pub fn peak_working_set_size(&self) -> usize {
        self.process.PeakWorkingSetSize
    }
    #[inline]
    pub fn peak_working_set_size_kb(&self) -> usize {
        self.process.PeakWorkingSetSize / 1024
    }
    #[inline]
    pub fn working_set_private_size(&self) -> i64 {
        unsafe { *self.process.WorkingSetPrivateSize.QuadPart() }
    }
    #[inline]
    pub fn working_set_private_size_kb(&self) -> i64 {
        self.working_set_private_size() / 1024
    }
    #[inline]
    pub fn virtual_size(&self) -> usize {
        self.process.VirtualSize
    }
    #[inline]
    pub fn virtual_size_mb(&self) -> usize {
        self.process.VirtualSize / (1024 * 1024)
    }
    #[inline]
    pub fn peak_virtual_size(&self) -> usize {
        self.process.PeakVirtualSize
    }
    #[inline]
    pub fn peak_virtual_size_mb(&self) -> usize {
        self.process.PeakVirtualSize / (1024 * 1024)
    }
    #[inline]
    pub fn pagefile_usage(&self) -> usize {
        self.process.PagefileUsage
    }
    #[inline]
    pub fn pagefile_usage_kb(&self) -> usize {
        self.process.PagefileUsage / 1024
    }
    #[inline]
    pub fn peak_pagefile_usage(&self) -> usize {
        self.process.PeakPagefileUsage
    }
    #[inline]
    pub fn peak_pagefile_usage_kb(&self) -> usize {
        self.process.PeakPagefileUsage / 1024
    }
    #[inline]
    pub fn private_page_count(&self) -> usize {
        self.process.PrivatePageCount
    }
    #[inline]
    pub fn private_page_count_kb(&self) -> usize {
        self.process.PrivatePageCount / 1024
    }
    #[inline]
    pub fn page_fault_count(&self) -> u32 {
        self.process.PageFaultCount
    }
    #[inline]
    pub fn hard_fault_count(&self) -> u32 {
        self.process.HardFaultCount
    }
    #[inline]
    pub fn quota_paged_pool_usage(&self) -> usize {
        self.process.QuotaPagedPoolUsage
    }
    #[inline]
    pub fn quota_paged_pool_usage_kb(&self) -> usize {
        self.process.QuotaPagedPoolUsage / 1024
    }
    #[inline]
    pub fn quota_peak_paged_pool_usage(&self) -> usize {
        self.process.QuotaPeakPagedPoolUsage
    }
    #[inline]
    pub fn quota_peak_paged_pool_usage_kb(&self) -> usize {
        self.process.QuotaPeakPagedPoolUsage / 1024
    }
    #[inline]
    pub fn quota_non_paged_pool_usage(&self) -> usize {
        self.process.QuotaNonPagedPoolUsage
    }
    #[inline]
    pub fn quota_non_paged_pool_usage_kb(&self) -> usize {
        self.process.QuotaNonPagedPoolUsage / 1024
    }
    #[inline]
    pub fn quota_peak_non_paged_pool_usage(&self) -> usize {
        self.process.QuotaPeakNonPagedPoolUsage
    }
    #[inline]
    pub fn quota_peak_non_paged_pool_usage_kb(&self) -> usize {
        self.process.QuotaPeakNonPagedPoolUsage / 1024
    }
    #[inline]
    pub fn read_operation_count(&self) -> i64 {
        unsafe { *self.process.ReadOperationCount.QuadPart() }
    }
    #[inline]
    pub fn write_operation_count(&self) -> i64 {
        unsafe { *self.process.WriteOperationCount.QuadPart() }
    }
    #[inline]
    pub fn other_operation_count(&self) -> i64 {
        unsafe { *self.process.OtherOperationCount.QuadPart() }
    }
    #[inline]
    pub fn total_operation_count(&self) -> i64 {
        self.read_operation_count() + self.write_operation_count() + self.other_operation_count()
    }
    #[inline]
    pub fn read_transfer_count(&self) -> i64 {
        unsafe { *self.process.ReadTransferCount.QuadPart() }
    }
    #[inline]
    pub fn read_transfer_count_kb(&self) -> i64 {
        self.read_transfer_count() / 1024
    }
    #[inline]
    pub fn write_transfer_count(&self) -> i64 {
        unsafe { *self.process.WriteTransferCount.QuadPart() }
    }
    #[inline]
    pub fn write_transfer_count_kb(&self) -> i64 {
        self.write_transfer_count() / 1024
    }
    #[inline]
    pub fn other_transfer_count(&self) -> i64 {
        unsafe { *self.process.OtherTransferCount.QuadPart() }
    }
    #[inline]
    pub fn other_transfer_count_kb(&self) -> i64 {
        self.other_transfer_count() / 1024
    }
    #[inline]
    pub fn total_transfer_count(&self) -> i64 {
        self.read_transfer_count() + self.write_transfer_count() + self.other_transfer_count()
    }
    #[inline]
    pub fn total_transfer_count_kb(&self) -> i64 {
        self.total_transfer_count() / 1024
    }
}

pub trait ThreadInfoExt {
    fn tid(&self) -> u32;
    fn pid(&self) -> u32;

    /// (100ns)
    fn kernel_time(&self) -> i64;
    fn user_time(&self) -> i64;
    fn total_time(&self) -> i64;
    fn create_time(&self) -> i64;
    fn wait_time_ms(&self) -> u32;

    fn kernel_time_secs(&self) -> f64;
    fn user_time_secs(&self) -> f64;
    fn total_time_secs(&self) -> f64;

    fn start_address(&self) -> usize;

    fn priority(&self) -> i32;
    fn base_priority(&self) -> i32;

    fn context_switches(&self) -> u32;
    fn state(&self) -> u32;
    fn wait_reason(&self) -> u32;

    fn is_running(&self) -> bool;
    fn is_ready(&self) -> bool;
    fn is_waiting(&self) -> bool;
    fn is_runnable(&self) -> bool;

    fn state_str(&self) -> &'static str;
    fn wait_reason_str(&self) -> &'static str;
}

impl ThreadInfoExt for SYSTEM_THREAD_INFORMATION {
    #[inline]
    fn tid(&self) -> u32 {
        self.ClientId.UniqueThread as usize as u32
    }
    #[inline]
    fn pid(&self) -> u32 {
        self.ClientId.UniqueProcess as usize as u32
    }
    #[inline]
    fn kernel_time(&self) -> i64 {
        unsafe { *self.KernelTime.QuadPart() }
    }
    #[inline]
    fn user_time(&self) -> i64 {
        unsafe { *self.UserTime.QuadPart() }
    }
    #[inline]
    fn total_time(&self) -> i64 {
        self.kernel_time() + self.user_time()
    }
    #[inline]
    fn create_time(&self) -> i64 {
        unsafe { *self.CreateTime.QuadPart() }
    }
    #[inline]
    fn wait_time_ms(&self) -> u32 {
        self.WaitTime
    }
    #[inline]
    fn kernel_time_secs(&self) -> f64 {
        self.kernel_time() as f64 / 10_000_000.0
    }
    #[inline]
    fn user_time_secs(&self) -> f64 {
        self.user_time() as f64 / 10_000_000.0
    }
    #[inline]
    fn total_time_secs(&self) -> f64 {
        self.total_time() as f64 / 10_000_000.0
    }
    #[inline]
    fn start_address(&self) -> usize {
        self.StartAddress as usize
    }
    #[inline]
    fn priority(&self) -> i32 {
        self.Priority
    }
    #[inline]
    fn base_priority(&self) -> i32 {
        self.BasePriority
    }
    #[inline]
    fn context_switches(&self) -> u32 {
        self.ContextSwitches
    }
    #[inline]
    fn state(&self) -> u32 {
        self.ThreadState
    }
    #[inline]
    fn wait_reason(&self) -> u32 {
        self.WaitReason
    }
    #[inline]
    fn is_running(&self) -> bool {
        self.ThreadState == 2
    }
    #[inline]
    fn is_ready(&self) -> bool {
        self.ThreadState == 1
    }
    #[inline]
    fn is_waiting(&self) -> bool {
        self.ThreadState == 5
    }
    #[inline]
    fn is_runnable(&self) -> bool {
        matches!(self.ThreadState, 1 | 2 | 3 | 7)
    }

    fn state_str(&self) -> &'static str {
        match self.ThreadState {
            0 => "Initialized",
            1 => "Ready",
            2 => "Running",
            3 => "Standby",
            4 => "Terminated",
            5 => "Waiting",
            6 => "Transition",
            7 => "DeferredReady",
            8 => "GateWaitObsolete",
            9 => "WaitingForProcessInSwap",
            _ => "Unknown",
        }
    }

    fn wait_reason_str(&self) -> &'static str {
        match self.WaitReason {
            0 => "Executive",
            1 => "FreePage",
            2 => "PageIn",
            3 => "PoolAllocation",
            4 => "DelayExecution",
            5 => "Suspended",
            6 => "UserRequest",
            7 => "WrExecutive",
            8 => "WrFreePage",
            9 => "WrPageIn",
            10 => "WrPoolAllocation",
            11 => "WrDelayExecution",
            12 => "WrSuspended",
            13 => "WrUserRequest",
            14 => "WrEventPair",
            15 => "WrQueue",
            16 => "WrLpcReceive",
            17 => "WrLpcReply",
            18 => "WrVirtualMemory",
            19 => "WrPageOut",
            20 => "WrRendezvous",
            21 => "WrKeyedEvent",
            22 => "WrTerminated",
            23 => "WrProcessInSwap",
            24 => "WrCpuRateControl",
            25 => "WrCalloutStack",
            26 => "WrKernel",
            27 => "WrResource",
            28 => "WrPushLock",
            29 => "WrMutex",
            30 => "WrQuantumEnd",
            31 => "WrDispatchInt",
            32 => "WrPreempted",
            33 => "WrYieldExecution",
            34 => "WrFastMutex",
            35 => "WrGuardedMutex",
            36 => "WrRundown",
            37 => "WrAlertByThreadId",
            38 => "WrDeferredPreempt",
            _ => "Unknown",
        }
    }
}
#[allow(dead_code)]
pub struct ProcessSnapshot {
    ///is used to store the snapshot of the processes, parsed in unsafe
    buffer: Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}

impl ProcessSnapshot {
    pub fn take() -> Result<Self, i32> {
        let mut buf_len: usize = 512 * 1024;
        let mut buffer: Vec<u8>;
        let mut return_len: u32 = 0;

        unsafe {
            loop {
                buffer = vec![0u8; buf_len];
                let status = NtQuerySystemInformation(SystemProcessInformation, buffer.as_mut_ptr() as *mut _, buf_len as u32, &mut return_len);

                // STATUS_INFO_LENGTH_MISMATCH = 0xC0000004
                const STATUS_INFO_LENGTH_MISMATCH: i32 = -1073741820i32;

                if status == STATUS_INFO_LENGTH_MISMATCH {
                    buf_len = if return_len > 0 { return_len as usize + 8192 } else { buf_len * 2 };
                    continue;
                }

                if status < 0 {
                    return Err(status);
                }

                buffer.truncate(return_len as usize);
                break;
            }

            let mut by_pid: HashMap<u32, ProcessEntry> = HashMap::new();
            let mut offset: usize = 0;
            let buf_ptr = buffer.as_ptr();

            loop {
                let process_entry_ptr = buf_ptr.add(offset) as *const SYSTEM_PROCESS_INFORMATION;
                let entry = &*process_entry_ptr;
                let threads_ptr = &(*process_entry_ptr).Threads as *const SYSTEM_THREAD_INFORMATION;
                let process_entry = ProcessEntry::new(*entry, threads_ptr);
                by_pid.insert(entry.UniqueProcessId as u32, process_entry);
                if entry.NextEntryOffset == 0 {
                    break;
                }
                offset += entry.NextEntryOffset as usize;
            }

            Ok(ProcessSnapshot { buffer, pid_to_process: by_pid })
        }
    }
    pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry> {
        self.pid_to_process.values().filter(|entry| (**entry).get_name() == name).collect()
    }
}

pub fn thread_state_str(state: u32) -> &'static str {
    match state {
        0 => "Initialized",
        1 => "Ready",
        2 => "Running",
        3 => "Standby",
        4 => "Terminated",
        5 => "Waiting",
        6 => "Transition",
        7 => "DeferredReady",
        8 => "GateWaitObsolete",
        9 => "WaitingForProcessInSwap",
        _ => "Unknown",
    }
}

pub fn wait_reason_str(reason: u32) -> &'static str {
    match reason {
        0 => "Executive",
        1 => "FreePage",
        2 => "PageIn",
        3 => "PoolAllocation",
        4 => "DelayExecution",
        5 => "Suspended",
        6 => "UserRequest",
        7 => "WrExecutive",
        8 => "WrFreePage",
        9 => "WrPageIn",
        10 => "WrPoolAllocation",
        11 => "WrDelayExecution",
        12 => "WrSuspended",
        13 => "WrUserRequest",
        14 => "WrEventPair",
        15 => "WrQueue",
        16 => "WrLpcReceive",
        17 => "WrLpcReply",
        18 => "WrVirtualMemory",
        19 => "WrPageOut",
        20 => "WrRendezvous",
        21 => "WrKeyedEvent",
        22 => "WrTerminated",
        23 => "WrProcessInSwap",
        24 => "WrCpuRateControl",
        25 => "WrCalloutStack",
        26 => "WrKernel",
        27 => "WrResource",
        28 => "WrPushLock",
        29 => "WrMutex",
        30 => "WrQuantumEnd",
        31 => "WrDispatchInt",
        32 => "WrPreempted",
        33 => "WrYieldExecution",
        34 => "WrFastMutex",
        35 => "WrGuardedMutex",
        36 => "WrRundown",
        37 => "WrAlertByThreadId",
        38 => "WrDeferredPreempt",
        _ => "Unknown",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    // High,     // Not Available
    // Critical, // Not Available
}
impl IOPriority {
    const TABLE: &'static [(Self, &'static str, Option<u32>)] = &[
        (Self::None, "none", None),
        (Self::VeryLow, "very low", Some(0)),
        (Self::Low, "low", Some(1)),
        (Self::Normal, "normal", Some(2)),
        // (Self::High, "high", Some(3)),           // Not Available
        // (Self::Critical, "critical", Some(4)), // Not Available
    ];

    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("fail as str")
    }

    pub fn as_win_const(&self) -> Option<u32> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, _, val)| *val).unwrap_or(None)
    }

    pub fn from_str(s: &str) -> Self {
        Self::TABLE
            .iter()
            .find(|(_, name, _)| s.to_lowercase() == *name)
            .map(|(v, _, _)| *v)
            .unwrap_or(Self::None)
    }
}

#[repr(C)]
#[derive(PartialEq)]
struct MemoryPriorityInformation(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
impl MemoryPriority {
    const TABLE: &'static [(Self, &'static str, Option<MEMORY_PRIORITY>)] = &[
        (Self::None, "none", None),
        (Self::VeryLow, "very low", Some(MEMORY_PRIORITY_VERY_LOW)),
        (Self::Low, "low", Some(MEMORY_PRIORITY_LOW)),
        (Self::Medium, "medium", Some(MEMORY_PRIORITY_MEDIUM)),
        (Self::BelowNormal, "below normal", Some(MEMORY_PRIORITY_BELOW_NORMAL)),
        (Self::Normal, "normal", Some(MEMORY_PRIORITY_NORMAL)),
    ];

    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, s, _)| *s).unwrap_or("none")
    }

    pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY> {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, _, val)| *val).unwrap_or(None)
    }

    pub fn from_str(s: &str) -> Self {
        Self::TABLE.iter().find(|(_, str, _)| *str == s).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }
}
pub fn log_to_find(msg: &str) {
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        println!("[{}]{}", time_prefix, msg);
    } else {
        let _ = writeln!(find_logger().lock().unwrap(), "[{}]{}", time_prefix, msg);
    }
}

pub fn log_process_find(process_name: &str) {
    if FINDS_SET.lock().unwrap().insert(process_name.to_string().clone()) {
        log_to_find(&format!("find {}", process_name));
    }
}

macro_rules! log {
    ($($arg:tt)*) => {
        log_message(format!($($arg)*).as_str());
    };
}
fn log_message(args: &str) {
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        println!("[{}]{}", time_prefix, args);
    } else {
        let _ = writeln!(logger().lock().unwrap(), "[{}]{}", time_prefix, args);
    }
}

fn read_config<P: AsRef<Path>>(path: P) -> io::Result<(HashMap<String, ProcessConfig>, ConfigConstants)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut configs = HashMap::new();
    let mut affinity_aliases = HashMap::new();

    // 默认常量值
    let mut constants = ConfigConstants {
        hysteresis_ratio: 1.2,
        absolute_keep_ratio: 0.7,
        entry_threshold_ratio: 0.42,
    };

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            // 空行或注释
            continue;
        } else if line.starts_with('@') {
            // 常量定义: @NAME=VALUE
            if let Some(eq_pos) = line.find('=') {
                let const_name = line[1..eq_pos].trim().to_uppercase();
                let const_value = line[eq_pos + 1..].trim();
                match const_name.as_str() {
                    "HYSTERESIS_RATIO" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.hysteresis_ratio = v;
                            log!("Config: HYSTERESIS_RATIO = {}", v);
                        }
                    }
                    "ABSOLUTE_KEEP_RATIO" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.absolute_keep_ratio = v;
                            log!("Config: ABSOLUTE_KEEP_RATIO = {}", v);
                        }
                    }
                    "ENTRY_THRESHOLD_RATIO" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.entry_threshold_ratio = v;
                            log!("Config: ENTRY_THRESHOLD_RATIO = {}", v);
                        }
                    }
                    _ => {
                        log_to_find(&format!("Unknown constant: {}", const_name));
                    }
                }
            }
            continue;
        } else if line.starts_with('*') {
            // 掩码别名: *NAME=VALUE
            if let Some(eq_pos) = line.find('=') {
                let alias_name = line[1..eq_pos].trim().to_lowercase();
                let alias_value = line[eq_pos + 1..].trim();
                let affinity_value = if alias_value.starts_with("0x") {
                    usize::from_str_radix(alias_value.trim_start_matches("0x"), 16).unwrap_or(0)
                } else {
                    alias_value.parse().unwrap_or(0)
                };
                affinity_aliases.insert(alias_name, affinity_value);
            }
            continue;
        }

        // 进程配置行
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            let name = parts[0].to_lowercase();
            let priority: ProcessPriority = ProcessPriority::from_str(&parts[1]);
            let affinity_def = parts[2].trim();
            let affinity = if affinity_def.starts_with('*') {
                *affinity_aliases.get(&affinity_def.trim_start_matches('*').to_lowercase()).unwrap_or(&0)
            } else if parts[2].trim_start().starts_with("0x") {
                usize::from_str_radix(affinity_def.trim_start_matches("0x"), 16).unwrap_or(0)
            } else {
                parts[2].parse().unwrap_or(0)
            };
            let cpuset_def = if parts.len() >= 4 { parts[3].trim() } else { "0" };
            let cpuset = if cpuset_def.starts_with('*') {
                *affinity_aliases.get(&cpuset_def.trim_start_matches('*').to_lowercase()).unwrap_or(&0)
            } else if parts[3].trim_start().starts_with("0x") {
                usize::from_str_radix(cpuset_def.trim_start_matches("0x"), 16).unwrap_or(0)
            } else {
                parts[3].parse().unwrap_or(0)
            };
            let prime_cpu_def = if parts.len() >= 5 { parts[4].trim() } else { "0" };
            let prime_cpuset = if prime_cpu_def.starts_with('*') {
                *affinity_aliases.get(&prime_cpu_def.trim_start_matches('*').to_lowercase()).unwrap_or(&0)
            } else if parts[4].trim_start().starts_with("0x") {
                usize::from_str_radix(prime_cpu_def.trim_start_matches("0x"), 16).unwrap_or(0)
            } else {
                parts[4].parse().unwrap_or(0)
            };
            let io_priority = if parts.len() >= 6 { IOPriority::from_str(parts[5]) } else { IOPriority::None };
            let memory_priority = if parts.len() >= 7 {
                MemoryPriority::from_str(parts[6])
            } else {
                MemoryPriority::None
            };
            configs.insert(
                name.clone(),
                ProcessConfig {
                    name,
                    priority,
                    affinity_mask: affinity,
                    cpu_set_mask: cpuset,
                    prime_cpu_mask: prime_cpuset,
                    io_priority,
                    memory_priority,
                },
            );
        }
    }
    Ok((configs, constants))
}

fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect())
}

fn error_from_code(code: u32) -> String {
    match code {
        0 => "SUCCESS".to_string(),
        2 => "FILE_NOT_FOUND".to_string(),
        5 => "ACCESS_DENIED".to_string(),
        6 => "INVALID_HANDLE".to_string(),
        8 => "NOT_ENOUGH_MEMORY".to_string(),
        87 => "INVALID_PARAMETER".to_string(),
        122 => "INSUFFICIENT_BUFFER".to_string(),
        126 => "MOD_NOT_FOUND".to_string(),
        127 => "PROC_NOT_FOUND".to_string(),
        1314 => "PRIVILEGE_NOT_HELD".to_string(),
        1450 => "NO_SYSTEM_RESOURCES".to_string(),
        1460 => "TIMEOUT".to_string(),
        998 => "NOACCESS".to_string(),
        1008 => "INVALID_HANDLE_STATE".to_string(),
        1060 => "SERVICE_DOES_NOT_EXIST".to_string(),
        193 => "BAD_EXE_FORMAT".to_string(),
        _ => ("code=".to_string()) + &code.to_string(),
    }
}

#[allow(unused_assignments)]
fn apply_config(pid: u32, config: &ProcessConfig, prime_core_scheduler: &mut PrimeCoreScheduler, processes: &mut ProcessSnapshot) {
    unsafe {
        match OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, false, pid) {
            /* this error instance don't contain any information inside, it not the one returned from winAPI, no need to receive it */
            Err(_) => {
                log_to_find(&format!(
                    "apply_config: [OPEN_FAILED][{}] {:>5}-{}",
                    error_from_code(GetLastError().0),
                    pid,
                    config.name
                ));
            }
            Ok(h_prc) => {
                if h_prc.is_invalid() {
                    log_to_find(&format!("apply_config: [INVALID_HANDLE] {:>5}-{}", pid, config.name));
                } else {
                    if let Some(priority_flag) = config.priority.as_win_const() {
                        if GetPriorityClass(h_prc) != priority_flag.0 {
                            if SetPriorityClass(h_prc, priority_flag).is_ok() {
                                log!("{:>5}-{} -> Priority: {}", pid, config.name, config.priority.as_str());
                            } else {
                                log_to_find(&format!(
                                    "apply_config: [SET_PRIORITY_FAILED][{}] {:>5}-{}",
                                    error_from_code(GetLastError().0),
                                    pid,
                                    config.name
                                ));
                            }
                        }
                    }

                    let mut current_mask: usize = 0;
                    let mut system_mask: usize = 0; // all possible cores
                    if config.affinity_mask != 0 || config.prime_cpu_mask != 0 {
                        match GetProcessAffinityMask(h_prc, &mut current_mask, &mut system_mask) {
                            Err(_) => {
                                log_to_find(&format!(
                                    "apply_config: [QUERY_AFFINITY_FAILED][{}] {:>5}-{}",
                                    error_from_code(GetLastError().0),
                                    pid,
                                    config.name
                                ));
                            }
                            Ok(_) => match config.affinity_mask {
                                0 => {}
                                config_mask if config_mask != current_mask => match SetProcessAffinityMask(h_prc, config_mask) {
                                    Err(_) => {
                                        log_to_find(&format!(
                                            "apply_config: [SET_AFFINITY_FAILED][{}] {:>5}-{}",
                                            error_from_code(GetLastError().0),
                                            pid,
                                            config.name
                                        ));
                                    }
                                    Ok(_) => {
                                        log!("{:>5}-{} {:#X}-> {:#X}", pid, config.name, current_mask, config_mask);
                                        current_mask = config_mask;
                                    }
                                },
                                _ => {}
                            },
                        }
                    }

                    if config.cpu_set_mask != 0 && !get_cpu_set_information().lock().unwrap().is_empty() {
                        let mut toset: bool = false;
                        let mut requiredidcount: u32 = 0;
                        match GetProcessDefaultCpuSets(h_prc, None, &mut requiredidcount).as_bool() {
                            true => {
                                // 0 is large enough, meaning there are no CPU sets for this process, so we need to set the default CPU set
                                toset = true;
                            }
                            false => {
                                let code = GetLastError().0;
                                if code != 122 {
                                    // 122 -> INSUFFICIENT_BUFFER, 0 is not large enough, meaning there are CPU sets for this process, so read them.
                                    log_to_find(&format!(
                                        "apply_config: [QUERY_CPUSET_FAILED][{}] {:>5}-{}-{}",
                                        error_from_code(code),
                                        pid,
                                        config.name,
                                        requiredidcount
                                    ));
                                } else {
                                    let mut current_cpusetids: Vec<u32> = vec![0u32; requiredidcount as usize];
                                    match GetProcessDefaultCpuSets(h_prc, Some(&mut current_cpusetids[..]), &mut requiredidcount).as_bool() {
                                        false => {
                                            log_to_find(&format!(
                                                "apply_config: [QUERY_CPUSET_FAILED][{}] {:>5}-{}-{}",
                                                error_from_code(GetLastError().0),
                                                pid,
                                                config.name,
                                                requiredidcount
                                            ));
                                        }
                                        true => {
                                            toset = current_cpusetids != cpusetids_from_mask(config.cpu_set_mask);
                                        }
                                    }
                                }
                            }
                        }
                        if toset {
                            match SetProcessDefaultCpuSets(h_prc, Some(&cpusetids_from_mask(config.cpu_set_mask))).as_bool() {
                                false => {
                                    log_to_find(&format!(
                                        "apply_config: [SET_CPUSET_FAILED][{}] {:>5}-{}",
                                        error_from_code(GetLastError().0),
                                        pid,
                                        config.name
                                    ));
                                }
                                true => {
                                    log!("{:>5}-{} -> (cpu set) {:#X}", pid, config.name, config.cpu_set_mask);
                                }
                            }
                        }
                    }

                    if config.prime_cpu_mask != 0 {
                        let cpu_setids = cpusetids_from_mask(config.prime_cpu_mask & current_mask);
                        if !cpu_setids.is_empty() {
                            let total_cores = get_cpu_set_information().lock().unwrap().len();
                            let max_vip_count = cpu_setids.len();

                            if let Some(process) = processes.pid_to_process.get_mut(&pid) {
                                // ========== 阶段 1: 用 delta_time 快速筛选 top total_cores 个候选者 ==========
                                let mut tid_to_delta_time: Vec<(u32, i64)> = Vec::new();

                                for thread in process.get_threads().values() {
                                    let tid = thread.tid();
                                    let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);

                                    let total_time = thread.total_time();
                                    let delta_time = total_time - thread_stats.last_total_time;
                                    thread_stats.last_total_time = total_time;

                                    tid_to_delta_time.push((tid, delta_time));
                                }

                                // 按 delta_time 降序排序, 取 top total_cores 个
                                tid_to_delta_time.sort_by(|a, b| b.1.cmp(&a.1));
                                let candidates: Vec<u32> = tid_to_delta_time.iter().take(total_cores).map(|(tid, _)| *tid).collect();

                                // ========== 阶段 2: 只对候选者查询 cycles, 并保留 handle ==========
                                let mut tid_to_delta_cycles: HashMap<u32, u64> = HashMap::new();

                                for tid in &candidates {
                                    let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);

                                    let (cycles, new_handle): (u64, Option<HANDLE>) = {
                                        let (handle, is_new) = match thread_stats.handle {
                                            Some(h) => (h, false),
                                            None => match OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION, false, *tid) {
                                                Ok(h) => (h, true),
                                                Err(_) => {
                                                    log_to_find(&format!(
                                                        "apply_config: [OPEN_THREAD_FOR_CYCLES_FAILED][{}] {:>5}-{}-{}",
                                                        error_from_code(GetLastError().0),
                                                        pid,
                                                        tid,
                                                        config.name
                                                    ));
                                                    (HANDLE::default(), false)
                                                }
                                            },
                                        };

                                        if handle.is_invalid() {
                                            (0, None)
                                        } else {
                                            let mut cycles: u64 = 0;
                                            match QueryThreadCycleTime(handle, &mut cycles) {
                                                Err(_) => {
                                                    log_to_find(&format!(
                                                        "apply_config: [QUERY_THREAD_CYCLE_TIME_FAILED][{}] {:>5}-{}-{}",
                                                        error_from_code(GetLastError().0),
                                                        pid,
                                                        tid,
                                                        config.name
                                                    ));
                                                    if is_new {
                                                        let _ = CloseHandle(handle);
                                                    }
                                                    (0, None)
                                                }
                                                Ok(()) => {
                                                    if is_new {
                                                        (cycles, Some(handle))
                                                    } else {
                                                        (cycles, None)
                                                    }
                                                }
                                            }
                                        }
                                    };

                                    // 如果获得了新 handle, 保存到 thread_stats
                                    if let Some(h) = new_handle {
                                        thread_stats.handle = Some(h);
                                    }

                                    let delta_cycles = cycles.saturating_sub(thread_stats.last_cycles);
                                    thread_stats.last_cycles = cycles;

                                    tid_to_delta_cycles.insert(*tid, delta_cycles);
                                }

                                // 找出最大的 delta_cycles
                                let max_delta_cycles = tid_to_delta_cycles.values().copied().max().unwrap_or(0);

                                // ========== 阶段 3: VIP 选择与三重容忍机制 ==========

                                // 阈值常量
                                let hysteresis_ratio: f64 = prime_core_scheduler.constants.hysteresis_ratio; // 候选者必须比 VIP * ? 强
                                let absolute_keep_ratio: f64 = prime_core_scheduler.constants.absolute_keep_ratio; // VIP >= 最大值 * ? 则保留
                                let entry_threshold_ratio: f64 = prime_core_scheduler.constants.entry_threshold_ratio; // 进入 VIP 需要 >= 最大值的 * ?

                                let absolute_keep_threshold = (max_delta_cycles as f64 * absolute_keep_ratio) as u64;
                                let entry_threshold = (max_delta_cycles as f64 * entry_threshold_ratio) as u64;

                                // 获取当前 VIP 及其 delta_cycles
                                let mut current_vips: Vec<(u32, u64)> = Vec::new();
                                if let Some(process_stats) = prime_core_scheduler.pid_to_process_stats.get(&pid) {
                                    for (tid, ts) in &process_stats.tid_to_thread_stats {
                                        if ts.handle.is_some() && ts.cpu_set_id.is_some() {
                                            let score = tid_to_delta_cycles.get(tid).copied().unwrap_or(0);
                                            current_vips.push((*tid, score));
                                        }
                                    }
                                }

                                // 获取非 VIP 候选者（有 handle 但还没分配 cpu_set_id 的）
                                // 新增: 必须达到 entry_threshold 才能成为候选者
                                let current_vip_tids: HashSet<u32> = current_vips.iter().map(|(t, _)| *t).collect();
                                let mut new_candidates: Vec<(u32, u64)> = tid_to_delta_cycles
                                    .iter()
                                    .filter(|(tid, score)| !current_vip_tids.contains(tid) && **score >= entry_threshold)
                                    .map(|(&tid, &score)| (tid, score))
                                    .collect();
                                new_candidates.sort_by(|a, b| b.1.cmp(&a.1)); // 降序

                                // 当前 VIP 按 delta_cycles 升序（最弱的在前）
                                current_vips.sort_by(|a, b| a.1.cmp(&b.1));

                                // 检查当前 VIP 是否低于 entry_threshold，如果是则需要降级
                                let mut vips_below_entry: Vec<u32> = Vec::new();
                                for (tid, score) in &current_vips {
                                    if *score < entry_threshold {
                                        vips_below_entry.push(*tid);
                                    }
                                }

                                let mut to_demote: Vec<u32> = vips_below_entry.clone();
                                let mut to_promote: Vec<u32> = Vec::new();

                                // 从 current_vips 中移除低于 entry_threshold 的
                                current_vips.retain(|(_, score)| *score >= entry_threshold);

                                // 先填空位（包括刚刚因低于 entry_threshold 而空出的）
                                let empty_slots = max_vip_count.saturating_sub(current_vips.len());
                                for (tid, _) in new_candidates.iter().take(empty_slots) {
                                    to_promote.push(*tid);
                                }

                                // 尝试替换弱者
                                let remaining = new_candidates.iter().skip(empty_slots);
                                let mut weak_iter = current_vips.iter().peekable();

                                for (cand_tid, cand_score) in remaining {
                                    match weak_iter.peek() {
                                        None => break,
                                        Some((weak_tid, weak_score)) => {
                                            // 条件 1: 绝对保留 - VIP 的 delta_cycles >= 最大值的 70%
                                            if *weak_score >= absolute_keep_threshold {
                                                break;
                                            }

                                            // 条件 2: 相对容忍 - 候选者必须比 VIP 强 20%
                                            let relative_threshold = (*weak_score as f64 * hysteresis_ratio) as u64;
                                            if *cand_score > relative_threshold {
                                                to_demote.push(*weak_tid);
                                                to_promote.push(*cand_tid);
                                                weak_iter.next();
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                }

                                // ========== 执行降级: 关闭落选者的 handle ==========
                                let mut freed_cpu_set_ids: Vec<u32> = Vec::new();
                                if let Some(process_stats) = prime_core_scheduler.pid_to_process_stats.get_mut(&pid) {
                                    for tid in &to_demote {
                                        if let Some(thread_stats) = process_stats.tid_to_thread_stats.get_mut(tid) {
                                            if let Some(handle) = thread_stats.handle.take() {
                                                let _ = SetThreadSelectedCpuSets(handle, &[]);
                                                let _ = CloseHandle(handle);
                                            }
                                            if let Some(cpu_id) = thread_stats.cpu_set_id.take() {
                                                freed_cpu_set_ids.push(cpu_id);
                                                let delta = tid_to_delta_cycles.get(tid).unwrap_or(&0);
                                                log!(
                                                    "{:>5}-{}-{} <- (demoted, freed {:#X}, cycles={})",
                                                    pid,
                                                    tid,
                                                    config.name,
                                                    mask_from_cpusetids(&[cpu_id]),
                                                    delta
                                                );
                                            }
                                        }
                                    }

                                    // 关闭未被选中的候选者的 handle（有 handle 但不在 to_promote 和 current_vips 中）
                                    let promoted_set: HashSet<u32> = to_promote.iter().copied().collect();
                                    let remaining_vip_tids: HashSet<u32> = current_vips.iter().map(|(t, _)| *t).collect();
                                    for tid in &candidates {
                                        if !remaining_vip_tids.contains(tid) && !promoted_set.contains(tid) {
                                            if let Some(thread_stats) = process_stats.tid_to_thread_stats.get_mut(tid) {
                                                if let Some(handle) = thread_stats.handle.take() {
                                                    let _ = CloseHandle(handle);
                                                }
                                            }
                                        }
                                    }
                                }

                                // ========== 构建可用核心池 ==========
                                let mut used_cpu_set_ids: HashSet<u32> = HashSet::new();
                                if let Some(process_stats) = prime_core_scheduler.pid_to_process_stats.get(&pid) {
                                    for (_, ts) in &process_stats.tid_to_thread_stats {
                                        if ts.handle.is_some() {
                                            if let Some(cpu_id) = ts.cpu_set_id {
                                                used_cpu_set_ids.insert(cpu_id);
                                            }
                                        }
                                    }
                                }

                                let mut available_cpu_set_ids: Vec<u32> = freed_cpu_set_ids;
                                for cpu_id in &cpu_setids {
                                    if !used_cpu_set_ids.contains(cpu_id) && !available_cpu_set_ids.contains(cpu_id) {
                                        available_cpu_set_ids.push(*cpu_id);
                                    }
                                }
                                available_cpu_set_ids.sort_by(|a, b| b.cmp(a)); // 优先大核

                                // ========== 执行升级: 直接使用已有的 handle ==========
                                let mut available_iter = available_cpu_set_ids.into_iter();
                                for tid in &to_promote {
                                    let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);

                                    match thread_stats.handle {
                                        None => {
                                            log_to_find(&format!("apply_config: [NO_HANDLE_FOR_PROMOTE] {:>5}-{}-{}", pid, tid, config.name));
                                            continue;
                                        }
                                        Some(handle) => {
                                            if let Some(cpu_id) = available_iter.next() {
                                                match SetThreadSelectedCpuSets(handle, &[cpu_id]).as_bool() {
                                                    false => {
                                                        log_to_find(&format!(
                                                            "apply_config: [SET_THREAD_SELECTED_CPU_SETS_FAILED][{}] {:>5}-{}-{}",
                                                            error_from_code(GetLastError().0),
                                                            pid,
                                                            tid,
                                                            config.name
                                                        ));
                                                    }
                                                    true => {
                                                        thread_stats.cpu_set_id = Some(cpu_id);
                                                        let delta = tid_to_delta_cycles.get(tid).unwrap_or(&0);
                                                        log!(
                                                            "{:>5}-{}-{} -> (promoted, {:#X}, cycles={})",
                                                            pid,
                                                            tid,
                                                            config.name,
                                                            mask_from_cpusetids(&[cpu_id]),
                                                            delta
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(io_priority_flag) = config.io_priority.as_win_const() {
                        let mut current_io_priority: u32 = 0;
                        let mut return_length: u32 = 0;
                        match NtQueryInformationProcess(
                            h_prc,
                            PROCESS_INFORMATION_IO_PRIORITY,
                            &mut current_io_priority as *mut _ as *mut std::ffi::c_void,
                            size_of::<u32>() as u32,
                            &mut return_length,
                        )
                        .0
                        {
                            query_result if query_result < 0 => {
                                log_to_find(&format!(
                                    "apply_config: [QUERY_IO_PRIORITY_FAILED][0x{:08X}] {:>5}-{} -> {}",
                                    query_result,
                                    pid,
                                    config.name,
                                    config.io_priority.as_str()
                                ));
                            }
                            query_result if query_result >= 0 => {
                                if current_io_priority != io_priority_flag {
                                    match NtSetInformationProcess(
                                        h_prc,
                                        PROCESS_INFORMATION_IO_PRIORITY,
                                        &io_priority_flag as *const _ as *const std::ffi::c_void,
                                        size_of::<u32>() as u32,
                                    )
                                    .0
                                    {
                                        set_result if set_result < 0 => {
                                            log_to_find(&format!(
                                                "apply_config: [SET_IO_PRIORITY_FAILED][0x{:08X}] {:>5}-{} -> {}",
                                                set_result,
                                                pid,
                                                config.name,
                                                config.io_priority.as_str()
                                            ));
                                        }
                                        set_result if set_result >= 0 => {
                                            log!("{:>5}-{} -> IO: {}", pid, config.name, config.io_priority.as_str());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    if let Some(memory_priority_flag) = config.memory_priority.as_win_const() {
                        // comment out memory priority query as it is not supported by Windows api
                        match SetProcessInformation(
                            h_prc,
                            ProcessMemoryPriority,
                            &MemoryPriorityInformation(memory_priority_flag.0) as *const _ as *const std::ffi::c_void,
                            size_of::<MemoryPriorityInformation>() as u32,
                        ) {
                            Err(_) => {
                                log_to_find(&format!(
                                    "apply_config: [SET_MEMORY_PRIORITY_FAILED][{}] {:>5}-{} -> {}",
                                    error_from_code(GetLastError().0),
                                    pid,
                                    config.name,
                                    config.memory_priority.as_str()
                                ));
                            }
                            Ok(_) => {}
                        }
                    }
                    let _ = CloseHandle(h_prc);
                }
            }
        }
    }
}

fn is_affinity_unset(pid: u32, process_name: &str) -> bool {
    unsafe {
        let mut result = false;

        match OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, false, pid) {
            Err(_) => {
                let code = GetLastError().0;
                log_to_find(&format!("is_affinity_unset: [OPEN_FAILED][{}] {:>5}-{}", error_from_code(code), pid, process_name));
                if code == 5 {
                    FAIL_SET.lock().unwrap().insert(process_name.to_string());
                }
            }
            Ok(h_proc) => {
                if h_proc.is_invalid() {
                    log_to_find(&format!("is_affinity_unset: [INVALID_HANDLE] {:>5}-{}", pid, process_name));
                } else {
                    let mut current_mask: usize = 0;
                    let mut system_mask: usize = 0;
                    match GetProcessAffinityMask(h_proc, &mut current_mask, &mut system_mask) {
                        Err(_) => {
                            let code = GetLastError().0;
                            log_to_find(&format!(
                                "is_affinity_unset: [AFFINITY_QUERY_FAILED][{}] {:>5}-{}",
                                error_from_code(code),
                                pid,
                                process_name
                            ));
                            if code == 5 {
                                FAIL_SET.lock().unwrap().insert(process_name.to_string());
                            }
                        }
                        Ok(_) => {
                            result = current_mask == system_mask;
                        }
                    }
                    let _ = CloseHandle(h_proc);
                }
            }
        }
        result
    }
}

fn split_trim_nonempty(s: &str) -> Vec<&str> {
    s.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
}

fn parse_mask(s: &str) -> usize {
    let mut mask = 0;
    for part in s.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((start, end)) = part.split_once('-') {
            let start: usize = start.trim().parse().unwrap_or(0);
            let end: usize = end.trim().parse().unwrap_or(start);
            for core in start..=end {
                mask |= 1 << core;
            }
        } else {
            let core: usize = part.parse().unwrap_or(0);
            mask |= 1 << core;
        }
    }
    mask
}

fn convert(in_file_name: Option<String>, out_file_name: Option<String>) {
    if let Some(ref in_file) = in_file_name {
        if let Some(ref out_file) = out_file_name {
            match read_utf16le_file(in_file) {
                Ok(in_content) => {
                    let mut configs: Vec<ProcessConfig> = Vec::new();
                    for line in in_content.lines() {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }
                        if let Some(rest) = line.strip_prefix("DefaultPriorities=") {
                            let fields = split_trim_nonempty(rest);
                            for chunk in fields.chunks(2) {
                                if chunk.len() == 2 {
                                    let name = chunk[0].to_lowercase();
                                    let priority = ProcessPriority::from_str(chunk[1]);
                                    match configs.iter_mut().find(|c| c.name == name) {
                                        Some(cfg) => cfg.priority = priority,
                                        None => configs.push(ProcessConfig {
                                            name,
                                            priority,
                                            affinity_mask: 0,
                                            cpu_set_mask: 0,
                                            prime_cpu_mask: 0,
                                            io_priority: IOPriority::None,
                                            memory_priority: MemoryPriority::None,
                                        }),
                                    }
                                } else {
                                    log!("Invalid priority configuration line: {}", line);
                                }
                            }
                        } else if let Some(rest) = line.strip_prefix("DefaultAffinitiesEx=") {
                            let fields = split_trim_nonempty(rest);
                            for chunk in fields.chunks(3) {
                                if chunk.len() == 3 {
                                    let name = chunk[0].to_lowercase();
                                    let mask = parse_mask(chunk[2]);
                                    match configs.iter_mut().find(|c| c.name == name) {
                                        Some(cfg) => cfg.affinity_mask = mask,
                                        None => configs.push(ProcessConfig {
                                            name,
                                            priority: ProcessPriority::None,
                                            affinity_mask: mask,
                                            cpu_set_mask: 0,
                                            prime_cpu_mask: 0,
                                            io_priority: IOPriority::None,
                                            memory_priority: MemoryPriority::None,
                                        }),
                                    }
                                } else {
                                    log!("Invalid affinity configuration line: {}", line);
                                }
                            }
                        }
                    }
                    match File::create(out_file) {
                        Ok(mut output) => {
                            for cfg in &configs {
                                let _ = writeln!(
                                    output,
                                    "{},{},0x{:X},0x{:X},0x{:X},{}",
                                    cfg.name,
                                    cfg.priority.as_str(),
                                    cfg.affinity_mask,
                                    cfg.cpu_set_mask,
                                    cfg.prime_cpu_mask,
                                    cfg.io_priority.as_str()
                                );
                            }
                            log!("convert done, {} process configs have been output", configs.len());
                        }
                        Err(_) => {
                            log!("cannot create output file: {}", out_file);
                        }
                    }
                }
                Err(_) => {
                    log!("cannot read from file: {}", in_file);
                }
            }
        } else {
            log!("not output file (-out <file>)!");
        }
    } else {
        log!("no input file (-in <file>)!");
    };
}

fn read_utf16le_file(path: &str) -> io::Result<String> {
    let bytes = fs::read(path)?;
    let utf16: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    Ok(String::from_utf16_lossy(&utf16))
}

fn parse_args(
    args: &[String],
    interval_ms: &mut u64,
    help_mode: &mut bool,
    help_all_mode: &mut bool,
    convert_mode: &mut bool,
    find_mode: &mut bool,
    config_file_name: &mut String,
    blacklist_file_name: &mut Option<String>,
    in_file_name: &mut Option<String>,
    out_file_name: &mut Option<String>,
    no_uac: &mut bool,
    loop_count: &mut Option<u32>,
    time_resolution: &mut u32,
    log_loop: &mut bool,
    skip_log_before_elevation: &mut bool,
) -> windows::core::Result<()> {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-help" | "--help" | "-?" | "/?" | "?" => {
                *help_mode = true;
            }
            "-helpall" | "--helpall" => {
                *help_all_mode = true;
            }
            "-console" => {
                *use_console().lock().unwrap() = true;
            }
            "-noUAC" | "-nouac" => {
                *no_uac = true;
            }
            "-convert" => {
                *convert_mode = true;
            }
            "-find" => {
                *find_mode = true;
            }
            "-interval" if i + 1 < args.len() => {
                *interval_ms = args[i + 1].parse().unwrap_or(5000).max(16);
                i += 1;
            }
            "-loop" if i + 1 < args.len() => {
                *loop_count = Some(args[i + 1].parse().unwrap_or(1).max(1));
                i += 1;
            }
            "-resolution" if i + 1 < args.len() => {
                *time_resolution = args[i + 1].parse().unwrap_or(0).max(0);
                i += 1;
            }
            "-logloop" => {
                *log_loop = true;
            }
            "-config" if i + 1 < args.len() => {
                *config_file_name = args[i + 1].clone();
                i += 1;
            }
            "-blacklist" if i + 1 < args.len() => {
                *blacklist_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-in" if i + 1 < args.len() => {
                *in_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-out" if i + 1 < args.len() => {
                *out_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-skip_log_before_elevation" => {
                *skip_log_before_elevation = true;
            }
            _ => {}
        }
        i += 1;
    }
    Ok(())
}

fn print_help() {
    println!("usage: AffinityServiceRust.exe [args]");
    println!();
    println!("A Windows service to manage process priority, CPU affinity, and IO priority.");
    println!();
    println!("Common Options:");
    println!("  -help | --help       show this help message");
    println!("  -console             output to console instead of log file");
    println!("  -config <file>       config file to use (default: config.ini)");
    println!("  -find                find processes with default affinity (-blacklist <file>)");
    println!("  -interval <ms>       check interval in milliseconds (default: 5000)");
    println!("  -noUAC               disable UAC elevation request");
    println!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    println!();
    println!("Modes:");
    println!("  -convert             convert Process Lasso config (-in <file> -out <file>)");
    println!();
    println!("Config Format: process_name,priority,affinity_mask,io_priority");
    println!("  Example: notepad.exe,above normal,0xFF,low");
    println!();
    println!("Use -helpall for detailed options and debugging features.");
}

fn print_help_all() {
    println!("usage: AffinityServiceRust.exe [args]");
    println!();
    println!("=== DETAILED HELP & DEBUG OPTIONS ===");
    println!();
    println!("Basic Arguments:");
    println!("  -help | --help       print basic help message");
    println!("  -helpall | --helpall print this detailed help with debug options");
    println!("  -? | /? | ?          print basic help message");
    println!("  -console             use console as output instead of log file");
    println!("  -noUAC | -nouac      disable UAC elevation request");
    println!("  -config <file>       the config file u wanna use (config.ini by default)");
    println!("  -find                find those whose affinity is same as system default which is all possible cores windows could use");
    println!("  -blacklist <file>    the blacklist for -find");
    println!("  -interval <ms>       set interval for checking again (5000 by default, minimal 16)");
    println!();
    println!("Operating Modes:");
    println!("  -convert             convert process configs from -in <file>(from process lasso) to -out <file>");
    println!("  -in <file>           input file for -convert");
    println!("  -out <file>          output file for -convert");
    println!();
    println!("Debug & Testing Options:");
    println!("  -loop <count>        number of loops to run (default: infinite) - for testing");
    println!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    println!("  -logloop             log a message at the start of each loop for testing");
    println!();
    println!("=== CONFIGURATION FORMAT ===");
    println!();
    println!("Config file format: process_name,priority,affinity_mask,io_priority");
    println!();
    println!("Priority Options:");
    println!("  none, idle, below normal, normal, above normal, high, real time");
    println!("  'none' means the program won't change it");
    println!();
    println!("Affinity Mask:");
    println!("  0 (no change), or hex/decimal mask (e.g., 0xFF, 255)");
    println!("  *alias_name to use predefined alias");
    println!("  Represents CPU cores as binary flags");
    println!();
    println!("Affinity Aliases:");
    println!("  Define reusable masks with: *alias_name = 0xFF");
    println!("  Examples:");
    println!("    *pcore = 0xFF        # Performance cores (0-7)");
    println!("    *ecore = 0xFFF00     # Efficiency cores (8-19)");
    println!("    *allcores = 0xFFFFF  # All cores");
    println!("  Then use: game.exe,high,*pcore,normal");
    println!();
    println!("IO Priority Options:");
    println!("  none, very low, low, normal");
    println!("  'none' means the program won't change it");
    println!("  Note: high/critical removed due to privilege requirements");
    println!();
    println!("Example Configuration:");
    println!("  # Define aliases first");
    println!("  *pcore = 0xFF");
    println!("  *ecore = 0xFFF00");
    println!("  # Use aliases in configs");
    println!("  notepad.exe,above normal,*pcore,low");
    println!("  game.exe,high,*pcore,normal,none");
    println!("  background.exe,idle,*ecore,very low");
    println!();
    println!("=== LIMITATIONS & NOTES ===");
    println!();
    println!("- Admin privileges needed for managing system processes");
    println!();
}

fn enable_debug_privilege() {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        match OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) {
            Err(_) => {
                log!("enable_debug_privilege: self OpenProcessToken failed");
            }
            Ok(_) => {
                let mut l_uid = LUID::default();
                match LookupPrivilegeValueW(None, SE_DEBUG_NAME, &mut l_uid) {
                    Err(_) => {
                        log!("enable_debug_privilege: LookupPrivilegeValueW failed");
                    }
                    Ok(_) => {
                        let tp = TOKEN_PRIVILEGES {
                            PrivilegeCount: 1,
                            Privileges: [windows::Win32::Security::LUID_AND_ATTRIBUTES {
                                Luid: l_uid,
                                Attributes: windows::Win32::Security::SE_PRIVILEGE_ENABLED,
                            }],
                        };
                        match AdjustTokenPrivileges(token, false, Some(&tp as *const _), 0, None, None) {
                            Err(_) => {
                                log!("enable_debug_privilege: AdjustTokenPrivileges failed");
                            }
                            Ok(_) => {
                                log!("enable_debug_privilege: AdjustTokenPrivileges succeeded");
                            }
                        }
                    }
                }
                let _ = CloseHandle(token);
            }
        }
    }
}

fn is_running_as_admin() -> bool {
    unsafe {
        let current_process = GetCurrentProcess();
        let mut token: HANDLE = HANDLE::default();
        let mut result = false;
        match OpenProcessToken(current_process, TOKEN_QUERY, &mut token) {
            Err(_) => {}
            Ok(_) => {
                let mut elevation: TOKEN_ELEVATION = TOKEN_ELEVATION::default();
                let mut return_length = 0u32;
                match GetTokenInformation(
                    token,
                    TokenElevation,
                    Some(&mut elevation as *mut _ as *mut _),
                    size_of::<TOKEN_ELEVATION>() as u32,
                    &mut return_length,
                ) {
                    Err(_) => {}
                    Ok(_) => result = elevation.TokenIsElevated != 0,
                }
                let _ = CloseHandle(token);
            }
        }
        result
    }
}

fn request_uac_elevation() -> io::Result<()> {
    let exe_path = env::current_exe()?;
    let mut args: Vec<String> = env::args().skip(1).collect();
    args.push("-skip_log_before_elevation".to_string());
    log!("Requesting UAC elevation...");
    let mut cmd = Command::new("powershell.exe");
    cmd.arg("-Command");
    let mut powershell_cmd = format!("Start-Process -FilePath '{}' -Verb RunAs", exe_path.display());
    if !args.is_empty() {
        let args_str = args.join(" ");
        powershell_cmd.push_str(&format!(" -ArgumentList '{}'", args_str));
    }
    cmd.arg(powershell_cmd);
    match cmd.spawn() {
        Ok(_) => {
            log!("UAC elevation request sent. Please approve the elevation prompt.");
            std::process::exit(0);
        }
        Err(e) => {
            log!("Failed to request UAC elevation: {}", e);
            Err(e)
        }
    }
}

fn init_cpu_set_information() -> Vec<SYSTEM_CPU_SET_INFORMATION> {
    let mut cpu_set_information: Vec<SYSTEM_CPU_SET_INFORMATION> = Vec::new();
    unsafe {
        let mut required_size: u32 = 0;
        let _ = GetSystemCpuSetInformation(None, 0, &mut required_size, Some(GetCurrentProcess()), Some(0));
        let mut buffer: Vec<u8> = vec![0u8; required_size as usize];
        match GetSystemCpuSetInformation(
            Some(buffer.as_mut_ptr() as *mut SYSTEM_CPU_SET_INFORMATION),
            required_size,
            &mut required_size,
            Some(GetCurrentProcess()),
            Some(0),
        )
        .as_bool()
        {
            false => log_to_find("GetSystemCpuSetInformation failed"),
            true => {}
        };
        let mut offset = 0;
        while offset < required_size as usize {
            let entry_ptr = buffer.as_ptr().add(offset) as *const SYSTEM_CPU_SET_INFORMATION;
            let entry = &*entry_ptr;
            cpu_set_information.push(*entry);
            offset += entry.Size as usize;
        }
    }
    cpu_set_information
}

fn cpusetids_from_mask(mask: usize) -> Vec<u32> {
    let mut cpuids: Vec<u32> = Vec::new();
    unsafe {
        get_cpu_set_information().lock().unwrap().iter().for_each(|entry| {
            if ((1 << entry.Anonymous.CpuSet.LogicalProcessorIndex) & mask) != 0 {
                cpuids.push(entry.Anonymous.CpuSet.Id);
            }
        });
    }
    cpuids
}

fn mask_from_cpusetids(cpuids: &[u32]) -> usize {
    let mut mask: usize = 0;
    unsafe {
        for entry in get_cpu_set_information().lock().unwrap().iter() {
            if cpuids.contains(&entry.Anonymous.CpuSet.Id) {
                mask |= 1 << entry.Anonymous.CpuSet.LogicalProcessorIndex;
            }
        }
    }
    mask
}

fn main() -> windows::core::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut interval_ms = 5000;
    let mut help_mode = false;
    let mut help_all_mode = false;
    let mut convert_mode = false;
    let mut find_mode = false;
    let mut config_file_name = "config.ini".to_string();
    let mut blacklist_file_name: Option<String> = None;
    let mut in_file_name: Option<String> = None;
    let mut out_file_name: Option<String> = None;
    let mut no_uac = false;
    let mut loop_count: Option<u32> = None;
    let mut time_resolution: u32 = 0;
    let mut log_loop = false;
    let mut skip_log_before_elevation = false;
    parse_args(
        &args,
        &mut interval_ms,
        &mut help_mode,
        &mut help_all_mode,
        &mut convert_mode,
        &mut find_mode,
        &mut config_file_name,
        &mut blacklist_file_name,
        &mut in_file_name,
        &mut out_file_name,
        &mut no_uac,
        &mut loop_count,
        &mut time_resolution,
        &mut log_loop,
        &mut skip_log_before_elevation,
    )?;
    if help_mode {
        print_help();
        return Ok(());
    }
    if help_all_mode {
        print_help_all();
        return Ok(());
    }
    if convert_mode {
        convert(in_file_name, out_file_name);
        return Ok(());
    }
    if !skip_log_before_elevation {
        log!("Affinity Service started");
        log!("time interval: {}", interval_ms);
    }
    let (configs, constants) = read_config(&config_file_name).unwrap_or_else(|e| {
        log!("Failed to read config: {}", e);
        (HashMap::new(), ConfigConstants::default())
    });
    let blacklist = if let Some(bf) = blacklist_file_name {
        read_list(bf).unwrap_or_default()
    } else {
        Vec::new()
    };
    let is_config_empty = configs.is_empty();
    let is_blacklist_empty = blacklist.is_empty();
    if is_config_empty && is_blacklist_empty {
        if !find_mode {
            if skip_log_before_elevation {
                log!("not even a single config, existing");
            }
            return Ok(());
        }
    } else {
        log!("{} configs load", configs.len());
        log!("{} blacklist items load", blacklist.len());
    }
    if !is_running_as_admin() {
        if no_uac {
            log!("Not running as administrator. UAC elevation disabled by -noUAC flag.");
            log!("Warning: May not be able to manage all processes without admin privileges.");
        } else {
            log!("Not running as administrator. Requesting UAC elevation...");
            match request_uac_elevation() {
                Ok(_) => {
                    log!("Running with administrator privileges.");
                }
                Err(e) => {
                    log!("Failed to request elevation: {}, may not manage all processes", e);
                }
            }
        }
    }
    enable_debug_privilege();
    if time_resolution != 0 {
        unsafe {
            let mut current_resolution = 0u32;
            match NtSetTimerResolution(time_resolution, true, &mut current_resolution as *mut _ as *mut std::ffi::c_void).0 {
                ntstatus if ntstatus < 0 => {
                    log!("Failed to set timer resolution: 0x{:08X}", ntstatus);
                }
                ntstatus if ntstatus >= 0 => {
                    log!("Succeed to set timer resolution: {:.4}ms", time_resolution as f64 / 10000f64);
                    log!("elder timer resolution: {:.4}ms", current_resolution);
                }
                _ => {}
            };
        }
    }
    let mut prime_core_scheduler = PrimeCoreScheduler::new(constants);
    let mut current_loop = 0u32;
    let mut should_continue = true;

    while should_continue {
        if log_loop {
            log!("Loop {} started", current_loop + 1);
        }
        match ProcessSnapshot::take() {
            Ok(mut processes) => {
                prime_core_scheduler.reset_alive();
                let to_process: Vec<(u32, String)> = processes
                    .pid_to_process
                    .values()
                    .filter_map(|entry| {
                        let name = entry.get_name();
                        if configs.contains_key(&name) { Some((entry.pid(), name)) } else { None }
                    })
                    .collect();
                for (pid, name) in to_process {
                    if let Some(config) = configs.get(&name) {
                        apply_config(pid, config, &mut prime_core_scheduler, &mut processes);
                    }
                }
                prime_core_scheduler.close_dead_process_handles();
            }
            Err(err) => {
                log!("Failed to take process snapshot: {}", err);
            }
        };
        if find_mode {
            unsafe {
                let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
                let mut pe32 = PROCESSENTRY32W::default();
                pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
                if Process32FirstW(snapshot, &mut pe32).is_ok() {
                    loop {
                        let process_name = String::from_utf16_lossy(&pe32.szExeFile[..pe32.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)]).to_lowercase();
                        if !FAIL_SET.lock().unwrap().contains(&process_name) && !configs.contains_key(&process_name) && !blacklist.contains(&process_name) {
                            if is_affinity_unset(pe32.th32ProcessID, process_name.as_str()) {
                                log_process_find(&process_name);
                            }
                        }
                        if !Process32NextW(snapshot, &mut pe32).is_ok() {
                            break;
                        }
                    }
                }
                let _ = CloseHandle(snapshot);
            }
        }
        let _ = find_logger().lock().unwrap().flush();
        let _ = logger().lock().unwrap().flush();
        current_loop += 1;
        if let Some(max_loops) = loop_count {
            if current_loop >= max_loops {
                if log_loop {
                    log!("Completed {} loops, exiting", max_loops);
                }
                should_continue = false;
            }
        }
        if should_continue {
            thread::sleep(Duration::from_millis(interval_ms));
            *LOCALTIME_BUFFER.lock().unwrap() = Local::now();
        }
    }
    Ok(())
}
