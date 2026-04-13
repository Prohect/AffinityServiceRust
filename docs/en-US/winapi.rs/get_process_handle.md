# get_process_handle function (winapi.rs)

Opens a process by PID with multiple access levels and returns a [ProcessHandle](ProcessHandle.md) containing all successfully opened handles. The two limited-access handles (`PROCESS_QUERY_LIMITED_INFORMATION` and `PROCESS_SET_LIMITED_INFORMATION`) are required — if either fails, the function returns `None`. The two full-access handles (`PROCESS_QUERY_INFORMATION` and `PROCESS_SET_INFORMATION`) are attempted but optional; they are stored as `Option<HANDLE>` and set to `None` if the caller lacks sufficient privileges.

## Syntax

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process. Obtained from process enumeration (e.g., `SYSTEM_PROCESS_INFORMATION.UniqueProcessId`). |
| `process_name` | `&str` | The image name of the target process (e.g., `"explorer.exe"`). Used only for diagnostic logging when a handle open fails. |

## Return value

| Value | Description |
|-------|-------------|
| `Some(ProcessHandle)` | A [ProcessHandle](ProcessHandle.md) with at least `r_limited_handle` and `w_limited_handle` valid. The `r_handle` and `w_handle` fields may be `None` if the higher-privilege opens failed. All `Some` handles are confirmed valid (non-invalid). |
| `None` | Either `PROCESS_QUERY_LIMITED_INFORMATION` or `PROCESS_SET_LIMITED_INFORMATION` could not be opened, or the returned handle was invalid. No handles are leaked on failure — any partially opened handles are closed before returning `None`. |

## Remarks

### Access levels

The function opens the process four times with progressively higher access rights:

| Order | Access right | Required | Field |
|-------|-------------|----------|-------|
| 1 | `PROCESS_QUERY_LIMITED_INFORMATION` | Yes | `r_limited_handle` |
| 2 | `PROCESS_SET_LIMITED_INFORMATION` | Yes | `w_limited_handle` |
| 3 | `PROCESS_QUERY_INFORMATION` | No | `r_handle` |
| 4 | `PROCESS_SET_INFORMATION` | No | `w_handle` |

The limited-information handles are sufficient for most operations (reading priority class, setting affinity via CPU sets, querying memory/IO priority). The full-information handles are needed for operations like `NtSetInformationProcess` with certain information classes. By acquiring what it can, the service degrades gracefully when `SeDebugPrivilege` is not held or the target process has restricted access.

### Error logging

Failures for the two required handles are logged via `log_to_find` using the `is_new_error` deduplication mechanism — each unique `(pid, operation, error_code)` combination is logged only once. Failures for the two optional handles are silently ignored (the logging calls are commented out in the source).

### Internal error-code mapping

The function uses synthetic `internal_op_code` values when calling `is_new_error` with `Operation::InvalidHandle`:

| Code | Meaning |
|------|---------|
| `0` | `r_limited_handle` was invalid |
| `1` | `w_limited_handle` was invalid |
| `2` | `r_handle` was invalid (currently not logged) |
| `3` | `w_handle` was invalid (currently not logged) |

### Handle cleanup on partial failure

If `r_limited_handle` opens successfully but `w_limited_handle` fails, the function closes `r_limited_handle` before returning `None`. This prevents handle leaks. On success, all handle cleanup is deferred to the [ProcessHandle](ProcessHandle.md) `Drop` implementation.

### Common failure causes

- **Access denied (error 5):** The target process is protected (e.g., `csrss.exe`, `System`) and the caller does not hold `SeDebugPrivilege`.
- **Invalid parameter (error 87):** The PID no longer exists (race between snapshot and open).
- **Process has exited:** The process terminated between enumeration and the `OpenProcess` call.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Callers** | [`apply_config_process_level`](../main.rs/apply_config_process_level.md), [`apply_config_thread_level`](../main.rs/apply_config_thread_level.md), main loop in [`main.rs`](../main.rs/README.md) |
| **Callees** | `OpenProcess` (Win32), `CloseHandle` (Win32), `GetLastError` (Win32), [`is_new_error`](../logging.rs/is_new_error.md), `log_to_find` |
| **API** | `OpenProcess` — [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) |
| **Privileges** | `SeDebugPrivilege` recommended for full-information handles on protected processes |

## See Also

| Topic | Link |
|-------|------|
| RAII process handle container | [ProcessHandle](ProcessHandle.md) |
| Thread handle acquisition | [get_thread_handle](get_thread_handle.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |
| Error deduplication | [`is_new_error`](../logging.rs/is_new_error.md) |
| Operation enum for error logging | [`Operation`](../logging.rs/Operation.md) |