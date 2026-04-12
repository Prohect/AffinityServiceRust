# FIND_LOG_FILE static (logging.rs)

Lazy-initialized, mutex-guarded file handle for the `.find.log` file, which records discovered process names during find mode and normal operation.

## Syntax

```rust
static FIND_LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(/* opens find log file */));
```

## Members

The static holds a `File` behind a `Mutex`, providing synchronized write access to the find log file from any point in the application.

## Remarks

The `FIND_LOG_FILE` is separate from the main [`LOG_FILE`](LOG_FILE.md) and is dedicated to recording process discovery output. When the application encounters a running process that matches a configuration rule, the process name is written to this file via [`log_to_find`](log_to_find.md) and [`log_process_find`](log_process_find.md).

The file is created lazily on first access using [`get_log_path`](get_log_path.md) with a `.find.log` suffix. It is placed alongside the main executable in the same directory as the primary log file.

The find log serves as input for the [`process_logs`](../main.rs/process_logs.md) feature, which reads `.find.log` files and uses `es.exe` (Everything search) to discover the full executable paths of found processes.

### Deduplication

Writes to this file are deduplicated by the [`FINDS_SET`](FINDS_SET.md) static — [`log_process_find`](log_process_find.md) checks whether a process name has already been logged before writing to the file, ensuring each process name appears at most once per session.

### Dust bin mode

When [`DUST_BIN_MODE`](DUST_BIN_MODE.md) is enabled (pre-UAC-elevation phase), writes to this file are suppressed along with all other log output, since the file will be abandoned when the non-elevated process exits.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L67 |
| **Written by** | [`log_to_find`](log_to_find.md), [`log_process_find`](log_process_find.md) |
| **Read by** | [`process_logs`](../main.rs/process_logs.md) (external consumer of the file) |

## See also

- [LOG_FILE static](LOG_FILE.md)
- [FINDS_SET static](FINDS_SET.md)
- [log_to_find function](log_to_find.md)
- [log_process_find function](log_process_find.md)
- [logging.rs module overview](README.md)