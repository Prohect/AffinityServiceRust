# set_thread_ideal_processor_ex function (winapi.rs)

Sets a thread's ideal processor to a specific processor number within a specific processor group. This is the group-aware variant of `SetThreadIdealProcessor` and is used by AffinityServiceRust to pin prime threads and assign ideal processors according to configuration rules on systems with one or more processor groups.

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
| `thread_handle` | `HANDLE` | A valid thread handle with `THREAD_SET_INFORMATION` access. Typically obtained from the `w_handle` field of a [ThreadHandle](ThreadHandle.md). |
| `group` | `u16` | The zero-based processor group number to assign. On single-group systems (≤ 64 logical processors), this is always `0`. |
| `number` | `u8` | The zero-based processor number within the group. For example, on a system with 16 cores in group 0, valid values are `0` through `15`. |

## Return value

| Value | Description |
|-------|-------------|
| `Ok(PROCESSOR_NUMBER)` | The thread's **previous** ideal processor, returned by the underlying `SetThreadIdealProcessorEx` call. The caller can use this to restore the original ideal processor later if needed. The returned `PROCESSOR_NUMBER` contains `Group`, `Number`, and `Reserved` fields. |
| `Err(Error)` | The Win32 call failed. The `Error` value wraps the underlying Windows error code. Common causes include an invalid handle, insufficient access rights (`THREAD_SET_INFORMATION` not granted), or the thread having exited. |

## Remarks

### Implementation

The function constructs a `PROCESSOR_NUMBER` struct with the specified `Group` and `Number` (and `Reserved` set to `0`), then calls the Win32 `SetThreadIdealProcessorEx` function. A mutable `PROCESSOR_NUMBER` is passed as the `lpPreviousIdealProcessor` out-parameter to capture the thread's prior ideal processor assignment.

### Ideal processor semantics

Setting a thread's ideal processor is a **hint** to the Windows scheduler, not a hard constraint. The scheduler prefers to schedule the thread on the ideal processor when it is available but will schedule the thread on other allowed processors when the ideal one is busy. For hard CPU pinning, use CPU sets via `SetThreadSelectedCpuSets` or process affinity via `SetProcessAffinityMask`.

### Relationship to get_thread_ideal_processor_ex

This function is the setter counterpart to [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md). Together they allow the service to read, modify, and potentially restore ideal processor assignments:

1. Read the current ideal processor with [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md).
2. Set a new ideal processor with `set_thread_ideal_processor_ex`.
3. The previous value is returned, which can be stored for later restoration.

### Usage in the apply module

The [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) and [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) functions call `set_thread_ideal_processor_ex` to direct hot threads toward preferred cores (e.g., performance cores on hybrid architectures). The [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) function uses it to reset ideal processors back to a round-robin assignment when CPU set changes require rebalancing.

### IdealProcessorState tracking

The [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) struct in the scheduler module tracks both the current and previous ideal processor assignments for each thread, enabling the service to detect when reassignment is needed and to avoid redundant Win32 calls.

### Handle requirements

The thread handle **must** have `THREAD_SET_INFORMATION` access. This corresponds to the `w_handle` field of [ThreadHandle](ThreadHandle.md). If `w_handle` is invalid (the access right was not granted during [get_thread_handle](get_thread_handle.md)), callers should skip the call rather than passing an invalid handle.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **Callees** | `SetThreadIdealProcessorEx` (Win32 `kernel32.dll`) |
| **API** | [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **Privileges** | `SeDebugPrivilege` may be required to set the ideal processor on threads in processes owned by other users |

## See Also

| Topic | Link |
|-------|------|
| Ideal processor getter | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| Thread handle RAII container | [ThreadHandle](ThreadHandle.md) |
| Thread handle acquisition | [get_thread_handle](get_thread_handle.md) |
| Ideal processor state tracking | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| Ideal processor application logic | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| Prime thread promotion | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| Ideal processor reset | [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| SetThreadIdealProcessorEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd