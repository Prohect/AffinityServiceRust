# log_to_find function (logging.rs)

Writes a message directly to the `.find.log` file, bypassing the normal log message formatting and deduplication.

## Syntax

```rust
pub fn log_to_find(msg: &str)
```

## Parameters

`msg`

The message string to write to the find log file. This is written as-is with a newline appended.

## Return value

This function does not return a value.

## Remarks

`log_to_find` is a low-level function that writes directly to the [`FIND_LOG_FILE`](FIND_LOG_FILE.md) handle. Unlike [`log_process_find`](log_process_find.md), it does **not** perform deduplication against [`FINDS_SET`](FINDS_SET.md) — the caller is responsible for ensuring that duplicate messages are not written if deduplication is desired.

The function acquires a lock on the [`FIND_LOG_FILE`](FIND_LOG_FILE.md) mutex, writes the message followed by a newline, and releases the lock. If [`DUST_BIN_MODE`](DUST_BIN_MODE.md) is active, the write may be suppressed along with all other log output.

This function is used for writing raw messages to the find log that are not necessarily process names — for example, header lines, timestamps, or other metadata that should appear in the find log output. For logging discovered process names with built-in deduplication, use [`log_process_find`](log_process_find.md) instead.

### Difference from log_process_find

| Function | Deduplication | Typical use |
| --- | --- | --- |
| **log_to_find** | None — writes every call | Raw messages, headers, metadata |
| [`log_process_find`](log_process_find.md) | Via [`FINDS_SET`](FINDS_SET.md) — writes once per process name | Process name discovery logging |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source lines** | L205–L212 |
| **Writes to** | [`FIND_LOG_FILE`](FIND_LOG_FILE.md) |
| **Called by** | [`main`](../main.rs/main.md) find mode, [`log_process_find`](log_process_find.md) |

## See also

- [log_process_find function](log_process_find.md)
- [FIND_LOG_FILE static](FIND_LOG_FILE.md)
- [FINDS_SET static](FINDS_SET.md)
- [logging.rs module overview](README.md)