# EtwProcessMonitor struct (event_trace.rs)

Manages an ETW (Event Tracing for Windows) real-time trace session for process start/stop monitoring. `EtwProcessMonitor` owns the session lifecycle — including the control handle, trace handle, properties buffer, and background processing thread — and automatically cleans up all resources when dropped.

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

| Field | Type | Description |
|-------|------|-------------|
| `control_handle` | `CONTROLTRACE_HANDLE` | Handle returned by `StartTraceW`, used to control (stop) the trace session via `ControlTraceW`. |
| `trace_handle` | `PROCESSTRACE_HANDLE` | Handle returned by `OpenTraceW`, used to consume events in real time and to close the trace via `CloseTrace`. |
| `properties_buf` | `Vec<u8>` | Backing buffer for the `EVENT_TRACE_PROPERTIES` structure, including the appended session name. Kept alive for the duration of the session because `ControlTraceW` (stop) requires a valid properties pointer. |
| `process_thread` | `Option<thread::JoinHandle<()>>` | Handle to the background thread running `ProcessTrace`. `Some` while the session is active; taken (`None`) when `stop()` is called and the thread is joined. |

All fields are **private**. External code interacts with `EtwProcessMonitor` exclusively through its public methods.

## Methods

### `EtwProcessMonitor::start`

```rust
pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>
```

Starts a new ETW trace session monitoring process start/stop events from the `Microsoft-Windows-Kernel-Process` provider. Returns a tuple of the monitor instance and an `mpsc::Receiver<EtwProcessEvent>` from which the caller can read events.

#### Steps performed

1. **Create the channel** — An `mpsc::channel` is created and the sender is installed into the global [ETW_SENDER](ETW_SENDER.md) static so the `extern "system"` callback can reach it.
2. **Prepare properties** — Allocates an `EVENT_TRACE_PROPERTIES` buffer with the session name `"AffinityServiceRust_EtwProcessMonitor"` appended, configured for real-time mode with QPC timestamps.
3. **Stop stale sessions** — Calls `stop_existing_session` to tear down any leftover session with the same name (e.g., from a previous crash).
4. **Start the trace** — Calls `StartTraceW` to create the session and obtain the `control_handle`.
5. **Enable the provider** — Calls `EnableTraceEx2` to subscribe to the `Microsoft-Windows-Kernel-Process` provider at `TRACE_LEVEL_INFORMATION` with the `WINEVENT_KEYWORD_PROCESS` keyword (0x10).
6. **Open the trace** — Calls `OpenTraceW` in real-time + event-record mode, registering the module-level `etw_event_callback` function as the event record callback.
7. **Spawn the background thread** — Sets [ETW_ACTIVE](ETW_ACTIVE.md) to `true` and spawns a thread named `"etw-process-trace"` that calls `ProcessTrace`, which blocks until the trace is closed.

If any step fails, all previously acquired resources are cleaned up and an `Err(String)` is returned with a descriptive message that includes the translated Win32 error code via [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md).

### `EtwProcessMonitor::stop`

```rust
pub fn stop(&mut self)
```

Stops the ETW trace session and releases all associated resources:

1. Checks [ETW_ACTIVE](ETW_ACTIVE.md); if already `false`, returns immediately (idempotent).
2. Sets `ETW_ACTIVE` to `false`.
3. Calls `CloseTrace(trace_handle)` to unblock the `ProcessTrace` call on the background thread.
4. Calls `ControlTraceW` with `EVENT_TRACE_CONTROL_STOP` to stop the trace session.
5. Joins the background thread (`process_thread.take().join()`).
6. Clears the global [ETW_SENDER](ETW_SENDER.md) to drop the channel sender.

### `EtwProcessMonitor::stop_existing_session` (private)

```rust
fn stop_existing_session(wide_name: &[u16])
```

Attempts to stop any pre-existing ETW session with the given name. This is called before starting a new session to handle the case where a previous instance of the application crashed without cleaning up its ETW session. Failures are silently ignored since the session may not exist.

## Remarks

- **RAII cleanup.** `EtwProcessMonitor` implements `Drop`, which delegates to `stop()`. This guarantees that the ETW session is torn down even if the monitor is dropped without an explicit `stop()` call, preventing orphaned system-wide trace sessions.

- **Single-instance constraint.** Only one ETW session with a given name can exist on a system at a time. The `stop_existing_session` call in `start()` handles stale sessions, but attempting to run two instances of AffinityServiceRust concurrently will cause the second `StartTraceW` to fail.

- **Global callback bridge.** Because the ETW event callback must be an `extern "system"` function pointer (no closures or captured state), the module uses the global [ETW_SENDER](ETW_SENDER.md) static to bridge events from the callback into the Rust `mpsc` channel. The callback extracts the process ID from the first 4 bytes of `UserData` and sends an [EtwProcessEvent](EtwProcessEvent.md) with `is_start = true` for event ID 1 (ProcessStart) and `is_start = false` for event ID 2 (ProcessStop).

- **Provider details.** The `Microsoft-Windows-Kernel-Process` provider GUID is `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`. The keyword `WINEVENT_KEYWORD_PROCESS` (`0x10`) filters events to only process lifecycle events (start/stop), excluding thread and image-load events from the same provider.

- **Thread naming.** The background thread is named `"etw-process-trace"` for diagnostic visibility in debuggers and thread profilers.

### Platform notes

- **Windows only.** Relies on the ETW infrastructure: `StartTraceW`, `EnableTraceEx2`, `OpenTraceW`, `ProcessTrace`, `CloseTrace`, `ControlTraceW`.
- Requires administrator privileges to create a real-time kernel trace session.
- The session uses QPC (Query Performance Counter) timestamps (`Wnode.ClientContext = 1`) for high-resolution event timing.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `event_trace.rs` |
| **Created by** | `EtwProcessMonitor::start()` |
| **Depends on** | [ETW_SENDER](ETW_SENDER.md), [ETW_ACTIVE](ETW_ACTIVE.md), [EtwProcessEvent](EtwProcessEvent.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | `StartTraceW`, `EnableTraceEx2`, `OpenTraceW`, `ProcessTrace`, `CloseTrace`, `ControlTraceW` |
| **ETW Provider** | `Microsoft-Windows-Kernel-Process` (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) |
| **Privileges** | Administrator (elevated) required for kernel trace sessions |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| EtwProcessEvent struct | [EtwProcessEvent](EtwProcessEvent.md) |
| ETW_SENDER static | [ETW_SENDER](ETW_SENDER.md) |
| ETW_ACTIVE static | [ETW_ACTIVE](ETW_ACTIVE.md) |
| error_from_code_win32 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| process module | [process.rs](../process.rs/README.md) |
| event_trace module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
