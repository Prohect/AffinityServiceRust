# log_to_find function (logging.rs)

Writes a timestamped message to the find-mode log file (or to the console if [USE_CONSOLE](USE_CONSOLE.md) is active). This function is the primary output mechanism for `-find` mode diagnostics and is called by [log_process_find](log_process_find.md) as well as directly from the find-mode processing loop. Unlike [log_message](log_message.md), this function does **not** check [DUST_BIN_MODE](DUST_BIN_MODE.md) — find-mode output is never suppressed.

## Syntax

```logging.rs
pub fn log_to_find(msg: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `msg` | `&str` | The message string to write. The function prepends a `[HH:MM:SS]` timestamp prefix before writing. |

## Return value

None (`()`). I/O errors are silently ignored — the function uses `let _ = writeln!(…)` to discard any write failures.

## Remarks

The function's behavior depends on the [USE_CONSOLE](USE_CONSOLE.md) flag:

- **Console mode (`true`):** The timestamped message is written to `stdout` via `writeln!(stdout(), "[{}]{}", time_prefix, msg)`.
- **File mode (`false`):** The timestamped message is written to the [FIND_LOG_FILE](FIND_LOG_FILE.md) handle via `writeln!(get_logger_find!(), "[{}]{}", time_prefix, msg)`.

### Timestamp source

The timestamp prefix is derived from the cached [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) value, formatted as `%H:%M:%S` (e.g., `14:32:07`). The main service loop updates this buffer at the start of each iteration, so all find-mode log entries within a single pass share the same timestamp.

### Differences from log_message

| Aspect | `log_to_find` | [log_message](log_message.md) |
|--------|---------------|-------------------------------|
| Output destination | [FIND_LOG_FILE](FIND_LOG_FILE.md) (`logs/YYYYMMDD.find.log`) | [LOG_FILE](LOG_FILE.md) (`logs/YYYYMMDD.log`) |
| Dust-bin suppression | No — always writes | Yes — suppressed when [DUST_BIN_MODE](DUST_BIN_MODE.md) is `true` |
| Purpose | Find-mode process discovery diagnostics | General service diagnostics |

### Output format

A call such as `log_to_find("find notepad.exe")` produces the following log line:

```/dev/null/example.log#L1-1
[14:32:07]find notepad.exe
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Callers | [log_process_find](log_process_find.md), [process_find](../main.rs/README.md) |
| Callees | [get_local_time!](get_local_time.md), [get_use_console!](get_use_console.md), [get_logger_find!](get_logger_find.md) |
| Reads | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md), [USE_CONSOLE](USE_CONSOLE.md), [FIND_LOG_FILE](FIND_LOG_FILE.md) |

## See Also

| Topic | Link |
|-------|------|
| Deduplicated process discovery logging | [log_process_find](log_process_find.md) |
| Main log writing function | [log_message](log_message.md) |
| Raw (untimestamped) log writing | [log_pure_message](log_pure_message.md) |
| Find-mode log file handle | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Console vs. file routing flag | [USE_CONSOLE](USE_CONSOLE.md) |
| Cached timestamp for log prefixes | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd