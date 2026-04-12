# apply_io_priority function (apply.rs)

Sets the I/O priority for a target process using the undocumented `NtQueryInformationProcess` and `NtSetInformationProcess` native API calls with information class 33 (`ProcessInformationIoPriority`).

## Syntax

```rust
pub fn apply_io_priority(
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

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing the desired `io_priority` setting. If `config.io_priority` is `None`, the function returns immediately without taking action.

`dry_run`

When `true`, the function records what changes would be made without calling any Windows APIs to modify the process.

`process_handle`

Reference to the [ProcessHandle](../winapi.rs/ProcessHandle.md) containing read and write handles to the target process. Both a read handle (for querying) and a write handle (for setting) are required.

`apply_config_result`

Mutable reference to an [ApplyConfigResult](ApplyConfigResult.md) that accumulates change messages and error messages produced during execution.

## Return value

This function does not return a value. Results are communicated through `apply_config_result`.

## Remarks

The function uses the NT native API rather than Win32 API because there is no documented Win32 function to get or set per-process I/O priority.

The information class constant `PROCESS_INFORMATION_IO_PRIORITY` (33) is used for both the query and set operations. The current I/O priority is read as a raw `u32` via `NtQueryInformationProcess`, compared against the target value from config, and set via `NtSetInformationProcess` if they differ.

**Privilege requirement:** Setting I/O priority to `High` requires `SeIncreaseBasePriorityPrivilege` and the process must be running as administrator. Without this privilege, `NtSetInformationProcess` will return an NTSTATUS error.

**Error handling:**

- If `NtQueryInformationProcess` fails (returns a negative NTSTATUS), the error is logged via [log_error_if_new](log_error_if_new.md) and the function does not attempt to set a new value.
- If `NtSetInformationProcess` fails, the error is logged via [log_error_if_new](log_error_if_new.md) with the NTSTATUS code translated by `error_from_ntstatus`.
- Errors are deduplicated per pid/operation combination so repeated failures on the same process do not spam the log.

**Change logged:** `"IO Priority: {old} -> {new}"` where old and new are human-readable names from the [IOPriority](../priority.rs/IOPriority.md) enum (e.g., `VeryLow`, `Low`, `Normal`, `High`).

**Dry-run behavior:** When `dry_run` is `true` and the current I/O priority differs from the target, the change message is recorded without calling `NtSetInformationProcess`.

### I/O Priority Values

| IOPriority Variant | Win32 Value |
| --- | --- |
| VeryLow | 0 |
| Low | 1 |
| Normal | 2 |
| High | 3 |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L420–L506 |
| **Called by** | [apply_config](../main.rs/apply_config.md) |
| **Calls** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md) |
| **Windows API** | `NtQueryInformationProcess`, `NtSetInformationProcess` (class 33) |
| **Privilege** | `SeIncreaseBasePriorityPrivilege` (for High I/O priority) |

## See also

- [apply_memory_priority](apply_memory_priority.md)
- [apply_priority](apply_priority.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)
- [IOPriority](../priority.rs/IOPriority.md)