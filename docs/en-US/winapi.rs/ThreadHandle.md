# ThreadHandle struct (winapi.rs)

RAII wrapper for a set of Windows thread handles opened at multiple access levels. Automatically closes all valid handles when dropped. The `r_limited_handle` is always valid when the struct exists; other handles may be invalid if the corresponding `OpenThread` call failed due to insufficient privileges.

## Syntax

```rust
#[derive(Debug)]
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## Members

| Member | Type | Access Right | Description |
|--------|------|-------------|-------------|
| `r_limited_handle` | `HANDLE` | `THREAD_QUERY_LIMITED_INFORMATION` | Always-valid read handle for basic thread queries such as cycle time retrieval. This is the minimum access level and is required for the struct to be constructed. |
| `r_handle` | `HANDLE` | `THREAD_QUERY_INFORMATION` | Full read handle for advanced queries such as [get_thread_start_address](get_thread_start_address.md) (via `NtQueryInformationThread`). May be an invalid handle (`HANDLE::default()`) if access was denied. Check with `is_invalid()` before use. |
| `w_limited_handle` | `HANDLE` | `THREAD_SET_LIMITED_INFORMATION` | Limited write handle for operations like setting thread CPU set assignment. May be invalid if access was denied. |
| `w_handle` | `HANDLE` | `THREAD_SET_INFORMATION` | Full write handle for operations like [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) and thread priority changes. May be invalid if access was denied. |

## Drop

```rust
impl Drop for ThreadHandle {
    fn drop(&mut self);
}
```

Closes all handles held by the struct. `r_limited_handle` is always closed unconditionally (it is guaranteed valid). The remaining three handles (`r_handle`, `w_limited_handle`, `w_handle`) are only closed if they are not invalid, as determined by `HANDLE::is_invalid()`.

### Handle closure order

1. `r_limited_handle` — always closed
2. `r_handle` — closed if not invalid
3. `w_limited_handle` — closed if not invalid
4. `w_handle` — closed if not invalid

Each closure calls `CloseHandle` and discards the result.

## Remarks

### Difference from ProcessHandle

Unlike [ProcessHandle](ProcessHandle.md), which uses `Option<HANDLE>` for its optional handles, `ThreadHandle` uses raw `HANDLE` values and relies on `is_invalid()` to distinguish valid from failed handles. This design difference exists because thread handles are more frequently accessed in tight loops (per-thread, per-iteration), and avoiding `Option` unwrapping reduces overhead.

### Access level strategy

The four handles represent a matrix of read/write × limited/full access:

|  | Limited | Full |
|--|---------|------|
| **Read** | `r_limited_handle` | `r_handle` |
| **Write** | `w_limited_handle` | `w_handle` |

Limited access rights succeed more often (e.g., for protected processes) but support fewer operations. The caller should prefer limited handles when possible and fall back to checking `is_invalid()` on full handles before use.

### Caching in ThreadStats

`ThreadHandle` instances are cached in [ThreadStats](../scheduler.rs/ThreadStats.md)`.handle` to avoid re-opening handles on every polling iteration. The handle persists for the lifetime of the thread or until the parent process exits and [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) drops the stats entry.

### Send safety

`ThreadHandle` is `Send` because `HANDLE` is a thin wrapper around a pointer-sized value and Windows handles can be used from any thread within the same process.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [get_thread_handle](get_thread_handle.md), [ThreadStats](../scheduler.rs/ThreadStats.md), [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Callees** | `CloseHandle` (Win32) |
| **Win32 API** | [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | None for drop; creation requires appropriate thread access rights (see [get_thread_handle](get_thread_handle.md)) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Thread handle factory | [get_thread_handle](get_thread_handle.md) |
| Lower-level thread opener | [try_open_thread](try_open_thread.md) |
| Process handle counterpart | [ProcessHandle](ProcessHandle.md) |
| Thread stats cache | [ThreadStats](../scheduler.rs/ThreadStats.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*