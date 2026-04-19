# log_to_find function (logging.rs)

Writes a timestamped message to the find-mode log file, or to the console when console output is enabled.

## Syntax

```rust
pub fn log_to_find(msg: &str)
```

## Parameters

`msg: &str`

The message string to log. A timestamp prefix `[HH:MM:SS]` is automatically prepended to the output.

## Return value

This function does not return a value.

## Remarks

This function is the dedicated logging sink for find-mode output. It writes to a separate log file (`YYYYMMDD.find.log`) rather than the main application log, keeping find-mode discoveries isolated for easy review.

### Output routing

| Condition | Destination |
|-----------|-------------|
| `USE_CONSOLE` is `true` | Standard output via `stdout` |
| `USE_CONSOLE` is `false` | Find log file obtained via `get_logger_find!()` |

### Differences from log_message

- **No dust-bin check:** Unlike [log_message](log_message.md), this function does **not** check the `DUST_BIN_MODE` flag. Find-mode logging is never suppressed by the dust-bin mechanism.
- **Separate log file:** Output goes to the find log file (`YYYYMMDD.find.log`) instead of the main log file (`YYYYMMDD.log`).

### Timestamp format

The timestamp is formatted as `[HH:MM:SS]` using the cached `LOCAL_TIME_BUFFER`. The final output line has the form:

```
[14:32:07]find chrome.exe
```

### Error handling

Write failures are silently ignored. The function uses `let _ = writeln!(...)` to discard any I/O errors.

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | [log_process_find](log_process_find.md) |
| **Callees** | `get_local_time!()`, `get_use_console!()`, `get_logger_find!()` |
| **Statics accessed** | `LOCAL_TIME_BUFFER`, `USE_CONSOLE`, `FIND_LOG_FILE` |
| **Privileges** | None (file must already be open) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Find-mode dedup wrapper | [log_process_find](log_process_find.md) |
| Main log function | [log_message](log_message.md) |
| Log file path construction | [get_log_path](get_log_path.md) |
| Raw logging without timestamp | [log_pure_message](log_pure_message.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*