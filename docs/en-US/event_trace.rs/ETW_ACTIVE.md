# ETW_ACTIVE static (event_trace.rs)

An atomic boolean flag that indicates whether the ETW (Event Tracing for Windows) trace session is currently active. This flag is shared across threads without mutex overhead, using `SeqCst` ordering for both loads and stores to ensure consistent visibility. It guards the [EtwProcessMonitor::stop](EtwProcessMonitor.md) method against redundant teardown and signals the overall session lifecycle state.

## Syntax

```event_trace.rs
static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
```

## Remarks

`ETW_ACTIVE` is initialized to `false` and transitions through the following states during the ETW session lifecycle:

| State | Set by | Meaning |
|-------|--------|---------|
| `false` → `true` | [EtwProcessMonitor::start](EtwProcessMonitor.md) | The trace session was started successfully and the background processing thread has been spawned. |
| `true` → `false` | [EtwProcessMonitor::stop](EtwProcessMonitor.md) | The trace session is being torn down. `CloseTrace` and `ControlTraceW` with `EVENT_TRACE_CONTROL_STOP` are called, and the background thread is joined. |

### Thread safety

`ETW_ACTIVE` uses `Ordering::SeqCst` for all loads and stores, which is the strongest memory ordering guarantee. This ensures that:

- The `stop` method on one thread sees the `true` value set by `start` on another thread.
- A second call to `stop` (including the implicit call from `Drop`) observes the `false` value set by the first call and short-circuits immediately.

### Guard against double-stop

The `stop` method checks `ETW_ACTIVE` at entry and returns immediately if it is already `false`. This prevents double-invocation issues when `stop` is called explicitly and then again via the `Drop` implementation on [EtwProcessMonitor](EtwProcessMonitor.md).

### Module-private visibility

This static is **not** marked `pub` — it is internal to the `event_trace` module. External code interacts with the ETW session exclusively through the [EtwProcessMonitor](EtwProcessMonitor.md) API.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `event_trace` |
| Type | `AtomicBool` (from `std::sync::atomic`) |
| Callers | [EtwProcessMonitor::start](EtwProcessMonitor.md), [EtwProcessMonitor::stop](EtwProcessMonitor.md) |
| Callees | *(none — atomic primitive)* |
| Privileges | None |

## See Also

| Topic | Link |
|-------|------|
| Global ETW event sender channel | [ETW_SENDER](ETW_SENDER.md) |
| ETW session manager struct | [EtwProcessMonitor](EtwProcessMonitor.md) |
| Process event payload | [EtwProcessEvent](EtwProcessEvent.md) |
| Module overview | [event_trace module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd