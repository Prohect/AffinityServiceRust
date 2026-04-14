# terminate_child_processes function (winapi.rs)

Terminates all child processes of the current process by taking a system-wide process snapshot, identifying entries whose parent PID matches the current process, and calling `TerminateProcess` on each. This is used during startup to clean up orphaned child processes — particularly the elevated PowerShell instance spawned by [request_uac_elevation](request_uac_elevation.md) — that may still be running from a previous launch.

## Syntax

```rust
pub fn terminate_child_processes()
```

## Parameters

None.

## Return value

None. The function logs success or failure for each child process it attempts to terminate and returns in all cases. It does not propagate errors to the caller.

## Remarks

### Algorithm

1. Obtains the current process ID via `GetCurrentProcessId`.
2. Creates a system-wide process snapshot via `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)`. If the snapshot cannot be created, the function returns immediately without logging.
3. Iterates through the snapshot using `Process32FirstW` / `Process32NextW`.
4. For each process entry whose `th32ParentProcessID` equals the current process's PID:
   - Extracts the child's image name from `szExeFile` (null-terminated UTF-16).
   - Opens the child process with `PROCESS_TERMINATE` access via `OpenProcess`.
   - Calls `TerminateProcess(handle, 0)` to force-terminate the child with exit code 0.
   - Closes the child process handle via `CloseHandle`.
5. Closes the snapshot handle after iteration completes.

### Logging

Each step of the termination process produces a log message:

| Outcome | Log message |
|---------|-------------|
| Child terminated successfully | `"terminate_child_processes: terminated '<name>' (PID <pid>)"` |
| `TerminateProcess` failed | `"terminate_child_processes: failed to terminate '<name>' (PID <pid>)"` |
| `OpenProcess` failed | `"terminate_child_processes: failed to open '<name>' (PID <pid>)"` |

### Why child cleanup is needed

When AffinityServiceRust requests UAC elevation via [request_uac_elevation](request_uac_elevation.md), it spawns a `powershell.exe` child process that in turn launches the elevated instance. The non-elevated parent process exits immediately via `std::process::exit(0)`, but the PowerShell child may linger. When the elevated instance starts, it calls `terminate_child_processes` to clean up any such orphaned children.

Additionally, console host processes (`conhost.exe`) may remain attached as children on some Windows configurations and are cleaned up by this function.

### Parent PID caveat

Windows does not maintain a strict parent-child process tree. The `th32ParentProcessID` field in `PROCESSENTRY32W` records the PID of the process that created the entry, but:

- If the parent has exited, the parent PID may have been **recycled** by the OS for an unrelated process. In that case, this function would incorrectly identify an unrelated process as a child.
- This risk is mitigated in practice because the function is called immediately at startup, before the current PID has had time to be recycled and reused.

### Snapshot safety

The `PROCESSENTRY32W` structure must have its `dwSize` field set to `size_of::<PROCESSENTRY32W>()` before the first call to `Process32FirstW`. The function initializes this field correctly. The snapshot handle is closed via `CloseHandle` in the final cleanup step, including in all code paths.

### Unsafe code

The entire iteration body is wrapped in an `unsafe` block because it calls Win32 FFI functions (`Process32FirstW`, `Process32NextW`, `OpenProcess`, `TerminateProcess`, `CloseHandle`). The safety invariants are upheld by:

- Only reading data from the snapshot structure after a successful `Process32FirstW` / `Process32NextW` call.
- Only dereferencing the process handle if `OpenProcess` returned `Ok`.
- Closing all handles before returning.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [`main`](../main.rs/README.md) (startup sequence, after UAC elevation) |
| **Callees** | `GetCurrentProcessId`, `CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`, `OpenProcess`, `TerminateProcess`, `CloseHandle` (Win32 ToolHelp / Threading) |
| **API** | [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot), [`TerminateProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |
| **Privileges** | `PROCESS_TERMINATE` access is required on each child process; `SeDebugPrivilege` may be needed for protected child processes |

## See Also

| Topic | Link |
|-------|------|
| UAC elevation (spawns child processes cleaned up here) | [request_uac_elevation](request_uac_elevation.md) |
| Admin privilege check | [is_running_as_admin](is_running_as_admin.md) |
| Process handle RAII wrapper | [ProcessHandle](ProcessHandle.md) |
| Service main entry point | [main module](../main.rs/README.md) |
| CreateToolhelp32Snapshot (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot) |
| TerminateProcess (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd