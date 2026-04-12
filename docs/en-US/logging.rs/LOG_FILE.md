# LOG_FILE static (logging.rs)

The main log file handle, lazily initialized on first use. All general log output produced by [`log_message`](log_message.md) is written to this file.

## Syntax

```rust
static LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(/* opens log file via get_log_path */));
```

## Members

The static holds a `std::fs::File` behind a `Mutex<File>`. The file is opened in append mode at the path returned by [`get_log_path`](get_log_path.md) with the default `.log` suffix.

## Remarks

`LOG_FILE` is initialized lazily via `once_cell::sync::Lazy` the first time any code path triggers a log write. The file is opened in create-or-append mode so that log output accumulates across multiple runs without overwriting previous entries.

All writes to `LOG_FILE` are serialized through the `Mutex`, ensuring that concurrent log calls from different parts of the application do not interleave output within a single log line.

When [`DUST_BIN_MODE`](DUST_BIN_MODE.md) is active, log writes are suppressed before they reach this file handle. This prevents writing to a log file that will be abandoned after UAC elevation relaunches the process.

The log file path is determined by [`get_log_path`](get_log_path.md), which constructs a path adjacent to the running executable. The exact file name includes a `.log` suffix (as opposed to `.find.log` used by [`FIND_LOG_FILE`](FIND_LOG_FILE.md)).

### Lifetime

The file handle is held open for the entire lifetime of the process. It is never explicitly closed — the OS reclaims the handle when the process exits. This is intentional, as log output may be needed up until the very last moment of execution.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L65 |
| **Written by** | [`log_message`](log_message.md), [`log_pure_message`](log_pure_message.md) |
| **Initialized via** | [`get_log_path`](get_log_path.md) |

## See also

- [FIND_LOG_FILE static](FIND_LOG_FILE.md)
- [log_message function](log_message.md)
- [log_pure_message function](log_pure_message.md)
- [get_log_path function](get_log_path.md)
- [logging.rs module overview](README.md)