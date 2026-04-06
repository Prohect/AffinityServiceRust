use chrono::{DateTime, Datelike, Local};
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};

pub static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));

pub static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

struct ApplyFailEntry {
    proc_name: String,
    alive: bool,
}

static APPLY_FAIL_MAP: Lazy<Mutex<HashMap<u32, ApplyFailEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn apply_fail_insert_if_new(pid: u32, proc_name: &str) -> bool {
    let mut map = APPLY_FAIL_MAP.lock().unwrap();
    match map.get(&pid) {
        Some(entry) if entry.proc_name == proc_name => false,

        _ => {
            map.insert(
                pid,
                ApplyFailEntry {
                    proc_name: proc_name.to_string(),
                    alive: true,
                },
            );
            true
        }
    }
}

pub fn purge_apply_fail_map(pids_and_names: &[(u32, String)]) {
    let mut map = APPLY_FAIL_MAP.lock().unwrap();
    for entry in map.values_mut() {
        entry.alive = false;
    }
    for (pid, name) in pids_and_names {
        if let Some(entry) = map.get_mut(pid)
            && entry.proc_name == *name
        {
            entry.alive = true;
        }
    }
    map.retain(|_, entry| entry.alive);
}

static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));

pub static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));

static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));

static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));

pub fn use_console() -> &'static Mutex<bool> {
    &USE_CONSOLE
}

pub fn logger() -> &'static Mutex<File> {
    &LOG_FILE
}

pub fn find_logger() -> &'static Mutex<File> {
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

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::log_message(format!($($arg)*).as_str())
    };
}

pub fn log_message(args: &str) {
    if *DUST_BIN_MODE.lock().unwrap() {
        return;
    }
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        let _ = writeln!(std::io::stdout(), "[{}]{}", time_prefix, args);
    } else {
        let _ = writeln!(logger().lock().unwrap(), "[{}]{}", time_prefix, args);
    }
}

pub fn log_pure_message(args: &str) {
    if *use_console().lock().unwrap() {
        let _ = writeln!(std::io::stdout(), "{}", args);
    } else {
        let _ = writeln!(logger().lock().unwrap(), "{}", args);
    }
}

pub fn log_to_find(msg: &str) {
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        let _ = writeln!(std::io::stdout(), "[{}]{}", time_prefix, msg);
    } else {
        let _ = writeln!(find_logger().lock().unwrap(), "[{}]{}", time_prefix, msg);
    }
}

pub fn log_process_find(process_name: &str) {
    if FINDS_SET.lock().unwrap().insert(process_name.to_string()) {
        log_to_find(&format!("find {}", process_name));
    }
}

pub fn error_from_code(code: u32) -> String {
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
