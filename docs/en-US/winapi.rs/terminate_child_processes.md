# terminate_child_processes function (winapi.rs)

Terminates any child processes spawned by the current process. This function is called during startup to clean up orphaned child processes, particularly the elevated PowerShell console host instance that may linger after a UAC elevation via [request_uac_elevation](request_uac_elevation.md).

## Syntax

```rust
pub fn terminate_child_processes()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value. Success or failure for each child process is reported via [`log_message`](../logging.rs/log_message.md) (through the `log!` macro).

## Remarks

### Algorithm

1. Obtains the current process ID via `GetCurrentProcessId`.
2. Creates a snapshot of all processes on the system via `CreateToolhelp32Snapshot` with `TH32CS_SNAPPROCESS`.
3. Iterates through the snapshot using `Process32FirstW` / `Process32NextW`.
4. For each process entry whose `th32ParentProcessID` matches the current process ID:
   - Extracts the child process name from `szExeFile` (null-terminated UTF-16 array).
   - Attempts to open the child process with `PROCESS_TERMINATE` access via `OpenProcess`.
   - Calls `TerminateProcess` with exit code `0` to forcefully terminate the child.
   - Closes the child process handle via `CloseHandle`.
5. Closes the snapshot handle.

### Logging output

| Condition | Log message |
|-----------|-------------|
| Child successfully terminated | `terminate_child_processes: terminated '<name>' (PID <pid>)` |
| `TerminateProcess` failed | `terminate_child_processes: failed to terminate '<name>' (PID <pid>)` |
| `OpenProcess` failed | `terminate_child_processes: failed to open '<name>' (PID <pid>)` |

### Important side effects

- This function **forcefully terminates** child processes without giving them a chance to perform cleanup. It uses `TerminateProcess` with exit code `0`, which does not invoke DLL detach routines or flush I/O buffers in the target process.
- The function terminates **all** immediate child processes of the current process, not just specific ones. Any process whose `th32ParentProcessID` matches the current PID will be targeted.
- If `CreateToolhelp32Snapshot` fails, the function returns silently without logging.

### Why this is needed

When AffinityServiceRust requests UAC elevation via [request_uac_elevation](request_uac_elevation.md), it spawns a `powershell.exe` child process using `Start-Process -Verb RunAs`. The original (non-elevated) process then calls `exit(0)`, but the console host process (`conhost.exe`) associated with the PowerShell command may become orphaned. Calling `terminate_child_processes` at the start of the elevated instance cleans up these orphans.

### Platform notes

- **Windows only.** Uses the Tool Help library (`CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`) and process management APIs (`OpenProcess`, `TerminateProcess`, `CloseHandle`).
- The `PROCESSENTRY32W` struct uses a fixed-size `szExeFile` array of 260 wide characters (`MAX_PATH`). Process names are extracted by finding the first null terminator in this array.
- The snapshot represents a point-in-time view. Processes that start or exit between the snapshot and the termination attempt may cause `OpenProcess` or `TerminateProcess` to fail, which is handled gracefully via logging.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `main.rs` — startup cleanup |
| **Callees** | `GetCurrentProcessId`, `CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`, `OpenProcess`, `TerminateProcess`, `CloseHandle` (Win32 API); `log!` macro |
| **Win32 API** | [CreateToolhelp32Snapshot](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot), [TerminateProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |
| **Privileges** | Requires `PROCESS_TERMINATE` access on child processes. Running as administrator with `SeDebugPrivilege` enabled ensures this succeeds for all child processes. |

## See Also

| Topic | Link |
|-------|------|
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| logging module | [logging.rs](../logging.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
