//! Process snapshot and entry management.
//!
//! Provides efficient access to system process and thread information
//! via NtQuerySystemInformation.

use ntapi::ntexapi::{NtQuerySystemInformation, SYSTEM_PROCESS_INFORMATION, SYSTEM_THREAD_INFORMATION, SystemProcessInformation};
use std::collections::HashMap;
use std::slice;

/// A snapshot of all running processes obtained via `NtQuerySystemInformation`.
///
/// This provides a consistent view of the system's process and thread state
/// at a single point in time. The snapshot is more efficient than repeatedly
/// calling process enumeration APIs.
///
/// # Safety
///
/// The internal buffer contains raw system data that is parsed in unsafe code.
/// The buffer must remain valid for the lifetime of this struct.
pub struct ProcessSnapshot {
    /// Buffer used to store the snapshot of processes, parsed in unsafe code.
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
    /// Takes a snapshot of all running processes.
    ///
    /// # Returns
    /// - `Ok(ProcessSnapshot)` on success
    /// - `Err(i32)` with NTSTATUS code on failure
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
    #[allow(dead_code)]
    pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry> {
        self.pid_to_process.values().filter(|entry| (**entry).get_name() == name).collect()
    }
}

/// A single process entry from a process snapshot.
///
/// Contains information about a process including its threads,
/// parsed from the raw `SYSTEM_PROCESS_INFORMATION` structure.
///
/// Use `get_threads()` to get all threads for this process, parsing them lazily if needed.
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
    /// Creates a new ProcessEntry from raw system information.
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

    /// Gets all threads for this process, lazily parsing them if needed.
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

    /// Gets a specific thread by TID.
    #[inline]
    #[allow(dead_code)]
    pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION> {
        self.get_threads().get(&tid)
    }

    /// Gets the process name (lowercase).
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets the process name preserving original case.
    #[inline]
    #[allow(dead_code)]
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

    /// Gets the process ID.
    #[inline]
    pub fn pid(&self) -> u32 {
        self.process.UniqueProcessId as usize as u32
    }

    /// Gets the number of threads in this process.
    #[inline]
    pub fn thread_count(&self) -> u32 {
        self.process.NumberOfThreads
    }
}
