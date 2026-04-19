# EtwProcessEvent struct (event_trace.rs)

Represents a single process lifecycle event received from the ETW (Event Tracing for Windows) real-time trace session. Each instance indicates that a process was either created or terminated, as reported by the Microsoft-Windows-Kernel-Process provider.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `pid` | `u32` | The process identifier extracted from the ETW event's `UserData` payload (first 4 bytes). |
| `is_start` | `bool` | `true` if the event represents a process start (ETW Event ID 1), `false` if it represents a process stop (ETW Event ID 2). |

## Remarks

- Instances of this struct are created inside the unsafe `etw_event_callback` function and sent through the global `ETW_SENDER` channel to the consumer on the main service thread.
- The struct derives `Clone` so that consumers can retain copies of events without lifetime concerns, and `Debug` for diagnostic logging.
- The `pid` value comes directly from the kernel event payload and is valid at the time the event fires. For process-stop events, the PID may already be recycled by the time the consumer processes the message, though this is rare in practice.

### Event ID mapping

| ETW Event ID | `is_start` value | Meaning |
|--------------|-------------------|---------|
| 1 | `true` | ProcessStart — a new process was created |
| 2 | `false` | ProcessStop — an existing process terminated |

All other event IDs from the Microsoft-Windows-Kernel-Process provider are filtered out by the callback and never produce an `EtwProcessEvent`.

## Requirements

| | |
|---|---|
| **Module** | `src/event_trace.rs` |
| **Created by** | `etw_event_callback` (unsafe extern "system" function) |
| **Consumed by** | Main service loop via `Receiver<EtwProcessEvent>` returned from [EtwProcessMonitor::start](EtwProcessMonitor.md) |
| **Dependencies** | None (plain data struct) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [event_trace.rs](README.md) |
| ETW session manager | [EtwProcessMonitor](EtwProcessMonitor.md) |
| Error deduplication (consumer side) | [is_new_error](../logging.rs/is_new_error.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*