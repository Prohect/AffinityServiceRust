# error_from_code_win32 function (error_codes.rs)

Maps a Win32 error code to a human-readable constant name string. Used throughout the project to produce meaningful log messages when Windows API calls fail.

## Syntax

```rust
pub fn error_from_code_win32(code: u32) -> String
```

## Parameters

`code: u32`

A Win32 error code as returned by `GetLastError` or extracted from an `HRESULT`. Common values include `5` (ACCESS_DENIED), `87` (INVALID_PARAMETER), and `998` (NOACCESS).

## Return value

`String` — The symbolic name of the error code (e.g., `"ACCESS_DENIED"`, `"INVALID_PARAMETER"`). If the code is not in the lookup table, returns a formatted hex string in the form `"WIN32_ERROR_CODE_0x{code:08X}"`.

## Remarks

The function uses a `match` statement over a curated set of Win32 error codes that are commonly encountered during process and thread manipulation. The set is not exhaustive — it covers the codes most relevant to AffinityServiceRust's operations:

### Recognized codes

| Code | Name | Typical context |
|------|------|-----------------|
| 0 | `SUCCESS` | Operation completed successfully |
| 2 | `FILE_NOT_FOUND` | Config file or module path not found |
| 5 | `ACCESS_DENIED` | Insufficient privileges to open/modify a process |
| 6 | `INVALID_HANDLE` | Stale or closed process/thread handle |
| 8 | `NOT_ENOUGH_MEMORY` | System memory exhaustion |
| 31 | `ERROR_GEN_FAILURE` | General device or driver failure |
| 87 | `INVALID_PARAMETER` | Invalid argument to a Win32 API call |
| 122 | `INSUFFICIENT_BUFFER` | Buffer too small for query results |
| 126 | `MOD_NOT_FOUND` | DLL or module not found |
| 127 | `PROC_NOT_FOUND` | Exported function not found in module |
| 193 | `BAD_EXE_FORMAT` | Invalid executable image |
| 565 | `TOO_MANY_THREADS` | Thread limit exceeded |
| 566 | `THREAD_NOT_IN_PROCESS` | Thread does not belong to the target process |
| 567 | `PAGEFILE_QUOTA_EXCEEDED` | Page file quota exceeded |
| 571 | `IO_PRIVILEGE_FAILED` | I/O privilege operation failed |
| 577 | `INVALID_IMAGE_HASH` | Code integrity check failure |
| 633 | `DRIVER_FAILED_SLEEP` | Driver cannot enter sleep state |
| 998 | `NOACCESS` | Invalid access to memory location |
| 1003 | `CALLER_CANNOT_MAP_VIEW` | Caller cannot map a memory view |
| 1006 | `VOLUME_CHANGED` | Volume has been changed externally |
| 1007 | `FULLSCREEN_MODE` | Exclusive fullscreen mode conflict |
| 1008 | `INVALID_HANDLE_STATE` | Handle is in an invalid state |
| 1058 | `SERVICE_DISABLED` | Windows service is disabled |
| 1060 | `SERVICE_DOES_NOT_EXIST` | Specified service does not exist |
| 1062 | `SERVICE_NOT_STARTED` | Service has not been started |
| 1073 | `ALREADY_RUNNING` | Process or service is already running |
| 1314 | `PRIVILEGE_NOT_HELD` | Required privilege not held by caller |
| 1330 | `INVALID_ACCOUNT_NAME` | Invalid account name format |
| 1331 | `LOGON_FAILURE` | Logon attempt failed |
| 1332 | `ACCOUNT_RESTRICTION` | Account restriction prevents operation |
| 1344 | `NO_LOGON_SERVERS` | No logon servers available |
| 1346 | `RPC_AUTH_LEVEL_MISMATCH` | RPC authentication level mismatch |
| 1444 | `INVALID_THREAD_ID` | Invalid thread identifier |
| 1445 | `NON_MDICHILD_WINDOW` | Not an MDI child window |
| 1450 | `NO_SYSTEM_RESOURCES` | Insufficient system resources |
| 1453 | `QUOTA_EXCEEDED` | Quota exceeded |
| 1455 | `PAGEFILE_TOO_SMALL` | Paging file is too small |
| 1460 | `TIMEOUT` | Operation timed out |
| 1500 | `EVT_INVALID_CHANNEL` | Invalid event channel |
| 1503 | `EVT_CHANNEL_ALREADY_EXISTS` | Event channel already exists |

### Fallback format

For any code not in the table, the function returns a string like `"WIN32_ERROR_CODE_0x00000039"`, using zero-padded 8-digit uppercase hexadecimal. This ensures that unrecognized errors still produce a searchable, unambiguous identifier in logs.

### Design rationale

A static lookup is preferred over calling `FormatMessage` at runtime to avoid additional Win32 API calls in error paths, keep output deterministic and concise (constant names rather than localized sentences), and eliminate the allocation and cleanup overhead of `FormatMessage` buffers.

## Requirements

| | |
|---|---|
| **Module** | `src/error_codes.rs` |
| **Callers** | [log_error_if_new](../apply.rs/log_error_if_new.md), logging helpers throughout the crate |
| **Callees** | None (pure function) |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [error_codes.rs](README.md) |
| NTSTATUS counterpart | [error_from_ntstatus](error_from_ntstatus.md) |
| Error deduplication | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| Microsoft Win32 error code reference | [System Error Codes](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*