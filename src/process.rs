use crate::collections::HashMap;
use ntapi::ntexapi::{NtQuerySystemInformation, SYSTEM_PROCESS_INFORMATION, SYSTEM_THREAD_INFORMATION, SystemProcessInformation};
use once_cell::sync::Lazy;
use std::slice;
use std::sync::Mutex;

/// DO NOT DIRECTLY ACCESS: Use struct `ProcessSnapshot` instead.
pub static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
/// DO NOT DIRECTLY ACCESS: Use struct `ProcessSnapshot` instead.
pub static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> = Lazy::new(|| Mutex::new(HashMap::default()));

pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}

impl<'a> Drop for ProcessSnapshot<'a> {
    fn drop(&mut self) {
        self.pid_to_process.clear();
        self.buffer.clear();
    }
}

impl<'a> ProcessSnapshot<'a> {
    /// Captures a snapshot of all processes and threads via NtQuerySystemInformation.
    ///
    /// Dynamically allocates buffer and retries if STATUS_INFO_LENGTH_MISMATCH.
    /// Parses SYSTEM_PROCESS_INFORMATION structures into ProcessEntry objects.
    pub fn take(buffer: &'a mut Vec<u8>, pid_to_process: &'a mut HashMap<u32, ProcessEntry>) -> Result<Self, i32> {
        let mut buf_len: usize = buffer.capacity();
        let mut return_len: u32 = 0;
        unsafe {
            loop {
                let status = NtQuerySystemInformation(
                    SystemProcessInformation,
                    buffer.as_mut_ptr() as *mut _,
                    buf_len as u32,
                    &mut return_len,
                );

                const STATUS_INFO_LENGTH_MISMATCH: i32 = -1073741820i32;
                if status == STATUS_INFO_LENGTH_MISMATCH {
                    buf_len = if return_len > 0 {
                        (((return_len / 8) + 1) * 8) as usize
                    } else {
                        buf_len * 2
                    };
                    *buffer = vec![0u8; buf_len];
                    continue;
                }
                if status < 0 {
                    return Err(status);
                }
                buffer.truncate(return_len as usize);
                break;
            }
            pid_to_process.clear();
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

    #[allow(dead_code)]
    pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry> {
        self.pid_to_process.values().filter(|entry| (**entry).get_name() == name).collect()
    }
}

#[derive(Clone)]
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}

// SAFETY: ProcessEntry is only accessed through Mutex, ensuring single-threaded access.
// The raw pointers inside SYSTEM_PROCESS_INFORMATION are only valid for the lifetime
// of the snapshot buffer and are never sent across threads independently.
unsafe impl Send for ProcessEntry {}

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
            threads: HashMap::default(),
            threads_base_ptr: threads_base_ptr as usize,
            name,
        }
    }

    /// Returns thread information map, lazily populating from raw pointer on first call.
    ///
    /// The raw thread array from SYSTEM_PROCESS_INFORMATION is parsed into a HashMap
    /// for efficient TID-based lookup. Cached on first access per process entry.
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
    #[allow(dead_code)]
    pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION> {
        self.get_threads().get(&tid)
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

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

    #[inline]
    pub fn pid(&self) -> u32 {
        self.process.UniqueProcessId as usize as u32
    }

    #[inline]
    pub fn thread_count(&self) -> u32 {
        self.process.NumberOfThreads
    }
}
