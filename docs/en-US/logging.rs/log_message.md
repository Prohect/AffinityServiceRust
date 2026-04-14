# log_message function (logging.rs)

Writes a timestamped log message to the console or the main log file. Each message is prefixed with the current time in `[HH:MM:SS]` format, read from the cached [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md). Output is routed to `stdout` when [USE_CONSOLE](USE_CONSOLE.md) is `true`, or to the [LOG_FILE](LOG_FILE.md) handle when `false`. If [DUST_BIN_MODE](DUST_BIN_MODE.md) is active, the function returns immediately without producing any output.

## Syntax

```logging.rs
pub fn log_message(args: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `args` | `&str` | The message text to log. This string is appended after the `[HH:MM:SS]` timestamp prefix on the same line. A trailing newline is added automatically by `writeln!`. |

## Return value

None (`()`).

## Remarks

The function performs the following steps in order:

1. **Dust-bin check:** Acquires the [DUST_BIN_MODE](DUST_BIN_MODE.md) lock via [get_dust_bin_mod!](get_dust_bin_mod.md). If the flag is `true`, returns immediately — the message is silently discarded. This prevents logging before UAC elevation.
2. **Timestamp formatting:** Acquires the [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) lock via [get_local_time!](get_local_time.md) and formats the cached `DateTime<Local>` as `%H:%M:%S` (e.g., `14:32:07`). The formatted string is stored in a local `String`.
3. **Output routing:** Acquires the [USE_CONSOLE](USE_CONSOLE.md) lock via [get_use_console!](get_use_console.md).
   - If `true`, writes `[{time_prefix}]{args}\n` to `stdout` via `writeln!(stdout(), ...)`.
   - If `false`, writes the same formatted line to the [LOG_FILE](LOG_FILE.md) handle via `writeln!(get_logger!(), ...)`.

### Output format

```/dev/null/example.log#L1-2
[14:32:07]Applied 3 rules to PID 1234
[14:32:07]Set priority class to "high" for notepad.exe
```

The timestamp is enclosed in square brackets with no space before the message body. Each call produces exactly one line of output (terminated by `\n`).

### Error handling

Write errors from `writeln!` are silently ignored (the `Result` is bound to `let _ = …`). This design choice prevents a failing log destination (e.g., a full disk) from crashing the service. The service continues operating even when logging is unavailable.

### Lock ordering

The function acquires up to three mutex locks in the following order within a single call:

1. `DUST_BIN_MODE` — released immediately after reading the flag.
2. `LOCAL_TIME_BUFFER` — released after formatting the timestamp string.
3. `USE_CONSOLE` and either `LOG_FILE` (via `get_logger!`) or `stdout` — released after the write completes.

Because the locks are acquired sequentially (never nested), there is no deadlock risk within this function. Callers should avoid holding any of these locks when calling `log_message`.

### Relationship to the log! macro

This function is not typically called directly. Instead, most call sites use the [log!](log.md) macro, which formats its arguments via `format!()` and passes the resulting `&str` to `log_message`. Direct calls are useful when the caller already has a pre-formatted `&str` and wants to avoid an additional `format!` allocation.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Callers | [log!](log.md) macro (primary), [log_error_if_new](../apply.rs/log_error_if_new.md), and various call sites across the crate |
| Callees | [get_dust_bin_mod!](get_dust_bin_mod.md), [get_local_time!](get_local_time.md), [get_use_console!](get_use_console.md), [get_logger!](get_logger.md) |
| Reads | [DUST_BIN_MODE](DUST_BIN_MODE.md), [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md), [USE_CONSOLE](USE_CONSOLE.md), [LOG_FILE](LOG_FILE.md) |
| Std dependencies | `std::io::Write`, `std::io::stdout` |

## See Also

| Topic | Link |
|-------|------|
| Convenience logging macro | [log!](log.md) |
| Raw log writing (no timestamp) | [log_pure_message](log_pure_message.md) |
| Find-mode timestamped logging | [log_to_find](log_to_find.md) |
| Log suppression flag | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Console routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| Main log file handle | [LOG_FILE](LOG_FILE.md) |
| Cached timestamp | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd