# log_process_find function (logging.rs)

Logs a discovered process name to the `.find.log` file with deduplication, ensuring each process name is recorded at most once per session.

## Syntax

```rust
pub fn log_process_find(process_name: &str)
```

## Parameters

`process_name`

The name of the discovered process (e.g., `"game.exe"`) to log to the find log file. This is the process display name as it appears in the system process list.

## Return value

This function does not return a value.

## Remarks

`log_process_find` is the high-level function for recording process discovery events to the [`FIND_LOG_FILE`](FIND_LOG_FILE.md). It wraps [`log_to_find`](log_to_find.md) with a deduplication check against the [`FINDS_SET`](FINDS_SET.md) static to ensure each process name is written at most once per application session.

The function performs the following steps:

1. Acquire a lock on [`FINDS_SET`](FINDS_SET.md).
2. Check whether `process_name` is already present in the set.
3. If **already present**, return immediately without writing to the find log — the process has already been recorded.
4. If **not present**, insert the process name into the set and call [`log_to_find`](log_to_find.md) to write it to the [`FIND_LOG_FILE`](FIND_LOG_FILE.md).

This deduplication is essential because the main loop in [`main`](../main.rs/main.md) discovers running processes every iteration. Without it, the find log would contain repeated entries for the same process on every loop cycle, making the output difficult to review and significantly larger than necessary.

### Difference from log_to_find

| Function | Deduplication | Typical use |
| --- | --- | --- |
| [`log_to_find`](log_to_find.md) | None — writes every call | Raw messages, headers, metadata |
| **log_process_find** | Via [`FINDS_SET`](FINDS_SET.md) — writes once per process name | Process name discovery logging |

Use `log_process_find` when recording a matched process name. Use [`log_to_find`](log_to_find.md) when writing auxiliary information to the find log that does not need deduplication.

### Session scope

The [`FINDS_SET`](FINDS_SET.md) is never cleared during the lifetime of the application. This means that if a process exits and is later relaunched, it will **not** be re-logged in the same session. A new session (restart of AffinityService) resets the set, allowing all processes to be discovered and logged fresh.

### Find log consumption

The `.find.log` file populated by this function is consumed by the [`process_logs`](../main.rs/process_logs.md) function, which reads the discovered process names and uses `es.exe` (Everything search) to locate the full executable paths on disk.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source lines** | L214–L223 |
| **Called by** | [`main`](../main.rs/main.md) loop process matching, find mode |
| **Calls** | [`log_to_find`](log_to_find.md) |
| **Uses** | [`FINDS_SET`](FINDS_SET.md) |

## See also

- [log_to_find function](log_to_find.md)
- [FINDS_SET static](FINDS_SET.md)
- [FIND_LOG_FILE static](FIND_LOG_FILE.md)
- [FINDS_FAIL_SET static](FINDS_FAIL_SET.md)
- [process_logs function](../main.rs/process_logs.md)
- [logging.rs module overview](README.md)