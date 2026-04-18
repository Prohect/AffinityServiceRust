# error_from_ntstatus function (error_codes.rs)

Converts an `i32` NTSTATUS value to its well-known symbolic name string. This function provides human-readable translations for common NT-native API status codes encountered during process and thread management operations, such as `STATUS_ACCESS_DENIED`, `STATUS_INVALID_HANDLE`, and `STATUS_PROCESS_IS_TERMINATING`. Unknown status codes are formatted as hexadecimal strings.

## Syntax

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | `i32` | The NTSTATUS value returned by an NT-native API call (e.g., `NtQueryInformationProcess`, `NtSetInformationProcess`, `NtQueryInformationThread`, `NtQuerySystemInformation`). NTSTATUS values are signed 32-bit integers where negative values indicate errors, zero indicates success, and positive values indicate informational or warning statuses. |

## Return value

Returns a `String` containing the symbolic name of the status code. If the status code is not recognized, the function returns a hexadecimal string in the format `"NTSTATUS_0x{code:08X}"`.

### Recognized status codes

| NTSTATUS Value | Unsigned Hex | Returned String |
|----------------|-------------|-----------------|
| `0` | `0x00000000` | `STATUS_SUCCESS` |
| `1` | `0x00000001` | `STATUS_WAIT_1` |
| `-1073741823` | `0xC0000001` | `STATUS_UNSUCCESSFUL` |
| `-1073741822` | `0xC0000002` | `STATUS_NOT_IMPLEMENTED` |
| `-1073741821` | `0xC0000003` | `STATUS_INVALID_INFO_CLASS` |
| `-1073741820` | `0xC0000004` | `STATUS_INFO_LENGTH_MISMATCH` |
| `-1073741816` | `0xC0000008` | `STATUS_INVALID_HANDLE` |
| `-1073741811` | `0xC000000D` | `STATUS_INVALID_PARAMETER` |
| `-1073741801` | `0xC0000017` | `STATUS_NO_MEMORY` |
| `-1073741800` | `0xC0000018` | `STATUS_CONFLICTING_ADDRESSES` |
| `-1073741790` | `0xC0000022` | `STATUS_ACCESS_DENIED` |
| `-1073741789` | `0xC0000023` | `STATUS_BUFFER_TOO_SMALL` |
| `-1073741772` | `0xC0000034` | `STATUS_OBJECT_NAME_NOT_FOUND` |
| `-1073741749` | `0xC000004B` | `STATUS_THREAD_IS_TERMINATING` |
| `-1073741727` | `0xC0000061` | `STATUS_PRIVILEGE_NOT_HELD` |
| `-1073741637` | `0xC00000BB` | `STATUS_NOT_SUPPORTED` |
| `-1073741558` | `0xC000010A` | `STATUS_PROCESS_IS_TERMINATING` |

### Fallback format

Unrecognized status codes are formatted as:

```text
NTSTATUS_0xC0000XXX
```

The raw `i32` value is cast to `u32` for hexadecimal formatting to produce the conventional unsigned NTSTATUS representation.

## Remarks

- The function uses `i32::cast_unsigned(status)` to convert the signed `i32` to its unsigned `u32` bit-equivalent before matching. This is necessary because NTSTATUS error codes (severity bits `11` in the high two bits) are conventionally written as unsigned hex values (e.g., `0xC0000022`) but are stored as negative `i32` values in Rust bindings.

- The set of recognized codes covers the most common statuses encountered during AffinityServiceRust's operation — particularly those returned by `NtQuerySystemInformation`, `NtQueryInformationProcess`, `NtQueryInformationThread`, `NtSetInformationProcess`, and `NtSetTimerResolution`.

- `STATUS_INFO_LENGTH_MISMATCH` (`0xC0000004`) is especially significant in this project: it is the retry signal used by [`ProcessSnapshot::take`](../process.rs/ProcessSnapshot.md) when the buffer passed to `NtQuerySystemInformation` is too small.

- `STATUS_PROCESS_IS_TERMINATING` (`0xC000010A`) and `STATUS_THREAD_IS_TERMINATING` (`0xC000004B`) are commonly seen when attempting to query or set properties on processes/threads that are in the process of exiting — a normal occurrence in a system-wide process manager.

- Unlike [`error_from_code_win32`](error_from_code_win32.md), which handles Win32 error codes (`u32` values from `GetLastError`), this function handles NTSTATUS values (`i32` values returned directly by NT-native APIs). The two code spaces are distinct and should not be mixed.

- The function allocates a new `String` on each call. For hot paths where the same status code is translated repeatedly, callers should consider caching the result or logging conditionally (as done by [`is_new_error`](../logging.rs/is_new_error.md)).

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `error_codes.rs` |
| **Callers** | `winapi.rs`, `process.rs` — anywhere an NT-native API returns an NTSTATUS value that needs to be logged or displayed. |
| **Callees** | None (pure function, no side effects) |
| **API** | None — this is a lookup table, not an API wrapper |
| **Platform** | Platform-independent logic; the status codes it translates are Windows NT-specific. |

## See Also

| Topic | Link |
|-------|------|
| error_from_code_win32 function | [error_from_code_win32](error_from_code_win32.md) |
| ProcessSnapshot::take | [ProcessSnapshot](../process.rs/ProcessSnapshot.md) |
| error_codes module overview | [README](README.md) |
| winapi module | [winapi.rs](../winapi.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
