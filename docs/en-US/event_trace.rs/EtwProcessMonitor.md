# EtwProcessMonitor struct (event_trace.rs)

Manages an ETW real-time trace session for process monitoring. Handles session creation, provider enablement, background event processing, and cleanup.

## Syntax

```rust
pub struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}
```

## Members

`control_handle`

Handle returned by `StartTraceW`, used to control (stop) the trace session.

`trace_handle`

Handle returned by `OpenTraceW`, used for `ProcessTrace` and `CloseTrace`.

`properties_buf`

Byte buffer containing the `EVENT_TRACE_PROPERTIES` structure and session name, used by both `StartTraceW` and `ControlTraceW`.

`process_thread`

Join handle for the background thread running `ProcessTrace`. Taken and joined during `stop()`.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **start** | `pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>` | Starts a new ETW session and returns the monitor and event receiver. |
| **stop** | `pub fn stop(&mut self)` | Stops the ETW session, joins the background thread, and cleans up. |

### start

Creates and configures an ETW real-time trace session:

1. Installs the global `ETW_SENDER` for the callback to use.
2. Prepares `EVENT_TRACE_PROPERTIES` with `EVENT_TRACE_REAL_TIME_MODE`.
3. Stops any existing session with the same name (cleanup from previous crash).
4. Calls `StartTraceW` to create the session.
5. Calls `EnableTraceEx2` with the `Microsoft-Windows-Kernel-Process` provider GUID and `WINEVENT_KEYWORD_PROCESS` keyword.
6. Calls `OpenTraceW` with `PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD`.
7. Spawns a background thread that calls `ProcessTrace` (blocks until session stops).

Returns `Err(String)` with a descriptive error message if any step fails, cleaning up partial state.

### stop

Stops the trace session and cleans up all resources:

1. Calls `CloseTrace` to unblock `ProcessTrace`.
2. Calls `ControlTraceW` with `EVENT_TRACE_CONTROL_STOP`.
3. Joins the background processing thread.
4. Clears the global `ETW_SENDER`.

Also invoked automatically via the `Drop` implementation.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/event_trace.rs |
| **Created by** | [`main`](../main.rs/main.md) |
| **Windows API** | `StartTraceW`, `EnableTraceEx2`, `OpenTraceW`, `ProcessTrace`, `CloseTrace`, `ControlTraceW` |

## See also

- [EtwProcessEvent](EtwProcessEvent.md)
- [ETW_SENDER](ETW_SENDER.md)
- [event_trace.rs module overview](README.md)