# event_trace module (AffinityServiceRust)

The `event_trace` module implements a minimal ETW (Event Tracing for Windows) consumer for real-time process start/stop monitoring. It subscribes to the **Microsoft-Windows-Kernel-Process** provider (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) and delivers `EtwProcessEvent` messages through a standard `mpsc` channel, enabling the main service loop to reactively apply configuration rules the moment a process is created or terminated — without polling.

## Statics

| Name | Type | Description |
|------|------|-------------|
| `ETW_SENDER` | `Lazy<Mutex<Option<Sender<EtwProcessEvent>>>>` | Global channel sender used by the OS-invoked [etw_event_callback](#etw_event_callback) to forward events. Because the callback is an `extern "system"` function pointer, a global is required to bridge into Rust's channel infrastructure. |
| `ETW_ACTIVE` | `AtomicBool` | Flag indicating whether an ETW session is currently active. Checked by [stop](EtwProcessMonitor.md#stop) to avoid double-cleanup. |

## Functions

| Name | Description |
|------|-------------|
| <a id="etw_event_callback"></a>`etw_event_callback` | `unsafe extern "system"` callback invoked by the OS for each ETW event record. Extracts the process ID from the first 4 bytes of `UserData`, maps Event ID 1 → start and Event ID 2 → stop, and sends an [EtwProcessEvent](EtwProcessEvent.md) through `ETW_SENDER`. Non-process events and null records are silently discarded. |

## Structs

| Name | Description |
|------|-------------|
| [EtwProcessEvent](EtwProcessEvent.md) | Lightweight value representing a process start or stop event received from ETW. |
| [EtwProcessMonitor](EtwProcessMonitor.md) | RAII handle that owns the ETW session lifecycle — creation, background processing, and teardown. |

## Constants

| Name | Value | Description |
|------|-------|-------------|
| `KERNEL_PROCESS_GUID` | `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}` | GUID for the Microsoft-Windows-Kernel-Process ETW provider. |
| `WINEVENT_KEYWORD_PROCESS` | `0x10` | Keyword bitmask that selects process-lifecycle events from the provider. |
| `SESSION_NAME` | `"AffinityServiceRust_EtwProcessMonitor"` | Name registered with the ETW subsystem for this trace session. Used to detect and clean up stale sessions on startup. |

## See Also

| Topic | Link |
|-------|------|
| Main service loop (ETW consumer) | [main.rs](../main.rs/README.md) |
| Error code formatting | [error_codes.rs](../error_codes.rs/README.md) |
| Microsoft-Windows-Kernel-Process provider | [Microsoft docs](https://learn.microsoft.com/en-us/windows/win32/etw/event-tracing-portal) |
| EVENT_TRACE_PROPERTIES | [Microsoft docs](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/ns-evntrace-event_trace_properties) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*