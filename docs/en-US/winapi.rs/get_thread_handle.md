# get_thread_handle function (winapi.rs)

Opens a set of Windows thread handles at multiple access levels for a given thread ID. Returns a [ThreadHandle](ThreadHandle.md) RAII wrapper that automatically closes all opened handles on drop. The function requires `THREAD_QUERY_LIMITED_INFORMATION` as a minimum; other access levels are attempted but allowed to fail gracefully.

## Syntax

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `tid` | `u32` | The thread identifier of the target thread. |
| `pid` | `u32` | The process identifier that owns the thread. Used only for error logging and deduplication via [is_new_error](../logging.rs/is_new_error.md). |
| `process_name` | `&str` | The name of the owning process. Used only for error logging context. |

## Return value

`Option<ThreadHandle>` — Returns `Some(ThreadHandle)` if the required `r_limited_handle` was successfully opened. Returns `None` if the minimum-required handle could not be obtained.

When `Some` is returned, the [ThreadHandle](ThreadHandle.md) contains:

| Handle field | Access right | Required | Behavior on failure |
|---|---|---|---|
| `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` | **Yes** | Function returns `None` |
| `r_handle` | `THREAD_QUERY_INFORMATION` | No | Set to an invalid `HANDLE` |
| `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` | No | Set to an invalid `HANDLE` |
| `w_handle` | `THREAD_SET_INFORMATION` | No | Set to an invalid `HANDLE` |

## Remarks

The function follows a tiered handle-opening strategy:

1. **Required handle** — `THREAD_QUERY_LIMITED_INFORMATION` is opened first via `OpenThread`. If this call fails or returns an invalid handle, the error is logged (once per unique pid/tid/operation/error combination via [is_new_error](../logging.rs/is_new_error.md)) and the function returns `None` immediately.

2. **Optional handles** — `THREAD_QUERY_INFORMATION`, `THREAD_SET_LIMITED_INFORMATION`, and `THREAD_SET_INFORMATION` are each attempted via [try_open_thread](try_open_thread.md). Failures for these handles are silently absorbed (error logging is commented out in the source), and the corresponding field is set to `HANDLE::default()` (an invalid handle). Callers must check `is_invalid()` before using these handles.

### Error code mapping

Each handle open attempt is assigned an internal operation code for error deduplication:

| Code | Handle | Access right |
|------|--------|-------------|
| `0` | `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` |
| `1` | `r_handle` | `THREAD_QUERY_INFORMATION` |
| `2` | `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` |
| `3` | `w_handle` | `THREAD_SET_INFORMATION` |

### Handle lifetime

All returned handles are owned by the [ThreadHandle](ThreadHandle.md) struct. They are closed automatically via `CloseHandle` when the `ThreadHandle` is dropped. Callers should not manually close these handles.

### Typical usage

Thread handles are typically opened once and cached in [ThreadStats::handle](../scheduler.rs/ThreadStats.md) for reuse across polling iterations. This avoids the overhead of calling `OpenThread` every cycle. The handle cache is cleared when the owning process exits.

### Access denied scenarios

When running without [SeDebugPrivilege](enable_debug_privilege.md), threads belonging to elevated or protected processes may reject the `THREAD_QUERY_LIMITED_INFORMATION` request with `ERROR_ACCESS_DENIED` (5), causing the function to return `None`. Enabling SeDebugPrivilege at startup resolves this for most processes.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [get_handles](../apply.rs/get_handles.md), [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) |
| **Callees** | [try_open_thread](try_open_thread.md), [is_new_error](../logging.rs/is_new_error.md) |
| **Win32 API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |
| **Privileges** | None required; [SeDebugPrivilege](enable_debug_privilege.md) recommended for full access |

## See Also

| Topic | Link |
|-------|------|
| Thread handle RAII wrapper | [ThreadHandle](ThreadHandle.md) |
| Lower-level thread open helper | [try_open_thread](try_open_thread.md) |
| Process handle equivalent | [get_process_handle](get_process_handle.md) |
| Thread stats that cache handles | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Module overview | [winapi.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*