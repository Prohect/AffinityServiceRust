# apply_priority function (apply.rs)

Reads the current process priority class and, if it differs from the configured target, sets it to the desired value. In dry-run mode the change is recorded without calling the Windows API. Errors are deduplicated via `log_error_if_new` so that repeated failures for the same process/operation/error-code combination do not generate duplicate log entries.

## Syntax

```AffinityServiceRust/src/apply.rs#L85-131
pub fn apply_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process. Used for error deduplication and log messages. |
| `config` | `&ProcessLevelConfig` | The process-level configuration containing the desired `priority` value (a `ProcessPriority` enum). If `config.priority` does not map to a Windows constant (i.e., `as_win_const()` returns `None`), the function returns immediately without action. |
| `dry_run` | `bool` | When `true`, the function records what *would* change in `apply_config_result` without calling `SetPriorityClass`. When `false`, the Windows API is called to apply the change. |
| `process_handle` | `&ProcessHandle` | A handle wrapper providing read and write access to the target process. The function extracts a read handle (for `GetPriorityClass`) and a write handle (for `SetPriorityClass`) via [`get_handles`](get_handles.md). If either handle is unavailable, the function returns immediately. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages. On success (or dry-run), a change string of the form `"Priority: <old> -> <new>"` is appended. On failure, an error string is appended (subject to deduplication). |

## Return value

This function does not return a value. All outcomes are communicated through the `apply_config_result` parameter.

## Remarks

- The function first calls `GetPriorityClass` with the read handle to obtain the current priority class. If the current value already matches the configured target, no action is taken and nothing is recorded.
- The current priority class is decoded back to a human-readable string via `ProcessPriority::from_win_const` for the change message.
- On failure of `SetPriorityClass`, the Win32 error code is retrieved with `GetLastError` and passed to [`log_error_if_new`](log_error_if_new.md), which records the error only if this specific `pid`/`Operation::SetPriorityClass`/error-code triple has not been seen before.
- The Windows priority class constants are standard values such as `IDLE_PRIORITY_CLASS`, `BELOW_NORMAL_PRIORITY_CLASS`, `NORMAL_PRIORITY_CLASS`, `ABOVE_NORMAL_PRIORITY_CLASS`, `HIGH_PRIORITY_CLASS`, and `REALTIME_PRIORITY_CLASS`.
- If `config.priority` is `ProcessPriority::None` (or any variant whose `as_win_const()` returns `None`), the function exits without querying or modifying the process.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows APIs | `GetPriorityClass`, `SetPriorityClass`, `GetLastError` |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that iterates matched processes |
| Callees | [`get_handles`](get_handles.md), [`log_error_if_new`](log_error_if_new.md), `ProcessPriority::as_win_const`, `ProcessPriority::from_win_const`, `error_from_code_win32` |
| Privileges | Requires a process handle with `PROCESS_QUERY_INFORMATION` or `PROCESS_QUERY_LIMITED_INFORMATION` (read) and `PROCESS_SET_INFORMATION` (write). |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| ProcessPriority | [`priority.rs/ProcessPriority`](../priority.rs/ProcessPriority.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*