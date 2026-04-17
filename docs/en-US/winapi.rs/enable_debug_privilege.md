# enable_debug_privilege function (winapi.rs)

Enables the `SeDebugPrivilege` privilege on the current process token. This privilege allows the process to open handles to other processes (including system and elevated processes) that would otherwise be denied, which is essential for AffinityServiceRust to manage CPU affinity and priority settings across all running processes.

## Syntax

```rust
pub fn enable_debug_privilege()
```

## Parameters

This function takes no parameters.

## Return value

This function does not return a value. Success or failure is reported via [`log_message`](../logging.rs/log_message.md) (through the `log!` macro).

## Remarks

The function performs the following steps:

1. **Open the process token** — Calls `OpenProcessToken` on the current process (`GetCurrentProcess()`) with `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` access. If this fails, the function logs an error and returns early.

2. **Look up the privilege LUID** — Calls `LookupPrivilegeValueW` with `SE_DEBUG_NAME` to obtain the locally unique identifier (LUID) for `SeDebugPrivilege`. If the lookup fails, the function logs an error, closes the token handle, and returns.

3. **Adjust the token privileges** — Calls `AdjustTokenPrivileges` with a `TOKEN_PRIVILEGES` structure containing a single `LUID_AND_ATTRIBUTES` entry with `SE_PRIVILEGE_ENABLED`. The result is logged as either success or failure.

4. **Close the token handle** — The token handle is closed via `CloseHandle` regardless of the outcome.

### Important side effects

- This function modifies the **current process token** in-place. Once `SeDebugPrivilege` is enabled, it remains enabled for the lifetime of the process (unless explicitly disabled).
- The privilege must already be **assigned** to the process token by the operating system. Typically, this means the process must be running under an administrator account. Enabling the privilege merely activates it — it does not grant it if it was never assigned.
- If the process is not running as administrator, `AdjustTokenPrivileges` will likely fail with `ERROR_NOT_ALL_ASSIGNED` (1300) or `PRIVILEGE_NOT_HELD` (1314).

### Platform notes

- **Windows only.** Relies on Win32 Security APIs: `OpenProcessToken`, `LookupPrivilegeValueW`, and `AdjustTokenPrivileges`.
- The `SE_DEBUG_NAME` constant resolves to the string `"SeDebugPrivilege"`.
- This function is typically called once at process startup, after [is_running_as_admin](is_running_as_admin.md) confirms that the process has administrator privileges.

### Logging output

| Condition | Log message |
|-----------|-------------|
| `OpenProcessToken` fails | `enable_debug_privilege: self OpenProcessToken failed` |
| `LookupPrivilegeValueW` fails | `enable_debug_privilege: LookupPrivilegeValueW failed` |
| `AdjustTokenPrivileges` fails | `enable_debug_privilege: AdjustTokenPrivileges failed` |
| `AdjustTokenPrivileges` succeeds | `enable_debug_privilege: AdjustTokenPrivileges succeeded` |

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `main.rs` — called during startup initialization |
| **Callees** | `OpenProcessToken`, `LookupPrivilegeValueW`, `AdjustTokenPrivileges`, `CloseHandle` (Win32 API); `log!` macro |
| **Win32 API** | `OpenProcessToken`, `GetCurrentProcess`, `LookupPrivilegeValueW`, `AdjustTokenPrivileges`, `CloseHandle` |
| **Privileges** | Must be running as administrator for the privilege to be present in the token. |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| enable_inc_base_priority_privilege | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| get_process_handle | [get_process_handle](get_process_handle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
