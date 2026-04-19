# log_pure_message function (logging.rs)

Writes a message to the log output **without** a timestamp prefix. Used for continuation lines, banners, or pre-formatted blocks where the caller manages its own formatting.

## Syntax

```rust
pub fn log_pure_message(args: &str)
```

## Parameters

`args: &str`

The message string to write. A newline is appended automatically via `writeln!`.

## Return value

This function does not return a value.

## Remarks

Unlike [log_message](log_message.md), this function does **not** prepend a `[HH:MM:SS]` timestamp and does **not** check the `DUST_BIN_MODE` flag. Output is always emitted regardless of dust-bin state.

### Output routing

| `USE_CONSOLE` | Destination |
|---------------|-------------|
| `true` | `stdout` |
| `false` | `LOG_FILE` (`logs/YYYYMMDD.log`) |

### Write errors

Errors from `writeln!` are silently discarded (the return value of `writeln!` is assigned to `_`). This prevents cascading failures when the log file is inaccessible.

### Comparison with log_message

| Aspect | `log_message` | `log_pure_message` |
|--------|---------------|-------------------|
| Timestamp prefix | `[HH:MM:SS]` | None |
| Respects `DUST_BIN_MODE` | Yes | No |
| Output destination | Console or file | Console or file |

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | Main loop banner output, multi-line log continuations |
| **Callees** | `get_use_console!`, `get_logger!` |
| **Statics accessed** | `USE_CONSOLE`, `LOG_FILE` |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Timestamped logging | [log_message](log_message.md) |
| Find-mode logging | [log_to_find](log_to_find.md) |
| Log file path computation | [get_log_path](get_log_path.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*