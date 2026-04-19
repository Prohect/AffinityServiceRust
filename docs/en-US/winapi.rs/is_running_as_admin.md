# is_running_as_admin function (winapi.rs)

Checks whether the current process is running with administrator (elevated) privileges by querying the process token for elevation status.

## Syntax

```rust
pub fn is_running_as_admin() -> bool
```

## Parameters

This function takes no parameters.

## Return value

`bool` — `true` if the current process token indicates the process is elevated (running as administrator); `false` otherwise, including when any intermediate API call fails.

## Remarks

The function queries the current process token using the following sequence of Win32 API calls:

1. **OpenProcessToken** — Opens the token of the current process (`GetCurrentProcess()`) with `TOKEN_QUERY` access.
2. **GetTokenInformation** — Queries the `TokenElevation` information class, filling a `TOKEN_ELEVATION` structure.
3. **CloseHandle** — Closes the token handle after the query.

The result is determined by the `TokenIsElevated` field of the `TOKEN_ELEVATION` structure. A non-zero value indicates the process is elevated.

### Failure behavior

If either `OpenProcessToken` or `GetTokenInformation` fails, the function returns `false` rather than propagating the error. This conservative default ensures the service treats itself as non-elevated when token inspection is not possible, which triggers the UAC elevation flow via [request_uac_elevation](request_uac_elevation.md).

### Usage in the service

This function is called early during service startup to determine whether UAC elevation is needed. If `is_running_as_admin()` returns `false` and the `noUAC` CLI flag is not set, the service launches an elevated copy of itself via [request_uac_elevation](request_uac_elevation.md) and exits the current (non-elevated) process.

### Token elevation vs. privilege checks

This function checks token *elevation status*, not whether specific privileges (such as `SeDebugPrivilege`) are enabled. Elevation indicates the process was launched via a UAC consent prompt or is running under a high-integrity token. Individual privileges must still be explicitly enabled after elevation — see [enable_debug_privilege](enable_debug_privilege.md) and [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md).

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Main startup logic in `src/main.rs` |
| **Callees** | None (leaf function) |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) (`TokenElevation`), [GetCurrentProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess), [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | None required — `TOKEN_QUERY` access to the process's own token is always available |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| UAC elevation request | [request_uac_elevation](request_uac_elevation.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |
| Base priority privilege enablement | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*