# enable_inc_base_priority_privilege function (winapi.rs)

Enables the `SeIncreaseBasePriorityPrivilege` privilege on the current process token, allowing the service to raise thread and process priority classes above `Normal`. Without this privilege, attempts to set `High` or `Realtime` priority classes will fail with `ERROR_PRIVILEGE_NOT_HELD`.

## Syntax

```rust
pub fn enable_inc_base_priority_privilege(no_inc_base_priority: bool)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `no_inc_base_priority` | `bool` | If `true`, the function logs a message indicating the privilege has been disabled by the `-noIncBasePriority` CLI flag and returns immediately without modifying the process token. If `false`, the function proceeds with privilege enablement. |

## Return value

This function does not return a value. Success or failure is communicated via log messages.

## Remarks

The function follows the same three-step privilege enablement pattern as [enable_debug_privilege](enable_debug_privilege.md):

1. **OpenProcessToken** — Opens the current process token with `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` access.
2. **LookupPrivilegeValueW** — Resolves the `SE_INC_BASE_PRIORITY_NAME` privilege name to a `LUID`.
3. **AdjustTokenPrivileges** — Enables the privilege by constructing a `TOKEN_PRIVILEGES` structure with `SE_PRIVILEGE_ENABLED` and passing it to the API.

The token handle is closed via `CloseHandle` after each operation, including on error paths.

### Early exit

When `no_inc_base_priority` is `true`, the function logs `"SeIncreaseBasePriorityPrivilege disabled by -noIncBasePriority flag"` and returns without opening the token. This allows users to opt out of priority elevation when running the service in a restricted context.

### Error handling

Each step logs a descriptive message on failure and returns early:

| Failure point | Log message |
|---|---|
| `OpenProcessToken` | `"enable_inc_base_priority_privilege: self OpenProcessToken failed"` |
| `LookupPrivilegeValueW` | `"enable_inc_base_priority_privilege: LookupPrivilegeValueW failed"` |
| `AdjustTokenPrivileges` | `"enable_inc_base_priority_privilege: AdjustTokenPrivileges failed"` |
| Success | `"enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded"` |

When `LookupPrivilegeValueW` fails, the token handle opened in the previous step is closed before returning.

### When is this privilege needed?

Windows requires `SeIncreaseBasePriorityPrivilege` to set a process priority class to `HIGH_PRIORITY_CLASS` or `REALTIME_PRIORITY_CLASS`, or to raise a thread's priority above `THREAD_PRIORITY_NORMAL` in certain scenarios. The service calls this function during startup so that subsequent [apply_priority](../apply.rs/apply_priority.md) calls can set elevated priority classes as defined in the configuration.

### Relationship to SeDebugPrivilege

This privilege is independent of [SeDebugPrivilege](enable_debug_privilege.md). `SeDebugPrivilege` controls the ability to open handles to processes owned by other users or protected processes, while `SeIncreaseBasePriorityPrivilege` controls the ability to raise scheduling priority. Both are typically enabled at startup but can be independently disabled via CLI flags.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Main startup logic in `src/main.rs` |
| **Callees** | None (leaf function; calls Win32 APIs directly) |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [LookupPrivilegeValueW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew), [AdjustTokenPrivileges](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges), [GetCurrentProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess), [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | Requires the process to be running under an account that holds `SeIncreaseBasePriorityPrivilege` (typically Administrators). The function *enables* the privilege; it cannot *grant* it if the account does not possess it. |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |
| Elevation check | [is_running_as_admin](is_running_as_admin.md) |
| UAC elevation request | [request_uac_elevation](request_uac_elevation.md) |
| Priority application | [apply_priority](../apply.rs/apply_priority.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*