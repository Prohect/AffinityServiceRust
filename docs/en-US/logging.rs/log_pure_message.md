# log_pure_message function (logging.rs)

Writes a message to the console or the main log file **without** a timestamp prefix. Unlike [log_message](log_message.md), which prepends a `[HH:MM:SS]` timestamp to every line, `log_pure_message` outputs the message string exactly as provided. This function is used for continuation lines, banners, and other output where a timestamp would be redundant or visually disruptive.

## Syntax

```logging.rs
pub fn log_pure_message(args: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `args` | `&str` | The message string to write. Written verbatim followed by a newline (`writeln!`). |

## Return value

None (`()`).

## Remarks

- The output destination is determined by the [USE_CONSOLE](USE_CONSOLE.md) flag:
  - When `true`, the message is written to `stdout` via `writeln!(stdout(), "{}", args)`.
  - When `false`, the message is written to the main [LOG_FILE](LOG_FILE.md) via `writeln!(get_logger!(), "{}", args)`.
- Unlike [log_message](log_message.md), this function does **not** check [DUST_BIN_MODE](DUST_BIN_MODE.md). Messages passed to `log_pure_message` are always emitted, even during the pre-elevation phase when normal logging is suppressed. Callers that need suppression semantics should check `DUST_BIN_MODE` themselves or use [log_message](log_message.md) / the [log!](log.md) macro instead.
- Write errors from `writeln!` are silently ignored (the `Result` is bound to `let _ = …`). This prevents I/O failures — such as a full disk or a broken pipe — from propagating panics into the service loop.
- The function acquires two mutex locks in sequence during a single call: first [USE_CONSOLE](USE_CONSOLE.md) (via [get_use_console!](get_use_console.md)), and then either `stdout` or [LOG_FILE](LOG_FILE.md) (via [get_logger!](get_logger.md)). The `USE_CONSOLE` guard is dropped before the file write occurs because the `if *get_use_console!()` temporary is evaluated and released before the branch body executes.

### Typical use cases

- **Startup banners:** Multi-line service identification output where only the first line carries a timestamp.
- **Continuation output:** Supplementary detail lines that follow a timestamped header line produced by [log_message](log_message.md).
- **Structured output blocks:** Configuration dumps, rule listings, or other formatted blocks where per-line timestamps would harm readability.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Callers | [main](../main.rs/README.md), [apply module](../apply.rs/README.md) |
| Callees | [get_use_console!](get_use_console.md), [get_logger!](get_logger.md) |
| Reads | [USE_CONSOLE](USE_CONSOLE.md), [LOG_FILE](LOG_FILE.md) |
| Does **not** read | [DUST_BIN_MODE](DUST_BIN_MODE.md), [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |

## See Also

| Topic | Link |
|-------|------|
| Timestamped log output | [log_message](log_message.md) |
| Convenience logging macro (timestamped) | [log!](log.md) |
| Find-mode log output | [log_to_find](log_to_find.md) |
| Console vs. file routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| Log suppression flag (not checked here) | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Main log file handle | [LOG_FILE](LOG_FILE.md) |
| logging module overview | [logging module](README.md) |