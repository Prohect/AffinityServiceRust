# terminate_child_processes function (winapi.rs)

Terminates all child processes spawned by the current process. Called during service startup to clean up orphaned child processes — particularly the PowerShell console host process left behind by the UAC elevation flow in [request_uac_elevation](request_uac_elevation.md).

## Syntax

```rust
pub fn terminate_child_processes()
```

## Parameters

This function takes no parameters. It operates on the current process (identified via `GetCurrentProcessId`).

## Return value

This function does not return a value. Failures during snapshot creation, process enumeration, or termination are logged but do not propagate as errors.

## Remarks

The function uses the Windows Tool Help library to enumerate all processes on the system and identify those whose `th32ParentProcessID` matches the current process's PID. For each child process found, it opens the process with `PROCESS_TERMINATE` access and calls `TerminateProcess` with exit code `0`.

### Algorithm

1. **GetCurrentProcessId** — Obtains the PID of the current (service) process.
2. **CreateToolhelp32Snapshot** — Creates a snapshot of all running processes (`TH32CS_SNAPPROCESS`). If the snapshot fails, the function returns silently.
3. **Process32FirstW / Process32NextW** — Iterates through every process in the snapshot.
4. For each process entry where `th32ParentProcessID == current_pid`:
   - Extracts the child process name from the null-terminated `szExeFile` field.
   - Opens the child process with `PROCESS_TERMINATE` access via `OpenProcess`.
   - Calls `TerminateProcess` with exit code `0`.
   - Closes the process handle via `CloseHandle`.
   - Logs success or failure for each step.
5. **CloseHandle** — Closes the snapshot handle after iteration completes.

### Logging

Each termination attempt produces a log message:

| Outcome | Log message format |
|---------|-------------------|
| Success | `"terminate_child_processes: terminated '{name}' (PID {pid})"` |
| TerminateProcess failure | `"terminate_child_processes: failed to terminate '{name}' (PID {pid})"` |
| OpenProcess failure | `"terminate_child_processes: failed to open '{name}' (PID {pid})"` |

### UAC elevation cleanup

When the service starts without administrator privileges, [request_uac_elevation](request_uac_elevation.md) spawns a new elevated instance via `powershell.exe Start-Process -Verb RunAs` and then exits. The PowerShell child process may persist as an orphan. On the next startup (now elevated), `terminate_child_processes` cleans up any such leftover child processes before the main polling loop begins.

### Safety considerations

- The function terminates **all** child processes indiscriminately. It does not filter by process name or purpose.
- `TerminateProcess` is an immediate, non-graceful termination — the child process does not receive shutdown notifications or run cleanup handlers.
- If no child processes exist, the function iterates the snapshot silently and returns without any action.
- The snapshot handle is always closed before the function returns, even if iteration fails partway through.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Main startup logic in `src/main.rs` |
| **Callees** | None (leaf function using Win32 APIs directly) |
| **Win32 API** | [GetCurrentProcessId](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid), [CreateToolhelp32Snapshot](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot), [Process32FirstW](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw), [Process32NextW](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw), [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) (`PROCESS_TERMINATE`), [TerminateProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess), [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | `PROCESS_TERMINATE` access to child processes; may require elevation for some child processes |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| UAC elevation that creates child processes | [request_uac_elevation](request_uac_elevation.md) |
| Admin check that triggers elevation | [is_running_as_admin](is_running_as_admin.md) |
| Module enumeration (uses similar snapshot pattern) | [enumerate_process_modules](enumerate_process_modules.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*