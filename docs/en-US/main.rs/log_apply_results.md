# log_apply_results function (main.rs)

Formats and emits log output for the changes and errors produced by a single configuration-application pass on one process. Errors are forwarded to the find-log sink, while successfully applied changes are written to the main log with aligned, multi-line formatting.

## Syntax

```rust
fn log_apply_results(pid: &u32, name: &String, result: ApplyConfigResult)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `&u32` | The Windows process ID that was just configured. Used as part of the log prefix for identification. |
| `name` | `&String` | The executable name of the process (e.g. `"game.exe"`). Displayed alongside the PID in the log prefix. |
| `result` | `ApplyConfigResult` | The accumulated result struct from `apply_process_level` and/or `apply_thread_level`, containing vectors of human-readable change descriptions and error strings. Consumed by this function. |

## Return value

This function does not return a value.

## Remarks

- The function is a no-op when `result.is_empty()` returns `true` (i.e. no changes and no errors were recorded).
- All strings in `result.errors` are forwarded to `log_to_find`, which writes them to the `.find.log` file for later analysis by the `-processLogs` mode.
- The first change string is logged with a formatted prefix of `"{pid}::{name}::{change}"`. Subsequent change strings are indented to align with the first change text, accounting for both the prefix width and the 10-character timestamp prefix (e.g. `[04:55:16]`) that the logging subsystem prepends.
- The alignment logic computes padding as `prefix_length - first_change_length + 10`, ensuring that multi-line output for a single process reads as a visually grouped block in the log file.
- This function takes ownership of `result` and drops it after processing.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main.rs` |
| Callers | [apply_config](apply_config.md), thread-level apply loop in [main](main.md) |
| Callees | `logging::log_to_find`, `logging::log_message`, `logging::log_pure_message`, `ApplyConfigResult::is_empty` |
| API | None (internal logging only) |
| Privileges | None |

## See Also

| Reference | Link |
|-----------|------|
| apply_config | [apply_config](apply_config.md) |
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| main | [main](main.md) |
| logging module | [logging](../logging.rs/README.md) |

---
Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
