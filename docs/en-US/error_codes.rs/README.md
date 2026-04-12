# error_codes.rs Module (error_codes.rs)

The `error_codes` module provides human-readable translations for Windows error codes and NTSTATUS values encountered during process and thread manipulation.

## Overview

When Windows API calls fail, they return numeric error codes that are difficult to interpret at a glance. This module maps the most commonly encountered codes to descriptive English strings, enabling meaningful log output for the user.

Two families of error codes are covered:

- **Win32 error codes** (`u32`) — returned by `GetLastError()` after Win32 API failures. Approximately 45 common codes are mapped.
- **NTSTATUS codes** (`i32`) — returned by NT-layer APIs such as `NtSetInformationProcess` and `NtQueryInformationProcess`. Approximately 17 common codes are mapped.

Unknown codes are formatted as hexadecimal strings so they can still be looked up manually.

## Items

### Functions

| Name | Description |
| --- | --- |
| [error_from_code_win32](error_from_code_win32.md) | Maps a Win32 error code to a human-readable string. |
| [error_from_ntstatus](error_from_ntstatus.md) | Maps an NTSTATUS code to a human-readable string. |

## Remarks

This module is a pure lookup utility with no state and no side effects. It is called from error-handling paths throughout the project, most notably from [log_error_if_new](../logging.rs/log_error_if_new.md) and the various `apply_*` functions in [apply.rs](../apply.rs/README.md).

The code lists are intentionally hardcoded rather than using `FormatMessage`, because `FormatMessage` output varies by system locale and can produce multi-line strings that are unsuitable for single-line log entries.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/error_codes.rs` |
| **Lines** | L1–L70 |
| **Called by** | [log_error_if_new](../logging.rs/log_error_if_new.md), [apply.rs](../apply.rs/README.md) functions, [winapi.rs](../winapi.rs/README.md) functions |
| **Dependencies** | None (pure function module) |

## See also

- [logging.rs module overview](../logging.rs/README.md)
- [apply.rs module overview](../apply.rs/README.md)