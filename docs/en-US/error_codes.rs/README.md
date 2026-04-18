# error_codes module (AffinityServiceRust)

The `error_codes` module provides human-readable string translations for Windows error codes. It contains two lookup functions that map numeric Win32 error codes and NTSTATUS values to their well-known symbolic names (e.g., `ACCESS_DENIED`, `STATUS_INVALID_HANDLE`). Unknown codes are formatted as hexadecimal strings. These functions are used throughout AffinityServiceRust for diagnostic logging and error reporting.

## Functions

| Function | Description |
|----------|-------------|
| [error_from_code_win32](error_from_code_win32.md) | Converts a `u32` Win32 error code to its symbolic name string. |
| [error_from_ntstatus](error_from_ntstatus.md) | Converts an `i32` NTSTATUS value to its symbolic name string. |

## See Also

| Topic | Link |
|-------|------|
| winapi module | [../winapi.rs/README.md](../winapi.rs/README.md) |
| logging module | [../logging.rs/README.md](../logging.rs/README.md) |
| event_trace module | [../event_trace.rs/README.md](../event_trace.rs/README.md) |

---

*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
