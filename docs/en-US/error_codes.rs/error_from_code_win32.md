# error_from_code_win32 function (error_codes.rs)

Maps a Win32 error code to a human-readable symbolic name string. This function provides a static lookup of the most commonly encountered Win32 error codes in AffinityServiceRust, enabling meaningful diagnostic output in log messages when Windows API calls fail. Unrecognized codes are formatted as a hexadecimal fallback string.

## Syntax

```error_codes.rs
pub fn error_from_code_win32(code: u32) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `code` | `u32` | The Win32 error code to translate. This is typically obtained from `GetLastError` or from the error code embedded in a `windows::core::Error`. |

## Return value

A `String` containing the symbolic constant name for the error code (e.g., `"ACCESS_DENIED"`, `"INVALID_HANDLE"`). If the code is not in the static lookup table, returns a formatted hexadecimal string of the form `"WIN32_ERROR_CODE_0x00000000"`.

## Remarks

The function uses a `match` statement covering the following Win32 error codes:

| Code | Symbolic name |
|------|---------------|
| 0 | `SUCCESS` |
| 2 | `FILE_NOT_FOUND` |
| 5 | `ACCESS_DENIED` |
| 6 | `INVALID_HANDLE` |
| 8 | `NOT_ENOUGH_MEMORY` |
| 31 | `ERROR_GEN_FAILURE` |
| 87 | `INVALID_PARAMETER` |
| 122 | `INSUFFICIENT_BUFFER` |
| 126 | `MOD_NOT_FOUND` |
| 127 | `PROC_NOT_FOUND` |
| 193 | `BAD_EXE_FORMAT` |
| 565 | `TOO_MANY_THREADS` |
| 566 | `THREAD_NOT_IN_PROCESS` |
| 567 | `PAGEFILE_QUOTA_EXCEEDED` |
| 571 | `IO_PRIVILEGE_FAILED` |
| 577 | `INVALID_IMAGE_HASH` |
| 633 | `DRIVER_FAILED_SLEEP` |
| 998 | `NOACCESS` |
| 1003 | `CALLER_CANNOT_MAP_VIEW` |
| 1006 | `VOLUME_CHANGED` |
| 1007 | `FULLSCREEN_MODE` |
| 1008 | `INVALID_HANDLE_STATE` |
| 1058 | `SERVICE_DISABLED` |
| 1060 | `SERVICE_DOES_NOT_EXIST` |
| 1062 | `SERVICE_NOT_STARTED` |
| 1073 | `ALREADY_RUNNING` |
| 1314 | `PRIVILEGE_NOT_HELD` |
| 1330 | `INVALID_ACCOUNT_NAME` |
| 1331 | `LOGON_FAILURE` |
| 1332 | `ACCOUNT_RESTRICTION` |
| 1344 | `NO_LOGON_SERVERS` |
| 1346 | `RPC_AUTH_LEVEL_MISMATCH` |
| 1444 | `INVALID_THREAD_ID` |
| 1445 | `NON_MDICHILD_WINDOW` |
| 1450 | `NO_SYSTEM_RESOURCES` |
| 1453 | `QUOTA_EXCEEDED` |
| 1455 | `PAGEFILE_TOO_SMALL` |
| 1460 | `TIMEOUT` |
| 1500 | `EVT_INVALID_CHANNEL` |
| 1503 | `EVT_CHANNEL_ALREADY_EXISTS` |

This is a static table, not a call to `FormatMessage`. It avoids the overhead and locale-dependent behavior of `FormatMessageW` while providing consistent, grep-friendly log output. The table covers the error codes that AffinityServiceRust realistically encounters during process handle acquisition, priority setting, affinity manipulation, CPU set management, ETW session control, and privilege operations.

Each returned string intentionally omits the `ERROR_` prefix that the official Win32 headers use (e.g., `"ACCESS_DENIED"` instead of `"ERROR_ACCESS_DENIED"`), keeping log lines shorter while remaining unambiguous. The hexadecimal fallback format (`WIN32_ERROR_CODE_0x{:08X}`) makes it straightforward to look up undocumented codes in Microsoft documentation.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `error_codes` |
| Callers | [log_error_if_new](../apply.rs/log_error_if_new.md), [EtwProcessMonitor::start](../event_trace.rs/EtwProcessMonitor.md), [get_process_handle](../winapi.rs/get_process_handle.md), [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| Callees | *(none — pure mapping)* |
| Win32 API | *(none — this function does not call any Win32 API)* |
| Privileges | None |

## See Also

| Topic | Link |
|-------|------|
| NTSTATUS code translation | [error_from_ntstatus](error_from_ntstatus.md) |
| Error deduplication in logging | [is_new_error](../logging.rs/is_new_error.md) |
| error_codes module overview | [error_codes module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd