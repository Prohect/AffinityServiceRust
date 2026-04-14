# is_running_as_admin function (winapi.rs)

Checks whether the current process is running with administrator (elevated) privileges by querying the process token for `TOKEN_ELEVATION` information. This is used at startup to determine whether UAC elevation should be requested before the service loop begins.

## Syntax

```rust
pub fn is_running_as_admin() -> bool
```

## Parameters

None.

## Return value

| Value | Description |
|-------|-------------|
| `true` | The current process token has elevation status (`TokenIsElevated != 0`), meaning the process is running with full administrator privileges. |
| `false` | The process is not elevated, or any step in the token query chain failed (token open failure, `GetTokenInformation` failure). The function defaults to `false` on error rather than panicking, so callers can safely use the result to decide whether to attempt elevation. |

## Remarks

### Algorithm

The function performs three sequential Win32 calls:

1. **`OpenProcessToken`** — Opens the current process's token with `TOKEN_QUERY` access.
2. **`GetTokenInformation`** — Queries `TokenElevation` information class, filling a `TOKEN_ELEVATION` structure.
3. **`CloseHandle`** — Closes the token handle regardless of the `GetTokenInformation` result.

The token handle is always closed before returning, even on failure paths, to prevent handle leaks.

### Failure behavior

Any failure in the chain causes the function to return `false`:

| Failure point | Behavior |
|---------------|----------|
| `OpenProcessToken` fails | Returns `false` immediately. |
| `GetTokenInformation` fails | Closes the token handle, returns `false`. |
| Success but `TokenIsElevated == 0` | Closes the token handle, returns `false` (not elevated). |

No errors are logged — the function is intentionally silent because it is called very early in the startup sequence, before logging may be fully initialized.

### Usage in startup flow

The main function calls `is_running_as_admin()` to decide whether to invoke [request_uac_elevation](request_uac_elevation.md). The typical flow is:

1. Parse CLI arguments.
2. Call `is_running_as_admin()`.
3. If `false` and the `--no_uac` flag is not set, call [request_uac_elevation](request_uac_elevation.md) to re-launch the process with admin rights.
4. If `true`, proceed with [enable_debug_privilege](enable_debug_privilege.md) and the main service loop.

### UAC and token elevation

On Windows with UAC enabled, a user in the Administrators group runs processes with a filtered (non-elevated) token by default. When elevation is granted (e.g., via "Run as administrator" or a UAC prompt), the process receives the full, unfiltered token with `TokenIsElevated` set to a non-zero value. This function detects that distinction.

### Relationship to privileges

Being elevated is a prerequisite for successfully enabling privileges like `SeDebugPrivilege` ([enable_debug_privilege](enable_debug_privilege.md)) and `SeIncreaseBasePriorityPrivilege` ([enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md)). Without elevation, those privilege-adjustment calls will fail silently, and the service will operate with reduced capability (unable to open protected processes or set Realtime priority).

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [`main`](../main.rs/README.md) (startup sequence) |
| **Callees** | `GetCurrentProcess`, `OpenProcessToken`, `GetTokenInformation` (`TokenElevation`), `CloseHandle` (Win32) |
| **API** | [`OpenProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [`GetTokenInformation`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) |
| **Privileges** | None — token self-query does not require elevation |

## See Also

| Topic | Link |
|-------|------|
| UAC elevation request | [request_uac_elevation](request_uac_elevation.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |
| Base priority privilege enablement | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| Service main entry point | [main module](../main.rs/README.md) |
| TOKEN_ELEVATION (MSDN) | [Microsoft Learn — TOKEN_ELEVATION](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/ns-securitybaseapi-token_elevation) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd