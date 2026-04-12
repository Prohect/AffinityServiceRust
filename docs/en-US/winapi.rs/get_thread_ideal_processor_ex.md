# get_thread_ideal_processor_ex function (winapi.rs)

Queries the current ideal processor assignment for a thread, returning the processor group and number.

## Syntax

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## Parameters

`thread_handle`

A `HANDLE` to the target thread, opened with at least `THREAD_QUERY_INFORMATION` access. This is typically the `r_handle` field of a [`ThreadHandle`](ThreadHandle.md).

## Return value

Returns `Ok(PROCESSOR_NUMBER)` on success, containing the thread's current ideal processor group and number. Returns `Err(Error)` if the underlying Windows API call fails.

The `PROCESSOR_NUMBER` structure contains:

- `Group` — the processor group (typically 0 on systems with fewer than 64 logical processors).
- `Number` — the zero-based processor number within the group.

## Remarks

This function wraps the Windows `GetThreadIdealProcessorEx` API. It retrieves the processor that the Windows scheduler will prefer when scheduling the specified thread, as previously set by [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md) or by the system's default assignment.

The ideal processor is a scheduling hint — it does not guarantee that the thread will always run on that processor. The scheduler uses it to improve cache locality by preferring the specified processor when it is available.

This function is used by the [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) function to read back the current ideal processor assignment before deciding whether a change is needed. By comparing the current assignment to the desired one, the application avoids unnecessary API calls when the thread is already assigned to the correct processor.

**Error handling:** If the call fails (e.g., due to an invalid handle or insufficient access rights), the function returns the Windows error wrapped in a `windows::core::Error`. The caller is responsible for handling or logging the error, typically via [`log_error_if_new`](../apply.rs/log_error_if_new.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L671–L677 |
| **Called by** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) |
| **Windows API** | [GetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |

## See also

- [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md)
- [ThreadHandle](ThreadHandle.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)
- [winapi.rs module overview](README.md)