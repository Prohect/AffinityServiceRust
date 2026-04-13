# EtwProcessEvent struct (event_trace.rs)

Represents a process lifecycle event received from the ETW (Event Tracing for Windows) real-time trace session. Each instance carries the process ID of the affected process and a flag indicating whether the event represents a process start or a process stop. Instances are produced by the internal `etw_event_callback` function and delivered to the main service loop through an MPSC channel.

## Syntax

```event_trace.rs
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `pid` | `u32` | The process identifier of the process that was created or terminated. Extracted from the first 4 bytes of the ETW event record's `UserData` payload. |
| `is_start` | `bool` | `true` if the event represents a process creation (ETW event ID 1), `false` if it represents a process termination (ETW event ID 2). |

## Remarks

- The struct derives `Debug` and `Clone`, making it suitable for diagnostic logging and for passing copies across thread boundaries. It is intentionally lightweight — two scalar fields with no heap allocation — to minimize overhead in the high-frequency ETW callback path.
- Instances are created inside the `extern "system"` callback `etw_event_callback`, which runs on the ETW processing thread spawned by [EtwProcessMonitor::start](EtwProcessMonitor.md). The callback sends each event through the global [ETW_SENDER](ETW_SENDER.md) channel. The receiving end is held by the main service loop, which uses `is_start` to decide whether to apply configuration rules to a newly launched process or to clean up state for a terminated one.
- The `pid` value comes directly from the `Microsoft-Windows-Kernel-Process` provider's event payload. For `ProcessStart` events (ID 1), this is the PID of the newly created process. For `ProcessStop` events (ID 2), this is the PID of the process that has exited.
- Because the ETW callback extracts the PID from raw `UserData` bytes, the callback performs a bounds check (`UserDataLength >= 4`) before reading. If the check fails, no `EtwProcessEvent` is produced.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `event_trace` |
| Producers | `etw_event_callback` (internal `extern "system"` function) |
| Consumers | Service main loop in [main module](../main.rs/README.md) |
| Channel | Sent via [ETW_SENDER](ETW_SENDER.md), received from the `Receiver<EtwProcessEvent>` returned by [EtwProcessMonitor::start](EtwProcessMonitor.md) |
| ETW provider | `Microsoft-Windows-Kernel-Process` (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) |

## See Also

| Topic | Link |
|-------|------|
| ETW session manager that produces these events | [EtwProcessMonitor](EtwProcessMonitor.md) |
| Global channel sender used by the callback | [ETW_SENDER](ETW_SENDER.md) |
| Active session flag | [ETW_ACTIVE](ETW_ACTIVE.md) |
| event_trace module overview | [event_trace module](README.md) |