# ProcessHandle struct (winapi.rs)

RAII wrapper around multiple Windows `HANDLE` values opened for a single process at different access levels. Provides read and write handles at both limited and full access tiers. All valid handles are automatically closed via `CloseHandle` when the struct is dropped.

## Syntax

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `r_limited_handle` | `HANDLE` | Process handle opened with `PROCESS_QUERY_LIMITED_INFORMATION` access. Always valid when the struct exists — construction fails if this handle cannot be obtained. Used for lightweight queries such as `QueryFullProcessImageNameW` and cycle time queries that do not require full query rights. |
| `r_handle` | `Option<HANDLE>` | Process handle opened with `PROCESS_QUERY_INFORMATION` access. `None` if the access right was denied (common for protected/system processes). Required for operations like `GetProcessAffinityMask` and `NtQueryInformationProcess`. |
| `w_limited_handle` | `HANDLE` | Process handle opened with `PROCESS_SET_LIMITED_INFORMATION` access. Always valid when the struct exists. Used for CPU set assignment via `SetProcessDefaultCpuSets`. |
| `w_handle` | `Option<HANDLE>` | Process handle opened with `PROCESS_SET_INFORMATION` access. `None` if the access right was denied. Required for operations like `SetProcessAffinityMask`, `SetPriorityClass`, and `NtSetInformationProcess`. |

## Drop

```rust
impl Drop for ProcessHandle {
    fn drop(&mut self);
}
```

Closes all held handles via `CloseHandle`:

- `r_limited_handle` and `w_limited_handle` are always closed (they are always valid).
- `r_handle` and `w_handle` are closed only if they are `Some(_)`.

Each `CloseHandle` call's result is intentionally discarded (`let _ = CloseHandle(...)`) since handle closure failures during cleanup are non-recoverable and should not panic.

## Remarks

### Access level tiers

The `ProcessHandle` struct captures four handles at two access tiers:

| Tier | Read | Write |
|------|------|-------|
| **Limited** | `PROCESS_QUERY_LIMITED_INFORMATION` | `PROCESS_SET_LIMITED_INFORMATION` |
| **Full** | `PROCESS_QUERY_INFORMATION` | `PROCESS_SET_INFORMATION` |

Limited-access handles succeed for most processes, including those in other sessions. Full-access handles may fail for protected processes, system processes, or processes with security descriptors that deny the requesting account. The [get_process_handle](get_process_handle.md) factory function handles this gracefully by wrapping full-access handles in `Option`.

### Handle selection in callers

The [get_handles](../apply.rs/get_handles.md) helper extracts a `(read_handle, write_handle)` pair from a `ProcessHandle`, preferring full-access handles when available and falling back to limited-access handles. This allows the apply functions to work with the best available access level without checking `Option` themselves.

### Validity guarantees

- **Construction:** A `ProcessHandle` is only created by [get_process_handle](get_process_handle.md), which returns `None` if either limited-access handle fails to open. If the struct exists, both `r_limited_handle` and `w_limited_handle` are guaranteed valid.
- **Lifetime:** The caller (typically the main polling loop) owns the `ProcessHandle` and drops it when the process is no longer being tracked.
- **Thread safety:** `ProcessHandle` is not `Send` or `Sync` by default due to the raw `HANDLE` values. It is used within a single-threaded context in the main polling loop.

### Error deduplication

When a handle open fails during construction, the error is logged via [is_new_error](../logging.rs/is_new_error.md) so that repeated failures for the same process/operation/error-code combination are suppressed after the first occurrence. Internal operation codes map to specific handles:

| Op Code | Handle |
|---------|--------|
| `0` | `r_limited_handle` (`PROCESS_QUERY_LIMITED_INFORMATION`) |
| `1` | `w_limited_handle` (`PROCESS_SET_LIMITED_INFORMATION`) |
| `2` | `r_handle` (`PROCESS_QUERY_INFORMATION`) |
| `3` | `w_handle` (`PROCESS_SET_INFORMATION`) |

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Main polling loop, [get_handles](../apply.rs/get_handles.md), all `apply_*` functions |
| **Factory** | [get_process_handle](get_process_handle.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | [SeDebugPrivilege](enable_debug_privilege.md) extends access to protected processes |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Factory function | [get_process_handle](get_process_handle.md) |
| Handle extraction helper | [get_handles](../apply.rs/get_handles.md) |
| Thread handle counterpart | [ThreadHandle](ThreadHandle.md) |
| Error deduplication | [is_new_error](../logging.rs/is_new_error.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*