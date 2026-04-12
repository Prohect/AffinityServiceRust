use chrono::{DateTime, Datelike, Local};
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    fs::{File, OpenOptions, create_dir_all},
    io::{Write, stdout},
    path::PathBuf,
    sync::Mutex,
};

pub static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::log_message(format!($($arg)*).as_str())
    };
}

#[macro_export]
macro_rules! get_use_console {
    () => {
        $crate::logging::USE_CONSOLE.lock().unwrap()
    };
}
#[macro_export]
macro_rules! get_dust_bin_mod {
    () => {
        $crate::logging::DUST_BIN_MODE.lock().unwrap()
    };
}
#[macro_export]
macro_rules! get_local_time {
    () => {
        $crate::logging::LOCAL_TIME_BUFFER.lock().unwrap()
    };
}
#[macro_export]
macro_rules! get_logger {
    () => {
        $crate::logging::LOG_FILE.lock().unwrap()
    };
}
#[macro_export]
macro_rules! get_logger_find {
    () => {
        $crate::logging::FIND_LOG_FILE.lock().unwrap()
    };
}
#[macro_export]
macro_rules! get_fail_find_set {
    () => {
        $crate::logging::FINDS_FAIL_SET.lock().unwrap()
    };
}
#[macro_export]
macro_rules! get_pid_map_fail_entry_set {
    () => {
        $crate::logging::PID_MAP_FAIL_ENTRY_SET.lock().unwrap()
    };
}
pub static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
pub static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
pub static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
pub static LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
pub static FIND_LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
pub static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Operation {
    OpenProcess2processQueryLimitedInformation,
    OpenProcess2processSetLimitedInformation,
    OpenProcess2processQueryInformation,
    OpenProcess2processSetInformation,
    OpenThread,
    SetPriorityClass,
    GetProcessAffinityMask,
    SetProcessAffinityMask,
    GetProcessDefaultCpuSets,
    SetProcessDefaultCpuSets,
    QueryThreadCycleTime,
    SetThreadSelectedCpuSets,
    SetThreadPriority,
    NtQueryInformationProcess2ProcessInformationIOPriority,
    NtSetInformationProcess2ProcessInformationIOPriority,
    GetProcessInformation2ProcessMemoryPriority,
    SetProcessInformation2ProcessMemoryPriority,
    SetThreadIdealProcessorEx,
    GetThreadIdealProcessorEx,
    InvalidHandle,
}
#[derive(PartialEq, Eq, Hash)]
struct ApplyFailEntry {
    pid: u32,
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}

/// Tracks operation failures to avoid spamming logs.
///
/// Returns true if this is the first failure for this pid/process_name/operation/error_code combination.
/// `A`: The fail_entry_set(get from map) is expected that all of its entries have same process_name as the given process_name.
/// This func clears the fail entry set if `A` is not satisfied before inserting the new entry.
///
/// if there's no error_code from contextual codes, leave error_code as 0 or custom one if you need to differ them.
pub fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool {
    let entry = ApplyFailEntry {
        pid,
        tid,
        process_name: process_name.to_string(),
        operation,
        error_code,
    };
    let mut map = get_pid_map_fail_entry_set!();
    match map.get_mut(&pid) {
        Some(fail_entry_set) => {
            if fail_entry_set.iter_mut().any(|(fail_entry, alive)| {
                if fail_entry == &entry {
                    *alive = true;
                    true
                } else {
                    false
                }
            }) {
                false
            } else {
                if fail_entry_set
                    .keys()
                    .next()
                    .is_some_and(|fail_entry| fail_entry.process_name != entry.process_name)
                {
                    fail_entry_set.clear();
                }
                fail_entry_set.insert(entry, true);
                true
            }
        }
        _ => {
            map.insert(pid, HashMap::from([(entry, true)]));
            true
        }
    }
}

/// Removes stale entries from the apply failure tracking map.
///
/// Marks all entries as dead, then re-marks currently running processes as alive.
/// Dead entries are removed to prevent unbounded growth.
#[inline]
pub fn purge_fail_map(pids_and_names: &[(u32, String)]) {
    let mut map = get_pid_map_fail_entry_set!();
    for fail_entry_set in map.values_mut() {
        fail_entry_set.values_mut().for_each(|alive| *alive = false);
    }
    for (pid, name) in pids_and_names {
        if let Some(fail_entry_set) = map.get_mut(pid)
            && fail_entry_set.iter().any(|fail_entry| fail_entry.0.process_name == *name)
        {
            let _ = fail_entry_set.values_mut().next().is_some_and(|alive| {
                *alive = true;
                false
            });
        }
    }
    map.retain(|_, fail_entry_set| fail_entry_set.iter().any(|(_, alive)| *alive));
}

fn get_log_path(suffix: &str) -> PathBuf {
    let time = get_local_time!();
    let (year, month, day) = (time.year(), time.month(), time.day());
    drop(time);
    let log_dir = PathBuf::from("logs");
    if !log_dir.exists() {
        let _ = create_dir_all(&log_dir);
    }
    log_dir.join(format!("{:04}{:02}{:02}{}.log", year, month, day, suffix))
}

pub fn log_message(args: &str) {
    if *get_dust_bin_mod!() {
        return;
    }
    let time_prefix = get_local_time!().format("%H:%M:%S").to_string();
    if *get_use_console!() {
        let _ = writeln!(stdout(), "[{}]{}", time_prefix, args);
    } else {
        let _ = writeln!(get_logger!(), "[{}]{}", time_prefix, args);
    }
}

pub fn log_pure_message(args: &str) {
    if *get_use_console!() {
        let _ = writeln!(stdout(), "{}", args);
    } else {
        let _ = writeln!(get_logger!(), "{}", args);
    }
}

pub fn log_to_find(msg: &str) {
    let time_prefix = get_local_time!().format("%H:%M:%S").to_string();
    if *get_use_console!() {
        let _ = writeln!(stdout(), "[{}]{}", time_prefix, msg);
    } else {
        let _ = writeln!(get_logger!(), "[{}]{}", time_prefix, msg);
    }
}

/// Logs a discovered process from -find mode, deduplicated per day.
///
/// Uses FINDS_SET to ensure each process is logged only once per session,
/// preventing log spam from repeatedly discovered processes.
#[inline]
pub fn log_process_find(process_name: &str) {
    if FINDS_SET.lock().unwrap().insert(process_name.to_string()) {
        log_to_find(&format!("find {}", process_name));
    }
}
