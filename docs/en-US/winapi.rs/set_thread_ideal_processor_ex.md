# set_thread_ideal_processor_ex function (winapi.rs)

Sets the ideal processor for a thread, specifying both the processor group and the processor number within that group.

## Syntax

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, Error>
```

## Parameters

`thread_handle`

A `HANDLE` to the target thread, opened with `THREAD_SET_INFORMATION` access. This is typically the `w_handle` field of a [`ThreadHandle`](ThreadHandle.md).

`group`

The processor group number to assign. On systems with a single processor group (most desktop systems with fewer than 64 logical processors), this is `0`.

`number`

The zero-based processor number within the specified group. This identifies the specific logical processor to set as the thread's ideal processor.

## Return value

On success, returns `Ok(PROCESSOR_NUMBER)` containing the thread's **previous** ideal processor assignment (group and number). This allows the caller to restore the original assignment if needed.

On failure, returns `Err(Error)` with the underlying Windows error. Common failure causes include invalid handles, access denied, or invalid processor numbers.

## Remarks

This function wraps the Windows API `SetThreadIdealProcessorEx`, which sets the preferred processor for thread scheduling. The ideal processor is a scheduling hint — the Windows scheduler will attempt to schedule the thread on the specified processor when it is available, but may schedule it elsewhere under load.

The function constructs a `PROCESSOR_NUMBER` struct from the provided `group` and `number` parameters, calls `SetThreadIdealProcessorEx`, and returns the previous ideal processor setting that the API provides as an output parameter.

This is used by two subsystems in the application:

- **Ideal processor assignment** — [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) uses this to assign threads to specific CPUs based on their start module prefix (e.g., assigning render threads to performance cores).
- **Prime thread scheduling** — [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) uses this to pin the highest-activity threads to designated fast cores.
- **Ideal processor reset** — [`reset_thread_ideal_processors`](../apply.rs/reset_thread_ideal_processors.md) redistributes ideal processors after affinity or CPU set changes.

The previous ideal processor returned by the function is stored in [`IdealProcessorState`](../scheduler.rs/IdealProcessorState.md) within [`ThreadStats`](../scheduler.rs/ThreadStats.md), enabling the application to detect changes made by the OS or other tools and to restore the original assignment when demoting threads.

### Processor groups

On systems with more than 64 logical processors, Windows organizes CPUs into processor groups. The `group` parameter selects which group, and `number` selects the processor within that group. Most consumer systems have a single group (group 0), but server and HEDT systems may have multiple groups.

The companion function [`get_thread_ideal_processor_ex`](get_thread_ideal_processor_ex.md) queries the current ideal processor assignment.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L658–L669 |
| **Called by** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md), [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md), [`reset_thread_ideal_processors`](../apply.rs/reset_thread_ideal_processors.md) |
| **Windows API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |

## See also

- [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md)
- [ThreadHandle](ThreadHandle.md)
- [IdealProcessorState](../scheduler.rs/IdealProcessorState.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)
- [winapi.rs module overview](README.md)