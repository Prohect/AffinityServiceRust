# LOCAL_TIME_BUFFER static (logging.rs)

Caches the current local time to ensure consistent timestamps across all log messages written within a single loop iteration.

## Syntax

```rust
static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
```

## Members

The static holds a `DateTime<Local>` behind a `Mutex`, representing the cached local timestamp for the current loop iteration.

## Remarks

During a single iteration of the main loop, the application may emit many log messages as it processes multiple matched processes and their threads. Without a consistent timestamp buffer, each `log!` call would capture a slightly different time, making it difficult to correlate log entries that belong to the same iteration.

`LOCAL_TIME_BUFFER` solves this by caching the timestamp at the start of each loop iteration. All subsequent calls to [`log_message`](log_message.md) within that iteration use the cached value, producing uniform timestamps in the log output.

The buffer is updated at the beginning of each main loop iteration in [`main`](../main.rs/main.md) by locking the mutex and writing a fresh `Local::now()` value. This ensures that the timestamp advances between iterations but remains stable within one.

### Initialization

The static is lazily initialized via `once_cell::sync::Lazy` with `Local::now()` at the time of first access. This initial value is only used for any log messages emitted before the main loop begins (e.g., during startup).

### Thread safety

All access is synchronized through the `Mutex`. In practice, the application is single-threaded, but the mutex satisfies Rust's `Sync` requirement for statics with interior mutability.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L64 |
| **Updated by** | [`main`](../main.rs/main.md) loop at the start of each iteration |
| **Read by** | [`log_message`](log_message.md) |
| **Dependencies** | `chrono::Local`, `chrono::DateTime`, `once_cell::sync::Lazy` |

## See also

- [log_message function](log_message.md)
- [DUST_BIN_MODE static](DUST_BIN_MODE.md)
- [logging.rs module overview](README.md)