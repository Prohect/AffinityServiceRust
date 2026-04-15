# ETW_SENDER static (event_trace.rs)

Global channel sender used by the ETW event callback to deliver process events to the consumer. Because the ETW callback is an `extern "system"` function pointer that cannot capture any state, a global static is required to bridge the OS-level callback with Rust's `mpsc` channel infrastructure.

## Syntax

```rust
static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
```

## Type

`once_cell::sync::Lazy<std::sync::Mutex<Option<std::sync::mpsc::Sender<EtwProcessEvent>>>>`

## Remarks

### Lifecycle

1. **Initial state** — `None`. The static is initialized as `Mutex::new(None)` on first access.
2. **Activated** — When [`EtwProcessMonitor::start`](EtwProcessMonitor.md) is called, it creates an `mpsc::channel`, installs the `Sender` half into `ETW_SENDER`, and returns the `Receiver` half to the caller.
3. **In use** — The `etw_event_callback` function (an `unsafe extern "system"` callback invoked by the OS for each ETW event) locks the mutex, checks for `Some(ref sender)`, and sends [`EtwProcessEvent`](EtwProcessEvent.md) instances through the channel.
4. **Deactivated** — When [`EtwProcessMonitor::stop`](EtwProcessMonitor.md) is called (or the monitor is dropped), the sender is replaced with `None`, which causes the receiver end to observe a disconnected channel.

### Why a global static?

Windows ETW requires the event record callback to be a raw function pointer (`unsafe extern "system" fn(*mut EVENT_RECORD)`). Rust closures and trait objects cannot be used as raw function pointers when they capture environment state. The global `ETW_SENDER` static provides a fixed, well-known location where the callback can find the channel sender without capturing any variables.

### Thread safety

- The `Mutex` ensures that concurrent access from the ETW callback thread and the control thread (which calls `start` / `stop`) is properly synchronized.
- The ETW callback thread acquires the lock briefly for each event to send it through the channel. If the lock is poisoned (e.g., a panic occurred while holding it), the callback silently drops the event.
- The `Lazy` wrapper from `once_cell` ensures thread-safe one-time initialization.

### Error handling in the callback

The callback uses a defensive access pattern:

```rust
if let Ok(guard) = ETW_SENDER.lock()
    && let Some(ref sender) = *guard
{
    let _ = sender.send(EtwProcessEvent { pid, is_start });
}
```

- If the mutex is poisoned, the event is silently dropped.
- If the sender is `None` (monitor not started or already stopped), the event is silently dropped.
- If the channel is disconnected (receiver dropped), `sender.send()` returns `Err`, which is ignored via `let _`.

### Cleanup

When the ETW session is stopped via [`EtwProcessMonitor::stop`](EtwProcessMonitor.md) or the monitor is dropped, the global sender is set back to `None`:

```rust
if let Ok(mut guard) = ETW_SENDER.lock() {
    *guard = None;
}
```

This also happens on error paths during [`EtwProcessMonitor::start`](EtwProcessMonitor.md) if `StartTraceW`, `EnableTraceEx2`, or `OpenTraceW` fails, ensuring no dangling sender remains installed.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `event_trace.rs` |
| **Visibility** | Private (module-internal) |
| **Accessed by** | `etw_event_callback` (the OS-invoked ETW callback), [`EtwProcessMonitor::start`](EtwProcessMonitor.md), [`EtwProcessMonitor::stop`](EtwProcessMonitor.md) |
| **Dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex`, `std::sync::mpsc::Sender`, [`EtwProcessEvent`](EtwProcessEvent.md) |
| **Platform** | Windows only |

## See Also

| Topic | Link |
|-------|------|
| ETW_ACTIVE static | [ETW_ACTIVE](ETW_ACTIVE.md) |
| EtwProcessEvent struct | [EtwProcessEvent](EtwProcessEvent.md) |
| EtwProcessMonitor struct | [EtwProcessMonitor](EtwProcessMonitor.md) |
| event_trace module overview | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
