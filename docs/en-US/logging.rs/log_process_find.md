# log_process_find function (logging.rs)

Logs a discovered process name to the find log file, deduplicated so that each process name is recorded at most once per session.

## Syntax

```rust
#[inline]
pub fn log_process_find(process_name: &str)
```

## Parameters

`process_name: &str`

The executable name of the discovered process (e.g., `"notepad.exe"`).

## Return value

This function does not return a value.

## Remarks

This function is the public entry point for find-mode process logging. It acquires a lock on the global `FINDS_SET` and attempts to insert `process_name`. If the insertion succeeds (the name was not already present), the function delegates to [log_to_find](log_to_find.md) with the formatted message `"find {process_name}"`. If the name was already recorded, the call is a no-op.

### Deduplication scope

The `FINDS_SET` static is initialized once at program start and is never cleared during the lifetime of the process. This means deduplication spans the entire service session — a process name discovered at minute 1 will not be logged again at minute 60, even if the process was restarted in between. The set resets only when the service itself restarts, which also rotates the log file by date.

### Output format

The message written to the find log has the form:

```
[HH:MM:SS]find notepad.exe
```

The timestamp prefix is added by [log_to_find](log_to_find.md), not by this function.

### Thread safety

The function acquires `FINDS_SET`'s `Mutex` lock. The lock is held only for the duration of the `HashSet::insert` call; the subsequent [log_to_find](log_to_find.md) call occurs after the lock is released (implicit drop at the end of the `if` condition).

### Console mode

When `USE_CONSOLE` is `true`, find log output is redirected to stdout via [log_to_find](log_to_find.md). This is typically the case when the tool is run interactively with the `-find` CLI flag.

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | Main polling loop (find-mode path), [scheduler](../scheduler.rs/README.md) |
| **Callees** | [log_to_find](log_to_find.md) |
| **Statics** | `FINDS_SET` (`Lazy<Mutex<HashSet<String>>>`) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Find log writer | [log_to_find](log_to_find.md) |
| General log function | [log_message](log_message.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*