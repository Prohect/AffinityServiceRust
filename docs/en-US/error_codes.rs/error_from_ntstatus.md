# error_from_ntstatus function (error_codes.rs)

Maps an NTSTATUS code to a human-readable English string describing the error condition. Used to produce meaningful log output when NT-layer APIs fail.

## Syntax

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

## Parameters

`status`

The NTSTATUS code returned by an NT-layer API call (e.g., `NtSetInformationProcess`, `NtQueryInformationProcess`, `NtQueryInformationThread`). NTSTATUS codes are signed 32-bit integers where negative values indicate errors.

## Return value

Returns a `String` containing a human-readable description of the NTSTATUS code. If the code is recognized, the string contains the symbolic name and a brief description (e.g., `"STATUS_ACCESS_DENIED"`). If the code is not recognized, the string contains the raw value formatted as a hexadecimal number (e.g., `"0xC00000BB"`).

## Remarks

This function provides a hardcoded lookup table covering approximately 17 common NTSTATUS codes that are encountered during process and thread manipulation. These include:

- `STATUS_SUCCESS` (0x00000000)
- `STATUS_ACCESS_DENIED` (0xC0000022)
- `STATUS_INFO_LENGTH_MISMATCH` (0xC0000004)
- `STATUS_INVALID_HANDLE` (0xC0000008)
- `STATUS_INVALID_PARAMETER` (0xC000000D)
- `STATUS_NOT_IMPLEMENTED` (0xC0000002)
- `STATUS_BUFFER_TOO_SMALL` (0xC0000023)
- And others commonly returned by `NtSetInformationProcess` and `NtQueryInformationThread`.

The lookup is implemented as a `match` statement mapping `i32` constants to static string descriptions. Unknown codes fall through to a default branch that formats the value as `"0x{:08X}"`.

### Design choice: hardcoded vs. FormatMessage

Like its companion [`error_from_code_win32`](error_from_code_win32.md), this function uses hardcoded mappings rather than the Windows `FormatMessage` API. This is because:

1. NTSTATUS codes are not well-supported by `FormatMessage` — the `ntdll.dll` message table must be loaded explicitly, and many codes have no message table entry.
2. Hardcoded strings ensure consistent, single-line output regardless of system locale.
3. The set of NTSTATUS codes encountered by this application is small and well-defined.

### When this is called

This function is called from error-handling paths in the apply module when NT-layer APIs return non-zero status codes, specifically:

- [`apply_io_priority`](../apply.rs/apply_io_priority.md) — after `NtQueryInformationProcess` or `NtSetInformationProcess` for I/O priority.
- Thread information queries that use `NtQueryInformationThread`.

The formatted string is incorporated into the error message logged via [`log_error_if_new`](../apply.rs/log_error_if_new.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/error_codes.rs |
| **Source lines** | L47–L70 |
| **Called by** | [`apply_io_priority`](../apply.rs/apply_io_priority.md), [apply.rs](../apply.rs/README.md) error handling paths |
| **Dependencies** | None (pure function) |

## See also

- [error_from_code_win32](error_from_code_win32.md)
- [error_codes.rs module overview](README.md)
- [apply_io_priority](../apply.rs/apply_io_priority.md)
- [log_error_if_new](../apply.rs/log_error_if_new.md)