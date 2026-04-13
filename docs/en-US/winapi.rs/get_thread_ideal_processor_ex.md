# get_thread_ideal_processor_ex function (winapi.rs)

Retrieves the current ideal processor for a thread, including processor group information. This is the group-aware counterpart to the legacy `GetThreadIdealProcessorEx` Win32 API wrapper, returning a `PROCESSOR_NUMBER` that identifies both the processor group and the processor number within that group.

## Syntax

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | A valid thread handle with `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` access. Typically obtained from the `r_handle` field of a [ThreadHandle](ThreadHandle.md). |

## Return value

| Value | Description |
|-------|-------------|
| `Ok(PROCESSOR_NUMBER)` | A `PROCESSOR_NUMBER` structure containing the thread's current ideal processor assignment. The `Group` field identifies the processor group (0-based), and the `Number` field identifies the processor within that group (0-based). |
| `Err(Error)` | The underlying `GetThreadIdealProcessorEx` Win32 call failed. The `Error` contains the Win32 error code. Common causes include an invalid or insufficiently-privileged handle. |

## Remarks

### PROCESSOR_NUMBER structure

The returned `PROCESSOR_NUMBER` contains:

| Field | Type | Description |
|-------|------|-------------|
| `Group` | `u16` | The processor group index (0 on most single-group systems). |
| `Number` | `u8` | The processor number within the group. |
| `Reserved` | `u8` | Reserved by the system; not meaningful to callers. |

### Relationship to set_thread_ideal_processor_ex

This function is the read counterpart to [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md). Together they form a get/set pair for managing thread ideal processor assignments:

- **Get** — `get_thread_ideal_processor_ex` reads the current ideal processor.
- **Set** — [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) writes a new ideal processor and returns the previous value.

### Usage in the scheduler

The [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) function and the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) use this function to read back a thread's current ideal processor assignment before deciding whether to change it. This avoids redundant calls to `SetThreadIdealProcessorEx` when the thread is already assigned to the desired processor.

### Thread handle requirements

The function requires a handle with at least `THREAD_QUERY_LIMITED_INFORMATION` access. In the [ThreadHandle](ThreadHandle.md) struct, the `r_handle` field (opened with `THREAD_QUERY_INFORMATION`) is typically used. If only `r_limited_handle` is available, the call may still succeed depending on the Windows version and the target thread's protection level.

### Multi-group systems

On systems with multiple processor groups (more than 64 logical processors), the `Group` field distinguishes processors that share the same `Number` value but belong to different groups. On single-group systems (the common case for desktop PCs), `Group` is always `0` and `Number` directly corresponds to the logical processor index.

### Error handling

The function propagates the `windows::core::Error` from the underlying Win32 call. Callers in the apply module typically log the error via the `is_new_error` deduplication system and continue processing other threads, rather than aborting the entire apply cycle.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| **Callees** | `GetThreadIdealProcessorEx` (Win32 `kernel32.dll`) |
| **API** | [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |
| **Privileges** | Thread handle must have `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` access |

## See Also

| Topic | Link |
|-------|------|
| Ideal processor setter | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| Thread handle container | [ThreadHandle](ThreadHandle.md) |
| Ideal processor state tracking | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| Ideal processor assignment logic | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| Thread statistics | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| GetThreadIdealProcessorEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |