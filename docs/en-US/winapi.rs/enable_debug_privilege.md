# enable_debug_privilege function (winapi.rs)

Enables `SeDebugPrivilege` for the current process token, allowing the application to open handles to processes owned by other users and system processes.

## Syntax

```rust
pub fn enable_debug_privilege()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value. Success or failure is logged internally.

## Remarks

`SeDebugPrivilege` is a powerful Windows privilege that grants the holder the ability to open any process on the system regardless of its security descriptor. This is essential for AffinityService because it needs to query and modify process attributes (affinity, priority, CPU sets, etc.) for processes running under different user accounts and for system processes.

The function performs the following steps:

1. Opens the current process token with `TOKEN_ADJUST_PRIVILEGES` access via `OpenProcessToken`.
2. Looks up the LUID (locally unique identifier) for the `SeDebugPrivilege` privilege name via `LookupPrivilegeValueW`.
3. Calls `AdjustTokenPrivileges` to enable the privilege on the token.

If any step fails, the error is logged but the application continues running — some processes will simply be inaccessible without the privilege. The function does not panic or return an error.

This function is called once during startup from [`main`](../main.rs/main.md), unless the `--no-debug-priv` CLI flag is set, which skips the call entirely.

### Privilege requirements

The calling process must be running in an elevated (administrator) context for `AdjustTokenPrivileges` to succeed. If the process is not elevated, the privilege adjustment will fail silently — this is expected in the pre-UAC-elevation phase and is one reason the application requests elevation via [`request_uac_elevation`](request_uac_elevation.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L503–L541 |
| **Called by** | [`main`](../main.rs/main.md) |
| **Windows API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [LookupPrivilegeValueW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew), [AdjustTokenPrivileges](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |

## See also

- [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md)
- [is_running_as_admin](is_running_as_admin.md)
- [request_uac_elevation](request_uac_elevation.md)
- [winapi.rs module overview](README.md)