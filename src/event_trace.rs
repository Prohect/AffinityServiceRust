//! Minimal ETW (Event Tracing for Windows) consumer for real-time process start/stop monitoring.
//!
//! Uses the Microsoft-Windows-Kernel-Process provider to receive notifications
//! when processes are created or terminated, enabling reactive rule application.

use crate::error_codes::error_from_code_win32;

use once_cell::sync::Lazy;
use std::{
    mem, ptr,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread,
};
use windows::{
    Win32::{Foundation::ERROR_SUCCESS, System::Diagnostics::Etw::*},
    core::{GUID, PCWSTR, PWSTR},
};

/// Microsoft-Windows-Kernel-Process provider GUID: {22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}
const KERNEL_PROCESS_GUID: GUID = GUID::from_u128(0x22fb2cd6_0e7b_422b_a0c7_2fad1fd0e716);

/// Keyword for process events from Microsoft-Windows-Kernel-Process
const WINEVENT_KEYWORD_PROCESS: u64 = 0x10;

/// ETW session name used by this monitor
const SESSION_NAME: &str = "AffinityServiceRust_EtwProcessMonitor";

/// Global sender for the ETW callback to use.
/// The ETW callback is an `extern "system"` fn pointer, so we need a global to send events.
static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));

/// Flag indicating whether the ETW session is active
static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);

/// The ETW event record callback invoked by the OS for each event.
///
/// Extracts process ID from the event's UserData and sends it through the global channel.
/// Event ID 1 = ProcessStart, Event ID 2 = ProcessStop.
unsafe extern "system" fn etw_event_callback(event_record: *mut EVENT_RECORD) {
    unsafe {
        if event_record.is_null() {
            return;
        }
        let record = &*event_record;
        let event_id = record.EventHeader.EventDescriptor.Id;

        // We only care about ProcessStart (1) and ProcessStop (2)
        if event_id != 1 && event_id != 2 {
            return;
        }

        // Extract ProcessID from UserData (first 4 bytes)
        if record.UserDataLength < 4 || record.UserData.is_null() {
            return;
        }

        let pid = *(record.UserData as *const u32);
        let is_start = event_id == 1;

        if let Ok(guard) = ETW_SENDER.lock()
            && let Some(ref sender) = *guard
        {
            let _ = sender.send(EtwProcessEvent { pid, is_start });
        }
    }
}

/// A process event received from ETW.
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}

/// Manages an ETW real-time trace session for process monitoring.
pub struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}

impl EtwProcessMonitor {
    /// Start a new ETW trace session monitoring process start/stop events.
    ///
    /// Returns the monitor handle and a receiver for process events.
    /// The trace runs on a background thread until `stop()` is called.
    pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String> {
        let (sender, receiver) = mpsc::channel();

        // Install the global sender
        {
            let mut global_sender = ETW_SENDER.lock().map_err(|e| format!("Failed to lock ETW_SENDER: {}", e))?;
            *global_sender = Some(sender);
        }

        // --- Step 1: Prepare EVENT_TRACE_PROPERTIES ---
        let wide_name: Vec<u16> = SESSION_NAME.encode_utf16().chain(std::iter::once(0)).collect();
        let wide_name_bytes = wide_name.len() * mem::size_of::<u16>();
        let props_size = mem::size_of::<EVENT_TRACE_PROPERTIES>() + wide_name_bytes;
        let mut props_buf = vec![0u8; props_size];

        unsafe {
            let props = &mut *(props_buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES);
            props.Wnode.BufferSize = props_size as u32;
            props.Wnode.Flags = WNODE_FLAG_TRACED_GUID;
            props.Wnode.ClientContext = 1; // QPC timestamps
            props.LogFileMode = EVENT_TRACE_REAL_TIME_MODE;
            props.LoggerNameOffset = mem::size_of::<EVENT_TRACE_PROPERTIES>() as u32;

            // Copy the session name after the struct
            let name_dest = props_buf.as_mut_ptr().add(mem::size_of::<EVENT_TRACE_PROPERTIES>()) as *mut u16;
            ptr::copy_nonoverlapping(wide_name.as_ptr(), name_dest, wide_name.len());
        }

        // --- Step 2: Stop any existing session with the same name ---
        Self::stop_existing_session(&wide_name);

        // --- Step 3: Start the trace session ---
        let mut control_handle = CONTROLTRACE_HANDLE::default();
        let start_err = unsafe {
            StartTraceW(
                &mut control_handle,
                PCWSTR::from_raw(wide_name.as_ptr()),
                props_buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES,
            )
        };
        if start_err != ERROR_SUCCESS {
            let mut global_sender = ETW_SENDER.lock().unwrap();
            *global_sender = None;
            return Err(format!("StartTraceW failed: {}", error_from_code_win32(start_err.0)));
        }

        // --- Step 4: Enable the Microsoft-Windows-Kernel-Process provider ---
        let enable_err = unsafe {
            EnableTraceEx2(
                control_handle,
                &KERNEL_PROCESS_GUID,
                EVENT_CONTROL_CODE_ENABLE_PROVIDER.0,
                TRACE_LEVEL_INFORMATION as u8,
                WINEVENT_KEYWORD_PROCESS,
                0,
                0,
                None,
            )
        };
        if enable_err != ERROR_SUCCESS {
            // Stop the session and clean up
            unsafe {
                let _ = ControlTraceW(
                    control_handle,
                    PCWSTR::null(),
                    props_buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES,
                    EVENT_TRACE_CONTROL_STOP,
                );
            }
            let mut global_sender = ETW_SENDER.lock().unwrap();
            *global_sender = None;
            return Err(format!("EnableTraceEx2 failed: {}", error_from_code_win32(enable_err.0)));
        }

        // --- Step 5: Open the trace for consuming ---
        let mut log_file = EVENT_TRACE_LOGFILEW {
            LoggerName: PWSTR(wide_name.as_ptr() as *mut u16),
            ..Default::default()
        };
        log_file.Anonymous1.ProcessTraceMode = PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD;
        log_file.Anonymous2.EventRecordCallback = Some(etw_event_callback);

        let trace_handle = unsafe { OpenTraceW(&mut log_file as *mut EVENT_TRACE_LOGFILEW) };
        if trace_handle.Value == u64::MAX {
            // INVALID_PROCESSTRACE_HANDLE
            unsafe {
                let _ = ControlTraceW(
                    control_handle,
                    PCWSTR::null(),
                    props_buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES,
                    EVENT_TRACE_CONTROL_STOP,
                );
            }
            let mut global_sender = ETW_SENDER.lock().unwrap();
            *global_sender = None;
            return Err("OpenTraceW failed: returned INVALID_PROCESSTRACE_HANDLE".to_string());
        }

        // --- Step 6: Spawn background thread to process events ---
        ETW_ACTIVE.store(true, Ordering::SeqCst);
        let process_thread = thread::Builder::new()
            .name("etw-process-trace".to_string())
            .spawn(move || unsafe {
                let _ = ProcessTrace(&[trace_handle], None, None);
            })
            .map_err(|e| format!("Failed to spawn ETW processing thread: {}", e))?;

        Ok((
            EtwProcessMonitor {
                control_handle,
                trace_handle,
                properties_buf: props_buf,
                process_thread: Some(process_thread),
            },
            receiver,
        ))
    }

    /// Stop the ETW trace session and clean up resources.
    pub fn stop(&mut self) {
        if !ETW_ACTIVE.load(Ordering::SeqCst) {
            return;
        }
        ETW_ACTIVE.store(false, Ordering::SeqCst);

        // Close the trace (unblocks ProcessTrace)
        unsafe {
            let _ = CloseTrace(self.trace_handle);
        }

        // Stop the trace session
        unsafe {
            let _ = ControlTraceW(
                self.control_handle,
                PCWSTR::null(),
                self.properties_buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES,
                EVENT_TRACE_CONTROL_STOP,
            );
        }

        // Wait for the processing thread to finish
        if let Some(handle) = self.process_thread.take() {
            let _ = handle.join();
        }

        // Clear the global sender
        if let Ok(mut guard) = ETW_SENDER.lock() {
            *guard = None;
        }
    }

    /// Attempt to stop any existing session with the same name.
    fn stop_existing_session(wide_name: &[u16]) {
        let props_size = mem::size_of::<EVENT_TRACE_PROPERTIES>() + std::mem::size_of_val(wide_name);
        let mut props_buf = vec![0u8; props_size];
        unsafe {
            let props = &mut *(props_buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES);
            props.Wnode.BufferSize = props_size as u32;
            props.LoggerNameOffset = mem::size_of::<EVENT_TRACE_PROPERTIES>() as u32;
            let _ = ControlTraceW(
                CONTROLTRACE_HANDLE::default(),
                PCWSTR::from_raw(wide_name.as_ptr()),
                props_buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES,
                EVENT_TRACE_CONTROL_STOP,
            );
        }
    }
}

impl Drop for EtwProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
