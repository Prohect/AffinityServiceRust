# FINDS_SET static (logging.rs)

Tracks process names that have already been logged to the `.find.log` file, preventing duplicate entries when the same process is discovered across multiple loop iterations.

## Syntax

```rust
static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## Members

The static holds a `HashSet<String>` behind a `Mutex`. Each entry is a process name (e.g., `"game.exe"`) that has already been written to the find log.

## Remarks

When [`log_process_find`](log_process_find.md) is called with a process name, it first checks whether `FINDS_SET` already contains that name. If it does, the function returns immediately without writing to the find log. If it does not, the name is inserted into the set and a new entry is written to the [`FIND_LOG_FILE`](FIND_LOG_FILE.md).

This deduplication is important because the main loop in [`main`](../main.rs/main.md) discovers running processes every iteration. Without `FINDS_SET`, the find log would contain repeated entries for every process found on every loop cycle, making it difficult to review which unique processes were detected during a session.

The set is never cleared during the lifetime of the application — once a process name has been logged, it remains in the set permanently. This means a process that exits and restarts will not be re-logged in the same session.

### Thread safety

All access to the `HashSet` is synchronized through the `Mutex`. The lock is acquired, the check-and-insert is performed, and the lock is released before any I/O operations on the find log file.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L11 |
| **Used by** | [`log_process_find`](log_process_find.md) |

## See also

- [log_process_find](log_process_find.md)
- [FIND_LOG_FILE static](FIND_LOG_FILE.md)
- [FINDS_FAIL_SET static](FINDS_FAIL_SET.md)
- [logging.rs module overview](README.md)