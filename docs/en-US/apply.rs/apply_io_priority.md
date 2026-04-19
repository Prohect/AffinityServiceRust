# apply_io_priority function (apply.rs)

Gets and sets the I/O priority of a process using the undocumented `NtQueryInformationProcess` and `NtSetInformationProcess` native API with information class `ProcessInformationClassIOPriority` (33).

## Syntax

```AffinityServiceRust/src/apply.rs#L403-410
pub fn apply_io_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process identifier of the target process.

`config`

Reference to the [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) containing the desired `io_priority` setting. If `io_priority` is `IOPriority::None`, the function returns immediately without action.

`dry_run`

If **true**, the function records what changes would be made to [ApplyConfigResult](ApplyConfigResult.md) without calling any Windows APIs to modify state.

`process_handle`

Reference to the [ProcessHandle](../winapi.rs/ProcessHandle.md) for the target process. Both a read handle (for querying) and a write handle (for setting) are extracted via [get_handles](get_handles.md).

`apply_config_result`

Mutable reference to an [ApplyConfigResult](ApplyConfigResult.md) that accumulates change descriptions and error messages.

## Return value

This function does not return a value. Results are communicated through the `apply_config_result` accumulator.

## Remarks

This function uses the NT native API rather than the documented Win32 API because there is no public Win32 function for getting or setting per-process I/O priority.

The information class constant `PROCESS_INFORMATION_IO_PRIORITY` (value **33**) is defined locally within the function body.

### Query phase

The current I/O priority is read by calling `NtQueryInformationProcess` with a `u32` output buffer. The NTSTATUS return value is checked:

- If negative (failure), the error is logged via [log_error_if_new](log_error_if_new.md) with operation `NtQueryInformationProcess2ProcessInformationIOPriority` and the function returns without attempting to set.
- If zero or positive (success), the current value is compared against the configured target.

### Set phase

If the current I/O priority differs from the configured value:

- In **dry_run** mode, a change message is recorded.
- Otherwise, `NtSetInformationProcess` is called with the target I/O priority value. On failure, the NTSTATUS error is logged via [log_error_if_new](log_error_if_new.md). On success, a change message in the format `"IO Priority: {current} -> {target}"` is recorded.

### I/O priority values

The [IOPriority](../priority.rs/IOPriority.md) enum maps to the Windows `IO_PRIORITY_HINT` values used by the NT kernel scheduler:

| IOPriority | Value | Effect |
|---|---|---|
| VeryLow | 0 | Background I/O, lowest scheduling priority |
| Low | 1 | Below-normal I/O scheduling |
| Normal | 2 | Default I/O scheduling priority |

### Error handling

Errors from both the query and set operations are deduplicated by [log_error_if_new](log_error_if_new.md) using the `(pid, operation, error_code)` key. NTSTATUS codes are formatted via `error_from_ntstatus` for human-readable error messages.

### Handle requirements

The read handle requires `PROCESS_QUERY_INFORMATION` or `PROCESS_QUERY_LIMITED_INFORMATION` access. The write handle requires `PROCESS_SET_INFORMATION` access. If either handle is not available, the function returns early.

## Requirements

| | |
|---|---|
| **Module** | `apply` |
| **Callers** | Main apply loop (process-level enforcement) |
| **Callees** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), `NtQueryInformationProcess`, `NtSetInformationProcess` |
| **API** | NT Native API (`ntdll.dll`) |
| **Privileges** | `SeDebugPrivilege` may be required for protected processes |

## See Also

| Topic | Description |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for changes and errors |
| [apply_memory_priority](apply_memory_priority.md) | Companion function for memory priority (uses documented Win32 API) |
| [apply_priority](apply_priority.md) | Sets process scheduling priority class |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | Configuration struct containing `io_priority` field |
| [IOPriority](../priority.rs/IOPriority.md) | I/O priority level enum |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*