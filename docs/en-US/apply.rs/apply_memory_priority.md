# apply_memory_priority function (apply.rs)

Sets the memory priority of a process to the value specified in the configuration, using the documented `GetProcessInformation` / `SetProcessInformation` Windows API with the `ProcessMemoryPriority` information class.

## Syntax

```AffinityServiceRust/src/apply.rs#L491-498
pub fn apply_memory_priority(
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

The [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) containing the desired `memory_priority` setting. If `memory_priority` is `MemoryPriority::None`, the function returns immediately without taking any action.

`dry_run: bool`

When **true**, the function records what change *would* be made in `apply_config_result` without calling any Windows API to modify the process. When **false**, the change is applied.

`process_handle: &ProcessHandle`

A [ProcessHandle](../winapi.rs/ProcessHandle.md) from which read and write `HANDLE` values are extracted via [get_handles](get_handles.md). Both handles are required; if either is unavailable the function returns early.

`apply_config_result: &mut ApplyConfigResult`

An [ApplyConfigResult](ApplyConfigResult.md) accumulator that collects change descriptions and error messages produced during execution.

## Return value

This function does not return a value. Results are communicated through `apply_config_result`.

## Remarks

### Algorithm

1. Extracts read and write OS handles from `process_handle` via [get_handles](get_handles.md). Returns early if either handle is missing.
2. Checks whether `config.memory_priority` maps to a valid Windows constant. If the configured value is `None`, the function is a no-op.
3. Queries the current memory priority by calling `GetProcessInformation` with `ProcessMemoryPriority` and a `MemoryPriorityInformation` struct.
4. If the query fails, logs an error via [log_error_if_new](log_error_if_new.md) with `Operation::GetProcessInformation2ProcessMemoryPriority` and returns.
5. Compares the current value with the target. If they already match, no action is taken.
6. In dry-run mode, records the intended change and returns.
7. Constructs a new `MemoryPriorityInformation` with the target value and calls `SetProcessInformation` with `ProcessMemoryPriority`.
8. On success, records the change as `"Memory Priority: <old> -> <new>"`.
9. On failure, logs the Win32 error via [log_error_if_new](log_error_if_new.md) with `Operation::SetProcessInformation2ProcessMemoryPriority`.

### Memory priority levels

Memory priority controls how aggressively the Memory Manager trims and repurposes a process's pages under memory pressure. The Windows-defined levels correspond to the values in [MemoryPriority](../priority.rs/MemoryPriority.md):

| Level | Numeric value | Behaviour |
|---|---|---|
| VeryLow | 1 | Pages are the first to be trimmed and repurposed. |
| Low | 2 | Pages are trimmed before Normal but after VeryLow. |
| MediumLow | 3 | Intermediate priority. |
| Medium | 4 | Intermediate priority. |
| Normal | 5 | Default priority — pages are the last to be trimmed. |

### MemoryPriorityInformation wrapper

The function uses a `MemoryPriorityInformation(u32)` newtype wrapper around the raw `MEMORY_PRIORITY_INFORMATION` value to interface with the Windows `ProcessMemoryPriority` information class. This keeps the struct layout compatible with what `GetProcessInformation` / `SetProcessInformation` expect.

### Error handling

Errors are reported through [log_error_if_new](log_error_if_new.md), which deduplicates messages by `(pid, operation, error_code)` to prevent log spam on repeatedly failing processes. Both the query and set paths have independent error logging.

### Platform notes

- This function targets **Windows 8 / Windows Server 2012** and later, where `GetProcessInformation` / `SetProcessInformation` with `ProcessMemoryPriority` are available.
- The calling process must hold appropriate access rights on the target process. A `PROCESS_SET_INFORMATION` right is required for the write handle, and `PROCESS_QUERY_LIMITED_INFORMATION` for the read handle.

## Requirements

| | |
|---|---|
| **Source module** | [apply.rs](README.md) |
| **Callers** | Main apply loop (per-process enforcement cycle) |
| **Callees** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), `GetProcessInformation`, `SetProcessInformation` |
| **Windows API** | `GetProcessInformation` (`ProcessMemoryPriority`), `SetProcessInformation` (`ProcessMemoryPriority`), `GetLastError` |
| **Privileges** | `PROCESS_QUERY_LIMITED_INFORMATION` (read), `PROCESS_SET_INFORMATION` (write) |

## See Also

| Topic | Description |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for changes and errors |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | Per-process configuration struct including `memory_priority` |
| [MemoryPriority](../priority.rs/MemoryPriority.md) | Enum defining memory priority levels |
| [apply_io_priority](apply_io_priority.md) | Companion function that sets IO priority |
| [apply_priority](apply_priority.md) | Companion function that sets process (CPU scheduling) priority |
| [ProcessHandle](../winapi.rs/ProcessHandle.md) | Handle wrapper providing read/write access to a process |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*