# EtwProcessEvent struct (event_trace.rs)

A process event received from ETW (Event Tracing for Windows), representing either a process start or process stop notification. Instances of this struct are produced by the ETW callback function and delivered to consumers through an `mpsc::Receiver<EtwProcessEvent>` channel returned by [`EtwProcessMonitor::start`](EtwProcessMonitor.md).

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `pid` | `u32` | The process identifier extracted from the ETW event's `UserData` payload (first 4 bytes). This is the PID of the process that was created or terminated. |
| `is_start` | `bool` | `true` if the event represents a process creation (ETW Event ID `1` — `ProcessStart`). `false` if the event represents a process termination (ETW Event ID `2` — `ProcessStop`). |

## Remarks

- The struct derives `Debug` and `Clone`, allowing it to be printed for diagnostics and copied freely across channel boundaries and collection operations.

- Instances are constructed inside the `extern "system"` ETW callback function (`etw_event_callback`), which extracts the PID from the raw `EVENT_RECORD.UserData` pointer and determines the event type from `EVENT_RECORD.EventHeader.EventDescriptor.Id`. Only Event IDs `1` (start) and `2` (stop) produce `EtwProcessEvent` values; all other event IDs are silently discarded by the callback.

- Events are sent through the global [`ETW_SENDER`](ETW_SENDER.md) channel. If the sender has been dropped or the channel is full, the event is silently lost (the callback uses `let _ = sender.send(...)` to ignore send errors).

- The consumer (typically the main scheduling loop) receives these events via the `mpsc::Receiver<EtwProcessEvent>` returned by [`EtwProcessMonitor::start`](EtwProcessMonitor.md) and uses them to reactively apply affinity/priority rules when new processes appear, rather than relying solely on polling-based snapshots from the [process module](../process.rs/README.md).

### ETW event ID mapping

| ETW Event ID | `is_start` value | Meaning |
|---------------|-------------------|---------|
| `1` | `true` | `ProcessStart` — a new process was created. |
| `2` | `false` | `ProcessStop` — an existing process was terminated. |

### ETW provider details

Events are sourced from the `Microsoft-Windows-Kernel-Process` provider (GUID `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) with keyword filter `WINEVENT_KEYWORD_PROCESS` (`0x10`), which ensures only process-related events are delivered to the callback.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `event_trace.rs` |
| **Produced by** | `etw_event_callback` (module-private `extern "system"` function) |
| **Consumed by** | Main scheduling loop via `mpsc::Receiver<EtwProcessEvent>` |
| **Delivered through** | [`ETW_SENDER`](ETW_SENDER.md) global channel |
| **Platform** | Windows only — requires ETW infrastructure |

## See Also

| Topic | Link |
|-------|------|
| EtwProcessMonitor struct | [EtwProcessMonitor](EtwProcessMonitor.md) |
| ETW_SENDER static | [ETW_SENDER](ETW_SENDER.md) |
| ETW_ACTIVE static | [ETW_ACTIVE](ETW_ACTIVE.md) |
| process module | [process.rs](../process.rs/README.md) |
| event_trace module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
