# error_codes module (AffinityServiceRust)

The `error_codes` module provides human-readable translations for Windows error codes. It maps numeric Win32 error codes and NTSTATUS values to their symbolic constant names (e.g., `5` → `"ACCESS_DENIED"`, `0xC0000022` → `"STATUS_ACCESS_DENIED"`), enabling meaningful log output when Windows API calls fail. Unrecognized codes are formatted as hexadecimal fallback strings.

## Functions

| Function | Description |
|----------|-------------|
| [error_from_code_win32](error_from_code_win32.md) | Maps a Win32 error code (`u32`) to a human-readable symbolic name string. |
| [error_from_ntstatus](error_from_ntstatus.md) | Maps an NTSTATUS code (`i32`) to a human-readable symbolic name string. |

## See Also

| Topic | Link |
|-------|------|
| Logging and error deduplication | [logging module](../logging.rs/README.md) |
| ETW session management (uses error formatting) | [event_trace module](../event_trace.rs/README.md) |
| Rule application (primary consumer of error formatting) | [apply module](../apply.rs/README.md) |
| Windows API wrappers | [winapi module](../winapi.rs/README.md) |