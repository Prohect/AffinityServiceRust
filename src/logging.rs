//! Logging utilities for the affinity service.
//!
//! Provides file-based and console logging with automatic log file rotation by date.

use chrono::{DateTime, Datelike, Local};
use once_cell::sync::Lazy;
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};

/// Global timestamp buffer, updated each loop iteration.
pub static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));

/// Set of process names that have been logged as "found" (to avoid duplicates).
pub static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Set of process names that failed to be accessed (to avoid repeated error logs).
pub static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Whether to output to console instead of log files.
static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));

/// Main log file handle.
static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));

/// Find log file handle (for process discovery logs).
static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));

/// Returns a reference to the console mode flag.
pub fn use_console() -> &'static Mutex<bool> {
    &USE_CONSOLE
}

/// Returns a reference to the main log file.
pub fn logger() -> &'static Mutex<File> {
    &LOG_FILE
}

/// Returns a reference to the find log file.
pub fn find_logger() -> &'static Mutex<File> {
    &FIND_LOG_FILE
}

/// Generates the log file path based on current date.
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

/// Macro for convenient logging with format strings.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::log_message(format!($($arg)*).as_str());
    };
}

/// Logs a message with prefix added to either console or log file based on current mode.
pub fn log_message(args: &str) {
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        println!("[{}]{}", time_prefix, args);
    } else {
        let _ = writeln!(logger().lock().unwrap(), "[{}]{}", time_prefix, args);
    }
}

/// Logs a pure message to either console or log file based on current mode.
pub fn log_pure_message(args: &str) {
    if *use_console().lock().unwrap() {
        println!("{}", args);
    } else {
        let _ = writeln!(logger().lock().unwrap(), "{}", args);
    }
}

/// Logs a message to the find log file (for process discovery and errors).
pub fn log_to_find(msg: &str) {
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        println!("[{}]{}", time_prefix, msg);
    } else {
        let _ = writeln!(find_logger().lock().unwrap(), "[{}]{}", time_prefix, msg);
    }
}

/// Logs a process discovery (only once per process name).
pub fn log_process_find(process_name: &str) {
    if FINDS_SET.lock().unwrap().insert(process_name.to_string()) {
        log_to_find(&format!("find {}", process_name));
    }
}

/// Converts a Windows error code to a human-readable string.
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
