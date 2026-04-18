# apply_io_priority function (apply.rs)

The `apply_io_priority` function reads the current I/O priority of a process via `NtQueryInformationProcess` and, if it differs from the configured target, sets it to the desired value via `NtSetInformationProcess`. In dry-run mode the change is recorded without issuing the set call. Errors are deduplicated via [`log_error_if_new`](log_error_if_new.md) to avoid flooding the log with repeated failures for the same process.

## Syntax

```AffinityServiceRust/src/apply.rs#L402-412
pub fn apply_io_priority(
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
| `config` | `&ProcessLevelConfig` | The process-level configuration containing the desired `io_priority` value (an `IOPriority` enum). If `config.io_priority` does not map to a Windows constant (i.e., `as_win_const()` returns `None`), the function returns immediately without action. |
| `dry_run` | `bool` | When `true`, the function records what *would* change in `apply_config_result` without calling `NtSetInformationProcess`. The current I/O priority is still queried so the change message can show the before/after values. |
| `process_handle` | `&ProcessHandle` | A handle wrapper providing read and write access to the target process. The function extracts a read handle (for `NtQueryInformationProcess`) and a write handle (for `NtSetInformationProcess`) via [`get_handles`](get_handles.md). If either handle is unavailable, the function returns early. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages. On success (or dry-run), a change string of the form `"IO Priority: <old> -> <new>"` is appended. On failure, an error string is appended (subject to deduplication). |

## Return value

This function does not return a value. All outcomes are communicated through the `apply_config_result` parameter.

## Remarks

- The function uses the undocumented `NtQueryInformationProcess` and `NtSetInformationProcess` NT APIs with the information class constant `PROCESS_INFORMATION_IO_PRIORITY` (value `33`). This constant is defined locally within the function body.
- The current I/O priority is queried by passing a `u32`-sized buffer to `NtQueryInformationProcess`. The NTSTATUS return value is checked: a negative value indicates failure, and the error is logged via [`log_error_if_new`](log_error_if_new.md) using `error_from_ntstatus` to decode the NTSTATUS into a human-readable string. On query failure the function returns without attempting to set.
- If the query succeeds and the current I/O priority already matches the target, no action is taken and nothing is recorded.
- When setting the I/O priority, `NtSetInformationProcess` is called with the write handle. A negative NTSTATUS return value indicates failure; the error is logged with a distinct `Operation` variant (`NtSetInformationProcess2ProcessInformationIOPriority`) to differentiate it from the query error.
- The change message is formatted as `"IO Priority: <current_name> -> <target_name>"` using `IOPriority::from_win_const` and `IOPriority::as_str` for human-readable names.
- Windows I/O priority levels typically include Very Low, Low, Normal, High, and Critical, represented as integer constants `0` through `4`.
- Error deduplication for the query operation uses `Operation::NtQueryInformationProcess2ProcessInformationIOPriority`, while the set operation uses `Operation::NtSetInformationProcess2ProcessInformationIOPriority`. These are distinct variants to allow independent suppression of query vs. set errors for the same process.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| NT APIs | `NtQueryInformationProcess` (information class 33), `NtSetInformationProcess` (information class 33) |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that iterates matched processes |
| Callees | [`get_handles`](get_handles.md), [`log_error_if_new`](log_error_if_new.md), `IOPriority::as_win_const`, `IOPriority::from_win_const`, `IOPriority::as_str`, `error_from_ntstatus` (error_codes module) |
| Privileges | Requires a process handle with `PROCESS_QUERY_INFORMATION` or `PROCESS_QUERY_LIMITED_INFORMATION` (read) and `PROCESS_SET_INFORMATION` (write). |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| IOPriority | [`priority.rs/IOPriority`](../priority.rs/IOPriority.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*