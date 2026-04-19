# log_message function (logging.rs)

Writes a timestamped log line to either the console or the daily log file. This is the primary logging function used throughout the service, typically invoked via the [`log!`](README.md) macro.

## Syntax

```rust
pub fn log_message(args: &str)
```

## Parameters

`args: &str`

The message body to log. This string is appended after the `[HH:MM:SS]` timestamp prefix with no separator space — the caller is responsible for including any leading whitespace or punctuation if desired.

## Return value

This function does not return a value.

## Remarks

### Output format

Each log line is formatted as:

```
[HH:MM:SS]message text here
```

The timestamp is obtained from the global [`LOCAL_TIME_BUFFER`](README.md) static via the `get_local_time!()` macro. Because this buffer is updated externally by the main loop, all log lines within a single apply cycle share the same timestamp, which groups related messages visually.

### Dust-bin mode suppression

When [`DUST_BIN_MODE`](README.md) is `true`, the function returns immediately without writing anything. This mode is activated during UAC elevation to prevent the non-elevated instance from producing output while the elevated instance is starting.

### Console vs. file routing

The destination is determined by the [`USE_CONSOLE`](README.md) flag:

| `USE_CONSOLE` | Destination |
|---------------|-------------|
| `true` | `stdout` via `writeln!` |
| `false` | Daily log file at `logs/YYYYMMDD.log` via the [`LOG_FILE`](README.md) static |

Console mode is used during interactive CLI execution (e.g., `--find`, `--apply-once`). File mode is used when running as a Windows service.

### Error handling

Write failures (e.g., disk full, broken pipe) are silently ignored — the `writeln!` result is discarded with `let _ = ...`. This prevents a logging failure from crashing the service.

### Typical usage via macro

The `log!` macro is the preferred way to call this function, as it supports `format!`-style arguments:

```rust
log!(" applied affinity mask 0x{:X} to pid {}", mask, pid);
```

This expands to:

```rust
crate::logging::log_message(format!(" applied affinity mask 0x{:X} to pid {}", mask, pid).as_str());
```

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | `log!` macro (used throughout all modules) |
| **Callees** | `get_dust_bin_mod!()`, `get_local_time!()`, `get_use_console!()`, `get_logger!()` |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| No-timestamp variant | [log_pure_message](log_pure_message.md) |
| Find-mode logging | [log_to_find](log_to_find.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*