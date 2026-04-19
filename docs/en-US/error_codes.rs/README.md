# error_codes module (AffinityServiceRust)

The `error_codes` module provides human-readable error message lookup for Win32 and NTSTATUS error codes. Instead of displaying raw numeric codes in log output, the rest of the application calls these functions to produce familiar symbolic names (e.g., `ACCESS_DENIED`, `STATUS_INVALID_HANDLE`) that match the constants defined in the Windows SDK headers. Unknown codes fall back to a hexadecimal formatted string.

## Functions

| Name | Description |
|------|-------------|
| [error_from_code_win32](error_from_code_win32.md) | Maps a Win32 `u32` error code to its symbolic name (e.g., `5` → `"ACCESS_DENIED"`). Returns `"WIN32_ERROR_CODE_0x{code:08X}"` for unmapped codes. |
| [error_from_ntstatus](error_from_ntstatus.md) | Maps an NTSTATUS `i32` status code to its symbolic name (e.g., `0xC0000022` → `"STATUS_ACCESS_DENIED"`). Returns `"NTSTATUS_0x{code:08X}"` for unmapped codes. |

## Remarks

Both functions use a `match` statement against a curated subset of error codes commonly encountered when manipulating process handles, thread priorities, affinity masks, and other Windows resource management APIs. The subset is intentionally small — only codes that have been observed during normal operation or debugging of AffinityServiceRust are included. This keeps the lookup fast (compiler-generated jump table) and avoids pulling in the full Windows SDK error catalog.

The functions are pure — they perform no I/O, hold no state, and are safe to call from any context.

## See Also

| Topic | Link |
|-------|------|
| Error deduplication in apply module | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| Apply module (primary consumer) | [apply.rs](../apply.rs/README.md) |
| Windows API wrappers | [winapi.rs](../winapi.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*