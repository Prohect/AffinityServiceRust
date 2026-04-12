# ThreadHandle struct (winapi.rs)

Holds read and write `HANDLE`s to a Windows thread at both limited and full access levels. Used for querying and modifying thread properties such as ideal processor, cycle time, and CPU set assignments.

## Syntax

```rust
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## Members

`r_limited_handle`

A read handle opened with `THREAD_QUERY_LIMITED_INFORMATION` access. This handle is always valid when the `ThreadHandle` is constructed. Used for lightweight queries such as thread cycle time.

`r_handle`

A read handle opened with `THREAD_QUERY_INFORMATION` access. Used for queries that require full information access, such as [`get_thread_start_address`](get_thread_start_address.md) via `NtQueryInformationThread`.

`w_limited_handle`

A write handle opened with `THREAD_SET_LIMITED_INFORMATION` access. Used for operations that accept limited-access handles.

`w_handle`

A write handle opened with `THREAD_SET_INFORMATION` access. Used for operations requiring full write access, such as [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md) and `SetThreadSelectedCpuSets`.

## Remarks

Unlike [`ProcessHandle`](ProcessHandle.md), where full handles (`r_handle`, `w_handle`) are `Option<HANDLE>` because protected processes may deny elevated access, all four handles in `ThreadHandle` are non-optional `HANDLE` values. Thread handles are generally more accessible than process handles, so all access levels are expected to succeed.

`ThreadHandle` is returned by [`get_thread_handle`](get_thread_handle.md) and is stored in [`ThreadStats`](../scheduler.rs/ThreadStats.md) for reuse across loop iterations. The handle is opened once when a thread is first encountered and cached for subsequent iterations.

Individual thread handles are opened by [`try_open_thread`](try_open_thread.md), which handles error logging and deduplication internally.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Lines** | L197–L204 |
| **Returned by** | [`get_thread_handle`](get_thread_handle.md) |
| **Stored in** | [`ThreadStats`](../scheduler.rs/ThreadStats.md) in the scheduler module |

## See also

- [ProcessHandle](ProcessHandle.md)
- [get_thread_handle](get_thread_handle.md)
- [try_open_thread](try_open_thread.md)
- [winapi.rs module overview](README.md)