# apply_memory_priority function (apply.rs)

Sets the memory priority of a process via `SetProcessInformation` with the `ProcessMemoryPriority` information class. Memory priority influences how aggressively the memory manager trims and repurposes a process's physical pages under memory pressure — lower-priority pages are reclaimed first. The function reads the current memory priority with `GetProcessInformation`, compares it to the configured target, and applies the change only when the values differ.

## Syntax

```AffinityServiceRust/src/apply.rs#L508-515
pub fn apply_memory_priority(
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
| `pid` | `u32` | Process identifier of the target process. Used for error deduplication in [log_error_if_new](log_error_if_new.md) and in formatted log messages. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | The parsed configuration rule matched to this process. The `memory_priority` field (a [MemoryPriority](../priority.rs/MemoryPriority.md) enum) specifies the desired memory priority level. When the value is `MemoryPriority::None`, the function returns immediately without querying or modifying anything. |
| `dry_run` | `bool` | When `true`, the function records the intended change in `apply_config_result` but does not call `SetProcessInformation`. |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | OS handle wrapper for the target process. Both a read handle (for `GetProcessInformation`) and a write handle (for `SetProcessInformation`) are extracted via [get_handles](get_handles.md). |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages produced during this call. |

## Return value

None (`()`). Results are communicated through `apply_config_result`.

## Remarks

### Control flow

1. [get_handles](get_handles.md) extracts the best available read and write `HANDLE`s. If either is `None`, the function returns immediately.

2. `config.memory_priority.as_win_const()` converts the [MemoryPriority](../priority.rs/MemoryPriority.md) enum to its Win32 [MEMORY_PRIORITY_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-memory_priority_information) value (wrapped in [MemoryPriorityInformation](../priority.rs/MemoryPriorityInformation.md)). If the configured priority is `None`, `as_win_const()` returns `None` and the function exits — no query, no change.

3. `GetProcessInformation` is called with the `ProcessMemoryPriority` class to read the current memory priority into a local [MemoryPriorityInformation](../priority.rs/MemoryPriorityInformation.md) struct.
   - On failure, the Win32 error code is captured and routed to [log_error_if_new](log_error_if_new.md) with `Operation::GetProcessInformation2ProcessMemoryPriority`. The function returns without attempting a set.

4. If the current value already matches the target, the function exits silently (idempotent).

5. In `dry_run` mode, a summary change message is recorded and the function returns.

6. Otherwise, a new `MemoryPriorityInformation` is constructed with the target value and passed to `SetProcessInformation`. On success the change is recorded; on failure the error is captured and logged.

### Change message format

On a successful set (non-dry-run):

```/dev/null/example.txt#L1
Memory Priority: Normal -> VeryLow
```

The message shows the human-readable names of both the old and new memory priority levels, obtained via `MemoryPriority::from_win_const()` and `MemoryPriority::as_str()` respectively.

### Memory priority levels

The [MemoryPriority](../priority.rs/MemoryPriority.md) enum maps to these Windows-defined values:

| Enum variant | Win32 value | Effect |
|-------------|-------------|--------|
| `VeryLow` | 1 | Pages are the first to be trimmed under memory pressure. |
| `Low` | 2 | Pages are trimmed before medium-priority pages. |
| `Medium` | 3 | Default for background processes. |
| `BelowNormal` | 4 | Slightly favoured over medium. |
| `Normal` | 5 | Default for foreground processes. Pages are trimmed last. |

### Error handling

Errors from both the query and set phases are routed through [log_error_if_new](log_error_if_new.md). Common failure scenarios:

| Win32 error | Typical cause |
|-------------|---------------|
| `ERROR_ACCESS_DENIED` (5) | The process handle was opened with insufficient access rights, or the target is a protected process. |
| `ERROR_INVALID_PARAMETER` (87) | An invalid memory priority value was passed (should not occur when using the enum). |

### Idempotency

The function is idempotent: when the current memory priority already matches the configured target, no Win32 call is made and no change is recorded. This avoids unnecessary kernel transitions on every polling cycle.

### Relationship to I/O priority

Memory priority and I/O priority ([apply_io_priority](apply_io_priority.md)) are independent settings. They are configured separately in [ProcessConfig](../config.rs/ProcessConfig.md) and applied by separate functions. Both affect how aggressively the OS reclaims resources from a process, but they target different subsystems (memory manager vs. I/O scheduler).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Callees | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md) |
| Win32 API | [`GetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation), [`SetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) with `ProcessMemoryPriority` class |
| Privileges | `PROCESS_QUERY_LIMITED_INFORMATION` (read via `GetProcessInformation`), `PROCESS_SET_INFORMATION` (write via `SetProcessInformation`). The service typically holds `SeDebugPrivilege` which grants both. |

## See Also

| Topic | Link |
|-------|------|
| apply module overview | [apply](README.md) |
| I/O priority setting | [apply_io_priority](apply_io_priority.md) |
| Priority class setting | [apply_priority](apply_priority.md) |
| Memory priority enum | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| MemoryPriorityInformation wrapper | [MemoryPriorityInformation](../priority.rs/MemoryPriorityInformation.md) |
| Configuration model | [ProcessConfig](../config.rs/ProcessConfig.md) |
| Handle extraction helper | [get_handles](get_handles.md) |
| Error deduplication | [log_error_if_new](log_error_if_new.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd