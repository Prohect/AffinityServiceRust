# get_thread_handle function (winapi.rs)

Opens a thread with multiple access levels and returns a [ThreadHandle](ThreadHandle.md) RAII container. The function requires `THREAD_QUERY_LIMITED_INFORMATION` as the minimum access right; if this fails, the function returns `None`. The remaining three access levels (`THREAD_QUERY_INFORMATION`, `THREAD_SET_LIMITED_INFORMATION`, `THREAD_SET_INFORMATION`) are attempted but their failure is non-fatal — the corresponding handle fields are set to invalid handles.

## Syntax

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `tid` | `u32` | The thread identifier of the thread to open. |
| `pid` | `u32` | The process identifier that owns the thread. Used only for error logging and deduplication via `is_new_error`. |
| `process_name` | `&str` | The image name of the owning process (e.g., `"explorer.exe"`). Used only for error logging and deduplication via `is_new_error`. |

## Return value

| Value | Description |
|-------|-------------|
| `Some(ThreadHandle)` | A [ThreadHandle](ThreadHandle.md) whose `r_limited_handle` is guaranteed valid. The `r_handle`, `w_limited_handle`, and `w_handle` fields may be invalid handles if the corresponding `OpenThread` calls failed. |
| `None` | The `THREAD_QUERY_LIMITED_INFORMATION` open failed or returned an invalid handle. An error is logged via `log_to_find` on the first occurrence for this PID/TID/operation combination. |

## Remarks

### Handle acquisition strategy

The function opens four handles in sequence, each requesting a different access right:

| Order | Access right | Field | Required |
|-------|-------------|-------|----------|
| 1 | `THREAD_QUERY_LIMITED_INFORMATION` | `r_limited_handle` | **Yes** — failure returns `None` |
| 2 | `THREAD_QUERY_INFORMATION` | `r_handle` | No — invalid handle on failure |
| 3 | `THREAD_SET_LIMITED_INFORMATION` | `w_limited_handle` | No — invalid handle on failure |
| 4 | `THREAD_SET_INFORMATION` | `w_handle` | No — invalid handle on failure |

The first handle is opened directly via `OpenThread`. The remaining three are opened through the helper function [try_open_thread](try_open_thread.md), which returns `HANDLE::default()` (an invalid handle) on failure instead of propagating an error.

### Error logging

Each failed `OpenThread` call is checked against the per-process/thread error deduplication system (`is_new_error`). Only the first failure for a given `(pid, tid, operation, error_code)` tuple is logged to the find log. The `internal_op_code` mapping is:

| Code | Meaning |
|------|---------|
| `0` | `THREAD_QUERY_LIMITED_INFORMATION` (fatal) |
| `1` | `THREAD_QUERY_INFORMATION` |
| `2` | `THREAD_SET_LIMITED_INFORMATION` |
| `3` | `THREAD_SET_INFORMATION` |

### RAII cleanup

The returned [ThreadHandle](ThreadHandle.md) implements `Drop`. When dropped, it unconditionally closes `r_limited_handle` and conditionally closes each of the other three handles only if they are not invalid.

### Caller expectations

Callers that need to set thread properties (ideal processor, CPU sets, thread priority) should check whether `w_handle` or `w_limited_handle` is valid before attempting write operations. The [apply module](../apply.rs/README.md) and [scheduler module](../scheduler.rs/README.md) routinely handle the case where only limited-access handles are available.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Callers** | [scheduler module](../scheduler.rs/README.md) (`ThreadStats` handle acquisition), [apply module](../apply.rs/README.md) (thread-level operations) |
| **Callees** | `OpenThread` (Windows API), [try_open_thread](try_open_thread.md), `is_new_error`, `log_to_find`, `error_from_code_win32` |
| **API** | `Win32::System::Threading::OpenThread` |
| **Privileges** | `SeDebugPrivilege` recommended for cross-process thread access to protected processes |

## See Also

| Topic | Link |
|-------|------|
| Thread handle container | [ThreadHandle](ThreadHandle.md) |
| Helper for non-required handle opens | [try_open_thread](try_open_thread.md) |
| Process handle acquisition | [get_process_handle](get_process_handle.md) |
| Process handle container | [ProcessHandle](ProcessHandle.md) |
| Error deduplication system | [logging module](../logging.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd