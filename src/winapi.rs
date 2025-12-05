//! Windows API helpers and privilege management.
//!
//! Provides wrappers for Windows system calls, privilege elevation,
//! and CPU set information retrieval.

use crate::log;
use crate::logging::{FAIL_SET, error_from_code, log_to_find};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::io;
use std::mem::size_of;
use std::process::Command;
use std::sync::Mutex;
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, LUID, NTSTATUS};
use windows::Win32::Security::{
    AdjustTokenPrivileges, GetTokenInformation, LookupPrivilegeValueW, SE_DEBUG_NAME, SE_INC_BASE_PRIORITY_NAME, TOKEN_ADJUST_PRIVILEGES, TOKEN_ELEVATION,
    TOKEN_PRIVILEGES, TOKEN_QUERY, TokenElevation,
};
use windows::Win32::System::ProcessStatus::{EnumProcessModulesEx, GetModuleBaseNameW, GetModuleInformation, LIST_MODULES_ALL, MODULEINFO};
use windows::Win32::System::SystemInformation::{GetSystemCpuSetInformation, SYSTEM_CPU_SET_INFORMATION};
use windows::Win32::System::Threading::{
    GetCurrentProcess, GetProcessAffinityMask, OpenProcess, OpenProcessToken, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION, PROCESS_VM_READ,
};

// Undocumented NTDLL imports for IO priority and timer resolution control.
// These APIs are stable but not officially documented by Microsoft.
#[link(name = "ntdll")]
unsafe extern "system" {
    /// Queries process information including IO priority (class 33).
    pub fn NtQueryInformationProcess(h_prc: HANDLE, process_information_class: u32, p_out: *mut std::ffi::c_void, out_length: u32, return_length: *mut u32) -> NTSTATUS;

    /// Sets process information including IO priority (class 33).
    pub fn NtSetInformationProcess(
        process_handle: HANDLE,
        process_information_class: u32,
        process_information: *const std::ffi::c_void,
        process_information_length: u32,
    ) -> NTSTATUS;

    /// Sets system timer resolution. Resolution is in 100ns units (e.g., 5000 = 0.5ms).
    /// Lower values improve scheduling latency but increase power consumption.
    pub fn NtSetTimerResolution(desired_resolution: u32, set_resolution: bool, p_current_resolution: *mut std::ffi::c_void) -> NTSTATUS;
}

/// Cached CPU set information for the system.
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<SYSTEM_CPU_SET_INFORMATION>>> = Lazy::new(|| {
    Mutex::new({
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
                false => log_to_find("GetSystemCpuSetInformation failed"),
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
    })
});

/// Returns a reference to the cached CPU set information.
pub fn get_cpu_set_information() -> &'static Mutex<Vec<SYSTEM_CPU_SET_INFORMATION>> {
    &CPU_SET_INFORMATION
}

/// Converts a vector of CPU indices to a vector of CPU Set IDs.
///
/// CPU Set IDs are opaque identifiers used by `SetThreadSelectedCpuSets`.
/// This maps logical processor indices to their corresponding CPU Set IDs.
///
/// # Arguments
/// * `cpu_indices` - A slice of logical processor indices (0-based across all groups)
///
/// # Returns
/// A vector of CPU Set IDs that can be used with CPU Set APIs.
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> {
    if cpu_indices.is_empty() {
        return Vec::new();
    }
    let mut cpuids: Vec<u32> = Vec::new();
    unsafe {
        get_cpu_set_information().lock().unwrap().iter().for_each(|entry| {
            let logical_index = entry.Anonymous.CpuSet.LogicalProcessorIndex as u32;
            if cpu_indices.contains(&logical_index) {
                cpuids.push(entry.Anonymous.CpuSet.Id);
            }
        });
    }
    cpuids
}

/// Converts a CPU bitmask to a vector of CPU Set IDs.
///
/// This is a legacy function for backward compatibility with ≤64 core systems.
/// For systems with >64 cores, use `cpusetids_from_indices` instead.
///
/// # Arguments
/// * `mask` - A bitmask where bit N corresponds to logical processor N
///
/// # Returns
/// A vector of CPU Set IDs for the processors indicated by the mask.
#[allow(dead_code)]
pub fn cpusetids_from_mask(mask: usize) -> Vec<u32> {
    if mask == 0 {
        return Vec::new();
    }
    let mut cpuids: Vec<u32> = Vec::new();
    unsafe {
        get_cpu_set_information().lock().unwrap().iter().for_each(|entry| {
            let logical_index = entry.Anonymous.CpuSet.LogicalProcessorIndex;
            if logical_index < 64 && ((1usize << logical_index) & mask) != 0 {
                cpuids.push(entry.Anonymous.CpuSet.Id);
            }
        });
    }
    cpuids
}

/// Converts CPU Set IDs to a vector of CPU indices.
///
/// # Arguments
/// * `cpuids` - A slice of CPU Set IDs
///
/// # Returns
/// A vector of logical processor indices (0-based across all groups).
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> {
    if cpuids.is_empty() {
        return Vec::new();
    }
    let mut indices: Vec<u32> = Vec::new();
    unsafe {
        for entry in get_cpu_set_information().lock().unwrap().iter() {
            if cpuids.contains(&entry.Anonymous.CpuSet.Id) {
                indices.push(entry.Anonymous.CpuSet.LogicalProcessorIndex as u32);
            }
        }
    }
    indices.sort();
    indices
}

/// Converts CPU Set IDs back to a bitmask (for ≤64 core systems).
///
/// # Arguments
/// * `cpuids` - A slice of CPU Set IDs
///
/// # Returns
/// A bitmask where bit N is set if logical processor N is in the CPU set.
/// Note: Only processors with index < 64 are included in the mask.
#[allow(dead_code)]
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize {
    if cpuids.is_empty() {
        return 0;
    }
    let mut mask: usize = 0;
    unsafe {
        for entry in get_cpu_set_information().lock().unwrap().iter() {
            if cpuids.contains(&entry.Anonymous.CpuSet.Id) {
                let idx = entry.Anonymous.CpuSet.LogicalProcessorIndex;
                if idx < 64 {
                    mask |= 1 << idx;
                }
            }
        }
    }
    mask
}

/// Filters CPU indices to only include those present in the given affinity mask.
///
/// This is used to intersect prime CPU indices with the current process affinity.
///
/// # Arguments
/// * `cpu_indices` - The CPU indices to filter
/// * `affinity_mask` - The current process affinity mask (from GetProcessAffinityMask)
///
/// # Returns
/// A vector of CPU indices that are both in the input and allowed by the affinity mask.
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> {
    cpu_indices
        .iter()
        .filter(|&&idx| idx < 64 && ((1usize << idx) & affinity_mask) != 0)
        .copied()
        .collect()
}

/// Checks if the current process is running with administrator privileges.
pub fn is_running_as_admin() -> bool {
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

/// Requests UAC elevation by relaunching the process with admin privileges.
pub fn request_uac_elevation() -> io::Result<()> {
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

/// Enables SeDebugPrivilege for the current process.
/// SeDebugPrivilege is required to open handles to system processes and processes owned by other users.
pub fn enable_debug_privilege() {
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

/// Enables SeIncreaseBasePriorityPrivilege for the current process.
/// Required to set high IO priority.
pub fn enable_inc_base_priority_privilege() {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        match OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) {
            Err(_) => {
                log!("enable_inc_base_priority_privilege: self OpenProcessToken failed");
            }
            Ok(_) => {
                let mut l_uid = LUID::default();
                match LookupPrivilegeValueW(None, SE_INC_BASE_PRIORITY_NAME, &mut l_uid) {
                    Err(_) => {
                        log!("enable_inc_base_priority_privilege: LookupPrivilegeValueW failed");
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
                                log!("enable_inc_base_priority_privilege: AdjustTokenPrivileges failed");
                            }
                            Ok(_) => {
                                log!("enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded");
                            }
                        }
                    }
                }
                let _ = CloseHandle(token);
            }
        }
    }
}

/// Checks if a process has default (unmodified) CPU affinity.
/// Returns true if current_mask == system_mask, meaning "use all available cores".
/// Used by find mode to discover processes without custom affinity settings.
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool {
    unsafe {
        let mut result = false;
        match OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION, false, pid) {
            Err(_) => {
                let code = GetLastError().0;
                log_to_find(&format!("is_affinity_unset: [OPEN][{}] {:>5}-{}", error_from_code(code), pid, process_name));
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
                            log_to_find(&format!("is_affinity_unset: [AFFINITY_QUERY][{}] {:>5}-{}", error_from_code(code), pid, process_name));
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

/// Cached module information for processes.
/// Key: PID, Value: Vec of (base_address, size, module_name)
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Resolves a memory address to a module name for a given process.
/// Returns the module name + offset (e.g., "ntdll.dll+0x1C320") or just the hex address if not found.
///
/// # Arguments
/// * `pid` - Process ID
/// * `address` - Memory address to resolve (e.g., thread start address)
///
/// # Returns
/// A string like "ntdll.dll+0x1C320" or "0x00007FFC36BDC320" if module not found.
pub fn resolve_address_to_module(pid: u32, address: usize) -> String {
    if address == 0 {
        return "0x0".to_string();
    }

    // Check cache first
    {
        let cache = MODULE_CACHE.lock().unwrap();
        if let Some(modules) = cache.get(&pid) {
            for (base, size, name) in modules {
                if address >= *base && address < base + size {
                    let offset = address - base;
                    return format!("{}+0x{:X}", name, offset);
                }
            }
            // Already cached but address not in any known module
            return format!("0x{:X}", address);
        }
    }

    // Not cached, enumerate modules
    let modules = enumerate_process_modules(pid);

    // Store in cache
    {
        let mut cache = MODULE_CACHE.lock().unwrap();
        cache.insert(pid, modules.clone());
    }

    // Search for the address
    for (base, size, name) in &modules {
        if address >= *base && address < base + size {
            let offset = address - base;
            return format!("{}+0x{:X}", name, offset);
        }
    }

    format!("0x{:X}", address)
}

/// Clears the module cache for a specific process (call when process exits).
#[allow(dead_code)]
pub fn clear_module_cache(pid: u32) {
    MODULE_CACHE.lock().unwrap().remove(&pid);
}

/// Enumerates all modules in a process and returns their base addresses, sizes, and names.
fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> {
    let mut result = Vec::new();

    unsafe {
        // Need PROCESS_QUERY_INFORMATION | PROCESS_VM_READ for module enumeration
        let h_proc = match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
            Ok(h) if !h.is_invalid() => h,
            _ => return result,
        };

        // First call to get the number of modules
        let mut modules: [windows::Win32::Foundation::HMODULE; 1024] = [windows::Win32::Foundation::HMODULE::default(); 1024];
        let mut cb_needed: u32 = 0;

        if EnumProcessModulesEx(
            h_proc,
            modules.as_mut_ptr(),
            (modules.len() * size_of::<windows::Win32::Foundation::HMODULE>()) as u32,
            &mut cb_needed,
            LIST_MODULES_ALL,
        )
        .is_err()
        {
            let _ = CloseHandle(h_proc);
            return result;
        }

        let module_count = (cb_needed as usize) / size_of::<windows::Win32::Foundation::HMODULE>();

        for i in 0..module_count.min(modules.len()) {
            let h_module = modules[i];

            // Get module info (base address and size)
            let mut mod_info = MODULEINFO::default();
            if GetModuleInformation(h_proc, h_module, &mut mod_info, size_of::<MODULEINFO>() as u32).is_err() {
                continue;
            }

            // Get module name
            let mut name_buf: [u16; 260] = [0; 260];
            let name_len = GetModuleBaseNameW(h_proc, Some(h_module), &mut name_buf);
            if name_len == 0 {
                continue;
            }

            let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
            let base = mod_info.lpBaseOfDll as usize;
            let size = mod_info.SizeOfImage as usize;

            result.push((base, size, name));
        }

        let _ = CloseHandle(h_proc);
    }

    result
}
