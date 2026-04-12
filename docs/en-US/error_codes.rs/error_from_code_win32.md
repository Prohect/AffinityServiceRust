# error_from_code_win32 function (error_codes.rs)

Maps a Win32 error code to a human-readable English string. Used throughout the application to produce meaningful error messages when Windows API calls fail.

## Syntax

```rust
pub fn error_from_code_win32(code: u32) -> String
```

## Parameters

`code`

The Win32 error code as returned by `GetLastError()` after a failed Windows API call. This is an unsigned 32-bit integer corresponding to the `ERROR_*` constants defined in the Windows SDK headers.

## Return value

Returns a `String` containing a human-readable description of the error. For recognized codes, this is a short English phrase (e.g., `"Access is denied"` for code 5). For unrecognized codes, the function returns the code formatted as a hexadecimal string (e.g., `"0x00000037"`), allowing the user to look it up manually.

## Remarks

This function uses a hardcoded `match` statement mapping approximately 45 commonly encountered Win32 error codes to their descriptions. The set of mapped codes was chosen based on the errors most frequently returned by the Windows APIs used in this application — primarily process and thread manipulation APIs such as `OpenProcess`, `SetPriorityClass`, `SetProcessAffinityMask`, `SetProcessDefaultCpuSets`, and related functions.

### Why not FormatMessage?

The Win32 API provides `FormatMessageW` for converting error codes to localized strings at runtime. This function intentionally avoids `FormatMessageW` for several reasons:

- **Locale independence** — `FormatMessage` returns strings in the system's display language, which makes log files difficult to share across machines or post in English-language forums.
- **Single-line output** — `FormatMessage` can return multi-line strings with trailing newlines, which break the application's single-line-per-entry log format.
- **Deterministic output** — hardcoded strings produce identical output on every system, making logs reproducible and searchable.

### Common mapped codes

| Code | Constant | Description |
| --- | --- | --- |
| 5 | `ERROR_ACCESS_DENIED` | Access is denied |
| 6 | `ERROR_INVALID_HANDLE` | The handle is invalid |
| 87 | `ERROR_INVALID_PARAMETER` | The parameter is incorrect |
| 299 | `ERROR_PARTIAL_COPY` | Only part of a ReadProcessMemory or WriteProcessMemory request was completed |
| 1314 | `ERROR_PRIVILEGE_NOT_HELD` | A required privilege is not held by the client |

### Relationship with error_from_ntstatus

The companion function [`error_from_ntstatus`](error_from_ntstatus.md) provides the same service for NTSTATUS codes returned by NT-layer APIs (`NtSetInformationProcess`, `NtQueryInformationProcess`, etc.). The two functions cover different error code namespaces and are used in different contexts depending on which API layer produced the error.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/error_codes.rs |
| **Source lines** | L1–L46 |
| **Called by** | [`get_process_handle`](../winapi.rs/get_process_handle.md), [`try_open_thread`](../winapi.rs/try_open_thread.md), [`log_error_if_new`](../apply.rs/log_error_if_new.md), [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md), various `apply_*` functions in [apply.rs](../apply.rs/README.md) |
| **Dependencies** | None (pure function) |

## See also

- [error_from_ntstatus](error_from_ntstatus.md)
- [is_new_error](../logging.rs/is_new_error.md)
- [Operation enum](../logging.rs/Operation.md)
- [error_codes.rs module overview](README.md)