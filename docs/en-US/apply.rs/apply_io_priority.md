# apply_io_priority function (apply.rs)

Sets the I/O priority of a process via the native `NtSetInformationProcess` API with information class `ProcessIoPriority` (33). Unlike process priority class and affinity which use documented Win32 functions, I/O priority is only exposed through the NT native API. The function queries the current I/O priority with `NtQueryInformationProcess`, compares it against the configured target, and applies the change when they differ.

## Syntax

```AffinityServiceRust/src/apply.rs#L420-428
pub fn apply_io_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used for error deduplication in [log_error_if_new](log_error_if_new.md) and for log messages. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | The parsed configuration for this process. The `io_priority` field (an [IOPriority](../priority.rs/IOPriority.md) enum) determines the desired I/O priority. When `io_priority` is `IOPriority::None`, `as_win_const()` returns `None` and the function exits immediately without querying or modifying anything. |
| `dry_run` | `bool` | When `true`, the function records the intended change in `apply_config_result` but does not call `NtSetInformationProcess`. |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | OS handles for the target process. Both a read handle (for `NtQueryInformationProcess`) and a write handle (for `NtSetInformationProcess`) are extracted via [get_handles](get_handles.md). |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages produced during this operation. |

## Return value

None (`()`). Results are communicated through `apply_config_result`.

## Remarks

### Native API usage

Windows does not expose a documented Win32 function for setting per-process I/O priority. Instead, this function calls the NT native API directly:

- **`NtQueryInformationProcess`** with `ProcessInformationClass = 33` (`PROCESS_INFORMATION_IO_PRIORITY`) to read the current I/O priority as a raw `u32`.
- **`NtSetInformationProcess`** with the same information class to write the new value.

Both functions return `NTSTATUS` values. A negative return value indicates failure and is formatted by [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md). The raw `NTSTATUS` `i32` is cast to `u32` via `i32::cast_unsigned` for storage in the error-deduplication map.

### Control flow

1. [get_handles](get_handles.md) extracts the best available read and write `HANDLE`s. If either is `None`, the function returns immediately.
2. `config.io_priority.as_win_const()` converts the [IOPriority](../priority.rs/IOPriority.md) enum to its raw `u32` value. If the configured priority is `None`, the function returns — no query, no change.
3. `NtQueryInformationProcess` reads the current I/O priority into a local `u32`. If this fails (negative `NTSTATUS`), the error is logged via [log_error_if_new](log_error_if_new.md) with `Operation::NtQueryInformationProcess2ProcessInformationIOPriority`, and the function returns without attempting a set.
4. If the current value already equals the target, the function returns silently.
5. In `dry_run` mode the change message is recorded and the function returns.
6. Otherwise, `NtSetInformationProcess` is called. On success the change is recorded; on failure the `NTSTATUS` is logged via [log_error_if_new](log_error_if_new.md) with `Operation::NtSetInformationProcess2ProcessInformationIOPriority`.

### I/O priority values

The raw `u32` values passed to the native API correspond to the [IOPriority](../priority.rs/IOPriority.md) enum:

| IOPriority variant | Raw value | Description |
|--------------------|-----------|-------------|
| `VeryLow` | 0 | Background I/O — lowest bandwidth allocation. |
| `Low` | 1 | Below-normal I/O throughput. |
| `Normal` | 2 | Default I/O priority for most processes. |
| `High` | 3 | Elevated I/O throughput. Requires `SeIncreaseBasePriorityPrivilege`. |

### Change message format

```/dev/null/example.txt#L1
IO Priority: Normal -> VeryLow
```

The message shows the human-readable names of both the old and new priorities, obtained via `IOPriority::from_win_const()` and `IOPriority::as_str()` respectively. The change message string is pre-formatted before the set call to capture the "before" state; it is only pushed into `apply_config_result` if the set succeeds.

### Error handling

Errors use NTSTATUS formatting rather than Win32 error codes, because the native API returns NTSTATUS directly rather than setting the thread-local Win32 error. Common failures include:

| NTSTATUS | Typical cause |
|----------|---------------|
| `STATUS_ACCESS_DENIED` (0xC0000022) | The handle lacks the required access rights for the target process. |
| `STATUS_INVALID_HANDLE` (0xC0000008) | The process has already exited or the handle is stale. |

### Idempotency

The function is idempotent: when the current I/O priority already matches the target, no native API set call is made and no change is recorded.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Callees | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| Native API | `NtQueryInformationProcess` (`ProcessIoPriority`, class 33), `NtSetInformationProcess` (`ProcessIoPriority`, class 33) |
| Privileges | `PROCESS_QUERY_INFORMATION` (read), `PROCESS_SET_INFORMATION` (write). `SeIncreaseBasePriorityPrivilege` may be required to set `High` I/O priority. |

## See Also

| Topic | Link |
|-------|------|
| I/O priority enum and value mapping | [IOPriority](../priority.rs/IOPriority.md) |
| Memory priority (similar pattern, documented Win32 API) | [apply_memory_priority](apply_memory_priority.md) |
| Process priority class | [apply_priority](apply_priority.md) |
| NTSTATUS error formatting | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| Configuration model | [ProcessConfig](../config.rs/ProcessConfig.md) |
| Handle extraction helper | [get_handles](get_handles.md) |
| apply module overview | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd