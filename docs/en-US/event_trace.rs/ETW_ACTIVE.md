# ETW_ACTIVE static (event_trace.rs)

Atomic flag indicating whether the ETW session is currently active.

## Syntax

```rust
static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
```

## Remarks

Set to `true` when `EtwProcessMonitor::start()` succeeds and back to `false` when `stop()` is called. Used to prevent double-stop operations.

## See also

- [EtwProcessMonitor](EtwProcessMonitor.md)