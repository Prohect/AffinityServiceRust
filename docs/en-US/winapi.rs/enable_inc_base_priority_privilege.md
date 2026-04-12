# enable_inc_base_priority_privilege function (winapi.rs)

Enables `SeIncreaseBasePriorityPrivilege` for the current process token, allowing the application to set process priority classes above `Normal`, including `High` and `Realtime`.

## Syntax

```rust
pub fn enable_inc_base_priority_privilege()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value. Success or failure is logged internally.

## Remarks

`SeIncreaseBasePriorityPrivilege` is a Windows privilege that allows a process to increase the base priority of another process. Without this privilege, attempts to set a process priority class to `High` or `Realtime` via `SetPriorityClass` will fail with `ERROR_ACCESS_DENIED` for processes not owned by the current user, or may be silently capped.

The function performs the following steps:

1. Opens the current process token with `TOKEN_ADJUST_PRIVILEGES` access via `OpenProcessToken`.
2. Looks up the LUID (locally unique identifier) for the `SeIncreaseBasePriorityPrivilege` privilege name via `LookupPrivilegeValueW`.
3. Calls `AdjustTokenPrivileges` to enable the privilege on the token.

If any step fails, the error is logged but the application continues running. Processes that require elevated priority classes will simply fail to have their priority set, and those errors will be logged through the normal [`is_new_error`](../logging.rs/is_new_error.md) deduplication path.

This function is called once during startup from [`main`](../main.rs/main.md), unless the `--no-inc-base-priority` CLI flag is set in [`CliArgs`](../cli.rs/CliArgs.md), which skips the call entirely.

### Privilege requirements

Like [`enable_debug_privilege`](enable_debug_privilege.md), this function requires the process to be running in an elevated (administrator) context for `AdjustTokenPrivileges` to succeed. If the process is not elevated, the privilege adjustment will fail silently. This is expected in the pre-UAC-elevation phase.

### Relationship with apply_priority

The [`apply_priority`](../apply.rs/apply_priority.md) function in the apply module relies on this privilege being enabled to successfully set `High` and `Realtime` priority classes on target processes. Without this privilege, only `Idle`, `BelowNormal`, `Normal`, and `AboveNormal` can be reliably set.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L543–L581 |
| **Called by** | [`main`](../main.rs/main.md) |
| **Windows API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [LookupPrivilegeValueW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew), [AdjustTokenPrivileges](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |

## See also

- [enable_debug_privilege](enable_debug_privilege.md)
- [apply_priority](../apply.rs/apply_priority.md)
- [is_running_as_admin](is_running_as_admin.md)
- [CliArgs](../cli.rs/CliArgs.md) (`no_inc_base_priority` flag)
- [winapi.rs module overview](README.md)