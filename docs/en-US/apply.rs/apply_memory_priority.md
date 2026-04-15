# apply_memory_priority function (apply.rs)

The `apply_memory_priority` function reads the current process memory priority via `GetProcessInformation` with the `ProcessMemoryPriority` information class and, if it differs from the configured target, sets the new value via `SetProcessInformation`. In dry-run mode, the intended change is recorded without calling any state-modifying APIs.

## Syntax

```AffinityServiceRust/src/apply.rs#L490-498
pub fn apply_memory_priority(
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
| `pid` | `u32` | The process identifier of the target process. Used for error deduplication keys and log messages. |
| `config` | `&ProcessLevelConfig` | The process-level configuration containing the desired `memory_priority` value (a `MemoryPriority` enum). If `config.memory_priority.as_win_const()` returns `None` (e.g., `MemoryPriority::None`), the function returns immediately without querying or modifying the process. |
| `dry_run` | `bool` | When `true`, the function records what *would* change in `apply_config_result` without calling `SetProcessInformation`. When `false`, the Windows API is called to apply the change. |
| `process_handle` | `&ProcessHandle` | A handle wrapper providing read and write access to the target process. The function extracts a read handle (for `GetProcessInformation`) and a write handle (for `SetProcessInformation`) via [`get_handles`](get_handles.md). If either handle is unavailable, the function returns early. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during execution. |

## Return value

This function does not return a value. All outcomes are communicated through the `apply_config_result` parameter.

## Remarks

- The function first calls `GetProcessInformation` with the `ProcessMemoryPriority` information class and a `MemoryPriorityInformation` struct to retrieve the current memory priority level. If this query fails, the Win32 error code is retrieved via `GetLastError` and logged through [`log_error_if_new`](log_error_if_new.md) with `Operation::GetProcessInformation2ProcessMemoryPriority`. No further action is taken after a query failure.
- If the current memory priority already matches the target, no change is recorded and the function returns silently.
- When applying the change (non-dry-run), `SetProcessInformation` is called with a new `MemoryPriorityInformation` struct containing the target value. On failure, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::SetProcessInformation2ProcessMemoryPriority`.
- The change message is formatted as `"Memory Priority: <current> -> <target>"` using the human-readable string representations from `MemoryPriority::from_win_const` and `config.memory_priority.as_str()`.
- The `MemoryPriorityInformation` wrapper type is a newtype around `u32` defined in the `priority` module. It matches the layout of the Windows `MEMORY_PRIORITY_INFORMATION` structure.
- Windows memory priority values range from 0 (lowest/very low) through 5 (normal). The `MemoryPriority` enum in the `priority` module maps user-facing names to these numeric constants.


## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows APIs | `GetProcessInformation` (`ProcessMemoryPriority`), `SetProcessInformation` (`ProcessMemoryPriority`), `GetLastError` |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that iterates matched processes |
| Callees | [`get_handles`](get_handles.md), [`log_error_if_new`](log_error_if_new.md), `MemoryPriority::as_win_const`, `MemoryPriority::from_win_const`, `MemoryPriority::as_str`, `error_from_code_win32` |
| Privileges | Requires a process handle with `PROCESS_QUERY_INFORMATION` or `PROCESS_QUERY_LIMITED_INFORMATION` (read) and `PROCESS_SET_INFORMATION` (write). |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| MemoryPriority | [`priority.rs/MemoryPriority`](../priority.rs/MemoryPriority.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*