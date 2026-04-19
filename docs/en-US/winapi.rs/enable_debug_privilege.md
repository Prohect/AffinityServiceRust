# enable_debug_privilege function (winapi.rs)

Enables the `SeDebugPrivilege` privilege on the current process token. This privilege allows the service to open handles to protected and system processes that would otherwise deny access, enabling full process/thread inspection and modification across all running processes.

## Syntax

```rust
pub fn enable_debug_privilege(no_debug_priv: bool)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `no_debug_priv` | `bool` | If `true`, the function logs a message and returns immediately without attempting to enable the privilege. Controlled by the `-noDebugPriv` CLI flag. |

## Return value

This function does not return a value. Success or failure is communicated via log messages.

## Remarks

The function performs the following sequence of Win32 API calls to enable the privilege:

1. **OpenProcessToken** — Opens the current process token (`GetCurrentProcess()`) with `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` access.
2. **LookupPrivilegeValueW** — Resolves the `SE_DEBUG_NAME` string constant to a `LUID` that identifies `SeDebugPrivilege` on the local system.
3. **AdjustTokenPrivileges** — Enables the privilege by passing a `TOKEN_PRIVILEGES` structure with `SE_PRIVILEGE_ENABLED` set on the resolved LUID.
4. **CloseHandle** — Closes the token handle after the adjustment, regardless of success or failure.

### Failure behavior

Each API call is checked individually. If any call fails, the function logs a descriptive error message and returns early. The token handle is always closed if it was successfully opened. Failures are not fatal — the service continues to operate with reduced capability (unable to open handles to some protected processes).

### Early exit on CLI flag

When `no_debug_priv` is `true` (the user passed `-noDebugPriv` on the command line), the function logs `"SeDebugPrivilege disabled by -noDebugPriv flag"` and returns without touching the token. This allows the user to run the service with reduced privileges for testing or in restricted environments where the privilege cannot be granted.

### When is this privilege needed?

Without `SeDebugPrivilege`, `OpenProcess` and `OpenThread` calls targeting processes owned by other users, system processes, or protected processes will fail with `ERROR_ACCESS_DENIED` (5). Enabling the privilege allows the service to:

- Open handles to all processes (including `csrss.exe`, `lsass.exe`, `System`, etc.)
- Query and modify thread scheduling parameters for any thread on the system
- Enumerate modules in protected processes for address resolution

### Privilege vs. elevation

`SeDebugPrivilege` is typically present in the token of an elevated administrator process but is disabled by default. This function *enables* an already-present privilege — it does not grant a privilege that the token does not have. Running as a non-administrator without the privilege in the token will cause `AdjustTokenPrivileges` to succeed but the privilege will not actually be enabled (a condition detectable via `GetLastError() == ERROR_NOT_ALL_ASSIGNED`, though this check is not performed here).

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Main startup logic in `src/main.rs` |
| **Callees** | None (leaf function) |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [LookupPrivilegeValueW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew) (`SE_DEBUG_NAME`), [AdjustTokenPrivileges](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges), [GetCurrentProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess), [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | Requires the calling process token to already contain `SeDebugPrivilege` (standard for elevated administrator tokens). The function enables it; it cannot add it. |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Companion privilege enablement | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| Elevation check | [is_running_as_admin](is_running_as_admin.md) |
| UAC elevation request | [request_uac_elevation](request_uac_elevation.md) |
| Process handle opening (benefits from this privilege) | [get_process_handle](get_process_handle.md) |
| Thread handle opening (benefits from this privilege) | [get_thread_handle](get_thread_handle.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*