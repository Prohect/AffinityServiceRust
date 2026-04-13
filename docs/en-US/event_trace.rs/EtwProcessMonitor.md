# EtwProcessMonitor struct (event_trace.rs)

Manages an ETW (Event Tracing for Windows) real-time trace session that monitors process start and stop events from the `Microsoft-Windows-Kernel-Process` provider. `EtwProcessMonitor` encapsulates the full lifecycle of an ETW consumer session — starting the trace, enabling the kernel process provider, opening the trace for real-time consumption, processing events on a background thread, and tearing everything down on stop or drop. Process events are delivered to the caller through an `mpsc::Receiver<EtwProcessEvent>` channel returned by `start`.

## Syntax

```event_trace.rs
pub struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `control_handle` | `CONTROLTRACE_HANDLE` | Handle returned by `StartTraceW`, used to control (stop) the trace session via `ControlTraceW`. |
| `trace_handle` | `PROCESSTRACE_HANDLE` | Handle returned by `OpenTraceW`, used to close the trace consumer via `CloseTrace`, which unblocks the `ProcessTrace` call on the background thread. |
| `properties_buf` | `Vec<u8>` | Heap-allocated buffer that holds the `EVENT_TRACE_PROPERTIES` structure plus the trailing wide-string session name. Kept alive for the duration of the session because `ControlTraceW(EVENT_TRACE_CONTROL_STOP)` writes back into this buffer. |
| `process_thread` | `Option<thread::JoinHandle<()>>` | Join handle for the background thread running `ProcessTrace`. Set to `None` after the thread is joined during `stop`. |

## Methods

### start

```event_trace.rs
pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>
```

Starts a new ETW real-time trace session and returns the monitor handle paired with a receiver for [EtwProcessEvent](EtwProcessEvent.md) values.

**Startup sequence:**

1. Creates an `mpsc::channel` and installs the sender in the [ETW_SENDER](ETW_SENDER.md) global.
2. Allocates and initializes an `EVENT_TRACE_PROPERTIES` structure configured for real-time mode with QPC timestamps.
3. Calls [stop_existing_session](#stop_existing_session) to clean up any orphaned session from a prior crash.
4. Calls `StartTraceW` to create the named session `"AffinityServiceRust_EtwProcessMonitor"`.
5. Calls `EnableTraceEx2` to enable the `Microsoft-Windows-Kernel-Process` provider (GUID `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) with keyword `WINEVENT_KEYWORD_PROCESS` (`0x10`) at `TRACE_LEVEL_INFORMATION`.
6. Calls `OpenTraceW` with `PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD` and the `etw_event_callback` function pointer.
7. Sets [ETW_ACTIVE](ETW_ACTIVE.md) to `true`.
8. Spawns a background thread named `"etw-process-trace"` that calls `ProcessTrace` (blocking until the trace is closed).

If any step fails, all previously acquired resources are cleaned up and the global sender is cleared before returning `Err(String)` with a descriptive error message that includes the translated Win32 error code from [error_from_code_win32](../error_codes.rs/error_from_code_win32.md).

**Return value**

On success, returns `Ok((EtwProcessMonitor, Receiver<EtwProcessEvent>))`. The caller should poll or iterate the receiver to process incoming [EtwProcessEvent](EtwProcessEvent.md) values. On failure, returns `Err(String)` describing which ETW API call failed and the associated error code.

### stop

```event_trace.rs
pub fn stop(&mut self)
```

Stops the ETW trace session and releases all associated resources. This method is idempotent — calling it multiple times has no effect after the first successful stop.

**Shutdown sequence:**

1. Checks [ETW_ACTIVE](ETW_ACTIVE.md); returns immediately if already `false`.
2. Sets `ETW_ACTIVE` to `false`.
3. Calls `CloseTrace` on `trace_handle`, which unblocks the `ProcessTrace` call running on the background thread.
4. Calls `ControlTraceW` with `EVENT_TRACE_CONTROL_STOP` to terminate the trace session.
5. Joins the background processing thread (waits for it to exit).
6. Clears the [ETW_SENDER](ETW_SENDER.md) global to `None`, dropping the sender and closing the channel.

After `stop` returns, the receiver obtained from `start` will yield no further events and any pending `recv` calls will return `Err(RecvError)`.

### stop_existing_session

```event_trace.rs
fn stop_existing_session(wide_name: &[u16])
```

Attempts to stop any previously existing ETW session with the same name. This is a private helper called during `start` to clean up orphaned sessions that may remain after an abnormal termination (crash, kill, debugger detach). It silently ignores errors, since the session may not exist.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `wide_name` | `&[u16]` | The null-terminated UTF-16 session name to stop. Always `"AffinityServiceRust_EtwProcessMonitor"` encoded as UTF-16. |

## Drop

`EtwProcessMonitor` implements `Drop`, which delegates to `stop()`. This ensures that the ETW session is always cleaned up even if the caller forgets to explicitly stop it or if the monitor goes out of scope due to an error path.

```event_trace.rs
impl Drop for EtwProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
```

## Remarks

- **Session naming:** The session is registered under the name `"AffinityServiceRust_EtwProcessMonitor"`. Only one ETW session with a given name can exist at a time on the system. The `stop_existing_session` cleanup at startup handles the case where a previous instance of the service crashed without stopping its trace.
- **Callback architecture:** Because ETW delivers events via an `extern "system"` function pointer callback (`etw_event_callback`), there is no way to pass a closure or receiver directly. Instead, the module uses the [ETW_SENDER](ETW_SENDER.md) global `Lazy<Mutex<Option<Sender<EtwProcessEvent>>>>` to bridge from the callback into Rust's `mpsc` channel.
- **Thread model:** `ProcessTrace` is a blocking call that does not return until the trace is closed. It runs on a dedicated background thread (`"etw-process-trace"`) to avoid blocking the service's main loop.
- **Event filtering:** The ETW provider is enabled with keyword `0x10` (`WINEVENT_KEYWORD_PROCESS`), which restricts delivery to process lifecycle events only (start/stop). The callback further filters to Event ID 1 (start) and Event ID 2 (stop), discarding all other events.
- **Privileges:** Starting an ETW trace session typically requires administrative privileges. AffinityServiceRust already runs elevated, so this is not an additional requirement.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `event_trace` |
| Callers | [main](../main.rs/README.md) (creates and owns the monitor for the service loop lifetime) |
| Callees | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md), [ETW_SENDER](ETW_SENDER.md), [ETW_ACTIVE](ETW_ACTIVE.md) |
| Win32 API | `StartTraceW`, `EnableTraceEx2`, `OpenTraceW`, `ProcessTrace`, `CloseTrace`, `ControlTraceW` |
| ETW Provider | `Microsoft-Windows-Kernel-Process` (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) |
| Privileges | Administrator elevation (inherited from service context) |

## See Also

| Topic | Link |
|-------|------|
| Event payload struct | [EtwProcessEvent](EtwProcessEvent.md) |
| Global sender for ETW callback | [ETW_SENDER](ETW_SENDER.md) |
| Active flag for ETW session | [ETW_ACTIVE](ETW_ACTIVE.md) |
| Win32 error code translation | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| event_trace module overview | [event_trace module](README.md) |