# event_trace module (AffinityServiceRust)

The `event_trace` module provides a minimal ETW (Event Tracing for Windows) consumer for real-time process start/stop monitoring. It uses the `Microsoft-Windows-Kernel-Process` provider (GUID `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) to receive notifications when processes are created or terminated, enabling reactive rule application without polling.

The module manages an ETW real-time trace session on a background thread. A global channel (`ETW_SENDER`) bridges the OS-level `extern "system"` callback with safe Rust code via `mpsc::Sender`. The `EtwProcessMonitor` struct owns the session lifetime and automatically cleans up on drop.

## Functions

This module does not expose public functions directly. All functionality is accessed through the `EtwProcessMonitor` struct methods.

## Structs

| Struct | Description |
|--------|-------------|
| [EtwProcessEvent](EtwProcessEvent.md) | A process event received from ETW, containing the process ID and whether it was a start or stop event. |
| [EtwProcessMonitor](EtwProcessMonitor.md) | Manages an ETW real-time trace session for process monitoring, including session setup, background thread lifecycle, and cleanup. |

## Statics

| Static | Description |
|--------|-------------|
| [ETW_SENDER](ETW_SENDER.md) | Global `Mutex<Option<Sender<EtwProcessEvent>>>` used by the ETW callback to send events to the consumer. Required because the ETW callback is an `extern "system"` function pointer that cannot capture state. |
| [ETW_ACTIVE](ETW_ACTIVE.md) | `AtomicBool` flag indicating whether the ETW session is currently active. Used to guard against redundant stop operations. |

## See Also

| Link | Description |
|------|-------------|
| [process.rs module](../process.rs/README.md) | Process snapshot enumeration used alongside ETW for process data lookup. |
| [logging.rs module](../logging.rs/README.md) | Logging utilities used for diagnostics and error reporting. |
| [error_codes.rs module](../error_codes.rs/README.md) | Win32 error code translation used when ETW API calls fail. |

---
Source commit: `7221ea0694670265d4eb4975582d8ed2ae02439d`
