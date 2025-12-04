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

struct PrimeThreadScheduler {
    // clear threadStats once process exit as pid or tid may be reused by OS
    pid_to_process_stats: HashMap<u32, ProcessStats>,
    constants: ConfigConstants,
}
impl PrimeThreadScheduler {
    fn new(constants: ConfigConstants) -> Self {
        Self {
            pid_to_process_stats: HashMap::new(),
            constants,
        }
    }
    fn reset_alive(&mut self) {
        self.pid_to_process_stats.values_mut().for_each(|stats| stats.alive = false);
    }
    fn set_alive(&mut self, pid: u32) {
        self.pid_to_process_stats.entry(pid).or_insert_with(ProcessStats::new).alive = true;
    }
    #[inline]
    fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats {
        self.pid_to_process_stats
            .entry(pid)
            .or_insert_with(ProcessStats::new)
            .tid_to_thread_stats
            .entry(tid)
            .or_insert_with(ThreadStats::new)
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
            }
            process_stats.alive
        });
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
struct ThreadStats {
    last_total_time: i64,
    last_cycles: u64,
    handle: Option<HANDLE>,
    cpu_set_ids: Vec<u32>,
    active_streak: u8,
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
#[derive(Debug, Clone, Copy)]
pub struct ConfigConstants {
    pub hysteresis_ratio: f64,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
impl Default for ConfigConstants {
    fn default() -> Self {
        Self {
            hysteresis_ratio: 1.259,
            keep_threshold: 0.69,
            entry_threshold: 0.42,
        }
    }
}

/// A snapshot of all running processes obtained via `NtQuerySystemInformation`.
///
/// This provides a consistent view of the system's process and thread state
/// at a single point in time.  The snapshot is more efficient than repeatedly
/// calling process enumeration APIs.
///
/// # Safety
///
/// The internal buffer contains raw system data that is parsed in unsafe code.
/// The buffer must remain valid for the lifetime of this struct.
pub struct ProcessSnapshot {
    ///is used to store the snapshot of processes, parsed in unsafe
    buffer: Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}
impl Drop for ProcessSnapshot {
    fn drop(&mut self) {
        self.pid_to_process.clear();
        self.buffer.clear();
    }
}
impl ProcessSnapshot {
    pub fn take() -> Result<Self, i32> {
        let mut buf_len: usize = 1024;
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
            let mut pid_to_process: HashMap<u32, ProcessEntry> = HashMap::new();
            let mut offset: usize = 0;
            let buf_ptr = buffer.as_ptr();
            loop {
                let process_entry_ptr = buf_ptr.add(offset) as *const SYSTEM_PROCESS_INFORMATION;
                let entry = &*process_entry_ptr;
                let threads_ptr = &(*process_entry_ptr).Threads as *const SYSTEM_THREAD_INFORMATION;
                let process_entry = ProcessEntry::new(*entry, threads_ptr);
                pid_to_process.insert(entry.UniqueProcessId as u32, process_entry);
                if entry.NextEntryOffset == 0 {
                    break;
                }
                offset += entry.NextEntryOffset as usize;
            }
            Ok(ProcessSnapshot { buffer, pid_to_process })
        }
    }
    /// Gets all processes matching the given name.
    ///
    /// # Arguments
    /// * `name` - The process name to search for (lowercase)
    ///
    /// # Returns
    /// A vector of references to matching process entries.
    pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry> {
        self.pid_to_process.values().filter(|entry| (**entry).get_name() == name).collect()
    }
}
/// A single process entry from a process snapshot.
///
/// Contains information about a process including its threads,
/// parsed from the raw `SYSTEM_PROCESS_INFORMATION` structure.
///
/// Use `get_threads()` to gets all threads for this process, parsing them lazily if needed.
///
/// Thread information is cached after the first call for efficiency.
#[derive(Clone)]
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}
impl ProcessEntry {
    pub fn new(process: SYSTEM_PROCESS_INFORMATION, threads_base_ptr: *const SYSTEM_THREAD_INFORMATION) -> Self {
        let name = unsafe {
            if process.ImageName.Length > 0 && !process.ImageName.Buffer.is_null() {
                let wchar_count = (process.ImageName.Length / 2) as usize;
                let wide_slice = slice::from_raw_parts(process.ImageName.Buffer, wchar_count);
                String::from_utf16_lossy(wide_slice).to_lowercase()
            } else {
                String::new()
            }
        };
        ProcessEntry {
            process,
            threads: HashMap::new(),
            threads_base_ptr: threads_base_ptr as usize,
            name,
        }
    }
    #[inline]
    pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION> {
        if self.process.NumberOfThreads as usize != self.threads.len() {
            let _ = &self.threads.clear();
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
    #[inline]
    pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION> {
        self.get_threads().get(&tid)
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline]
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
    pub fn thread_count(&self) -> u32 {
        self.process.NumberOfThreads
    }
}

fn use_console() -> &'static Mutex<bool> {
    static CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
    &CONSOLE
}
fn logger() -> &'static Mutex<File> {
    static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
    &LOG_FILE
}
fn find_logger() -> &'static Mutex<File> {
    static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
    &FIND_LOG_FILE
}
fn get_log_path(suffix: &str) -> PathBuf {
    let time = LOCALTIME_BUFFER.lock().unwrap();
    let (year, month, day) = (time.year(), time.month(), time.day());
    drop(time);
    let log_dir = PathBuf::from("logs");
    if !log_dir.exists() {
        let _ = fs::create_dir_all(&log_dir);
    }
    log_dir.join(format!("{:04}{:02}{:02}{}.log", year, month, day, suffix))
}

fn get_cpu_set_information() -> &'static Mutex<Vec<SYSTEM_CPU_SET_INFORMATION>> {
    static CPU_SET_INFORMATION: Lazy<Mutex<Vec<SYSTEM_CPU_SET_INFORMATION>>> = Lazy::new(|| {
        Mutex::new({
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
        })
    });
    &CPU_SET_INFORMATION
}

static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

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
pub fn log_to_find(msg: &str) {
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        println!("[{}]{}", time_prefix, msg);
    } else {
        let _ = writeln!(find_logger().lock().unwrap(), "[{}]{}", time_prefix, msg);
    }
}
pub fn log_process_find(process_name: &str) {
    if FINDS_SET.lock().unwrap().insert(process_name.to_string()) {
        log_to_find(&format!("find {}", process_name));
    }
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

fn read_config<P: AsRef<Path>>(path: P) -> io::Result<(HashMap<String, ProcessConfig>, ConfigConstants)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut configs = HashMap::new();
    let mut affinity_aliases = HashMap::new();
    let mut constants = ConfigConstants::default();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        } else if line.starts_with('@') {
            // define constant: @NAME=VALUE
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
                    "KEEP_THRESHOLD" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.keep_threshold = v;
                            log!("Config: KEEP_THRESHOLD = {}", v);
                        }
                    }
                    "ENTRY_THRESHOLD" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.entry_threshold = v;
                            log!("Config: ENTRY_THRESHOLD = {}", v);
                        }
                    }
                    _ => {
                        log_to_find(&format!("Unknown constant: {}", const_name));
                    }
                }
            }
            continue;
        } else if line.starts_with('*') {
            // define mask alias: *NAME=VALUE
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

        // process configuration line
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
            } else if cpuset_def.starts_with("0x") {
                usize::from_str_radix(cpuset_def.trim_start_matches("0x"), 16).unwrap_or(0)
            } else {
                cpuset_def.parse().unwrap_or(0)
            };
            let prime_cpu_def = if parts.len() >= 5 { parts[4].trim() } else { "0" };
            let prime_cpuset = if prime_cpu_def.starts_with('*') {
                *affinity_aliases.get(&prime_cpu_def.trim_start_matches('*').to_lowercase()).unwrap_or(&0)
            } else if prime_cpu_def.starts_with("0x") {
                usize::from_str_radix(prime_cpu_def.trim_start_matches("0x"), 16).unwrap_or(0)
            } else {
                prime_cpu_def.parse().unwrap_or(0)
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
fn read_utf16le_file(path: &str) -> io::Result<String> {
    let bytes = fs::read(path)?;
    let utf16: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    Ok(String::from_utf16_lossy(&utf16))
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
#[inline]
fn split_trim_nonempty(s: &str) -> Vec<&str> {
    s.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
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

fn is_affinity_unset(pid: u32, process_name: &str) -> bool {
    unsafe {
        let mut result = false;
        match OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, false, pid) {
            Err(_) => {
                let code = GetLastError().0;
                log_to_find(&format!("is_affinity_unset: [OPEN][{}] {:>5}-{}", error_from_code(code), pid, process_name));
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
                            log_to_find(&format!("is_affinity_unset: [AFFINITY_QUERY][{}] {:>5}-{}", error_from_code(code), pid, process_name));
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

fn apply_config(pid: u32, config: &ProcessConfig, prime_core_scheduler: &mut PrimeThreadScheduler, processes: &mut ProcessSnapshot) {
    let h_prc = match unsafe { OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, false, pid) } {
        Err(_) => {
            log_to_find(&format!(
                "apply_config: [OPEN][{}] {:>5}-{}",
                error_from_code(unsafe { GetLastError().0 }),
                pid,
                config.name
            ));
            return;
        }
        Ok(h_prc) => h_prc,
    };
    if h_prc.is_invalid() {
        log_to_find(&format!("apply_config: [INVALID_HANDLE] {:>5}-{}", pid, config.name));
        return;
    }

    // Priority
    if let Some(priority_flag) = config.priority.as_win_const() {
        if unsafe { GetPriorityClass(h_prc) } != priority_flag.0 {
            if unsafe { SetPriorityClass(h_prc, priority_flag) }.is_ok() {
                log!("{:>5}-{} -> Priority: {}", pid, config.name, config.priority.as_str());
            } else {
                log_to_find(&format!(
                    "apply_config: [SET_PRIORITY][{}] {:>5}-{}",
                    error_from_code(unsafe { GetLastError().0 }),
                    pid,
                    config.name
                ));
            }
        }
    }

    // Affinity
    let mut current_mask: usize = 0;
    let mut system_mask: usize = 0;
    if config.affinity_mask != 0 || config.prime_cpu_mask != 0 {
        match unsafe { GetProcessAffinityMask(h_prc, &mut current_mask, &mut system_mask) } {
            Err(_) => {
                log_to_find(&format!(
                    "apply_config: [QUERY_AFFINITY][{}] {:>5}-{}",
                    error_from_code(unsafe { GetLastError().0 }),
                    pid,
                    config.name
                ));
            }
            Ok(_) => {
                if config.affinity_mask != 0 && config.affinity_mask != current_mask {
                    match unsafe { SetProcessAffinityMask(h_prc, config.affinity_mask) } {
                        Err(_) => {
                            log_to_find(&format!(
                                "apply_config: [SET_AFFINITY][{}] {:>5}-{}",
                                error_from_code(unsafe { GetLastError().0 }),
                                pid,
                                config.name
                            ));
                        }
                        Ok(_) => {
                            log!("{:>5}-{} {:#X}-> {:#X}", pid, config.name, current_mask, config.affinity_mask);
                            current_mask = config.affinity_mask;
                        }
                    }
                }
            }
        }
    }

    // process default CPU Set
    if config.cpu_set_mask != 0 && !get_cpu_set_information().lock().unwrap().is_empty() {
        let mut toset: bool = false;
        let mut requiredidcount: u32 = 0;
        let query_result = unsafe { GetProcessDefaultCpuSets(h_prc, None, &mut requiredidcount).as_bool() };
        if query_result {
            // 0 is large enough, meaning there are no default CPU sets for this process, so we need to set the default CPU set
            toset = true;
        } else {
            let code = unsafe { GetLastError().0 };
            if code != 122 {
                log_to_find(&format!(
                    "apply_config: [QUERY_CPUSET][{}] {:>5}-{}-{}",
                    error_from_code(code),
                    pid,
                    config.name,
                    requiredidcount
                ));
            } else {
                let mut current_cpusetids: Vec<u32> = vec![0u32; requiredidcount as usize];
                let second_query = unsafe { GetProcessDefaultCpuSets(h_prc, Some(&mut current_cpusetids[..]), &mut requiredidcount).as_bool() };
                if !second_query {
                    log_to_find(&format!(
                        "apply_config: [QUERY_CPUSET][{}] {:>5}-{}-{}",
                        error_from_code(unsafe { GetLastError().0 }),
                        pid,
                        config.name,
                        requiredidcount
                    ));
                } else {
                    toset = current_cpusetids != cpusetids_from_mask(config.cpu_set_mask);
                }
            }
        }
        if toset {
            let set_result = unsafe { SetProcessDefaultCpuSets(h_prc, Some(&cpusetids_from_mask(config.cpu_set_mask))).as_bool() };
            if !set_result {
                log_to_find(&format!(
                    "apply_config: [SET_CPUSET][{}] {:>5}-{}",
                    error_from_code(unsafe { GetLastError().0 }),
                    pid,
                    config.name
                ));
            } else {
                log!("{:>5}-{} -> (cpu set) {:#X}", pid, config.name, config.cpu_set_mask);
            }
        }
    }

    // Prime thread Scheduling
    if config.prime_cpu_mask != 0 {
        let cpu_setids = cpusetids_from_mask(config.prime_cpu_mask & current_mask);
        if !cpu_setids.is_empty() {
            prime_core_scheduler.set_alive(pid);
            let process = processes.pid_to_process.get_mut(&pid).unwrap();
            let thread_count = process.thread_count() as usize;
            let candidate_count = get_cpu_set_information().lock().unwrap().len().min(thread_count);
            let mut candidate_tids: Vec<u32> = vec![0u32; candidate_count];
            let mut tid_with_delta_cycles: Vec<(u32, u64, bool)> = vec![(0u32, 0u64, false); candidate_count];

            // Step 1: Sort threads by delta time and select top candidates
            {
                let mut tid_with_delta_time: Vec<(u32, i64)> = Vec::with_capacity(thread_count);
                process.get_threads().iter().for_each(|(tid, thread)| {
                    let total_time = unsafe { thread.KernelTime.QuadPart() + thread.UserTime.QuadPart() };
                    let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);
                    tid_with_delta_time.push((*tid, total_time - thread_stats.last_total_time));
                    thread_stats.last_total_time = total_time;
                });
                tid_with_delta_time.sort_by_key(|&(_, delta)| delta);
                let precandidate_len = tid_with_delta_time.len();
                for i in 0..candidate_count {
                    candidate_tids[i] = tid_with_delta_time[precandidate_len - i - 1].0;
                }
            }

            // Step 2: Open thread handles and query cycle times
            for i in 0..candidate_count {
                let tid = candidate_tids[i];
                let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
                let process_name = &config.name;
                match thread_stats.handle {
                    None => {
                        match unsafe { OpenThread(THREAD_QUERY_INFORMATION | THREAD_SET_LIMITED_INFORMATION, false, tid) } {
                            Err(_) => {
                                let error_code = error_from_code(unsafe { GetLastError().0 });
                                log_to_find(&format!("apply_config: [OPEN_THREAD][{}] {:>5}-{}-{}", error_code, pid, tid, process_name));
                            }
                            Ok(handle) => {
                                let mut cycles: u64 = 0;
                                match unsafe { QueryThreadCycleTime(handle, &mut cycles) } {
                                    Err(_) => {
                                        let error_code = error_from_code(unsafe { GetLastError().0 });
                                        log_to_find(&format!("apply_config: [QUERY_THREAD_CYCLE][{}] {:>5}-{}-{}", error_code, pid, tid, process_name));
                                    }
                                    Ok(_) => {
                                        tid_with_delta_cycles[i] = (tid, cycles, false);
                                    }
                                };
                                thread_stats.handle = Some(handle);
                            }
                        };
                    }
                    Some(handle) => {
                        let mut cycles: u64 = 0;
                        match unsafe { QueryThreadCycleTime(handle, &mut cycles) } {
                            Err(_) => {
                                let error_code = error_from_code(unsafe { GetLastError().0 });
                                log_to_find(&format!("apply_config: [QUERY_THREAD_CYCLE][{}] {:>5}-{}-{}", error_code, pid, tid, process_name));
                            }
                            Ok(_) => {
                                tid_with_delta_cycles[i] = (tid, cycles - thread_stats.last_cycles, false);
                                thread_stats.last_cycles = cycles;
                            }
                        };
                    }
                }
            }

            // Step 3: Sort by delta_cycles descending and calculate thresholds
            tid_with_delta_cycles.sort_by_key(|&(_, delta_cycles, _)| std::cmp::Reverse(delta_cycles));
            let max_cycles = tid_with_delta_cycles.first().map(|&(_, c, _)| c).unwrap_or(0u64);
            let entry_min_cycles = (max_cycles as f64 * prime_core_scheduler.constants.entry_threshold) as u64;
            let keep_min_cycles = (max_cycles as f64 * prime_core_scheduler.constants.keep_threshold) as u64;
            let prime_count = cpu_setids.len().min(candidate_count);
            // Update active_streak for all candidate threads
            for &(tid, delta_cycles, _) in &tid_with_delta_cycles {
                if tid == 0 {
                    continue;
                }
                let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
                if delta_cycles >= entry_min_cycles {
                    thread_stats.active_streak = thread_stats.active_streak.saturating_add(1).min(254);
                } else {
                    thread_stats.active_streak = 0;
                }
            }
            let mut new_prime_count: usize = 0;
            // First pass: mark protected prime threads
            for (tid, delta_cycles, is_prime) in tid_with_delta_cycles.iter_mut() {
                if *tid == 0 || new_prime_count >= prime_count {
                    continue;
                }
                if !prime_core_scheduler.get_thread_stats(pid, *tid).cpu_set_ids.is_empty() && *delta_cycles >= keep_min_cycles {
                    *is_prime = true;
                    new_prime_count += 1;
                }
            }
            // Second pass: mark new candidates
            for (tid, delta_cycles, is_prime) in tid_with_delta_cycles.iter_mut() {
                if new_prime_count >= prime_count {
                    break;
                }
                if *tid == 0 || *is_prime {
                    continue;
                }
                if *delta_cycles >= entry_min_cycles && prime_core_scheduler.get_thread_stats(pid, *tid).active_streak >= 2 {
                    *is_prime = true;
                    new_prime_count += 1;
                }
            }

            // Step 4: Promote new threads
            for &(tid, delta_cycles, is_prime) in &tid_with_delta_cycles {
                if !is_prime {
                    continue;
                }
                let thread_stats = prime_core_scheduler.get_thread_stats(pid, tid);
                if let Some(handle) = thread_stats.handle {
                    if !handle.is_invalid() && thread_stats.cpu_set_ids.is_empty() {
                        let set_result = unsafe { SetThreadSelectedCpuSets(handle, &cpu_setids).as_bool() };
                        if !set_result {
                            let error_code = error_from_code(unsafe { GetLastError().0 });
                            log_to_find(&format!("apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}", error_code, pid, tid, config.name));
                        } else {
                            thread_stats.cpu_set_ids = cpu_setids.clone();
                            let promoted_mask = mask_from_cpusetids(&cpu_setids);
                            log!("{:>5}-{}-{} -> (promoted, {:#X}, cycles={})", pid, tid, config.name, promoted_mask, delta_cycles);
                        }
                    }
                }
            }

            // Step 5: Demote threads that are no longer prime
            process.get_threads().iter().for_each(|(tid, _)| {
                let thread_stats = prime_core_scheduler.get_thread_stats(pid, *tid);
                if !tid_with_delta_cycles.iter().any(|&(t, _, p)| t == *tid && p) && !thread_stats.cpu_set_ids.is_empty() {
                    if let Some(handle) = thread_stats.handle {
                        if !handle.is_invalid() {
                            let set_result = unsafe { SetThreadSelectedCpuSets(handle, &[]).as_bool() };
                            if !set_result {
                                let error_code = error_from_code(unsafe { GetLastError().0 });
                                log_to_find(&format!("apply_config: [SET_THREAD_CPU_SETS][{}] {:>5}-{}-{}", error_code, pid, tid, config.name));
                            } else {
                                log!("{:>5}-{}-{} -> (demoted)", pid, tid, config.name);
                            }
                        }
                    }
                    thread_stats.cpu_set_ids = vec![];
                };
            });
        }
    }

    // IO Priority
    if let Some(io_priority_flag) = config.io_priority.as_win_const() {
        const PROCESS_INFORMATION_IO_PRIORITY: u32 = 33;
        let mut current_io_priority: u32 = 0;
        let mut return_length: u32 = 0;
        let query_result = unsafe {
            NtQueryInformationProcess(
                h_prc,
                PROCESS_INFORMATION_IO_PRIORITY,
                &mut current_io_priority as *mut _ as *mut std::ffi::c_void,
                size_of::<u32>() as u32,
                &mut return_length,
            )
            .0
        };
        if query_result < 0 {
            log_to_find(&format!(
                "apply_config: [QUERY_IO_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                query_result,
                pid,
                config.name,
                config.io_priority.as_str()
            ));
        } else if current_io_priority != io_priority_flag {
            let set_result = unsafe {
                NtSetInformationProcess(
                    h_prc,
                    PROCESS_INFORMATION_IO_PRIORITY,
                    &io_priority_flag as *const _ as *const std::ffi::c_void,
                    size_of::<u32>() as u32,
                )
                .0
            };
            if set_result < 0 {
                log_to_find(&format!(
                    "apply_config: [SET_IO_PRIORITY][0x{:08X}] {:>5}-{} -> {}",
                    set_result,
                    pid,
                    config.name,
                    config.io_priority.as_str()
                ));
            } else {
                log!("{:>5}-{} -> IO: {}", pid, config.name, config.io_priority.as_str());
            }
        }
    }

    // Memory Priority
    if let Some(memory_priority_flag) = config.memory_priority.as_win_const() {
        let set_result = unsafe {
            SetProcessInformation(
                h_prc,
                ProcessMemoryPriority,
                &MemoryPriorityInformation(memory_priority_flag.0) as *const _ as *const std::ffi::c_void,
                size_of::<MemoryPriorityInformation>() as u32,
            )
        };
        if set_result.is_err() {
            log_to_find(&format!(
                "apply_config: [SET_MEMORY_PRIORITY][{}] {:>5}-{} -> {}",
                error_from_code(unsafe { GetLastError().0 }),
                pid,
                config.name,
                config.memory_priority.as_str()
            ));
        }
    }

    unsafe {
        let _ = CloseHandle(h_prc);
    }
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
    let mut prime_core_scheduler = PrimeThreadScheduler::new(constants);
    let mut current_loop = 0u32;
    let mut should_continue = true;

    while should_continue {
        if log_loop {
            log!("Loop {} started", current_loop + 1);
        }
        match ProcessSnapshot::take() {
            Ok(mut processes) => {
                prime_core_scheduler.reset_alive();
                for i in 0..processes.pid_to_process.values().len() {
                    let process = processes.pid_to_process.values().nth(i).unwrap();
                    if let Some(config) = configs.get(process.get_name()) {
                        apply_config(process.pid(), config, &mut prime_core_scheduler, &mut processes);
                    }
                }
                prime_core_scheduler.close_dead_process_handles();
                drop(processes);
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
