# ETW_SENDER static (event_trace.rs)

Global sender for the ETW callback to dispatch process events through an `mpsc` channel.

## Syntax

```rust
static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
```

## Remarks

The ETW event callback is an `extern "system"` function pointer that cannot capture any environment. This global sender provides the mechanism for the callback to communicate events to the main loop. It is set to `Some(sender)` during `EtwProcessMonitor::start()` and cleared to `None` during `stop()`.

## See also

- [EtwProcessMonitor](EtwProcessMonitor.md)
- [EtwProcessEvent](EtwProcessEvent.md)