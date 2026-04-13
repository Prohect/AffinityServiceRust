# EtwProcessEvent struct (event_trace.rs)

A process start/stop event received from ETW, containing the process ID and whether the event represents a process creation or termination.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## Members

`pid`

The process identifier extracted from the ETW event's `UserData` (first 4 bytes).

`is_start`

`true` if the event is a process start (ETW event ID 1), `false` if it is a process stop (ETW event ID 2).

## Remarks

Events are sent through an `mpsc` channel from the ETW callback to the main loop. The main loop drains the channel each iteration and uses the events to:

- **Start events**: Add the PID to `process_level_pending` for immediate rule application.
- **Stop events**: Clean up the PID from scheduler state, error deduplication maps, and applied tracking sets.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/event_trace.rs |
| **Created by** | `etw_event_callback` |
| **Consumed by** | [`main`](../main.rs/main.md) loop |

## See also

- [EtwProcessMonitor](EtwProcessMonitor.md)
- [event_trace.rs module overview](README.md)