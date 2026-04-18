# is_running_as_admin function (winapi.rs)

Checks whether the current process is running with administrator (elevated) privileges by querying the process token for elevation status.

## Syntax

```rust
pub fn is_running_as_admin() -> bool
```

## Parameters

This function takes no parameters.

## Return value

Returns `true` if the current process token indicates the process is running elevated (as administrator). Returns `false` if the process is not elevated, or if any of the underlying Windows API calls fail.

## Remarks

The function performs the following steps:

1. Obtains a handle to the current process via `GetCurrentProcess`.
2. Opens the process token with `TOKEN_QUERY` access using `OpenProcessToken`.
3. Queries the token for `TokenElevation` information via `GetTokenInformation`.
4. Inspects the `TOKEN_ELEVATION.TokenIsElevated` field — a nonzero value indicates the process is elevated.
5. Closes the token handle before returning.

If `OpenProcessToken` or `GetTokenInformation` fails, the function returns `false` as a conservative default (assumes not elevated).

This function is typically called early during startup to determine whether UAC elevation is needed. If it returns `false` and the service requires administrator privileges, the caller may invoke [request_uac_elevation](request_uac_elevation.md) to restart the process with elevated rights.

### Platform notes

- **Windows only.** Uses `TOKEN_ELEVATION` and `TokenElevation` from the Win32 Security API.
- On systems where UAC is disabled, the function still returns the correct elevation status based on the token.
- The function does not cache its result. Each call queries the token afresh, though in practice the elevation state of a process cannot change after launch.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `main.rs` — startup logic to decide whether UAC elevation is required. |
| **Callees** | `GetCurrentProcess`, `OpenProcessToken`, `GetTokenInformation`, `CloseHandle` (Win32 API) |
| **API** | [GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) with `TokenElevation` class |
| **Privileges** | None required — every process can query its own token. |

## See Also

| Topic | Link |
|-------|------|
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| enable_inc_base_priority_privilege | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| winapi module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
