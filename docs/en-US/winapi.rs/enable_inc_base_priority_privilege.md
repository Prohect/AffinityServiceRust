# enable_inc_base_priority_privilege function (winapi.rs)

Enables the `SeIncreaseBasePriorityPrivilege` privilege on the current process token, allowing AffinityServiceRust to raise process priority classes above `Normal` (including `High` and `Realtime`). Without this privilege, calls to `SetPriorityClass` with elevated priority classes will fail with `ERROR_PRIVILEGE_NOT_HELD`.

## Syntax

```rust
pub fn enable_inc_base_priority_privilege()
```

## Parameters

None.

## Return value

None. The function logs success or failure to the application log and returns in all cases. It does not propagate errors to the caller.

## Remarks

### Privilege purpose

Windows requires `SeIncreaseBasePriorityPrivilege` to set a process's priority class to `HIGH_PRIORITY_CLASS` or `REALTIME_PRIORITY_CLASS`. By default this privilege is present in administrator tokens but disabled. This function enables it so that the [apply_priority](../apply.rs/apply_priority.md) function can raise process priority classes as specified in the configuration.

### Implementation

The function follows the standard Windows privilege-adjustment pattern, identical in structure to [enable_debug_privilege](enable_debug_privilege.md):

1. **Open the process token** — Calls `OpenProcessToken` on the current process (`GetCurrentProcess()`) with `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` access. On failure, logs `"enable_inc_base_priority_privilege: self OpenProcessToken failed"` and returns.

2. **Look up the privilege LUID** — Calls `LookupPrivilegeValueW` with `SE_INC_BASE_PRIORITY_NAME` to obtain the locally unique identifier for the privilege. On failure, logs `"enable_inc_base_priority_privilege: LookupPrivilegeValueW failed"`, closes the token handle, and returns.

3. **Adjust the token** — Constructs a `TOKEN_PRIVILEGES` structure with a single `LUID_AND_ATTRIBUTES` entry (the looked-up LUID with `SE_PRIVILEGE_ENABLED` attribute) and calls `AdjustTokenPrivileges`. On success, logs `"enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded"`; on failure, logs `"enable_inc_base_priority_privilege: AdjustTokenPrivileges failed"`.

4. **Clean up** — Closes the token handle via `CloseHandle` regardless of outcome.

### Relationship to enable_debug_privilege

Both [enable_debug_privilege](enable_debug_privilege.md) and `enable_inc_base_priority_privilege` follow the same three-step pattern and differ only in the privilege name constant:

| Function | Privilege constant | SE_* name |
|----------|--------------------|-----------|
| [enable_debug_privilege](enable_debug_privilege.md) | `SE_DEBUG_NAME` | `SeDebugPrivilege` |
| **enable_inc_base_priority_privilege** | `SE_INC_BASE_PRIORITY_NAME` | `SeIncreaseBasePriorityPrivilege` |

### CLI opt-out

The `--no_inc_base_priority` CLI flag skips calling this function. When the flag is set, the service does not attempt to enable the privilege and priority elevation beyond `Normal` may silently fail depending on the token's default privilege state.

### Prerequisites

The privilege must already exist (but be disabled) in the process token. This is typically the case for processes running as a member of the `Administrators` group with UAC elevation. Non-administrator tokens do not contain this privilege at all, and `AdjustTokenPrivileges` will fail silently (the function does not distinguish between "privilege not held" and other failures).

### Error handling

All errors are logged but not propagated. The service continues operating with whatever privileges are available. If the privilege cannot be enabled, any subsequent `SetPriorityClass` calls requesting `High` or `Realtime` priority will fail and be logged by the [apply_priority](../apply.rs/apply_priority.md) function.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [`main`](../main.rs/README.md) (during startup, unless `--no_inc_base_priority` is set) |
| **Callees** | `OpenProcessToken`, `GetCurrentProcess`, `LookupPrivilegeValueW`, `AdjustTokenPrivileges`, `CloseHandle` (Win32 Security / Threading) |
| **API** | [`AdjustTokenPrivileges`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges), [`LookupPrivilegeValueW`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew) |
| **Privileges** | Token must already contain `SeIncreaseBasePriorityPrivilege` (disabled). Requires administrator elevation. |

## See Also

| Topic | Link |
|-------|------|
| Debug privilege enablement (same pattern) | [enable_debug_privilege](enable_debug_privilege.md) |
| UAC elevation request | [request_uac_elevation](request_uac_elevation.md) |
| Admin check | [is_running_as_admin](is_running_as_admin.md) |
| Priority application that depends on this privilege | [apply_priority](../apply.rs/apply_priority.md) |
| Process priority enum | [ProcessPriority](../priority.rs/README.md) |
| CLI arguments | [cli module](../cli.rs/README.md) |
| AdjustTokenPrivileges (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |