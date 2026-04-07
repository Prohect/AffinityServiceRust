# Error Codes Module Documentation

Windows error code translations.

## Overview

This module maps Windows error codes to human-readable strings for logging.

## Called By

- `apply.rs` - Translating API errors
- `winapi.rs` - Translating handle operation errors
- `logging.rs` - Error message formatting

## Functions

### error_from_code_win32

Translates Win32 error codes to strings.

```rust
pub fn error_from_code_win32(code: u32) -> String
```

**Parameters:**
- `code` - Win32 error code from `GetLastError()`

**Returns:**
- Named constant string (e.g., `"ACCESS_DENIED"`)
- Or formatted hex code (e.g., `"WIN32_ERROR_CODE_0x00000005"`)

**Mapped Codes:**

| Code | Name | Description |
|------|------|-------------|
| 0 | `SUCCESS` | Operation successful |
| 2 | `FILE_NOT_FOUND` | File not found |
| 5 | `ACCESS_DENIED` | Access denied |
| 6 | `INVALID_HANDLE` | Invalid handle |
| 8 | `NOT_ENOUGH_MEMORY` | Insufficient memory |
| 31 | `ERROR_GEN_FAILURE` | General failure |
| 87 | `INVALID_PARAMETER` | Invalid parameter |
| 122 | `INSUFFICIENT_BUFFER` | Buffer too small |
| 126 | `MOD_NOT_FOUND` | Module not found |
| 127 | `PROC_NOT_FOUND` | Procedure not found |
| 193 | `BAD_EXE_FORMAT` | Bad executable format |
| 565 | `TOO_MANY_THREADS` | Too many threads |
| 566 | `THREAD_NOT_IN_PROCESS` | Thread not in process |
| 567 | `PAGEFILE_QUOTA_EXCEEDED` | Pagefile quota exceeded |
| 571 | `IO_PRIVILEGE_FAILED` | I/O privilege failed |
| 577 | `INVALID_IMAGE_HASH` | Invalid image hash |
| 633 | `DRIVER_FAILED_SLEEP` | Driver failed sleep transition |
| 998 | `NOACCESS` | Invalid access to memory |
| 1003 | `CALLER_CANNOT_MAP_VIEW` | Cannot map view |
| 1006 | `VOLUME_CHANGED` | Volume changed |
| 1007 | `FULLSCREEN_MODE` | Fullscreen mode |
| 1008 | `INVALID_HANDLE_STATE` | Invalid handle state |
| 1058 | `SERVICE_DISABLED` | Service disabled |
| 1060 | `SERVICE_DOES_NOT_EXIST` | Service doesn't exist |
| 1062 | `SERVICE_NOT_STARTED` | Service not started |
| 1073 | `ALREADY_RUNNING` | Service already running |
| 1314 | `PRIVILEGE_NOT_HELD` | Required privilege not held |
| 1330 | `INVALID_ACCOUNT_NAME` | Invalid account name |
| 1331 | `LOGON_FAILURE` | Logon failure |
| 1332 | `ACCOUNT_RESTRICTION` | Account restriction |
| 1344 | `NO_LOGON_SERVERS` | No logon servers |
| 1346 | `RPC_AUTH_LEVEL_MISMATCH` | RPC auth level mismatch |
| 1444 | `INVALID_THREAD_ID` | Invalid thread ID |
| 1445 | `NON_MDICHILD_WINDOW` | Not an MDI child window |
| 1450 | `NO_SYSTEM_RESOURCES` | Insufficient system resources |
| 1460 | `TIMEOUT` | Operation timed out |
| 1453 | `QUOTA_EXCEEDED` | Quota exceeded |
| 1455 | `PAGEFILE_TOO_SMALL` | Pagefile too small |
| 1500 | `EVT_INVALID_CHANNEL` | Invalid event channel |
| 1503 | `EVT_CHANNEL_ALREADY_EXISTS` | Event channel already exists |

**Example:**
```rust
let code = unsafe { GetLastError().0 };
let msg = error_from_code_win32(code);
// code=5 → msg="ACCESS_DENIED"
// code=9999 → msg="WIN32_ERROR_CODE_0x0000270F"
```

### error_from_ntstatus

Translates NTSTATUS codes to strings.

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

**Parameters:**
- `status` - NTSTATUS from NT API functions

**Returns:**
- Named constant string (e.g., `"STATUS_ACCESS_DENIED"`)
- Or formatted hex code (e.g., `"NTSTATUS_0xC0000005"`)

**Mapped Codes:**

| Code | Name | Description |
|------|------|-------------|
| 0x00000000 | `STATUS_SUCCESS` | Success |
| 0x00000001 | `STATUS_WAIT_1` | Wait 1 |
| 0xC0000001 | `STATUS_UNSUCCESSFUL` | Unspecified error |
| 0xC0000002 | `STATUS_NOT_IMPLEMENTED` | Not implemented |
| 0xC0000003 | `STATUS_INVALID_INFO_CLASS` | Invalid info class |
| 0xC0000004 | `STATUS_INFO_LENGTH_MISMATCH` | Info length mismatch |
| 0xC0000008 | `STATUS_INVALID_HANDLE` | Invalid handle |
| 0xC000000D | `STATUS_INVALID_PARAMETER` | Invalid parameter |
| 0xC0000017 | `STATUS_NO_MEMORY` | Insufficient memory |
| 0xC0000018 | `STATUS_CONFLICTING_ADDRESSES` | Conflicting addresses |
| 0xC0000022 | `STATUS_ACCESS_DENIED` | Access denied |
| 0xC0000023 | `STATUS_BUFFER_TOO_SMALL` | Buffer too small |
| 0xC0000034 | `STATUS_OBJECT_NAME_NOT_FOUND` | Object name not found |
| 0xC000004B | `STATUS_THREAD_IS_TERMINATING` | Thread is terminating |
| 0xC0000061 | `STATUS_PRIVILEGE_NOT_HELD` | Privilege not held |
| 0xC00000BB | `STATUS_NOT_SUPPORTED` | Not supported |
| 0xC000010A | `STATUS_PROCESS_IS_TERMINATING` | Process is terminating |

**Common NTSTATUS Values:**

| Value | Meaning |
|-------|---------|
| `0` (0x00000000) | Success |
| Negative | Error (high bit set) |
| `0xC0000005` | Access violation |
| `0xC0000022` | Access denied |
| `0xC0000034` | Object not found |

**Example:**
```rust
let status = NtSetInformationProcess(...).0;
let msg = error_from_ntstatus(status);
// status=-1073741790 (0xC0000022) → msg="STATUS_ACCESS_DENIED"
```

## Design Notes

### Coverage

These functions provide mappings for the most common error codes encountered during process/thread management. Unknown codes are formatted as hex for debugging.

### Win32 vs NTSTATUS

**Win32 Error Codes:**
- Returned by most Win32 APIs via `GetLastError()`
- Range: 0 to 65535 (typically)
- Format: `ERROR_*` constants

**NTSTATUS Codes:**
- Returned by NT native APIs
- Range: 32-bit signed (0 for success, negative for error)
- Format: `STATUS_*` constants
- Often more specific than Win32 errors

### Conversion

Use `i32::cast_unsigned()` when converting NTSTATUS to `u32` for logging:

```rust
let status: NTSTATUS = ...;
let status_u32 = i32::cast_unsigned(status.0);
```

## Usage Examples

### Win32 API Error

```rust
match unsafe { SetProcessAffinityMask(handle, mask) } {
    Ok(_) => {},
    Err(_) => {
        let code = unsafe { GetLastError().0 };
        log!("Failed: {}", error_from_code_win32(code));
        // "Failed: ACCESS_DENIED"
    }
}
```

### NT API Error

```rust
let status = unsafe {
    NtQueryInformationProcess(handle, class, ptr, len, &mut ret_len)
}.0;

if status < 0 {
    log!("Failed: {}", error_from_ntstatus(status));
    // "Failed: STATUS_ACCESS_DENIED"
}
```

### Error Code Logging

```rust
// In apply.rs error handling
log_error_if_new(pid, name, operation, error_code, result, || {
    format!("{}: [{}] {}-{}", 
        fn_name, 
        error_from_code_win32(error_code),
        pid, 
        name
    )
});
// Output: "apply_affinity: [ACCESS_DENIED] 1234-notepad.exe"
```

## Dependencies

- No external crate dependencies
- Pure Rust string formatting

## Extending

To add new error codes:

1. Add match arm to appropriate function
2. Use consistent naming (UPPER_SNAKE_CASE)
3. Prefer standard Windows constant names
4. Place in numerical order

Example:
```rust
1234 => "NEW_ERROR_CODE".to_string(),
```
