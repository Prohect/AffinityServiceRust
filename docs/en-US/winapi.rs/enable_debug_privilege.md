# enable_debug_privilege function (winapi.rs)

Enables the `SeDebugPrivilege` privilege on the current process's access token. This privilege allows the service to open handles to processes owned by other users and protected system processes, which is essential for applying affinity, priority, CPU set, and other settings across the entire system.

## Syntax

```rust
pub fn enable_debug_privilege()
```

## Parameters

None.

## Return value

None. The function logs success or failure internally and does not return a result. Callers should proceed regardless of the outcome — the service will still function with reduced capability if the privilege cannot be enabled.

## Remarks

### Privilege enablement sequence

The function follows the standard Windows privilege-adjustment pattern:

1. **Open the process token** — Calls `OpenProcessToken` on `GetCurrentProcess()` with `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` access. If this fails, the function logs the failure and returns immediately.

2. **Look up the privilege LUID** — Calls `LookupPrivilegeValueW` with `SE_DEBUG_NAME` to obtain the locally unique identifier (LUID) for `SeDebugPrivilege`. If the lookup fails, the token handle is closed and the function returns.

3. **Adjust the token** — Constructs a `TOKEN_PRIVILEGES` structure with a single `LUID_AND_ATTRIBUTES` entry (`SE_PRIVILEGE_ENABLED`) and calls `AdjustTokenPrivileges`. Success or failure is logged.

4. **Clean up** — The token handle is closed via `CloseHandle` regardless of the outcome.

### Effect on the service

Without `SeDebugPrivilege`, `OpenProcess` calls targeting protected processes (e.g., `csrss.exe`, `smss.exe`, anti-cheat services) will fail with `ERROR_ACCESS_DENIED` (5). With the privilege enabled, the service can open these processes and apply the configured rules. This privilege is typically available only to administrators, so the function is most effective when running elevated (see [is_running_as_admin](is_running_as_admin.md) and [request_uac_elevation](request_uac_elevation.md)).

### CLI integration

The `--no_debug_priv` CLI flag allows the user to skip calling this function. When the flag is present, the main loop in [`main.rs`](../main.rs/README.md) does not call `enable_debug_privilege`, which can be useful for testing or when running intentionally without elevated access.

### Logging

All outcomes produce a log message:

| Outcome | Log message |
|---------|-------------|
| `OpenProcessToken` failed | `"enable_debug_privilege: self OpenProcessToken failed"` |
| `LookupPrivilegeValueW` failed | `"enable_debug_privilege: LookupPrivilegeValueW failed"` |
| `AdjustTokenPrivileges` failed | `"enable_debug_privilege: AdjustTokenPrivileges failed"` |
| Success | `"enable_debug_privilege: AdjustTokenPrivileges succeeded"` |

### Relationship to SeIncreaseBasePriorityPrivilege

This function is the `SeDebugPrivilege` counterpart to [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md), which enables `SeIncreaseBasePriorityPrivilege`. Both follow the same three-step pattern (open token → lookup LUID → adjust privileges) and are typically called together during service startup.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [`main`](../main.rs/README.md) (during startup, unless `--no_debug_priv` is set) |
| **Callees** | `OpenProcessToken`, `GetCurrentProcess`, `LookupPrivilegeValueW`, `AdjustTokenPrivileges`, `CloseHandle` (Win32 Security) |
| **API** | [AdjustTokenPrivileges (Microsoft Learn)](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |
| **Privileges** | The calling token must already hold `SeDebugPrivilege` in its list of available privileges (standard for Administrator accounts). This function *enables* an existing privilege; it cannot *grant* a privilege not already assigned to the token. |

## See Also

| Topic | Link |
|-------|------|
| Companion privilege enablement | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| Admin elevation check | [is_running_as_admin](is_running_as_admin.md) |
| UAC elevation request | [request_uac_elevation](request_uac_elevation.md) |
| Process handle acquisition (benefits from this privilege) | [get_process_handle](get_process_handle.md) |
| Thread handle acquisition (benefits from this privilege) | [get_thread_handle](get_thread_handle.md) |
| CLI arguments (--no_debug_priv flag) | [cli module](../cli.rs/README.md) |
| AdjustTokenPrivileges (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |
| SeDebugPrivilege overview | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd