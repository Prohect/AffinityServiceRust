//! Logging utilities for the affinity service.
//!
//! Provides file-based and console logging with automatic log file rotation by date.

use chrono::{DateTime, Datelike, Local};
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
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

/// Entry in `APPLY_FAIL_MAP` for a specific PID that returned ACCESS_DENIED from
/// `apply_config` OpenProcess.  The `alive` flag is used during each snapshot reconcile
/// to evict PIDs that have exited or been reused for a different process name.
struct ApplyFailEntry {
    proc_name: String,
    alive: bool,
}

/// Map of PID → `ApplyFailEntry` for processes that returned ACCESS_DENIED from
/// `apply_config` OpenProcess.  Keyed by PID so multiple instances of the same
/// executable (e.g. `svchost.exe`) are tracked independently: one inaccessible
/// instance does not suppress errors for other accessible-but-currently-failing ones.
static APPLY_FAIL_MAP: Lazy<Mutex<HashMap<u32, ApplyFailEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Checks whether `(pid, proc_name)` is already in the fail map; if not, inserts it.
/// Returns `true` when the entry is new — the caller should emit the error exactly once.
pub fn apply_fail_insert_if_new(pid: u32, proc_name: &str) -> bool {
    let mut map = APPLY_FAIL_MAP.lock().unwrap();
    match map.get(&pid) {
        // Same PID, same name → already logged, suppress.
        Some(entry) if entry.proc_name == proc_name => false,
        // New PID, or PID reused for a different name → insert and allow logging.
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

/// Reconciles the fail map against the current process snapshot.
///
/// Algorithm:
/// 1. Mark every stored entry as potentially dead (`alive = false`).
/// 2. Revive entries whose PID is still present in the snapshot **with the same name**.
/// 3. Drop all entries still marked dead (process exited or PID was reused).
///
/// Call this once per loop iteration right after `pids_and_names` is built from the
/// fresh `ProcessSnapshot`, so stale entries never accumulate.
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

/// Whether to output to console instead of log files.
static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));

/// Whether to eat all logs ovO.
pub static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));

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
        $crate::logging::log_message(format!($($arg)*).as_str())
    };
}

/// Logs a message with prefix added to either console or log file based on current mode.
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

/// Logs a pure message to either console or log file based on current mode.
pub fn log_pure_message(args: &str) {
    if *use_console().lock().unwrap() {
        let _ = writeln!(std::io::stdout(), "{}", args);
    } else {
        let _ = writeln!(logger().lock().unwrap(), "{}", args);
    }
}

/// Logs a message to the find log file (for process discovery and errors).
pub fn log_to_find(msg: &str) {
    let time_prefix = LOCALTIME_BUFFER.lock().unwrap().format("%H:%M:%S").to_string();
    if *use_console().lock().unwrap() {
        let _ = writeln!(std::io::stdout(), "[{}]{}", time_prefix, msg);
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
