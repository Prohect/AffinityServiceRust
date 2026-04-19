# log_apply_results function (main.rs)

Formats and logs the changes and errors collected in an [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) after a configuration application pass for a single process. Errors are routed to the find log; changes are written to the main log with right-aligned, multi-line formatting.

## Syntax

```rust
fn log_apply_results(pid: &u32, name: &String, result: ApplyConfigResult)
```

## Parameters

`pid: &u32`

The process identifier of the target process. Right-padded to 5 characters in the formatted output.

`name: &String`

The process name from the configuration rule that matched this process (e.g., `"game.exe"`). Used as the middle segment of the log line prefix.

`result: ApplyConfigResult`

The [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) accumulator containing changes and errors from the current apply cycle. Consumed by this function.

## Return value

This function does not return a value.

## Remarks

The function exits immediately if `result.is_empty()` returns `true`, producing no log output for processes that required no changes and encountered no errors.

### Error logging

All entries in `result.errors` are written to the find log via `log_to_find`. This separates error output from normal change tracking, allowing errors to be reviewed independently (e.g., during `-process_logs` post-processing).

### Change logging

Changes are formatted with an aligned, multi-line layout:

1. The **first** change is logged as a single line in the format:

   `{pid:>5}::{name}::{change}`

   For example: `" 1234::game.exe::Priority: normal -> high"`

2. **Subsequent** changes are logged with a padding prefix calculated to align them directly under the first change's text. The padding accounts for the `[HH:MM:SS]` time prefix (10 characters) that the logging infrastructure prepends.

This alignment ensures that all changes for a single process appear as a visually grouped block in the log file, improving readability when many settings are applied simultaneously.

### Logging infrastructure

- `log_message` writes to the main log file with a timestamp prefix.
- `log_pure_message` writes to the main log file without adding its own timestamp, relying on the caller-provided padding for alignment.
- `log_to_find` writes to the `.find.log` file used by find-mode post-processing.

## Requirements

| | |
|---|---|
| **Module** | `src/main.rs` |
| **Callers** | [apply_config](apply_config.md), thread-level-only apply path in [main](main.md) |
| **Callees** | `log_to_find`, `log_message`, `log_pure_message` |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Result accumulator | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| Combined apply entry point | [apply_config](apply_config.md) |
| Logging infrastructure | [logging.rs](../logging.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*