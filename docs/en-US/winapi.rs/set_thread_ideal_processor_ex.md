# set_thread_ideal_processor_ex function (winapi.rs)

Sets the ideal processor for a thread, specifying both a processor group and a processor number within that group. The ideal processor is a scheduling hint to the Windows kernel indicating which logical processor the thread prefers to run on.

## Syntax

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, Error>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | A valid thread handle opened with `THREAD_SET_INFORMATION` access. Typically the `w_handle` field from a [`ThreadHandle`](ThreadHandle.md). |
| `group` | `u16` | The processor group number (zero-based). On systems with 64 or fewer logical processors, this is always `0`. |
| `number` | `u8` | The processor number within the specified group (zero-based). This is the logical processor index relative to the group. |

## Return value

On success, returns `Ok(PROCESSOR_NUMBER)` containing the **previous** ideal processor setting for the thread. The returned `PROCESSOR_NUMBER` includes `Group`, `Number`, and `Reserved` fields.

On failure, returns `Err(Error)` containing the Windows error from `SetThreadIdealProcessorEx`.

## Remarks

- The function constructs a `PROCESSOR_NUMBER` struct with the provided `group` and `number` values (and `Reserved` set to `0`), then calls `SetThreadIdealProcessorEx` from the Win32 API.

- The ideal processor is a **hint**, not a hard constraint. The Windows scheduler may still run the thread on a different processor if the ideal processor is busy. For hard affinity constraints, use CPU set APIs (`SetProcessDefaultCpuSets`, `SetThreadSelectedCpuSets`) or affinity masks instead.

- The previous ideal processor is returned so that callers can restore it later or log the change. This is provided directly by the Win32 `SetThreadIdealProcessorEx` API through its output parameter.

- This function requires the thread handle to have `THREAD_SET_INFORMATION` access. If the [`ThreadHandle`](ThreadHandle.md) struct's `w_handle` field is invalid (the open call failed), callers must not pass it to this function. Check `HANDLE::is_invalid()` before calling.

- Unlike the legacy `SetThreadIdealProcessor` API, `SetThreadIdealProcessorEx` supports processor groups, making it suitable for systems with more than 64 logical processors.

### Platform notes

- **Windows only.** Uses `SetThreadIdealProcessorEx` from `windows::Win32::System::Threading`.
- Requires Windows 7 / Windows Server 2008 R2 or later.
- On single-group systems (≤ 64 logical processors), `group` should always be `0`.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` — ideal processor assignment logic for module-aware thread placement |
| **Callees** | [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) (Win32 API) |
| **Win32 API** | `kernel32.dll` — `SetThreadIdealProcessorEx` |
| **Privileges** | Requires a thread handle with `THREAD_SET_INFORMATION` access. `SeDebugPrivilege` may be needed for threads owned by other users. |
| **Platform** | Windows 7+ / Windows Server 2008 R2+ |

## See Also

| Topic | Link |
|-------|------|
| get_thread_ideal_processor_ex | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| ThreadHandle struct | [ThreadHandle](ThreadHandle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| winapi module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
