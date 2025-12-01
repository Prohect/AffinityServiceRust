use chrono::{DateTime, Datelike, Local};
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    process::Command,
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
            MEMORY_PRIORITY_VERY_LOW, NORMAL_PRIORITY_CLASS, OpenProcess, OpenProcessToken, PROCESS_CREATION_FLAGS, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION,
            ProcessMemoryPriority, REALTIME_PRIORITY_CLASS, SetPriorityClass, SetProcessAffinityMask, SetProcessDefaultCpuSets, SetProcessInformation,
        },
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
#[derive(Debug, Clone)]
struct ProcessConfig {
    name: String,
    priority: ProcessPriority,
    affinity_mask: usize,
    cpu_set_mask: usize,
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
    // High,     // Requires SeIncreaseBasePriorityPrivilege
    // Critical, // Requires SeIncreaseBasePriorityPrivilege
}
impl IOPriority {
    const TABLE: &'static [(Self, &'static str, Option<u32>)] = &[
        (Self::None, "none", None),
        (Self::VeryLow, "very low", Some(0)),
        (Self::Low, "low", Some(1)),
        (Self::Normal, "normal", Some(2)),
        // (Self::High, "high", Some(3)),           // Requires SeIncreaseBasePriorityPrivilege
        // (Self::Critical, "critical", Some(4)), // Requires SeIncreaseBasePriorityPrivilege
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

fn read_config<P: AsRef<Path>>(path: P) -> io::Result<Vec<ProcessConfig>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut configs = Vec::new();
    let mut affinity_aliases = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        } else if line.starts_with('*') {
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
            let cpuset_def = parts[3].trim();
            let cpuset = if cpuset_def.starts_with('*') {
                *affinity_aliases.get(&cpuset_def.trim_start_matches('*').to_lowercase()).unwrap_or(&0)
            } else if parts[3].trim_start().starts_with("0x") {
                usize::from_str_radix(cpuset_def.trim_start_matches("0x"), 16).unwrap_or(0)
            } else {
                parts[3].parse().unwrap_or(0)
            };
            let io_priority = if parts.len() >= 5 { IOPriority::from_str(parts[4]) } else { IOPriority::None };
            let memory_priority = if parts.len() >= 6 {
                MemoryPriority::from_str(parts[5])
            } else {
                MemoryPriority::None
            };
            configs.push(ProcessConfig {
                name,
                priority,
                affinity_mask: affinity,
                cpu_set_mask: cpuset,
                io_priority,
                memory_priority,
            });
        }
    }
    Ok(configs)
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

fn apply_config(pid: u32, config: &ProcessConfig) {
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

                    if config.affinity_mask != 0 {
                        let mut current_mask: usize = 0;
                        let mut system_mask: usize = 0;
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
                                mask if mask != current_mask => match SetProcessAffinityMask(h_prc, mask) {
                                    Err(_) => {
                                        log_to_find(&format!(
                                            "apply_config: [SET_AFFINITY_FAILED][{}] {:>5}-{}",
                                            error_from_code(GetLastError().0),
                                            pid,
                                            config.name
                                        ));
                                    }
                                    Ok(_) => {
                                        log!("{:>5}-{} {:#X}-> {:#X}", pid, config.name, current_mask, mask);
                                    }
                                },
                                _ => {}
                            },
                        }
                    }

                    if config.cpu_set_mask != 0 {
                        let mut toset: bool = false;
                        let mut requiredidcount: u32 = 0;
                        match GetProcessDefaultCpuSets(h_prc, None, &mut requiredidcount).as_bool() {
                            true => {
                                toset = true;
                            }
                            false => {
                                let mut current_cpusetids: Vec<u32> = vec![0u32; requiredidcount as usize];
                                match GetProcessDefaultCpuSets(h_prc, Some(&mut current_cpusetids[..]), &mut requiredidcount).as_bool() {
                                    false => {
                                        log_to_find(&format!("apply_config: [QUERY_CPUSET_FAILED] {:>5}-{}-{}", pid, config.name, requiredidcount));
                                    }
                                    true => {
                                        toset = current_cpusetids != cpusetids_from_mask(config.cpu_set_mask);
                                    }
                                }
                            }
                        }
                        if toset {
                            match SetProcessDefaultCpuSets(h_prc, Some(&cpusetids_from_mask(config.cpu_set_mask))).as_bool() {
                                false => {
                                    log_to_find(&format!("apply_config: [SET_CPUSET_FAILED] {:>5}-{}", pid, config.name));
                                }
                                true => {
                                    log!("{:>5}-{} -> (cpu set) {:#X}", pid, config.name, config.cpu_set_mask);
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
                                    "{},{},0x{:X},0x{:X},{}",
                                    cfg.name,
                                    cfg.priority.as_str(),
                                    cfg.affinity_mask,
                                    cfg.cpu_set_mask,
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
            false => panic!("GetSystemCpuSetInformation failed"),
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
    let configs = read_config(&config_file_name).unwrap_or_else(|_| {
        if !skip_log_before_elevation {
            log!("cannot read configs: {}", config_file_name);
        }
        Vec::new()
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
    let mut current_loop = 0u32;
    let mut should_continue = true;
    while should_continue {
        if log_loop {
            log!("Loop {} started", current_loop + 1);
        }
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
            let mut pe32 = PROCESSENTRY32W::default();
            pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
            if Process32FirstW(snapshot, &mut pe32).is_ok() {
                'out_loop: loop {
                    let process_name = String::from_utf16_lossy(&pe32.szExeFile[..pe32.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)]).to_lowercase();
                    if !FAIL_SET.lock().unwrap().contains(&process_name) {
                        for config in &configs {
                            if process_name == config.name {
                                apply_config(pe32.th32ProcessID, &config);
                                if !Process32NextW(snapshot, &mut pe32).is_ok() {
                                    break 'out_loop;
                                }
                                continue 'out_loop;
                            }
                        }
                        if find_mode {
                            if blacklist.contains(&process_name) {
                                if !Process32NextW(snapshot, &mut pe32).is_ok() {
                                    break;
                                }
                                continue;
                            }
                            if is_affinity_unset(pe32.th32ProcessID, process_name.as_str()) {
                                log_process_find(&process_name);
                            }
                        }
                    }
                    if !Process32NextW(snapshot, &mut pe32).is_ok() {
                        break;
                    }
                }
            }
            let _ = CloseHandle(snapshot);
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
