# apply_priority function (apply.rs)

Reads the current process priority class and, if it differs from the configured value, sets it to the desired level. Changes and errors are recorded in the provided [`ApplyConfigResult`](ApplyConfigResult.md).

## Syntax

```AffinityServiceRust/src/apply.rs#L85-91
pub fn apply_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid: u32`

The process identifier of the target process.

`config: &ProcessLevelConfig`

The [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) containing the desired [`ProcessPriority`](../priority.rs/ProcessPriority.md) value. If `config.priority` does not map to a valid Windows constant (`as_win_const()` returns `None`), the function returns immediately with no effect.

`dry_run: bool`

When **true**, the function records what *would* change in `apply_config_result` without calling any Windows API. When **false**, the change is applied to the live process.

`process_handle: &ProcessHandle`

A [`ProcessHandle`](../winapi.rs/ProcessHandle.md) for the target process. Both a read handle (to query current priority) and a write handle (to set new priority) are extracted via [`get_handles`](get_handles.md).

`apply_config_result: &mut ApplyConfigResult`

Accumulator for change descriptions and error messages. See [`ApplyConfigResult`](ApplyConfigResult.md).

## Return value

This function does not return a value. Results are communicated through `apply_config_result`.

## Remarks

The function follows a read-compare-write pattern:

1. **Extract handles** — Calls [`get_handles`](get_handles.md) to obtain read and write `HANDLE` values. If either is `None`, the function returns silently (the process handle was not successfully opened).
2. **Check config** — If `config.priority` is `ProcessPriority::None` (i.e., `as_win_const()` returns `None`), no action is taken.
3. **Read current** — Calls `GetPriorityClass` with the read handle to obtain the current priority class.
4. **Compare** — If the current value already matches the configured value, no change is needed.
5. **Write new** — In dry-run mode, the change message is recorded without calling any API. Otherwise, `SetPriorityClass` is called with the write handle.
6. **Log outcome** — On success, a change message of the form `"Priority: <old> -> <new>"` is added. On failure, [`log_error_if_new`](log_error_if_new.md) records the error only if this specific (pid, operation, error_code) combination has not been logged before.

### Error handling

Errors from `SetPriorityClass` are captured via `GetLastError` and passed through [`log_error_if_new`](log_error_if_new.md) with operation `Operation::SetPriorityClass`. Duplicate errors for the same process and error code are suppressed to avoid log spam.

### Platform notes

- `GetPriorityClass` and `SetPriorityClass` are Win32 Thread API functions. The write handle typically requires `PROCESS_SET_INFORMATION` access right.
- Priority class values are defined by Windows (e.g., `IDLE_PRIORITY_CLASS`, `NORMAL_PRIORITY_CLASS`, `HIGH_PRIORITY_CLASS`, `REALTIME_PRIORITY_CLASS`, etc.).

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Callers** | Main enforcement loop (via `src/main.rs`) |
| **Callees** | [`get_handles`](get_handles.md), [`log_error_if_new`](log_error_if_new.md), `GetPriorityClass`, `SetPriorityClass`, `GetLastError` |
| **Win32 API** | [`GetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass), [`SetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass) |
| **Privileges** | `PROCESS_QUERY_LIMITED_INFORMATION` (read), `PROCESS_SET_INFORMATION` (write) |

## See Also

| Topic | Description |
|---|---|
| [`ApplyConfigResult`](ApplyConfigResult.md) | Accumulator for changes and errors |
| [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) | Configuration struct containing the target priority |
| [`ProcessPriority`](../priority.rs/ProcessPriority.md) | Enum mapping friendly names to Windows priority class constants |
| [`apply_affinity`](apply_affinity.md) | Companion function that applies CPU affinity masks |
| [`apply_io_priority`](apply_io_priority.md) | Companion function that applies IO priority |
| [`apply_memory_priority`](apply_memory_priority.md) | Companion function that applies memory priority |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*