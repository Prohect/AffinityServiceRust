# enable_inc_base_priority_privilege function (winapi.rs)

Enables the `SeIncreaseBasePriorityPrivilege` privilege on the current process token. This privilege is required to raise a process or thread's scheduling priority class above `NORMAL_PRIORITY_CLASS` (e.g., to `HIGH_PRIORITY_CLASS` or `REALTIME_PRIORITY_CLASS`). Without this privilege, calls to `SetPriorityClass` with elevated priority values may fail with `ERROR_PRIVILEGE_NOT_HELD`.

## Syntax

```rust
pub fn enable_inc_base_priority_privilege(no_inc_base_priority: bool)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `no_inc_base_priority` | `bool` | If `true`, the function logs a message indicating the privilege is disabled and returns immediately without modifying the process token. This flag is typically set by the `-noIncBasePriority` CLI argument. |

## Return value

This function does not return a value. Success or failure is reported via [`log_message`](../logging.rs/log_message.md) (through the `log!` macro).

## Remarks

### Early-return when disabled

When `no_inc_base_priority` is `true`, the function logs `"SeIncreaseBasePriorityPrivilege disabled by -noIncBasePriority flag"` and returns immediately without opening the process token or calling any Win32 privilege APIs.

### Privilege-enablement steps

The function follows the standard Windows privilege-enablement pattern:

1. **Open the process token** — Calls `OpenProcessToken` on the current process (`GetCurrentProcess()`) requesting `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` access. If this fails, the function logs a message and returns early.

2. **Look up the privilege LUID** — Calls `LookupPrivilegeValueW` with `SE_INC_BASE_PRIORITY_NAME` to obtain the locally unique identifier (LUID) for the `SeIncreaseBasePriorityPrivilege`. If the lookup fails, the token handle is closed and the function returns.

3. **Adjust the token privileges** — Constructs a `TOKEN_PRIVILEGES` structure with a single `LUID_AND_ATTRIBUTES` entry that has `SE_PRIVILEGE_ENABLED` set, then calls `AdjustTokenPrivileges`. The result (success or failure) is logged.

4. **Close the token handle** — The token handle is unconditionally closed before the function returns, regardless of success or failure.

### Relationship to enable_debug_privilege

This function is structurally identical to [`enable_debug_privilege`](enable_debug_privilege.md) but targets a different privilege constant (`SE_INC_BASE_PRIORITY_NAME` vs. `SE_DEBUG_NAME`). Both are typically called during application startup.

### When this privilege is needed

- Setting a process to `HIGH_PRIORITY_CLASS` or above.
- Setting thread priorities to `THREAD_PRIORITY_TIME_CRITICAL`.
- Configuring `REALTIME_PRIORITY_CLASS` for latency-sensitive workloads.

Without this privilege enabled, the operating system silently caps the effective priority or returns an error, depending on the API used.

### Platform notes

- **Windows only.** The `SeIncreaseBasePriorityPrivilege` is a Windows security privilege.
- The privilege must already be **assigned** to the user or group running the process (typically via Local Security Policy or Group Policy). This function can only **enable** an already-assigned privilege; it cannot grant a privilege that has not been assigned.
- Running as administrator typically includes this privilege by default.

### Logging output

| Condition | Log Message |
|-----------|-------------|
| `no_inc_base_priority` is `true` | `SeIncreaseBasePriorityPrivilege disabled by -noIncBasePriority flag` |
| `OpenProcessToken` fails | `enable_inc_base_priority_privilege: self OpenProcessToken failed` |
| `LookupPrivilegeValueW` fails | `enable_inc_base_priority_privilege: LookupPrivilegeValueW failed` |
| `AdjustTokenPrivileges` fails | `enable_inc_base_priority_privilege: AdjustTokenPrivileges failed` |
| `AdjustTokenPrivileges` succeeds | `enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded` |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | Application startup (`main.rs`) |
| **Callees** | `OpenProcessToken`, `GetCurrentProcess`, `LookupPrivilegeValueW`, `AdjustTokenPrivileges`, `CloseHandle` (Win32 API); `log!` macro |
| **Win32 API** | `advapi32.dll` — `OpenProcessToken`, `LookupPrivilegeValueW`, `AdjustTokenPrivileges` |
| **Privileges** | `SeIncreaseBasePriorityPrivilege` must be assigned to the current user/group. |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| logging module | [logging.rs](../logging.rs/README.md) |
| winapi module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*