# terminate_child_processes function (winapi.rs)

Kills orphaned console host processes that were spawned as a side effect of the UAC elevation flow, preventing them from lingering after the non-elevated instance exits.

## Syntax

```rust
pub fn terminate_child_processes()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value.

## Remarks

When the application triggers UAC elevation via [`request_uac_elevation`](request_uac_elevation.md), it spawns a PowerShell process that in turn uses `Start-Process -Verb RunAs` to launch an elevated copy of the application. This chain of process creation can leave behind orphaned child processes — particularly `conhost.exe` instances — that remain running after the original (non-elevated) process exits.

This function enumerates child processes of the current process and terminates them. It is called by the non-elevated instance after [`request_uac_elevation`](request_uac_elevation.md) returns successfully, just before the non-elevated process exits. This ensures a clean handoff to the elevated instance without leaving zombie processes.

The function uses the Windows `CreateToolhelp32Snapshot` API with `TH32CS_SNAPPROCESS` to enumerate all running processes, identifies those whose parent PID matches the current process, and calls `TerminateProcess` on each one.

### When this is called

The typical flow is:

1. [`main`](../main.rs/main.md) detects that the process is not running as admin via [`is_running_as_admin`](is_running_as_admin.md).
2. [`request_uac_elevation`](request_uac_elevation.md) is called, spawning a new elevated instance.
3. `terminate_child_processes` is called to clean up any spawned helper processes.
4. The non-elevated instance exits.

### Safety

The function only terminates processes whose parent PID matches the current process, so it will not affect unrelated processes. If the snapshot or termination calls fail, errors are silently ignored since the non-elevated process is about to exit anyway.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L715–L765 |
| **Called by** | [`main`](../main.rs/main.md) |
| **Windows API** | [CreateToolhelp32Snapshot](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot), [Process32First](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32first), [Process32Next](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32next), [TerminateProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |

## See also

- [request_uac_elevation](request_uac_elevation.md)
- [is_running_as_admin](is_running_as_admin.md)
- [winapi.rs module overview](README.md)