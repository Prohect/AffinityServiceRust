# apply_priority function (apply.rs)

Sets the process priority class for a target process.

## Syntax

```rust
pub fn apply_priority(
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

Reference to a [ProcessConfig](../config.rs/ProcessConfig.md) containing the desired priority class in `config.priority`.

`dry_run`

When `true`, the function records what changes *would* be made without calling any Windows APIs to apply them.

`process_handle`

Reference to a [ProcessHandle](../winapi.rs/ProcessHandle.md) providing read and write access to the target process. Both a read handle (for querying current priority) and a write handle (for setting new priority) are required.

`apply_config_result`

Mutable reference to an [ApplyConfigResult](ApplyConfigResult.md) that collects change descriptions and error messages produced during the operation.

## Return value

This function does not return a value. Results are communicated through `apply_config_result`.

## Remarks

The function first extracts read and write handles via [get_handles](get_handles.md). If either handle is unavailable, the function returns immediately without action.

If `config.priority` is `ProcessPriority::None`, no action is taken because `as_win_const()` returns `None`.

The function queries the current priority class with `GetPriorityClass` and compares it to the configured target. If they already match, no change is made.

**Dry-run mode:** When `dry_run` is `true`, the change message is recorded but `SetPriorityClass` is not called.

**Change logged:** `"Priority: {old} -> {new}"` where both values are human-readable priority names (e.g. `Normal`, `High`, `AboveNormal`).

**Error handling:** If `SetPriorityClass` fails, the Win32 error code is retrieved via `GetLastError` and passed to [log_error_if_new](log_error_if_new.md) for deduplicated error logging. The error is only recorded once per pid/operation combination.

### Priority values

The supported priority classes are defined in [ProcessPriority](../priority.rs/ProcessPriority.md): `Idle`, `BelowNormal`, `Normal`, `AboveNormal`, `High`, and `Realtime`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Source lines** | L83–L129 |
| **Called by** | [apply_config](../main.rs/apply_config.md) |
| **Calls** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md) |
| **Windows API** | [GetPriorityClass](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass), [SetPriorityClass](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass), [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |

## See also

- [apply_affinity](apply_affinity.md)
- [apply_io_priority](apply_io_priority.md)
- [apply_memory_priority](apply_memory_priority.md)
- [ProcessPriority enum](../priority.rs/ProcessPriority.md)