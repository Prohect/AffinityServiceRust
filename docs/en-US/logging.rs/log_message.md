# log_message function (logging.rs)

Writes a timestamped log message to the main log file or to stdout, depending on the current console mode setting. If dust-bin mode is active, the message is silently discarded. This function is the primary logging entry point and is typically invoked indirectly via the `log!` macro.

## Syntax

```rust
pub fn log_message(args: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `args` | `&str` | The message string to log. This is typically the output of `format!(...)` when called through the `log!` macro. |

## Return value

This function does not return a value.

## Remarks

### Behavior

1. **Dust-bin mode check.** The function first acquires the `DUST_BIN_MODE` mutex (via `get_dust_bin_mod!()`). If the value is `true`, the function returns immediately without writing anything. This mode is used to suppress all log output during certain operational phases.

2. **Timestamp formatting.** The current time is read from the `LOCAL_TIME_BUFFER` static (via `get_local_time!()`) and formatted as `HH:MM:SS` using `chrono`'s `format` method.

3. **Output routing.** The function checks the `USE_CONSOLE` static (via `get_use_console!()`):
   - If `true`, the message is written to **stdout** via `writeln!(stdout(), "[{}]{}", time_prefix, args)`.
   - If `false`, the message is written to the **main log file** by locking the `LOG_FILE` static (via `get_logger!()`) and calling `writeln!`.

4. **Error suppression.** Write errors are silently ignored via `let _ = writeln!(...)`. This prevents logging failures from propagating and crashing the application.

### Output format

```text
[HH:MM:SS]<message>
```

For example, if `args` is `" Starting scheduler loop"`, the output would be:

```text
[14:32:07] Starting scheduler loop
```

Note that there is **no space** between the timestamp bracket `]` and the start of the message. If spacing is desired, the caller must include it in the `args` string.

### Relationship to the `log!` macro

The `log!` macro is the preferred way to call this function:

```rust
log!("Processing {} threads for PID {}", thread_count, pid);
```

This expands to:

```rust
crate::logging::log_message(format!("Processing {} threads for PID {}", thread_count, pid).as_str())
```

The macro handles `format!`-style argument interpolation and passes the resulting `String` as a `&str` to `log_message`.

### Locking order

This function acquires up to three mutex locks in the following order:

1. `DUST_BIN_MODE` — checked first for early exit.
2. `LOCAL_TIME_BUFFER` — read for timestamp formatting.
3. `USE_CONSOLE` and then either stdout or `LOG_FILE` — for output routing.

Each lock is acquired and released independently (not held simultaneously), so deadlock risk is minimal. However, interleaving with other logging functions ([`log_pure_message`](log_pure_message.md), [`log_to_find`](log_to_find.md)) on concurrent threads may produce out-of-order log lines.

### Platform notes

- On Windows, stdout writes go to the console window if one is attached, or are discarded if the process has no console.
- Log files are opened in append mode by the `LOG_FILE` static, so messages accumulate across the session without overwriting earlier entries.
- The `LOCAL_TIME_BUFFER` static caches the current time and must be updated externally by the scheduling loop for timestamps to advance. If not updated, all log messages within a cycle share the same timestamp.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Callers** | All modules via the `log!` macro |
| **Callees** | `get_dust_bin_mod!`, `get_local_time!`, `get_use_console!`, `get_logger!`, `std::io::stdout`, `writeln!` |
| **Dependencies** | `chrono::DateTime`, `std::io::Write`, `std::io::stdout` |
| **Statics accessed** | [`DUST_BIN_MODE`](statics.md#dust_bin_mode), [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer), [`USE_CONSOLE`](statics.md#use_console), [`LOG_FILE`](statics.md#log_file) |
| **Platform** | Windows (stdout behavior), cross-platform file I/O |

## See Also

| Topic | Link |
|-------|------|
| log_pure_message function | [log_pure_message](log_pure_message.md) |
| log_to_find function | [log_to_find](log_to_find.md) |
| log_process_find function | [log_process_find](log_process_find.md) |
| Logging statics | [statics](statics.md) |
| logging module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
