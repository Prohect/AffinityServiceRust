# ETW_ACTIVE static (event_trace.rs)

An `AtomicBool` flag indicating whether the ETW (Event Tracing for Windows) session is currently active. This flag is used to guard against redundant stop operations and to coordinate the lifecycle of the ETW trace session managed by [`EtwProcessMonitor`](EtwProcessMonitor.md).

## Syntax

```rust
static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
```

## Type

`std::sync::atomic::AtomicBool`

## Remarks

### State transitions

| Transition | Trigger | Ordering |
|------------|---------|----------|
| `false` → `true` | [`EtwProcessMonitor::start`](EtwProcessMonitor.md) successfully spawns the background processing thread. | `SeqCst` (store) |
| `true` → `false` | [`EtwProcessMonitor::stop`](EtwProcessMonitor.md) is called (either explicitly or via `Drop`). | `SeqCst` (store) |

### Usage pattern

- **In `start()`**: After the ETW session has been fully initialized (trace started, provider enabled, trace opened, and processing thread spawned), `ETW_ACTIVE` is set to `true` via `store(true, Ordering::SeqCst)`.

- **In `stop()`**: The method first checks `ETW_ACTIVE` via `load(Ordering::SeqCst)`. If the flag is already `false`, the stop operation returns immediately — this prevents double-close of trace handles and redundant cleanup. If `true`, the flag is set to `false` before proceeding with cleanup (closing the trace, stopping the session, joining the thread, and clearing the global sender).

### Thread safety

`ETW_ACTIVE` uses `AtomicBool` with `SeqCst` ordering for all operations, providing the strongest memory ordering guarantee. This ensures that:

- The flag update in `stop()` is visible to any concurrent reader immediately.
- The guard check at the top of `stop()` correctly prevents concurrent or repeated stop calls from racing.

### Visibility

This static is **module-private** (no `pub` modifier). It is only accessed internally by the `EtwProcessMonitor::start` and `EtwProcessMonitor::stop` methods.

### Initialization

Unlike the other statics in this module (which use `once_cell::sync::Lazy`), `ETW_ACTIVE` is a plain `AtomicBool` initialized to `false` at compile time. No lazy initialization is needed because `AtomicBool::new` is a `const fn`.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `event_trace.rs` |
| **Visibility** | Private (module-internal) |
| **Accessed by** | [`EtwProcessMonitor::start`](EtwProcessMonitor.md), [`EtwProcessMonitor::stop`](EtwProcessMonitor.md) |
| **Dependencies** | `std::sync::atomic::{AtomicBool, Ordering}` |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| EtwProcessMonitor struct | [EtwProcessMonitor](EtwProcessMonitor.md) |
| ETW_SENDER static | [ETW_SENDER](ETW_SENDER.md) |
| EtwProcessEvent struct | [EtwProcessEvent](EtwProcessEvent.md) |
| event_trace module overview | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
