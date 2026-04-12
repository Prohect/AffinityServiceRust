# get_thread_handle function (winapi.rs)

Opens a thread by its thread ID and returns a [`ThreadHandle`](ThreadHandle.md) containing multiple HANDLEs at different access levels.

## Syntax

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## Parameters

`tid`

The thread identifier of the target thread.

`pid`

The process identifier of the process that owns the target thread. Used for error logging context.

`process_name`

The display name of the owning process. Used for error logging context and deduplication via [`is_new_error`](../logging.rs/is_new_error.md).

## Return value

Returns `Some(ThreadHandle)` if the thread was successfully opened with at least limited read access. Returns `None` if the thread could not be opened at all.

## Remarks

This function opens multiple handles to the same thread at different access levels using [`try_open_thread`](try_open_thread.md):

1. **`r_limited_handle`** — opened with `THREAD_QUERY_LIMITED_INFORMATION`. This is the minimum access level and is always expected to succeed for accessible threads.
2. **`r_handle`** — opened with `THREAD_QUERY_INFORMATION`. Required for operations like [`get_thread_start_address`](get_thread_start_address.md) that need full query access.
3. **`w_limited_handle`** — opened with `THREAD_SET_LIMITED_INFORMATION`. Used for operations that require basic write access.
4. **`w_handle`** — opened with `THREAD_SET_INFORMATION`. Required for operations like [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md).

If the limited read handle cannot be opened, the function returns `None` since no useful operations can be performed on the thread. The full-access handles may fail to open for protected or restricted threads, but the function still returns a valid [`ThreadHandle`](ThreadHandle.md) with whatever access was obtained.

Unlike [`ProcessHandle`](ProcessHandle.md), the [`ThreadHandle`](ThreadHandle.md) stores all four handles directly (not as `Option`), though some may be invalid HANDLEs when access was denied.

Errors during handle opening are logged via [`is_new_error`](../logging.rs/is_new_error.md) with the corresponding [`Operation`](../logging.rs/Operation.md) variant to prevent duplicate log entries.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L229–L271 |
| **Called by** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md), [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| **Calls** | [`try_open_thread`](try_open_thread.md), [`is_new_error`](../logging.rs/is_new_error.md) |
| **Windows API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |

## See also

- [ThreadHandle](ThreadHandle.md)
- [get_process_handle](get_process_handle.md)
- [try_open_thread](try_open_thread.md)