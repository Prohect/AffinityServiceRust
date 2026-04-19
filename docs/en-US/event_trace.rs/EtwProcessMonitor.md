# EtwProcessMonitor struct (event_trace.rs)

Manages the lifecycle of an ETW (Event Tracing for Windows) real-time trace session that monitors process start and stop events. The monitor subscribes to the **Microsoft-Windows-Kernel-Process** provider and delivers [EtwProcessEvent](EtwProcessEvent.md) values through a standard `mpsc` channel.

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

| Member | Type | Description |
|--------|------|-------------|
| `control_handle` | `CONTROLTRACE_HANDLE` | Handle returned by `StartTraceW`, used to control (stop) the ETW session. |
| `trace_handle` | `PROCESSTRACE_HANDLE` | Handle returned by `OpenTraceW`, used to close the trace consumer and unblock the processing thread. |
| `properties_buf` | `Vec<u8>` | Heap-allocated buffer backing the `EVENT_TRACE_PROPERTIES` structure and the appended session name. Kept alive for the duration of the session because `ControlTraceW` requires it at stop time. |
| `process_thread` | `Option<thread::JoinHandle<()>>` | Join handle for the background thread running `ProcessTrace`. Set to `None` after the thread is joined during `stop()`. |

## Methods

### start

```rust
pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>
```

Creates and starts a new ETW real-time trace session for process monitoring.

**Return value**

`Result<(EtwProcessMonitor, Receiver<EtwProcessEvent>), String>` — On success, returns the monitor instance and a channel receiver that yields [EtwProcessEvent](EtwProcessEvent.md) values. On failure, returns a human-readable error string.

**Remarks**

The startup sequence performs six steps in order:

1. **Create channel** — Allocates an `mpsc` channel and installs the sender into the global [ETW_SENDER](README.md) static so the OS callback can reach it.
2. **Prepare `EVENT_TRACE_PROPERTIES`** — Allocates a buffer large enough for the properties struct plus the UTF-16 session name `"AffinityServiceRust_EtwProcessMonitor"`. Configures real-time mode with QPC timestamps.
3. **Stop existing session** — Calls `stop_existing_session` to clean up any stale session with the same name (e.g., from a previous crash).
4. **Start trace** — Calls `StartTraceW` to create the ETW session and obtain the control handle.
5. **Enable provider** — Calls `EnableTraceEx2` with the `Microsoft-Windows-Kernel-Process` provider GUID (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) and the `WINEVENT_KEYWORD_PROCESS` (`0x10`) keyword at `TRACE_LEVEL_INFORMATION`.
6. **Open and process trace** — Calls `OpenTraceW` with `PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD` and the [etw_event_callback](README.md) function pointer. Spawns a background thread named `"etw-process-trace"` that calls `ProcessTrace`, which blocks until the trace is closed.

If any step fails, all previously acquired resources are released (session stopped, sender cleared) before returning the error.

### stop

```rust
pub fn stop(&mut self)
```

Stops the ETW trace session and releases all resources.

**Remarks**

The shutdown sequence is:

1. Checks and clears the global `ETW_ACTIVE` flag. If already inactive, returns immediately (idempotent).
2. Calls `CloseTrace` on `trace_handle` to unblock the `ProcessTrace` call in the background thread.
3. Calls `ControlTraceW` with `EVENT_TRACE_CONTROL_STOP` to terminate the ETW session.
4. Joins the background processing thread.
5. Clears the global `ETW_SENDER` to drop the channel sender, which causes the receiver to observe a hangup.

### stop_existing_session

```rust
fn stop_existing_session(wide_name: &[u16])
```

Attempts to stop any pre-existing ETW session with the given name. This is a static helper that does not require an `EtwProcessMonitor` instance.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `wide_name` | `&[u16]` | Null-terminated UTF-16 session name to stop. |

**Remarks**

Allocates a temporary `EVENT_TRACE_PROPERTIES` buffer and calls `ControlTraceW` with `EVENT_TRACE_CONTROL_STOP`. Errors are silently ignored because the session may not exist.

## Trait Implementations

### Drop

```rust
impl Drop for EtwProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
```

The `Drop` implementation calls `stop()`, ensuring that the ETW session is always cleaned up when the monitor goes out of scope, even on early returns or panics.

## Remarks

- Only one `EtwProcessMonitor` should be active at a time because the callback communicates through a single global sender (`ETW_SENDER`). Starting a second monitor replaces the sender, orphaning the first monitor's receiver.
- The session name `"AffinityServiceRust_EtwProcessMonitor"` is a fixed constant. If the service crashes without calling `stop()`, the stale session persists in the kernel until the next call to `start()` cleans it up via `stop_existing_session`.
- `ProcessTrace` is a blocking Win32 call that only returns when the trace handle is closed. This is why it runs on a dedicated background thread rather than the main thread.
- The `properties_buf` must remain valid and at the same address for the entire session lifetime because `ControlTraceW` at stop time writes back into the same buffer.

## Requirements

| | |
|---|---|
| **Module** | `src/event_trace.rs` |
| **Callers** | Main service loop (`src/main.rs`), scheduler (`src/scheduler.rs`) |
| **Callees** | [etw_event_callback](README.md) (registered as OS callback) |
| **Win32 API** | [StartTraceW](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-starttracew), [EnableTraceEx2](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-enabletraceex2), [OpenTraceW](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-opentracew), [ProcessTrace](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-processtrace), [CloseTrace](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-closetrace), [ControlTraceW](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-controltracew) |
| **Privileges** | Administrator or `SeSystemProfilePrivilege` (required by ETW kernel-level providers) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [event_trace.rs](README.md) |
| Event payload | [EtwProcessEvent](EtwProcessEvent.md) |
| Error code helper | [error_from_code_win32](../error_codes.rs/README.md) |
| Scheduler integration | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*