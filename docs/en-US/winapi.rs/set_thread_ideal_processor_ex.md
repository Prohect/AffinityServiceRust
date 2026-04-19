# set_thread_ideal_processor_ex function (winapi.rs)

Wrapper around the Windows `SetThreadIdealProcessorEx` API that sets the ideal processor hint for a thread, specifying both the processor group and the logical processor number within that group. Returns the previous ideal processor assignment.

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
| `thread_handle` | `HANDLE` | A valid thread handle with `THREAD_SET_INFORMATION` access. Typically sourced from [ThreadHandle](ThreadHandle.md)`.w_handle`. The caller must verify the handle is not invalid before calling this function. |
| `group` | `u16` | The processor group number for the ideal processor. On single-group systems (≤64 logical processors), this is always `0`. |
| `number` | `u8` | The logical processor number within the specified `group` to set as the thread's ideal processor. For example, `0` targets the first processor in the group. |

## Return value

`Result<PROCESSOR_NUMBER, Error>` — On success, returns the thread's **previous** ideal processor as a `PROCESSOR_NUMBER` struct (containing `Group`, `Number`, and `Reserved` fields). On failure, returns a Windows `Error` with the underlying Win32 error code.

## Remarks

- The function constructs a `PROCESSOR_NUMBER` struct from the `group` and `number` parameters (with `Reserved` set to `0`) and passes it to `SetThreadIdealProcessorEx`. A mutable `previous` output parameter captures the prior ideal processor assignment returned by the API.
- The ideal processor is a **scheduling hint**, not a hard constraint. The Windows scheduler prefers to schedule the thread on the indicated processor but may place it on any processor within the thread's affinity mask if the ideal processor is busy. For hard CPU pinning, use CPU Sets via `SetThreadSelectedCpuSets`.
- This function is called by [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) and [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) to guide the scheduler toward specific cores for latency-sensitive threads.
- The previous ideal processor returned by this function is used by [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) to track assignment changes across polling iterations.

### Error scenarios

| Condition | Behavior |
|-----------|----------|
| Invalid thread handle | Returns `Err(Error)` with `ERROR_INVALID_HANDLE` |
| Handle lacks `THREAD_SET_INFORMATION` access | Returns `Err(Error)` with `ERROR_ACCESS_DENIED` |
| Invalid group or number (exceeds system topology) | Returns `Err(Error)` — behavior is OS-version-dependent |

### Relationship to get_thread_ideal_processor_ex

This function is the write counterpart to [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md). Together they form a read/write pair for managing thread ideal processor hints. The service typically reads the current assignment to detect changes from external tools, then writes a new assignment when the configuration dictates a different processor.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **Callees** | `SetThreadIdealProcessorEx` (Win32) |
| **Win32 API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **Privileges** | Requires `THREAD_SET_INFORMATION` access on the thread handle |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Read counterpart | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| Ideal processor state tracking | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| Thread handle providing write access | [ThreadHandle](ThreadHandle.md) |
| Apply function that sets ideal processors | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*