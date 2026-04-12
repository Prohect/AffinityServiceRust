# log_pure_message function (logging.rs)

Writes a log message to the log file without a timestamp prefix. Used for continuation lines, headers, and other output that should not carry a timestamp.

## Syntax

```rust
pub fn log_pure_message(args: &str)
```

## Parameters

`args`

The message string to write to the log file. This string is written as-is without any timestamp or prefix formatting.

## Return value

This function does not return a value.

## Remarks

Unlike [`log_message`](log_message.md), which prepends a timestamp from [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md) to every line, `log_pure_message` writes the provided text directly to the [`LOG_FILE`](LOG_FILE.md) with no additional formatting. This is useful for multi-line output where only the first line should carry a timestamp, or for section headers and separators in the log file.

The function respects [`DUST_BIN_MODE`](DUST_BIN_MODE.md) — when dust bin mode is active, the message is silently discarded without being written to the file or console. This ensures consistent suppression of all log output during the pre-UAC-elevation phase.

If [`USE_CONSOLE`](USE_CONSOLE.md) is enabled, the message is also printed to stdout without a timestamp prefix, matching the file output format.

The function acquires the [`LOG_FILE`](LOG_FILE.md) mutex for the duration of the write operation, ensuring that the output is not interleaved with concurrent writes from [`log_message`](log_message.md).

### Typical usage

`log_pure_message` is commonly used for:

- Continuation lines in multi-line log entries (e.g., listing multiple changes for a single process)
- Banner or separator lines at startup
- Config dump output where timestamps would be distracting

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source lines** | L197–L203 |
| **Writes to** | [`LOG_FILE`](LOG_FILE.md) |
| **Respects** | [`DUST_BIN_MODE`](DUST_BIN_MODE.md), [`USE_CONSOLE`](USE_CONSOLE.md) |

## See also

- [log_message function](log_message.md)
- [LOG_FILE static](LOG_FILE.md)
- [DUST_BIN_MODE static](DUST_BIN_MODE.md)
- [logging.rs module overview](README.md)