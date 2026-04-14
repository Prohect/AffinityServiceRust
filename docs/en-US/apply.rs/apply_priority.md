# apply_priority function (apply.rs)

Sets the process priority class to the value specified in the configuration. The function reads the current priority via `GetPriorityClass`, compares it against the configured target, and—when they differ—calls `SetPriorityClass` to apply the change. In `dry_run` mode the change is recorded but not executed.

## Syntax

```AffinityServiceRust/src/apply.rs#L83-129
pub fn apply_priority(
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
| `config` | `&ProcessConfig` | The parsed [ProcessConfig](../config.rs/ProcessConfig.md) for this process. The `priority` field (a [ProcessPriority](../priority.rs/ProcessPriority.md) enum) determines the desired priority class. When `priority` is `ProcessPriority::None`, the function returns immediately without querying or modifying anything. |
| `dry_run` | `bool` | When `true`, the function records the intended change in `apply_config_result` but does not call `SetPriorityClass`. |
| `process_handle` | `&ProcessHandle` | A [ProcessHandle](../winapi.rs/ProcessHandle.md) opened for the target process. Both a read handle (for `GetPriorityClass`) and a write handle (for `SetPriorityClass`) are extracted via [get_handles](get_handles.md). |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for changes and errors. See [ApplyConfigResult](ApplyConfigResult.md). |

## Return value

None (`()`). Results are communicated through `apply_config_result`.

## Remarks

### Control flow

1. [get_handles](get_handles.md) extracts the best available read and write `HANDLE`s. If either is `None` (which should not happen for a valid `ProcessHandle`), the function returns immediately.
2. `config.priority.as_win_const()` converts the [ProcessPriority](../priority.rs/ProcessPriority.md) enum to its Win32 `PROCESS_CREATION_FLAGS` constant. If the config priority is `None`, `as_win_const()` returns `None` and the function exits—no query, no change.
3. `GetPriorityClass` reads the current priority from the OS.
4. If the current priority already matches the target, the function exits silently.
5. If `dry_run` is `true`, the change message is recorded and the function exits.
6. Otherwise, `SetPriorityClass` is called. On success the change is recorded; on failure the Win32 error code is captured via `GetLastError` and passed to [log_error_if_new](log_error_if_new.md).

### Change message format

```/dev/null/example.txt#L1
Priority: Normal -> High
```

The message shows the human-readable names of both the old and new priority classes, obtained via `ProcessPriority::from_win_const()` and `ProcessPriority::as_str()` respectively.

### Error handling

Errors from `SetPriorityClass` are routed through [log_error_if_new](log_error_if_new.md) with `Operation::SetPriorityClass`. Common failure causes include:

| Win32 error | Typical cause |
|-------------|---------------|
| `ERROR_ACCESS_DENIED` (5) | The service lacks `PROCESS_SET_INFORMATION` access to the target process (e.g. a protected process). |
| `ERROR_INVALID_PARAMETER` (87) | The requested priority class is not valid for the target process (e.g. `Realtime` without `SeIncreaseBasePriorityPrivilege`). |

Because errors are deduplicated, a process that persistently denies access will only generate one log entry across all polling cycles until the deduplication map is purged (see [purge_fail_map](../logging.rs/purge_fail_map.md)).

### Idempotency

The function is idempotent: if the current priority already matches the target, no Win32 call is made and no change is recorded. This avoids unnecessary kernel transitions on every polling cycle.

### Caller context

`apply_priority` is called from [apply_config_process_level](../main.rs/apply_config_process_level.md) during the process-level apply phase, which runs once per process per configuration cycle. It is intentionally separated from thread-level operations ([apply_prime_threads](apply_prime_threads.md), [apply_ideal_processors](apply_ideal_processors.md)) which run on a different cadence.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Callees | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md) |
| Win32 API | [`GetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass), [`SetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass), [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| Privileges | `PROCESS_QUERY_LIMITED_INFORMATION` (read), `PROCESS_SET_INFORMATION` or `PROCESS_SET_LIMITED_INFORMATION` (write). `SeIncreaseBasePriorityPrivilege` is required to set `Realtime` or `High` priority on processes owned by other users. |

## See Also

| Topic | Link |
|-------|------|
| Priority enum and Win32 constant mapping | [ProcessPriority](../priority.rs/ProcessPriority.md) |
| Process-level apply orchestration | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Configuration model | [ProcessConfig](../config.rs/ProcessConfig.md) |
| Handle extraction helper | [get_handles](get_handles.md) |
| Error deduplication | [log_error_if_new](log_error_if_new.md) |
| apply module overview | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd