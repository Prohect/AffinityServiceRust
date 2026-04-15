# get_thread_ideal_processor_ex function (winapi.rs)

Retrieves the ideal processor assignment for a thread, returning the processor group and number. This is used to read back the current ideal processor setting before or after applying configuration rules.

## Syntax

```AffinityServiceRust/src/winapi.rs#L673-679
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | A valid thread handle opened with at least `THREAD_QUERY_LIMITED_INFORMATION` access. Typically the `r_limited_handle` or `r_handle` field from a [`ThreadHandle`](ThreadHandle.md). |

## Return value

Returns `Result<PROCESSOR_NUMBER, Error>`.

| Outcome | Description |
|---------|-------------|
| `Ok(PROCESSOR_NUMBER)` | A `PROCESSOR_NUMBER` struct containing the `Group` (processor group number), `Number` (processor number within the group), and `Reserved` fields representing the thread's current ideal processor. |
| `Err(Error)` | A `windows::core::Error` describing why the underlying `GetThreadIdealProcessorEx` Win32 call failed. |

## Remarks

- This function is a thin safe wrapper around the Win32 [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) API. It initializes a default `PROCESSOR_NUMBER` struct and passes it to the API to be populated.

- The ideal processor is a **scheduling hint** to the Windows scheduler indicating which logical processor the thread should preferentially run on. It does not guarantee execution on that processor.

- If the thread has never had an ideal processor explicitly set, the OS returns whatever default ideal processor the scheduler assigned when the thread was created.

- This function does **not** log errors internally. The caller is responsible for handling the `Err` variant and deciding whether to log or propagate the error.

- The companion function [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md) is used to assign the ideal processor, and returns the **previous** ideal processor as part of its success value.

### PROCESSOR_NUMBER layout

| Field | Type | Description |
|-------|------|-------------|
| `Group` | `u16` | The processor group number (0 on systems with ≤ 64 logical processors). |
| `Number` | `u8` | The processor number within the group. |
| `Reserved` | `u8` | Reserved; should be ignored. |

### Platform notes

- **Windows only.** Uses the `windows::Win32::System::Kernel::PROCESSOR_NUMBER` type and `GetThreadIdealProcessorEx` from `processthreadsapi.h`.
- On systems with a single processor group (≤ 64 logical processors), the `Group` field is always `0`.
- On multi-group systems (> 64 logical processors), the `Group` field identifies which processor group the ideal processor belongs to.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` — reads ideal processor before/after setting it for comparison and logging. |
| **Callees** | [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) (Win32 API) |
| **Win32 API** | `kernel32.dll` — `GetThreadIdealProcessorEx` |
| **Privileges** | Requires a valid thread handle with query access. |

## See Also

| Topic | Link |
|-------|------|
| set_thread_ideal_processor_ex | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| ThreadHandle struct | [ThreadHandle](ThreadHandle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| winapi module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
