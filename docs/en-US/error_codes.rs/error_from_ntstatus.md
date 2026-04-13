# error_from_ntstatus function (error_codes.rs)

Maps an NTSTATUS code to a human-readable symbolic name string. NTSTATUS values are returned by NT native API functions such as `NtSetInformationProcess` and `NtQueryInformationProcess`, which AffinityServiceRust uses for I/O priority management. This function translates the most commonly encountered status codes into their well-known constant names for diagnostic logging.

## Syntax

```error_codes.rs
pub fn error_from_ntstatus(status: i32) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | `i32` | The NTSTATUS code returned by an NT native API call. NTSTATUS values are signed 32-bit integers where negative values (high bit set) indicate errors, zero indicates success, and positive values indicate informational or warning statuses. |

## Return value

A `String` containing the symbolic name for the given NTSTATUS code. If the code is not recognized, a hexadecimal fallback string in the format `"NTSTATUS_0x{code:08X}"` is returned.

## Remarks

The function uses a `match` expression against the unsigned representation of the status code (via `i32::cast_unsigned`) to handle the common pattern where NTSTATUS error codes like `0xC0000022` are naturally expressed as unsigned hex literals.

The following NTSTATUS codes are recognized:

| Code | Symbolic name | Description |
|------|---------------|-------------|
| `0x00000000` | `STATUS_SUCCESS` | The operation completed successfully. |
| `0x00000001` | `STATUS_WAIT_1` | The caller-specified wait completed on object index 1. |
| `0xC0000001` | `STATUS_UNSUCCESSFUL` | An unsuccessful generic status. |
| `0xC0000002` | `STATUS_NOT_IMPLEMENTED` | The requested operation is not implemented. |
| `0xC0000003` | `STATUS_INVALID_INFO_CLASS` | The information class specified is not valid for the operation. |
| `0xC0000004` | `STATUS_INFO_LENGTH_MISMATCH` | The supplied buffer length is incorrect for the information class. |
| `0xC0000008` | `STATUS_INVALID_HANDLE` | An invalid HANDLE was specified. |
| `0xC000000D` | `STATUS_INVALID_PARAMETER` | An invalid parameter was passed to a service or function. |
| `0xC0000017` | `STATUS_NO_MEMORY` | Insufficient virtual memory or paging file quota. |
| `0xC0000018` | `STATUS_CONFLICTING_ADDRESSES` | The specified address range conflicts with an existing allocation. |
| `0xC0000022` | `STATUS_ACCESS_DENIED` | The caller does not have the required access rights. |
| `0xC0000023` | `STATUS_BUFFER_TOO_SMALL` | The supplied buffer is too small to receive the requested data. |
| `0xC0000034` | `STATUS_OBJECT_NAME_NOT_FOUND` | The named object does not exist. |
| `0xC000004B` | `STATUS_THREAD_IS_TERMINATING` | The target thread is in the process of terminating. |
| `0xC0000061` | `STATUS_PRIVILEGE_NOT_HELD` | A required privilege is not held by the caller. |
| `0xC00000BB` | `STATUS_NOT_SUPPORTED` | The request is not supported. |
| `0xC000010A` | `STATUS_PROCESS_IS_TERMINATING` | The target process is in the process of terminating. |

### Common scenarios in AffinityServiceRust

- **`STATUS_ACCESS_DENIED` (0xC0000022):** Returned when attempting to set I/O priority on a protected process without sufficient privileges.
- **`STATUS_PROCESS_IS_TERMINATING` (0xC000010A):** Returned when the target process exits between handle acquisition and the API call — a benign race condition in the polling loop.
- **`STATUS_THREAD_IS_TERMINATING` (0xC000004B):** Similar race condition at the thread level.
- **`STATUS_PRIVILEGE_NOT_HELD` (0xC0000061):** Returned when trying to set `High` I/O priority without `SeIncreaseBasePriorityPrivilege`.

### Difference from Win32 error codes

NTSTATUS codes use a different numbering scheme than Win32 error codes. While both can represent the same conceptual errors (e.g., access denied), they are numerically distinct and must not be interchanged. Use [error_from_code_win32](error_from_code_win32.md) for Win32 `GetLastError`-style codes and this function for NTSTATUS values from NT native API calls.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `error_codes` |
| Callers | [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| Callees | *(none — pure data mapping)* |
| NT API | `NtSetInformationProcess`, `NtQueryInformationProcess` (callers produce the status codes translated here) |

## See Also

| Topic | Link |
|-------|------|
| Win32 error code translation | [error_from_code_win32](error_from_code_win32.md) |
| I/O priority enum (primary use case) | [IOPriority](../priority.rs/IOPriority.md) |
| Memory priority enum | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| Logging and error deduplication | [logging module](../logging.rs/README.md) |
| Module overview | [error_codes module](README.md) |