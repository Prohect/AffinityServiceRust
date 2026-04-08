use crate::{
    error_codes::error_from_code_win32,
    log,
    logging::{FINDS_FAIL_SET, Operation, is_new_error, log_to_find},
};

use once_cell::sync::Lazy;
use std::{collections::HashMap, env, ffi::c_void, io, mem::size_of, process::Command, process::exit, sync::Mutex};
use windows::{
    Win32::{
        Foundation::{CloseHandle, GetLastError, HANDLE, HMODULE, LUID, NTSTATUS},
        Security::{
            AdjustTokenPrivileges, GetTokenInformation, LUID_AND_ATTRIBUTES, LookupPrivilegeValueW, SE_DEBUG_NAME, SE_INC_BASE_PRIORITY_NAME,
            SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_ELEVATION, TOKEN_PRIVILEGES, TOKEN_QUERY, TokenElevation,
        },
        System::{
            Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS},
            Kernel::PROCESSOR_NUMBER,
            ProcessStatus::{EnumProcessModulesEx, GetModuleBaseNameW, GetModuleInformation, LIST_MODULES_ALL, MODULEINFO},
            SystemInformation::{GetSystemCpuSetInformation, SYSTEM_CPU_SET_INFORMATION},
            Threading::{
                GetCurrentProcess, GetCurrentProcessId, GetProcessAffinityMask, GetThreadIdealProcessorEx, OpenProcess, OpenProcessToken,
                OpenThread, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SET_INFORMATION,
                PROCESS_SET_LIMITED_INFORMATION, PROCESS_TERMINATE, PROCESS_VM_READ, SetThreadIdealProcessorEx, THREAD_ACCESS_RIGHTS,
                THREAD_QUERY_INFORMATION, THREAD_QUERY_LIMITED_INFORMATION, THREAD_SET_INFORMATION, THREAD_SET_LIMITED_INFORMATION,
                TerminateProcess,
            },
        },
    },
    core::Error,
};

#[link(name = "ntdll")]
unsafe extern "system" {

    pub fn NtQueryInformationProcess(
        h_prc: HANDLE,
        process_information_class: u32,
        p_out: *mut c_void,
        out_length: u32,
        return_length: *mut u32,
    ) -> NTSTATUS;

    pub fn NtQueryInformationThread(
        thread_handle: HANDLE,
        thread_information_class: u32,
        thread_information: *mut c_void,
        thread_information_length: u32,
        return_length: *mut u32,
    ) -> NTSTATUS;

    pub fn NtSetInformationProcess(
        process_handle: HANDLE,
        process_information_class: u32,
        process_information: *const c_void,
        process_information_length: u32,
    ) -> NTSTATUS;

    pub fn NtSetTimerResolution(desired_resolution: u32, set_resolution: bool, p_current_resolution: *mut c_void) -> NTSTATUS;
}

#[derive(Clone, Copy)]
pub struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}

/// Make sure all handles are valid when newing this.
/// Automatically close any valid handle when dropped.
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.r_handle {
            unsafe {
                let _ = CloseHandle(handle);
            }
        }
        if let Some(handle) = self.w_handle {
            unsafe {
                let _ = CloseHandle(handle);
            }
        }
        unsafe {
            let _ = CloseHandle(self.r_limited_handle);
            let _ = CloseHandle(self.w_limited_handle);
        }
    }
}

/// Opens a process handle for the given PID, returning a `ProcessHandle` if successful.
/// All Some variants in the return value are confirmed valid handles.
/// Handles in returned result get automatically close when dropped.
///
/// internal error_code mapped to invalid handle for is_new_error check func:
/// 0 -> PROCESS_QUERY_LIMITED_INFORMATION
/// 1 -> PROCESS_SET_LIMITED_INFORMATION
/// 2 -> PROCESS_QUERY_INFORMATION
/// 3 -> PROCESS_SET_INFORMATIONW
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle> {
    let r_limited_request = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) };
    let Some(r_limited_handle) = r_limited_request.ok() else {
        let error_code = unsafe { GetLastError().0 };
        if is_new_error(
            pid,
            0,
            process_name,
            Operation::OpenProcess2processQueryLimitedInformation,
            error_code,
        ) {
            log_to_find(&format!(
                "get_process_handle: [{}] r_limited_handle {:>5}-{}",
                error_from_code_win32(error_code),
                pid,
                process_name
            ));
        }
        return None;
    };
    if r_limited_handle.is_invalid() {
        if is_new_error(pid, 0, process_name, Operation::InvalidHandle, 0) {
            log_to_find(&format!("get_process_handle: Invalid r_limited_handle {:>5}-{}", pid, process_name));
        }
        return None;
    }
    let w_limited_request = unsafe { OpenProcess(PROCESS_SET_LIMITED_INFORMATION, false, pid) };
    let Some(w_limited_handle) = w_limited_request.ok() else {
        unsafe {
            let _ = CloseHandle(r_limited_handle);
        };
        let error_code = unsafe { GetLastError().0 };
        if is_new_error(
            pid,
            0,
            process_name,
            Operation::OpenProcess2processSetLimitedInformation,
            error_code,
        ) {
            log_to_find(&format!(
                "get_process_handle: [{}] w_limited_handle {:>5}-{}",
                error_from_code_win32(error_code),
                pid,
                process_name
            ));
        }
        return None;
    };
    if w_limited_handle.is_invalid() {
        unsafe {
            let _ = CloseHandle(r_limited_handle);
        };
        if is_new_error(pid, 0, process_name, Operation::InvalidHandle, 1) {
            log_to_find(&format!("get_process_handle: Invalid w_limited_handle {:>5}-{}", pid, process_name));
        }
        return None;
    }
    let r_request = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) };
    let r_handle = if let Ok(r_handle) = r_request {
        if !r_handle.is_invalid() {
            Some(r_handle)
        } else {
            // if is_new_error(pid, 0, process_name, Operation::InvalidHandle, 2) {}
            None
        }
    } else {
        // let error_code = unsafe { GetLastError().0 };
        // if is_new_error(pid, 0, process_name, Operation::OpenProcess2processQueryInformation, error_code) {}
        None
    };
    let w_request = unsafe { OpenProcess(PROCESS_SET_INFORMATION, false, pid) };
    let w_handle = if let Ok(w_handle) = w_request {
        if !w_handle.is_invalid() {
            Some(w_handle)
        } else {
            // if is_new_error(pid, 0, process_name, Operation::InvalidHandle, 3) {}
            None
        }
    } else {
        // let error_code = unsafe { GetLastError().0 };
        // if is_new_error(pid, 0, process_name, Operation::OpenProcess2processSetInformation, error_code) {}
        None
    };

    Some(ProcessHandle {
        r_limited_handle,
        w_limited_handle,
        r_handle,
        w_handle,
    })
}

/// r_limited_handle is always valid (required when creating this struct).
#[derive(Debug)]
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}

impl Drop for ThreadHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.r_limited_handle);
        }
        if !self.r_handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.r_handle);
            }
        }
        if !self.w_limited_handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.w_limited_handle);
            }
        }
        if !self.w_handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.w_handle);
            }
        }
    }
}

/// Opens a thread handle for the given TID, returning a ThreadHandle if successful.
/// r_limited_handle is always valid. Other handles are tried but may be invalid if failed.
///
/// error_code mapping for is_new_error:
/// 0 -> THREAD_QUERY_LIMITED_INFORMATION (required, fail = return None)
/// 1 -> THREAD_QUERY_INFORMATION
/// 2 -> THREAD_SET_LIMITED_INFORMATION
/// 3 -> THREAD_SET_INFORMATION
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle> {
    // Open required r_limited_handle
    let r_limited_request = unsafe { OpenThread(THREAD_QUERY_LIMITED_INFORMATION, false, tid) };
    let Some(r_limited_handle) = r_limited_request.ok() else {
        let error_code = unsafe { GetLastError().0 };
        if is_new_error(pid, tid, process_name, Operation::OpenThread, error_code) {
            log_to_find(&format!(
                "get_thread_handle: [{}] r_limited_handle {:>5}-{:>5}-{}",
                error_from_code_win32(error_code),
                pid,
                tid,
                process_name
            ));
        }
        return None;
    };
    if r_limited_handle.is_invalid() {
        if is_new_error(pid, tid, process_name, Operation::InvalidHandle, 0) {
            log_to_find(&format!(
                "get_thread_handle: Invalid r_limited_handle {:>5}-{:>5}-{}",
                pid, tid, process_name
            ));
        }
        return None;
    }
    let r_handle = try_open_thread(pid, tid, process_name, THREAD_QUERY_INFORMATION, 1);
    let w_limited_handle = try_open_thread(tid, pid, process_name, THREAD_SET_LIMITED_INFORMATION, 2);
    let w_handle = try_open_thread(pid, tid, process_name, THREAD_SET_INFORMATION, 3);
    Some(ThreadHandle {
        r_limited_handle,
        r_handle,
        w_limited_handle,
        w_handle,
    })
}

/// the return value is an invalid handle on failure.
#[inline(always)]
#[allow(unused_variables)]
fn try_open_thread(pid: u32, tid: u32, process_name: &str, access: THREAD_ACCESS_RIGHTS, internal_op_code: u32) -> HANDLE {
    #[allow(dead_code)]
    fn error_detail(internal_op_code: &u32) -> String {
        match internal_op_code {
            1 => "r_handle".to_owned(),
            2 => "w_limited_handle".to_owned(),
            3 => "w_handle".to_owned(),
            _ => "UNKNOWN_OP_CODE".to_owned(),
        }
    }
    match unsafe { OpenThread(access, false, tid) } {
        Ok(handle) => {
            if handle.is_invalid() {
                // if is_new_error(pid, tid, process_name, Operation::InvalidHandle, internal_op_code) {}
                HANDLE::default() // Return invalid handle
            } else {
                handle
            }
        }
        Err(_) => {
            // let error_code = unsafe { GetLastError().0 };
            // if is_new_error(pid, tid, process_name, Operation::OpenThread, error_code) {}
            HANDLE::default() // Return invalid handle
        }
    }
}

unsafe fn extract_cpu_set_data(entry: &SYSTEM_CPU_SET_INFORMATION) -> CpuSetData {
    unsafe {
        CpuSetData {
            id: entry.Anonymous.CpuSet.Id,
            logical_processor_index: entry.Anonymous.CpuSet.LogicalProcessorIndex,
        }
    }
}

static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
    Mutex::new({
        let mut cpu_set_data: Vec<CpuSetData> = Vec::new();
        let mut required_size: u32 = 0;

        let current_process = unsafe { GetCurrentProcess() };

        let _ = unsafe { GetSystemCpuSetInformation(None, 0, &mut required_size, Some(current_process), Some(0)) };

        let mut buffer: Vec<u8> = vec![0u8; required_size as usize];

        let success = unsafe {
            GetSystemCpuSetInformation(
                Some(buffer.as_mut_ptr() as *mut SYSTEM_CPU_SET_INFORMATION),
                required_size,
                &mut required_size,
                Some(current_process),
                Some(0),
            )
            .as_bool()
        };

        if !success {
            log_to_find("GetSystemCpuSetInformation failed");
        } else {
            let mut offset = 0;
            while offset < required_size as usize {
                let entry = unsafe {
                    let entry_ptr = buffer.as_ptr().add(offset) as *const SYSTEM_CPU_SET_INFORMATION;
                    &*entry_ptr
                };

                let data = unsafe { extract_cpu_set_data(entry) };
                cpu_set_data.push(data);
                offset += entry.Size as usize;
            }
        }
        cpu_set_data
    })
});

pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> {
    &CPU_SET_INFORMATION
}

/// Converts logical CPU indices to CPU Set IDs.
///
/// Windows CPU Sets use opaque IDs that don't match logical processor numbers.
/// This maps user-friendly indices (0, 1, 2...) to the system's CPU Set IDs.
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> {
    if cpu_indices.is_empty() {
        return Vec::new();
    }
    let mut cpuids: Vec<u32> = Vec::new();
    let guard = get_cpu_set_information().lock().unwrap();
    for entry in guard.iter() {
        let logical_index = entry.logical_processor_index as u32;
        if cpu_indices.contains(&logical_index) {
            cpuids.push(entry.id);
        }
    }
    cpuids
}

#[allow(dead_code)]
pub fn cpusetids_from_mask(mask: usize) -> Vec<u32> {
    if mask == 0 {
        return Vec::new();
    }
    let mut cpuids: Vec<u32> = Vec::new();
    let guard = get_cpu_set_information().lock().unwrap();
    for entry in guard.iter() {
        let logical_index = entry.logical_processor_index;
        if logical_index < 64 && ((1usize << logical_index) & mask) != 0 {
            cpuids.push(entry.id);
        }
    }
    cpuids
}

/// Converts CPU Set IDs back to logical CPU indices.
///
/// Used when reading back CPU Set assignments.
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> {
    if cpuids.is_empty() {
        return Vec::new();
    }
    let mut indices: Vec<u32> = Vec::new();
    let guard = get_cpu_set_information().lock().unwrap();
    for entry in guard.iter() {
        if cpuids.contains(&entry.id) {
            indices.push(entry.logical_processor_index as u32);
        }
    }
    indices.sort();
    indices
}

#[allow(dead_code)]
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize {
    if cpuids.is_empty() {
        return 0;
    }
    let mut mask: usize = 0;
    let guard = get_cpu_set_information().lock().unwrap();
    for entry in guard.iter() {
        if cpuids.contains(&entry.id) {
            let idx = entry.logical_processor_index;
            if idx < 64 {
                mask |= 1 << idx;
            }
        }
    }
    mask
}

/// Filters CPU indices to only those allowed by the affinity mask.
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> {
    cpu_indices
        .iter()
        .filter(|&&idx| idx < 64 && ((1usize << idx) & affinity_mask) != 0)
        .copied()
        .collect()
}

pub fn is_running_as_admin() -> bool {
    let current_process = unsafe { GetCurrentProcess() };
    let mut token: HANDLE = HANDLE::default();

    let open_result = unsafe { OpenProcessToken(current_process, TOKEN_QUERY, &mut token) };

    if open_result.is_err() {
        return false;
    }

    let mut elevation: TOKEN_ELEVATION = TOKEN_ELEVATION::default();
    let mut return_length = 0u32;

    let info_result = unsafe {
        GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        )
    };

    let _ = unsafe { CloseHandle(token) };

    match info_result {
        Err(_) => false,
        Ok(_) => elevation.TokenIsElevated != 0,
    }
}

/// Restarts the process with administrator privileges via PowerShell.
///
/// Uses Start-Process -Verb RunAs to trigger UAC prompt.
/// Current process exits after spawning the elevated child.
/// The -skip_log_before_elevation flag attached prevents duplicate logging.
pub fn request_uac_elevation(console: bool) -> io::Result<()> {
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
            if console {
                log!(
                    "Warning: process is running as non-administrator without 'noUAC' flag with 'console' flag, the log after elevation will not be shown in currerent session."
                );
            }
            exit(0);
        }
        Err(e) => {
            log!("Failed to request UAC elevation: {}", e);
            Err(e)
        }
    }
}

pub fn enable_debug_privilege() {
    let mut token: HANDLE = HANDLE::default();

    let open_result = unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) };

    if open_result.is_err() {
        log!("enable_debug_privilege: self OpenProcessToken failed");
        return;
    }

    let mut l_uid = LUID::default();

    let lookup_result = unsafe { LookupPrivilegeValueW(None, SE_DEBUG_NAME, &mut l_uid) };

    if lookup_result.is_err() {
        log!("enable_debug_privilege: LookupPrivilegeValueW failed");

        let _ = unsafe { CloseHandle(token) };
        return;
    }

    let tp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: l_uid,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };

    let adjust_result = unsafe { AdjustTokenPrivileges(token, false, Some(&tp as *const _), 0, None, None) };

    if adjust_result.is_err() {
        log!("enable_debug_privilege: AdjustTokenPrivileges failed");
    } else {
        log!("enable_debug_privilege: AdjustTokenPrivileges succeeded");
    }

    let _ = unsafe { CloseHandle(token) };
}

pub fn enable_inc_base_priority_privilege() {
    let mut token: HANDLE = HANDLE::default();

    let open_result = unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) };

    if open_result.is_err() {
        log!("enable_inc_base_priority_privilege: self OpenProcessToken failed");
        return;
    }

    let mut l_uid = LUID::default();

    let lookup_result = unsafe { LookupPrivilegeValueW(None, SE_INC_BASE_PRIORITY_NAME, &mut l_uid) };

    if lookup_result.is_err() {
        log!("enable_inc_base_priority_privilege: LookupPrivilegeValueW failed");

        let _ = unsafe { CloseHandle(token) };
        return;
    }

    let tp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: l_uid,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };

    let adjust_result = unsafe { AdjustTokenPrivileges(token, false, Some(&tp as *const _), 0, None, None) };

    if adjust_result.is_err() {
        log!("enable_inc_base_priority_privilege: AdjustTokenPrivileges failed");
    } else {
        log!("enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded");
    }

    let _ = unsafe { CloseHandle(token) };
}

/// Checks if a process has default affinity (all system CPUs).
///
/// Used by -find mode to identify processes that haven't been configured yet.
/// Returns true if process affinity equals system affinity mask.
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool {
    let h_proc = match unsafe { OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, false, pid) } {
        Err(_) => {
            let code = unsafe { GetLastError() }.0;
            log_to_find(&format!(
                "is_affinity_unset: [OPEN][{}] {:>5}-{}",
                error_from_code_win32(code),
                pid,
                process_name
            ));
            if code == 5 {
                FINDS_FAIL_SET.lock().unwrap().insert(process_name.to_string());
            }
            return false;
        }
        Ok(h) => h,
    };

    if h_proc.is_invalid() {
        log_to_find(&format!("is_affinity_unset: [INVALID_HANDLE] {:>5}-{}", pid, process_name));
        return false;
    }

    let mut current_mask: usize = 0;
    let mut system_mask: usize = 0;

    let affinity_result = unsafe { GetProcessAffinityMask(h_proc, &mut current_mask, &mut system_mask) };

    let result = match affinity_result {
        Err(_) => {
            let code = unsafe { GetLastError() }.0;
            log_to_find(&format!(
                "is_affinity_unset: [AFFINITY_QUERY][{}] {:>5}-{}",
                error_from_code_win32(code),
                pid,
                process_name
            ));
            if code == 5 {
                FINDS_FAIL_SET.lock().unwrap().insert(process_name.to_string());
            }
            false
        }
        Ok(_) => current_mask == system_mask,
    };

    let _ = unsafe { CloseHandle(h_proc) };

    result
}

/// Gets the start address of a thread via NtQueryInformationThread.
///
/// This address is used to identify which module a thread belongs to
/// for module-based ideal processor assignment.
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize {
    let mut start_address: usize = 0;
    let mut return_len: u32 = 0;

    let status = unsafe {
        NtQueryInformationThread(
            thread_handle,
            9,
            &mut start_address as *mut _ as *mut c_void,
            size_of::<usize>() as u32,
            &mut return_len,
        )
    };

    if status.is_ok() { start_address } else { 0 }
}

pub fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, Error> {
    let ideal = PROCESSOR_NUMBER {
        Group: group,
        Number: number,
        Reserved: 0,
    };
    let mut previous = PROCESSOR_NUMBER::default();
    unsafe {
        SetThreadIdealProcessorEx(thread_handle, &ideal, Some(&mut previous))?;
    }
    Ok(previous)
}

pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error> {
    let mut ideal = PROCESSOR_NUMBER::default();
    unsafe {
        GetThreadIdealProcessorEx(thread_handle, &mut ideal)?;
    }
    Ok(ideal)
}

#[allow(clippy::type_complexity)]
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Resolves a memory address to a module name with offset.
///
/// Uses cached module enumeration to map addresses like 0x7FF12345 to "kernel32.dll+0x345".
/// Cache is populated on first call per process and cleared when process exits or next main loop iteration.
pub fn resolve_address_to_module(pid: u32, address: usize) -> String {
    if address == 0 {
        return "0x0".to_string();
    }

    let modules = {
        let mut cache = MODULE_CACHE.lock().unwrap();
        if let Some(cached_modules) = cache.get(&pid) {
            cached_modules.clone()
        } else {
            let new_modules = enumerate_process_modules(pid);
            cache.insert(pid, new_modules.clone());
            new_modules
        }
    };

    if let Some((module_base, _module_size, module_name)) = modules.iter().find(|(base, size, _)| address >= *base && address < base + size) {
        let offset = address - module_base;
        return format!("{}+0x{:X}", module_name, offset);
    }

    format!("0x{:X}", address)
}

pub fn drop_module_cache(pid: u32) {
    let mut cache = MODULE_CACHE.lock().unwrap();
    cache.remove(&pid);
}

/// Terminates any child processes spawned by this process.
///
/// Called on startup to clean up orphaned child console host processes,
/// particularly the elevated PowerShell instance from UAC elevation.
pub fn terminate_child_processes() {
    let current_pid = unsafe { GetCurrentProcessId() };

    let snapshot = unsafe {
        match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
            Ok(s) => s,
            Err(_) => return,
        }
    };

    let mut pe32 = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    unsafe {
        if Process32FirstW(snapshot, &mut pe32).is_ok() {
            loop {
                if pe32.th32ParentProcessID == current_pid {
                    let name_len = pe32.szExeFile.iter().position(|&c| c == 0).unwrap_or(0);
                    let child_name = String::from_utf16_lossy(&pe32.szExeFile[..name_len]);
                    let child_pid = pe32.th32ProcessID;

                    match OpenProcess(PROCESS_TERMINATE, false, child_pid) {
                        Ok(h) => {
                            match TerminateProcess(h, 0) {
                                Ok(_) => log!("terminate_child_processes: terminated '{}' (PID {})", child_name, child_pid),
                                Err(_) => log!(
                                    "terminate_child_processes: failed to terminate '{}' (PID {})",
                                    child_name,
                                    child_pid
                                ),
                            }
                            let _ = CloseHandle(h);
                        }
                        Err(_) => log!("terminate_child_processes: failed to open '{}' (PID {})", child_name, child_pid),
                    }
                }

                if Process32NextW(snapshot, &mut pe32).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);
    }
}

fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> {
    let mut result = Vec::new();

    let h_proc = match unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) } {
        Ok(h) if !h.is_invalid() => h,
        _ => return result,
    };

    let mut modules: [HMODULE; 1024] = [HMODULE::default(); 1024];
    let mut cb_needed: u32 = 0;

    let enum_result = unsafe {
        EnumProcessModulesEx(
            h_proc,
            modules.as_mut_ptr(),
            (modules.len() * size_of::<HMODULE>()) as u32,
            &mut cb_needed,
            LIST_MODULES_ALL,
        )
    };

    if enum_result.is_err() {
        let _ = unsafe { CloseHandle(h_proc) };
        return result;
    }

    let module_count = (cb_needed as usize) / size_of::<HMODULE>();

    for h_module in modules.iter().take(module_count) {
        let mut mod_info = MODULEINFO::default();

        if unsafe { GetModuleInformation(h_proc, *h_module, &mut mod_info, size_of::<MODULEINFO>() as u32) }.is_err() {
            continue;
        }

        let mut name_buf: [u16; 260] = [0; 260];

        let name_len = unsafe { GetModuleBaseNameW(h_proc, Some(*h_module), &mut name_buf) };

        if name_len == 0 {
            continue;
        }

        let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
        let base = mod_info.lpBaseOfDll as usize;
        let size = mod_info.SizeOfImage as usize;

        result.push((base, size, name));
    }

    let _ = unsafe { CloseHandle(h_proc) };

    result
}
