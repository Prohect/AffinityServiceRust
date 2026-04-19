# error_from_ntstatus function (error_codes.rs)

Maps an NTSTATUS code to a human-readable constant name. Used throughout the project to produce meaningful log messages when native API calls (`Nt*` / `Zw*` functions) return failure status codes.

## Syntax

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

## Parameters

`status: i32`

The NTSTATUS code returned by a native Windows API function. NTSTATUS values are signed 32-bit integers where the high bit indicates severity: values with bit 31 set (negative when interpreted as `i32`) are error codes, zero is success, and small positive values are informational.

## Return value

`String` — The symbolic name of the status code (e.g., `"STATUS_ACCESS_DENIED"`). If the code is not in the lookup table, returns a formatted fallback string in the form `"NTSTATUS_0x{code:08X}"` using the unsigned representation.

## Remarks

### Recognised codes

The function recognises the following NTSTATUS codes via a `match` on the unsigned representation of `status` (obtained via `i32::cast_unsigned`):

| Code | Name |
|------|------|
| `0x00000000` | `STATUS_SUCCESS` |
| `0x00000001` | `STATUS_WAIT_1` |
| `0xC0000001` | `STATUS_UNSUCCESSFUL` |
| `0xC0000002` | `STATUS_NOT_IMPLEMENTED` |
| `0xC0000003` | `STATUS_INVALID_INFO_CLASS` |
| `0xC0000004` | `STATUS_INFO_LENGTH_MISMATCH` |
| `0xC0000008` | `STATUS_INVALID_HANDLE` |
| `0xC000000D` | `STATUS_INVALID_PARAMETER` |
| `0xC0000017` | `STATUS_NO_MEMORY` |
| `0xC0000018` | `STATUS_CONFLICTING_ADDRESSES` |
| `0xC0000022` | `STATUS_ACCESS_DENIED` |
| `0xC0000023` | `STATUS_BUFFER_TOO_SMALL` |
| `0xC0000034` | `STATUS_OBJECT_NAME_NOT_FOUND` |
| `0xC000004B` | `STATUS_THREAD_IS_TERMINATING` |
| `0xC0000061` | `STATUS_PRIVILEGE_NOT_HELD` |
| `0xC00000BB` | `STATUS_NOT_SUPPORTED` |
| `0xC000010A` | `STATUS_PROCESS_IS_TERMINATING` |

### Unsigned conversion

The function uses `i32::cast_unsigned(status)` to convert the signed `i32` to its unsigned `u32` bit-equivalent before matching. This avoids the need for negative literal patterns and matches the conventional hexadecimal representation of NTSTATUS codes as used in the Windows SDK headers (e.g., `ntstatus.h`).

### Relationship to error_from_code_win32

This function is the NTSTATUS counterpart to [error_from_code_win32](error_from_code_win32.md), which handles Win32 error codes. The two functions cover different error code spaces:

- **Win32 errors** — returned by functions like `GetLastError()`, typically small positive integers.
- **NTSTATUS codes** — returned by `Nt*`/`Zw*` native API functions, typically `0xC*` values for errors.

Callers choose the appropriate function based on which API produced the error code.

### Fallback format

Unknown codes are formatted as `"NTSTATUS_0x{code:08X}"` where `code` is cast to `u32` for display, ensuring consistent unsigned hexadecimal output regardless of the sign of the input.

## Requirements

| | |
|---|---|
| **Module** | `src/error_codes.rs` |
| **Callers** | [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md), native API wrappers in [winapi.rs](../winapi.rs/README.md) |
| **Callees** | None |
| **Win32 API** | None (pure lookup function) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [error_codes.rs](README.md) |
| Win32 error lookup | [error_from_code_win32](error_from_code_win32.md) |
| IO priority application | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| Memory priority application | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*