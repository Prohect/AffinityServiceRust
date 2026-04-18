# error_from_code_win32 function (error_codes.rs)

Converts a `u32` Win32 error code to its well-known symbolic name string. This function provides human-readable translations for the most commonly encountered Win32 error codes in the context of process and thread management, privilege operations, and ETW tracing. Unknown codes are formatted as zero-padded hexadecimal strings.

## Syntax

```rust
pub fn error_from_code_win32(code: u32) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `code` | `u32` | A Win32 error code, typically obtained from `GetLastError()` or from the error payload of a `windows::core::Error`. |

## Return value

Returns a `String` containing the symbolic name of the error code. If the code is not recognized, returns a string in the format `"WIN32_ERROR_CODE_0x{code:08X}"` (e.g., `"WIN32_ERROR_CODE_0x000003E9"`).

## Recognized codes

| Code | Returned string | Win32 constant |
|------|----------------|----------------|
| `0` | `"SUCCESS"` | `ERROR_SUCCESS` |
| `2` | `"FILE_NOT_FOUND"` | `ERROR_FILE_NOT_FOUND` |
| `5` | `"ACCESS_DENIED"` | `ERROR_ACCESS_DENIED` |
| `6` | `"INVALID_HANDLE"` | `ERROR_INVALID_HANDLE` |
| `8` | `"NOT_ENOUGH_MEMORY"` | `ERROR_NOT_ENOUGH_MEMORY` |
| `31` | `"ERROR_GEN_FAILURE"` | `ERROR_GEN_FAILURE` |
| `87` | `"INVALID_PARAMETER"` | `ERROR_INVALID_PARAMETER` |
| `122` | `"INSUFFICIENT_BUFFER"` | `ERROR_INSUFFICIENT_BUFFER` |
| `126` | `"MOD_NOT_FOUND"` | `ERROR_MOD_NOT_FOUND` |
| `127` | `"PROC_NOT_FOUND"` | `ERROR_PROC_NOT_FOUND` |
| `193` | `"BAD_EXE_FORMAT"` | `ERROR_BAD_EXE_FORMAT` |
| `565` | `"TOO_MANY_THREADS"` | `ERROR_TOO_MANY_THREADS` |
| `566` | `"THREAD_NOT_IN_PROCESS"` | `ERROR_THREAD_NOT_IN_PROCESS` |
| `567` | `"PAGEFILE_QUOTA_EXCEEDED"` | `ERROR_PAGEFILE_QUOTA_EXCEEDED` |
| `571` | `"IO_PRIVILEGE_FAILED"` | `ERROR_IO_PRIVILEGE_FAILED` |
| `577` | `"INVALID_IMAGE_HASH"` | `ERROR_INVALID_IMAGE_HASH` |
| `633` | `"DRIVER_FAILED_SLEEP"` | `ERROR_DRIVER_FAILED_SLEEP` |
| `998` | `"NOACCESS"` | `ERROR_NOACCESS` |
| `1003` | `"CALLER_CANNOT_MAP_VIEW"` | `ERROR_CALLER_CANNOT_MAP_VIEW` |
| `1006` | `"VOLUME_CHANGED"` | `ERROR_VOLUME_CHANGED` |
| `1007` | `"FULLSCREEN_MODE"` | `ERROR_FULLSCREEN_MODE` |
| `1008` | `"INVALID_HANDLE_STATE"` | `ERROR_INVALID_HANDLE_STATE` |
| `1058` | `"SERVICE_DISABLED"` | `ERROR_SERVICE_DISABLED` |
| `1060` | `"SERVICE_DOES_NOT_EXIST"` | `ERROR_SERVICE_DOES_NOT_EXIST` |
| `1062` | `"SERVICE_NOT_STARTED"` | `ERROR_SERVICE_NOT_STARTED` |
| `1073` | `"ALREADY_RUNNING"` | `ERROR_SERVICE_ALREADY_RUNNING` |
| `1314` | `"PRIVILEGE_NOT_HELD"` | `ERROR_PRIVILEGE_NOT_HELD` |
| `1330` | `"INVALID_ACCOUNT_NAME"` | `ERROR_INVALID_ACCOUNT_NAME` |
| `1331` | `"LOGON_FAILURE"` | `ERROR_LOGON_FAILURE` |
| `1332` | `"ACCOUNT_RESTRICTION"` | `ERROR_ACCOUNT_RESTRICTION` |
| `1344` | `"NO_LOGON_SERVERS"` | `ERROR_NO_LOGON_SERVERS` |
| `1346` | `"RPC_AUTH_LEVEL_MISMATCH"` | `RPC_S_AUTHN_LEVEL_NOT_SUPPORTED` |
| `1444` | `"INVALID_THREAD_ID"` | `ERROR_INVALID_THREAD_ID` |
| `1445` | `"NON_MDICHILD_WINDOW"` | `ERROR_NON_MDICHILD_WINDOW` |
| `1450` | `"NO_SYSTEM_RESOURCES"` | `ERROR_NO_SYSTEM_RESOURCES` |
| `1453` | `"QUOTA_EXCEEDED"` | `ERROR_QUOTA_EXCEEDED` |
| `1455` | `"PAGEFILE_TOO_SMALL"` | `ERROR_COMMITMENT_LIMIT` |
| `1460` | `"TIMEOUT"` | `ERROR_TIMEOUT` |
| `1500` | `"EVT_INVALID_CHANNEL"` | `ERROR_EVT_INVALID_CHANNEL_PATH` |
| `1503` | `"EVT_CHANNEL_ALREADY_EXISTS"` | `ERROR_EVT_CHANNEL_ALREADY_EXISTS` |

## Remarks

- The function uses a `match` statement against literal integer values for O(1) dispatch (compiled as a jump table or binary search by the Rust compiler). No hash map or external lookup table is used.

- Each match arm allocates a new `String` via `.to_string()`. Callers that use the result only for formatting or logging may wish to consider this allocation cost in hot paths.

- The set of recognized codes is curated for the specific error scenarios encountered by AffinityServiceRust: process/thread handle operations, privilege management, ETW session management, module enumeration, and service control. It is **not** an exhaustive mapping of all Win32 error codes.

- The fallback format `"WIN32_ERROR_CODE_0x{code:08X}"` uses uppercase hexadecimal with 8-digit zero padding, producing values like `WIN32_ERROR_CODE_0x000003E9`. This makes unrecognized codes easy to look up in Microsoft documentation or with the `net helpmsg` command.

- Several of the most commonly seen codes in AffinityServiceRust contexts:
  - `5` (`ACCESS_DENIED`) — the target process is protected or elevated and the caller lacks `SeDebugPrivilege`.
  - `6` (`INVALID_HANDLE`) — a handle was closed prematurely or was never valid.
  - `87` (`INVALID_PARAMETER`) — an API received an out-of-range argument (e.g., invalid CPU set ID, invalid priority class).
  - `1314` (`PRIVILEGE_NOT_HELD`) — the required privilege (e.g., `SeDebugPrivilege`, `SeIncreaseBasePriorityPrivilege`) has not been enabled on the process token.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `error_codes.rs` |
| **Callers** | [`get_process_handle`](../winapi.rs/get_process_handle.md), [`get_thread_handle`](../winapi.rs/get_thread_handle.md), [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md), [`EtwProcessMonitor::start`](../event_trace.rs/EtwProcessMonitor.md), `apply.rs` rule application logic |
| **Callees** | None (pure function, no side effects) |
| **Dependencies** | Standard library only (`String`, `format!`) |
| **Platform** | Platform-independent function; the error codes it maps are Windows-specific. |

## See Also

| Topic | Link |
|-------|------|
| error_from_ntstatus function | [error_from_ntstatus](error_from_ntstatus.md) |
| error_codes module overview | [README](README.md) |
| get_process_handle function | [get_process_handle](../winapi.rs/get_process_handle.md) |
| get_thread_handle function | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| is_affinity_unset function | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| EtwProcessMonitor struct | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |
| logging module | [logging.rs](../logging.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
