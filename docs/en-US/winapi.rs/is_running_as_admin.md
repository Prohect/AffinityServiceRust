# is_running_as_admin function (winapi.rs)

Checks whether the current process is running with administrator (elevated) privileges.

## Syntax

```rust
pub fn is_running_as_admin() -> bool
```

## Parameters

This function takes no parameters.

## Return value

Returns `true` if the current process is running with an elevated administrator token. Returns `false` if the process is running with standard user privileges or if the check itself fails.

## Remarks

This function queries the current process token to determine whether it has administrator privileges. The check is performed by opening the process token with `TOKEN_QUERY` access and examining the token's elevation status.

The result is used by [`main`](../main.rs/main.md) to decide whether UAC elevation is required. If the process is not running as admin and the `--no-uac` flag is not set, [`request_uac_elevation`](request_uac_elevation.md) is called to relaunch the process with elevated privileges.

If the token query fails for any reason (e.g., insufficient access to the own process token), the function conservatively returns `false`, which will trigger the UAC elevation flow.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L437–L466 |
| **Called by** | [`main`](../main.rs/main.md) in main.rs |
| **Windows API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) |

## See also

- [request_uac_elevation](request_uac_elevation.md)
- [enable_debug_privilege](enable_debug_privilege.md)
- [winapi.rs module overview](README.md)