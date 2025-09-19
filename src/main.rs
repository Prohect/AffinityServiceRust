use chrono::{DateTime, Datelike, Local};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fmt::Arguments;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{
    env, fs,
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
    thread,
    time::Duration,
};
use windows::Win32::{
    Foundation::*,
    System::{Diagnostics::ToolHelp::*, Threading::*},
};
fn get_log_path(suffix: &str) -> PathBuf {
    let year = (*localtime().lock().unwrap()).year();
    let month = (*localtime().lock().unwrap()).month();
    let day = (*localtime().lock().unwrap()).day();
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
fn localtime() -> &'static Mutex<DateTime<Local>> {
    static LOCALTIME: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
    &LOCALTIME
}
fn use_console() -> &'static Mutex<bool> {
    static CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
    &CONSOLE
}
#[derive(Debug, Clone)]
struct ProcessConfig {
    name: String,
    priority: ProcessPriority,
    affinity_mask: usize,
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
        Self::TABLE.iter().find(|(_, name, _)| s.to_lowercase() == *name).map(|(v, _, _)| *v).unwrap_or(Self::None)
    }
}

pub fn log_process_find(process_name: &str) {
    let lower_name = process_name.to_lowercase();
    let time_prefix = (*localtime().lock().unwrap()).format("%H:%M:%S").to_string();
    let mut set = FINDS_SET.lock().unwrap();
    if set.insert(lower_name.clone()) {
        if *use_console().lock().unwrap() {
            println!("[{}]find {}", time_prefix, process_name);
        } else {
            let _ = writeln!(find_logger().lock().unwrap(), "[{}]find {}", time_prefix, process_name);
        }
    }
}

macro_rules! log {
    ($($arg:tt)*) => {
        log_message(format_args!($($arg)*));
    };
}
fn log_message(args: Arguments) {
    let time_prefix = (*localtime().lock().unwrap()).format("%H:%M:%S").to_string();
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
            configs.push(ProcessConfig {
                name,
                priority,
                affinity_mask: affinity,
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

fn set_priority_and_affinity(pid: u32, config: &ProcessConfig) {
    unsafe {
        if let Ok(h_proc) = OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, bool::from(FALSE), pid) {
            if !h_proc.is_invalid() {
                match config.priority.as_win_const() {
                    Some(priority_flag) => {
                        if SetPriorityClass(h_proc, priority_flag).is_ok() {
                            log!("{:>5}-{} -> {}", pid, config.name, config.priority.as_str());
                        };
                    }
                    None => (),
                }
                let mut current_mask: usize = 0;
                let mut system_mask: usize = 0;
                if GetProcessAffinityMask(h_proc, &mut current_mask, &mut system_mask).is_ok() {
                    if config.affinity_mask != 0 && current_mask != config.affinity_mask {
                        if SetProcessAffinityMask(h_proc, config.affinity_mask).is_ok() {
                            let msg = format!("{:>5}-{} -> {:#X}", pid, config.name, config.affinity_mask);
                            log!("{}", msg);
                        }
                    }
                }
                let _ = CloseHandle(h_proc);
            }
        }
    }
}

fn is_affinity_unset(pid: u32) -> bool {
    unsafe {
        if let Ok(h_proc) = OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, bool::from(FALSE), pid) {
            if !h_proc.is_invalid() {
                let mut current_mask: usize = 0;
                let mut system_mask: usize = 0;
                if GetProcessAffinityMask(h_proc, &mut current_mask, &mut system_mask).is_ok() {
                    return current_mask == system_mask;
                }
                let _ = CloseHandle(h_proc);
            }
        }
        false
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
                                        None => configs.push(ProcessConfig { name, priority, affinity_mask: 0 }),
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
    convert_mode: &mut bool,
    find_mode: &mut bool,
    config_file_name: &mut String,
    blacklist_file_name: &mut Option<String>,
    in_file_name: &mut Option<String>,
    out_file_name: &mut Option<String>,
) -> windows::core::Result<()> {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-help" | "--help" | "-?" | "/?" | "?" => {
                print_help();
                return Ok(());
            }
            "-console" => {
                *use_console().lock().unwrap() = true;
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
    println!("  -convert             convert process configs from -in <file>(from process lasso) to -out <file>");
    println!("  -find                find those whose affinity is same as system default which is all possible cores windows could use");
    println!("  -interval <ms>       set interval for checking again (5000 by default, minimal 16)");
    println!("  -config <file>       the config file u wanna use (config.ini by default)");
    println!("  -blacklist <file>    the blacklist for -find");
    println!("  -in <file>           input file for -convert");
    println!("  -out <file>          output file for -convert");
    println!();
}

fn main() -> windows::core::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut interval_ms = 5000;
    let mut convert_mode = false;
    let mut find_mode = false;
    let mut config_file_name = "config.ini".to_string();
    let mut blacklist_file_name: Option<String> = None;
    let mut in_file_name: Option<String> = None;
    let mut out_file_name: Option<String> = None;
    parse_args(
        &args,
        &mut interval_ms,
        &mut convert_mode,
        &mut find_mode,
        &mut config_file_name,
        &mut blacklist_file_name,
        &mut in_file_name,
        &mut out_file_name,
    )?;
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
    loop {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
            let mut pe32 = PROCESSENTRY32W::default();
            pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
            if Process32FirstW(snapshot, &mut pe32).is_ok() {
                'out_loop: loop {
                    let process_name = String::from_utf16_lossy(&pe32.szExeFile[..pe32.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)]).to_lowercase();
                    for config in &configs {
                        if process_name == config.name {
                            set_priority_and_affinity(pe32.th32ProcessID, &config);
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
                        if is_affinity_unset(pe32.th32ProcessID) {
                            log_process_find(process_name.as_str());
                        }
                    }
                    if !Process32NextW(snapshot, &mut pe32).is_ok() {
                        break;
                    }
                }
            }
            let _ = CloseHandle(snapshot);
        }
        thread::sleep(Duration::from_millis(interval_ms));
        *localtime().lock().unwrap() = Local::now();
    }
}
