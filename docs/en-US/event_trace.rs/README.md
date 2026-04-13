# event_trace module (AffinityServiceRust)

The `event_trace` module provides a minimal ETW (Event Tracing for Windows) consumer for real-time process start/stop monitoring. It subscribes to the `Microsoft-Windows-Kernel-Process` provider to receive notifications when processes are created or terminated, enabling the service to reactively apply configuration rules to newly launched processes without waiting for the next polling interval. The module manages the full ETW session lifecycle — starting, consuming, and stopping trace sessions — and dispatches process events through a channel for the main service loop to consume.

## Statics

| Static | Description |
|--------|-------------|
| [ETW_SENDER](ETW_SENDER.md) | Global mutex-guarded sender half of the MPSC channel used by the ETW callback to dispatch process events. |
| [ETW_ACTIVE](ETW_ACTIVE.md) | Atomic boolean flag indicating whether the ETW trace session is currently active. |

## Structs

| Struct | Description |
|--------|-------------|
| [EtwProcessEvent](EtwProcessEvent.md) | Lightweight value type carrying a process ID and a start/stop flag, produced by the ETW callback. |
| [EtwProcessMonitor](EtwProcessMonitor.md) | Owns the ETW trace session handles and background processing thread; provides `start`, `stop`, and `stop_existing_session` methods. |

## See Also

| Topic | Link |
|-------|------|
| Win32 error code translation used in ETW error reporting | [error_codes module](../error_codes.rs/README.md) |
| Service main loop that consumes ETW events | [main module](../main.rs/README.md) |
| Logging infrastructure for diagnostics | [logging module](../logging.rs/README.md) |
| Windows API wrappers (handles, privileges) | [winapi module](../winapi.rs/README.md) |