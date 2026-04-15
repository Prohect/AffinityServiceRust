# log_pure_message function (logging.rs)

Writes a message to the main log file or stdout **without** a timestamp prefix. This is used for continuation lines, banners, or structured output where the `[HH:MM:SS]` prefix added by [`log_message`](log_message.md) would be undesirable.

## Syntax

```rust
pub fn log_pure_message(args: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `args` | `&str` | The message string to write. A trailing newline is appended automatically via `writeln!`. |

## Return value

This function does not return a value. Write errors from `writeln!` are silently ignored.

## Remarks

- Unlike [`log_message`](log_message.md), this function does **not** check the [`DUST_BIN_MODE`](statics.md#dust_bin_mode) flag. Messages sent through `log_pure_message` are always written regardless of dust-bin mode.

- The output destination is determined by the [`USE_CONSOLE`](statics.md#use_console) flag:
  - When `true`, the message is written to `stdout` via `writeln!(stdout(), ...)`.
  - When `false`, the message is written to the main daily log file via `writeln!(get_logger!(), ...)`.

- No timestamp is prepended — the raw `args` string is written directly. This is the key difference from [`log_message`](log_message.md), which formats the current time from [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer) as a `[HH:MM:SS]` prefix.

- Write errors (e.g., broken pipe, full disk) are discarded via `let _ = writeln!(...)`. The function does not propagate I/O errors to the caller.

### Comparison with other logging functions

| Function | Timestamp | Dust-bin check | Output target |
|----------|-----------|----------------|---------------|
| [`log_message`](log_message.md) | `[HH:MM:SS]` prefix | Yes — skips if `DUST_BIN_MODE` is `true` | Main log / stdout |
| **log_pure_message** | None | No — always writes | Main log / stdout |
| [`log_to_find`](log_to_find.md) | `[HH:MM:SS]` prefix | No — always writes | Find log / stdout |

### Locking behavior

The function acquires up to two mutex locks per call:

1. `USE_CONSOLE` — to check the console mode flag.
2. Either `LOG_FILE` (via `get_logger!()`) or no additional lock for `stdout`.

Callers should avoid holding other logging-related locks when calling this function to prevent potential deadlocks.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Callers** | `main.rs`, `scheduler.rs` — for banner lines and structured output that should not carry a timestamp. |
| **Callees** | `get_use_console!()` macro, `get_logger!()` macro, `std::io::stdout`, `writeln!` |
| **Statics read** | [`USE_CONSOLE`](statics.md#use_console), [`LOG_FILE`](statics.md#log_file) |
| **Platform** | Windows (log file paths assume Windows directory conventions) |

## See Also

| Topic | Link |
|-------|------|
| log_message function | [log_message](log_message.md) |
| log_to_find function | [log_to_find](log_to_find.md) |
| log_process_find function | [log_process_find](log_process_find.md) |
| logging statics | [statics](statics.md) |
| logging module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
