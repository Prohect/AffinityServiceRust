use chrono::{DateTime, Datelike, Local};
use once_cell::sync::Lazy;
use std::{
    collections::HashSet,
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::Mutex,
    thread,
    time::Duration,
};
use windows::Win32::System::Threading::GetPriorityClass;
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE, LUID, NTSTATUS},
    Security::{AdjustTokenPrivileges, GetTokenInformation, LookupPrivilegeValueW, SE_DEBUG_NAME, TOKEN_ADJUST_PRIVILEGES, TOKEN_ELEVATION, TOKEN_PRIVILEGES, TOKEN_QUERY, TokenElevation},
    System::{
        Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS},
        Threading::{
            ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, GetCurrentProcess, GetProcessAffinityMask, HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, OpenProcess,
            OpenProcessToken, PROCESS_CREATION_FLAGS, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION, REALTIME_PRIORITY_CLASS, SetPriorityClass, SetProcessAffinityMask,
        },
    },
};

// IO Priority constants
const IO_PRIORITY_VERY_LOW: u32 = 0;
const IO_PRIORITY_LOW: u32 = 1;
const IO_PRIORITY_NORMAL: u32 = 2;
// const IO_PRIORITY_HIGH: u32 = 3;        // Requires SeIncreaseBasePriorityPrivilege
// const IO_PRIORITY_CRITICAL: u32 = 4;    // Requires SeIncreaseBasePriorityPrivilege

// Process Information Class for IO Priority
const PROCESS_INFORMATION_IO_PRIORITY: u32 = 33;

// Memory Priority constants - COMMENTED OUT due to API limitations
// const MEMORY_PRIORITY_VERY_LOW: u32 = 1;
// const MEMORY_PRIORITY_LOW: u32 = 2;
// const MEMORY_PRIORITY_MEDIUM: u32 = 3;
// const MEMORY_PRIORITY_BELOW_NORMAL: u32 = 4;
// const MEMORY_PRIORITY_NORMAL: u32 = 5;

// Process Information Class for Memory Priority - COMMENTED OUT
// const PROCESS_INFORMATION_MEMORY_PRIORITY: u32 = 61;  // Returns STATUS_INVALID_INFO_CLASS (0xC0000003)

// External function declarations for process information
#[link(name = "ntdll")]
unsafe extern "system" {
    fn NtSetInformationProcess(process_handle: HANDLE, process_information_class: u32, process_information: *const u32, process_information_length: u32) -> NTSTATUS;
    fn NtQueryInformationProcess(process_handle: HANDLE, process_information_class: u32, process_information: *mut u32, process_information_length: u32, return_length: *mut u32) -> NTSTATUS;
}

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
    io_priority: IOPriority,
    // memory_priority: MemoryPriority,  // Commented out - Windows API not working with standard privileges
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    // High,     // Requires SeIncreaseBasePriorityPrivilege - commented out
    // Critical, // Requires SeIncreaseBasePriorityPrivilege - commented out
}

// Memory Priority enum - COMMENTED OUT due to Windows API limitations
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum MemoryPriority {
//     None,
//     VeryLow,
//     Low,
//     Medium,
//     BelowNormal,
//     Normal,
// }
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
        Self::TABLE.iter().find(|(_, name, _)| s.to_lowercase() == *name).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }
}

impl IOPriority {
    const TABLE: &'static [(Self, &'static str, u32)] = &[
        (Self::None, "none", IO_PRIORITY_NORMAL),
        (Self::VeryLow, "very low", IO_PRIORITY_VERY_LOW),
        (Self::Low, "low", IO_PRIORITY_LOW),
        (Self::Normal, "normal", IO_PRIORITY_NORMAL),
        // (Self::High, "high", IO_PRIORITY_HIGH),           // Requires SeIncreaseBasePriorityPrivilege
        // (Self::Critical, "critical", IO_PRIORITY_CRITICAL), // Requires SeIncreaseBasePriorityPrivilege
    ];

    pub fn as_str(&self) -> &'static str {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("fail as str")
    }

    pub fn as_win_const(&self) -> u32 {
        Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, _, val)| *val).unwrap_or(IO_PRIORITY_NORMAL)
    }

    pub fn from_str(s: &str) -> Self {
        Self::TABLE.iter().find(|(_, name, _)| s.to_lowercase() == *name).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }
}

// Memory Priority implementation - COMMENTED OUT due to Windows API limitations
// Returns STATUS_INVALID_INFO_CLASS (0xC0000003) - likely requires different approach or privileges
// impl MemoryPriority {
//     const TABLE: &'static [(Self, &'static str, u32)] = &[
//         (Self::None, "none", MEMORY_PRIORITY_NORMAL),
//         (Self::VeryLow, "very low", MEMORY_PRIORITY_VERY_LOW),
//         (Self::Low, "low", MEMORY_PRIORITY_LOW),
//         (Self::Medium, "medium", MEMORY_PRIORITY_MEDIUM),
//         (Self::BelowNormal, "below normal", MEMORY_PRIORITY_BELOW_NORMAL),
//         (Self::Normal, "normal", MEMORY_PRIORITY_NORMAL),
//     ];
//
//     pub fn as_str(&self) -> &'static str {
//         Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, name, _)| *name).unwrap_or("fail as str")
//     }
//
//     pub fn as_win_const(&self) -> u32 {
//         Self::TABLE.iter().find(|(v, _, _)| v == self).map(|(_, _, val)| *val).unwrap_or(MEMORY_PRIORITY_NORMAL)
//     }
//
//     pub fn from_str(s: &str) -> Self {
//         Self::TABLE.iter().find(|(_, name, _)| s.to_lowercase() == *name).map(|(v, _, _)| *v).unwrap_or(Self::None)
//     }
// }

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
        log_to_find(&format!("find {}", process_name))
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
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            let name = parts[0].to_lowercase();
            let priority: ProcessPriority = ProcessPriority::from_str(&parts[1]);
            let affinity = if parts[2].trim_start().starts_with("0x") {
                usize::from_str_radix(parts[2].trim_start_matches("0x"), 16).unwrap_or(0)
            } else {
                parts[2].parse().unwrap_or(0)
            };
            let io_priority = if parts.len() >= 4 { IOPriority::from_str(parts[3]) } else { IOPriority::None };
            // let memory_priority = if parts.len() >= 5 { MemoryPriority::from_str(parts[4]) } else { MemoryPriority::None };
            configs.push(ProcessConfig {
                name,
                priority,
                affinity_mask: affinity,
                io_priority,
                // memory_priority,  // Commented out - Windows API not working
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
                let code = GetLastError().0;
                log_to_find(&format!("set_priority_and_affinity: [OPEN_FAILED][{}] {:>5}-{}", error_from_code(code), pid, config.name));
            }
            Ok(h_proc) => {
                if h_proc.is_invalid() {
                    log_to_find(&format!("set_priority_and_affinity: [INVALID_HANDLE] {:>5}-{}", pid, config.name));
                } else {
                    if let Some(priority_flag) = config.priority.as_win_const() {
                        if GetPriorityClass(h_proc) != priority_flag.0 {
                            if SetPriorityClass(h_proc, priority_flag).is_ok() {
                                log!("{:>5}-{} -> {}", pid, config.name, config.priority.as_str());
                            } else {
                                let code = GetLastError().0;
                                log_to_find(&format!("set_priority_and_affinity: [SET_PRIORITY_FAILED][{}] {:>5}-{}", error_from_code(code), pid, config.name));
                            }
                        }
                    }

                    let mut current_mask: usize = 0;
                    let mut system_mask: usize = 0;
                    match GetProcessAffinityMask(h_proc, &mut current_mask, &mut system_mask) {
                        Err(_) => {
                            let code = GetLastError().0;
                            log_to_find(&format!(
                                "set_priority_and_affinity: [AFFINITY_QUERY_FAILED][{}] {:>5}-{}",
                                error_from_code(code),
                                pid,
                                config.name
                            ));
                        }
                        Ok(_) => match config.affinity_mask {
                            0 => {}
                            mask if mask != current_mask => match SetProcessAffinityMask(h_proc, mask) {
                                Err(_) => {
                                    let code = GetLastError().0;
                                    log_to_find(&format!("set_priority_and_affinity: [SET_AFFINITY_FAILED][{}] {:>5}-{}", error_from_code(code), pid, config.name));
                                }
                                Ok(_) => {
                                    log!("{:>5}-{} -> {:#X}", pid, config.name, mask);
                                }
                            },
                            _ => {}
                        },
                    }

                    // Apply IO Priority if not None
                    if config.io_priority != IOPriority::None {
                        let mut current_io_priority: u32 = 0;
                        let mut return_length: u32 = 0;
                        let query_result = NtQueryInformationProcess(
                            h_proc,
                            PROCESS_INFORMATION_IO_PRIORITY,
                            &mut current_io_priority as *mut u32,
                            std::mem::size_of::<u32>() as u32,
                            &mut return_length as *mut u32,
                        );

                        let desired_io_priority = config.io_priority.as_win_const();

                        // Only set IO priority if it's different from current or query failed
                        if query_result.0 < 0 || current_io_priority != desired_io_priority {
                            let result = NtSetInformationProcess(h_proc, PROCESS_INFORMATION_IO_PRIORITY, &desired_io_priority as *const u32, std::mem::size_of::<u32>() as u32);

                            if result.0 >= 0 {
                                log!("{:>5}-{} -> IO: {}", pid, config.name, config.io_priority.as_str());
                            } else {
                                log_to_find(&format!(
                                    "set_io_priority: [SET_IO_PRIORITY_FAILED][0x{:08X}] {:>5}-{} -> {}",
                                    result.0,
                                    pid,
                                    config.name,
                                    config.io_priority.as_str()
                                ));
                            }
                        }
                    }

                    // Apply Memory Priority - COMMENTED OUT due to Windows API limitations
                    // Returns STATUS_INVALID_INFO_CLASS (0xC0000003) with PROCESS_INFORMATION_MEMORY_PRIORITY
                    // if config.memory_priority != MemoryPriority::None {
                    //     let memory_priority_value = config.memory_priority.as_win_const();
                    //     let result = NtSetInformationProcess(
                    //         h_proc,
                    //         PROCESS_INFORMATION_MEMORY_PRIORITY,
                    //         &memory_priority_value as *const u32,
                    //         std::mem::size_of::<u32>() as u32,
                    //     );
                    //
                    //     if result.0 >= 0 {
                    //         log!("{:>5}-{} -> Memory: {}", pid, config.name, config.memory_priority.as_str());
                    //     } else {
                    //         log_to_find(&format!(
                    //             "set_memory_priority: [SET_MEMORY_PRIORITY_FAILED][0x{:08X}] {:>5}-{} -> {}",
                    //             result.0,
                    //             pid,
                    //             config.name,
                    //             config.memory_priority.as_str()
                    //         ));
                    //     }
                    // }

                    let _ = CloseHandle(h_proc);
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
                            log_to_find(&format!("is_affinity_unset: [AFFINITY_QUERY_FAILED][{}] {:>5}-{}", error_from_code(code), pid, process_name));
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
                                            io_priority: IOPriority::None,
                                            // memory_priority: MemoryPriority::None,  // Commented out
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
                                            io_priority: IOPriority::None,
                                            // memory_priority: MemoryPriority::None,  // Commented out
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
                                let _ = writeln!(output, "{},{},0x{:X}", cfg.name, cfg.priority.as_str(), cfg.affinity_mask);
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
    convert_mode: &mut bool,
    find_mode: &mut bool,
    config_file_name: &mut String,
    blacklist_file_name: &mut Option<String>,
    in_file_name: &mut Option<String>,
    out_file_name: &mut Option<String>,
    no_uac: &mut bool,
    loop_count: &mut Option<u32>,
    log_loop: &mut bool,
) -> windows::core::Result<()> {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-help" | "--help" | "-?" | "/?" | "?" => {
                *help_mode = true;
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
            _ => {}
        }
        i += 1;
    }
    Ok(())
}

fn print_help() {
    println!("usage: AffinityServiceRust.exe args");
    println!();
    println!("  -help | --help       print this help message");
    println!("  -? | /? | ?          print this help message");
    println!("  -console             use console as output instead of log file");
    println!("  -noUAC | -nouac      disable UAC elevation request");
    println!("  -convert             convert process configs from -in <file>(from process lasso) to -out <file>");
    println!("  -find                find those whose affinity is same as system default which is all possible cores windows could use");
    println!("  -interval <ms>       set interval for checking again (5000 by default, minimal 16)");
    println!("  -loop <count>        number of loops to run (default: infinite)");
    println!("  -logloop             log a message at the start of each loop for testing");
    println!("  -config <file>       the config file u wanna use (config.ini by default)");
    println!("  -blacklist <file>    the blacklist for -find");
    println!("  -in <file>           input file for -convert");
    println!("  -out <file>          output file for -convert");
    println!();
    println!("Config file format: process_name,priority,affinity_mask,io_priority");
    println!("  Priority:        none, idle, below normal, normal, above normal, high, real time");
    println!("  Affinity:        0 (no change), or hex/decimal mask (e.g., 0xFF, 255)");
    println!("  IO Priority:     none, very low, low, normal");
    println!("  Example:         notepad.exe,above normal,0xFF,low");
    println!("  Note:            Memory priority management is not yet supported due to Windows API limitations");
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

        // Open process token
        if OpenProcessToken(current_process, TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation: TOKEN_ELEVATION = TOKEN_ELEVATION::default();
        let mut return_length = 0u32;

        // Check if token is elevated
        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        let _ = CloseHandle(token);

        if result.is_ok() { elevation.TokenIsElevated != 0 } else { false }
    }
}

fn request_uac_elevation() -> std::io::Result<()> {
    let exe_path = env::current_exe()?;
    let args: Vec<String> = env::args().skip(1).collect();

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

fn main() -> windows::core::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut interval_ms = 5000;
    let mut help_mode = false;
    let mut convert_mode = false;
    let mut find_mode = false;
    let mut config_file_name = "config.ini".to_string();
    let mut blacklist_file_name: Option<String> = None;
    let mut in_file_name: Option<String> = None;
    let mut out_file_name: Option<String> = None;
    let mut no_uac = false;
    let mut loop_count: Option<u32> = None;
    let mut log_loop = false;
    parse_args(
        &args,
        &mut interval_ms,
        &mut help_mode,
        &mut convert_mode,
        &mut find_mode,
        &mut config_file_name,
        &mut blacklist_file_name,
        &mut in_file_name,
        &mut out_file_name,
        &mut no_uac,
        &mut loop_count,
        &mut log_loop,
    )?;
    if help_mode {
        print_help();
        return Ok(());
    }
    if convert_mode {
        convert(in_file_name, out_file_name);
        return Ok(());
    }
    log!("Affinity Service started");
    log!("time interval: {}", interval_ms);
    let configs = read_config(&config_file_name).unwrap_or_else(|_| {
        log!("cannot read configs: {}", config_file_name);
        Vec::new()
    });
    let blacklist = if let Some(bf) = blacklist_file_name { read_list(bf).unwrap_or_default() } else { Vec::new() };
    if configs.is_empty() && !find_mode {
        log!("not even a single config, existing");
        return Ok(());
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
