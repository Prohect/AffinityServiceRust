# ETW_SENDER static (event_trace.rs)

Global mutex-guarded sender half of an MPSC channel, used by the ETW event record callback to dispatch [EtwProcessEvent](EtwProcessEvent.md) values to the main service loop. Because the ETW callback is an `extern "system"` function pointer with no closure state, the sender must be stored in a global static so the callback can access it.

## Syntax

```event_trace.rs
static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. The value is created on first access. |
| Middle | `Mutex<…>` | Provides interior mutability and thread-safe access from both the ETW callback thread and the main thread. |
| Inner | `Option<Sender<EtwProcessEvent>>` | `Some(sender)` while an ETW session is active; `None` when no session is running or after cleanup. |

## Remarks

- The static is initialized to `Mutex::new(None)`. When [EtwProcessMonitor::start](EtwProcessMonitor.md) is called, it creates an `mpsc::channel`, installs the `Sender` half into this global, and returns the `Receiver` half to the caller.
- The `extern "system"` ETW event record callback (`etw_event_callback`) locks this mutex on every event delivery and sends an [EtwProcessEvent](EtwProcessEvent.md) through the channel. If the lock or send fails, the event is silently dropped — this is intentional to avoid panicking inside a Windows kernel callback.
- When [EtwProcessMonitor::stop](EtwProcessMonitor.md) is called (or when the monitor is dropped), the inner `Option` is set back to `None`, which drops the `Sender` and causes the receiver end to observe a disconnected channel.
- Because `ETW_SENDER` is a process-wide singleton, only one `EtwProcessMonitor` session should be active at a time. Starting a second session overwrites the sender, orphaning the previous receiver.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `event_trace` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex`, `std::sync::mpsc::Sender` |
| Written by | [EtwProcessMonitor::start](EtwProcessMonitor.md) |
| Read by | `etw_event_callback` (module-private `extern "system"` fn) |
| Cleared by | [EtwProcessMonitor::stop](EtwProcessMonitor.md) |

## See Also

| Topic | Link |
|-------|------|
| Atomic flag for session-active status | [ETW_ACTIVE](ETW_ACTIVE.md) |
| Event payload type sent through the channel | [EtwProcessEvent](EtwProcessEvent.md) |
| Monitor that manages the ETW session lifecycle | [EtwProcessMonitor](EtwProcessMonitor.md) |
| Module overview | [event_trace module](README.md) |