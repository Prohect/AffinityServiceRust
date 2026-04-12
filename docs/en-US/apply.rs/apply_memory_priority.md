# apply_memory_priority function (apply.rs)

Sets the memory page priority for a target process. Memory priority influences how quickly the operating system reclaims pages belonging to the process under memory pressure—lower priority pages are reclaimed first.

## Syntax

```rust
pub fn apply_memory_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process identifier of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing the desired `memory_priority` setting. If `config.memory_priority` is `None`, the function returns immediately without action.

`dry_run`

When `true`, the function records what change *would* be made in `apply_config_result` without calling any Windows APIs to modify the process.

`process_handle`

Reference to the [ProcessHandle](../winapi.rs/ProcessHandle.md) providing read and write access to the target process. Both a read handle (for querying) and a write handle (for setting) are required; the function returns immediately if either is unavailable.

`apply_config_result`

Mutable reference to the [ApplyConfigResult](ApplyConfigResult.md) that accumulates change descriptions and error messages.

## Return value

This function does not return a value. Results are reported through `apply_config_result`.

## Remarks

The function uses the `GetProcessInformation` and `SetProcessInformation` APIs with the `ProcessMemoryPriority` information class and a `MEMORY_PRIORITY_INFORMATION` structure (wrapped as `MemoryPriorityInformation(u32)` in the codebase).

### Workflow

1. Calls [get_handles](get_handles.md) to extract read and write `HANDLE` values from `process_handle`. Returns early if either handle is missing.
2. Checks whether `config.memory_priority` resolves to a valid Windows constant via `as_win_const()`. If `None`, the function exits without action.
3. Queries the current memory priority with `GetProcessInformation(r_handle, ProcessMemoryPriority, ...)`.
   - On failure, logs the error via [log_error_if_new](log_error_if_new.md) using `Operation::GetProcessInformation2ProcessMemoryPriority` and returns.
4. Compares the current memory priority value against the target. If they are equal, no action is taken.
5. If `dry_run` is `true`, records the intended change and returns.
6. Otherwise, calls `SetProcessInformation(w_handle, ProcessMemoryPriority, ...)` with the target priority.
   - On failure, logs the error via [log_error_if_new](log_error_if_new.md) using `Operation::SetProcessInformation2ProcessMemoryPriority`.
   - On success, records the change.

### Change logged

```
Memory Priority: {old} -> {new}
```

Where `{old}` and `{new}` are the human-readable names of the `MemoryPriority` enum variants (e.g., `Normal`, `Low`, `VeryLow`).

### Valid memory priority levels

| Level | Description |
| --- | --- |
| **VeryLow** | Pages reclaimed first under memory pressure |
| **Low** | Low priority pages |
| **Medium** | Medium priority pages |
| **BelowNormal** | Below default priority |
| **Normal** | Default memory priority |

See [MemoryPriority](../priority.rs/MemoryPriority.md) for the full enum definition.

### Error handling

All errors are deduplicated through [log_error_if_new](log_error_if_new.md), which uses `logging::is_new_error()` to ensure each unique pid/operation/error-code combination is logged only once.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/apply.rs` |
| **Lines** | L508–L595 |
| **Called by** | [apply_config](../main.rs/apply_config.md) in `main.rs` |
| **Calls** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md) |
| **Windows API** | `GetProcessInformation` (`ProcessMemoryPriority`), `SetProcessInformation` (`ProcessMemoryPriority`) |

## See also

- [ApplyConfigResult](ApplyConfigResult.md)
- [apply_priority](apply_priority.md)
- [apply_io_priority](apply_io_priority.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)
- [MemoryPriority](../priority.rs/MemoryPriority.md)