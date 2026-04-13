# USE_CONSOLE static (logging.rs)

Global flag that controls whether log output is directed to the console (standard output) or to the log file on disk. When `true`, all logging functions write to `stdout`; when `false` (the default), they write to the [LOG_FILE](LOG_FILE.md) or [FIND_LOG_FILE](FIND_LOG_FILE.md) handles as appropriate.

## Syntax

```logging.rs
pub static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. Created on first access. |
| Inner | `Mutex<bool>` | Thread-safe interior mutability. Guards a `bool` value. |
| Default | `false` | Log output is directed to the file system by default. |

## Remarks

- The flag is typically set to `true` early in `main` when the service detects it is running interactively (attached to a console) rather than as a background Windows service. Once set, it remains constant for the lifetime of the process.
- All logging functions — [log_message](log_message.md), [log_pure_message](log_pure_message.md), and [log_to_find](log_to_find.md) — check this flag to decide their output destination. When `true`, they call `writeln!(stdout(), …)` instead of writing to the file handles.
- The flag is accessed through the [get_use_console!](get_use_console.md) macro, which locks the mutex and returns a `MutexGuard<bool>`. Callers dereference the guard to read the value.
- Because the flag is read on every log call but written only once at startup, the mutex contention is effectively zero after initialization. The `Mutex` is used for safe interior mutability rather than for protecting concurrent writes.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex` |
| Written by | [main](../main.rs/README.md) (during startup) |
| Read by | [log_message](log_message.md), [log_pure_message](log_pure_message.md), [log_to_find](log_to_find.md) |
| Accessor macro | [get_use_console!](get_use_console.md) |

## See Also

| Topic | Link |
|-------|------|
| Suppression flag for pre-elevation logging | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Main log file handle (used when flag is `false`) | [LOG_FILE](LOG_FILE.md) |
| Find-mode log file handle | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Timestamped log output function | [log_message](log_message.md) |
| logging module overview | [logging module](README.md) |