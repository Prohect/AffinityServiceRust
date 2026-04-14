# FINDS_SET static (logging.rs)

A lazily-initialized, mutex-guarded `HashSet<String>` that tracks which process names have already been logged during a `-find` mode session. This set provides per-session deduplication so that each discovered process is written to the find log only once, regardless of how many polling intervals encounter it.

## Syntax

```logging.rs
pub static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. The set is created empty on first access. |
| Middle | `Mutex<…>` | Provides interior mutability and thread-safe access from the main service loop. |
| Inner | `HashSet<String>` | Contains the lowercased process names that have already been logged during the current session. |

## Remarks

- The set starts empty and grows monotonically for the lifetime of the process. There is no purge mechanism — entries persist until the service is restarted. This is by design: `-find` mode is a diagnostic tool meant to produce a deduplicated list of all processes observed during a single run.
- [log_process_find](log_process_find.md) is the sole writer. It calls `FINDS_SET.lock().unwrap().insert(process_name)` and only proceeds to write the log line if `insert` returns `true` (i.e., the name was not already present).
- The convenience macro [get_fail_find_set!](get_fail_find_set.md) does **not** lock `FINDS_SET`; it locks the separate [FINDS_FAIL_SET](FINDS_FAIL_SET.md) static. Direct access to `FINDS_SET` is done inline in [log_process_find](log_process_find.md).
- Because `-find` mode typically enumerates on the order of hundreds of distinct process names, the memory footprint of this set is negligible.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex`, `std::collections::HashSet` |
| Written by | [log_process_find](log_process_find.md) |
| Read by | [log_process_find](log_process_find.md) |
| Cleared by | *(never cleared — lifetime matches the process)* |

## See Also

| Topic | Link |
|-------|------|
| Function that writes to this set | [log_process_find](log_process_find.md) |
| Find-mode log file handle | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Dedup set for failed find operations | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| Find-mode entry point | [process_find](../main.rs/README.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd