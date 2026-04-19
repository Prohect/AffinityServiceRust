# get_thread_ideal_processor_ex function (winapi.rs)

Retrieves the current ideal processor assignment for a thread. Wraps the Windows `GetThreadIdealProcessorEx` API, returning the processor group and number as a `PROCESSOR_NUMBER` structure.

## Syntax

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | A valid thread handle with `THREAD_QUERY_LIMITED_INFORMATION` or `THREAD_QUERY_INFORMATION` access. Typically sourced from [ThreadHandle](ThreadHandle.md)`.r_limited_handle` or `.r_handle`. |

## Return value

`Result<PROCESSOR_NUMBER, Error>` — On success, returns a `PROCESSOR_NUMBER` structure containing:

| Field | Type | Description |
|-------|------|-------------|
| `Group` | `u16` | The processor group of the thread's ideal processor. On single-group systems (≤64 logical processors), this is always `0`. |
| `Number` | `u8` | The logical processor number within the group that is the thread's current ideal processor. |
| `Reserved` | `u8` | Reserved; always `0`. |

On failure, returns a `windows::core::Error` containing the underlying Win32 error code (e.g., `ERROR_INVALID_HANDLE` if the handle is invalid or lacks the required access right).

## Remarks

- The function allocates a default `PROCESSOR_NUMBER` on the stack and passes it to `GetThreadIdealProcessorEx` as an out-parameter. On success, the filled structure is returned.
- The ideal processor is a scheduling *hint* — Windows prefers to run the thread on the specified processor but may schedule it elsewhere under load. This function reads the current hint, which may have been set by the OS, by the application itself, or by a prior call to [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md).
- This function is used by the [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) logic to read the current ideal processor before deciding whether an update is needed. The returned group and number are compared against the [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) to avoid redundant `SetThreadIdealProcessorEx` calls.

### Relationship to set_thread_ideal_processor_ex

| Function | Direction | API |
|----------|-----------|-----|
| **get_thread_ideal_processor_ex** | Read | `GetThreadIdealProcessorEx` |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | Write | `SetThreadIdealProcessorEx` |

Both functions operate on the same per-thread ideal processor attribute. The get variant requires only read access; the set variant requires write access (`THREAD_SET_INFORMATION`).

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **Callees** | None (thin Win32 wrapper) |
| **Win32 API** | [GetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |
| **Privileges** | `THREAD_QUERY_LIMITED_INFORMATION` or `THREAD_QUERY_INFORMATION` access on the thread handle |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Write counterpart | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| Ideal processor tracking state | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| Thread handle wrapper | [ThreadHandle](ThreadHandle.md) |
| Ideal processor application logic | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*