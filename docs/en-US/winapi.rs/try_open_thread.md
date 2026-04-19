# try_open_thread function (winapi.rs)

Lower-level helper that attempts to open a single thread handle with a specific access level. Returns a valid `HANDLE` on success or an invalid `HANDLE` on failure, allowing the caller to continue without aborting the entire handle acquisition.

## Syntax

```rust
#[inline(always)]
fn try_open_thread(
    pid: u32,
    tid: u32,
    process_name: &str,
    access: THREAD_ACCESS_RIGHTS,
    internal_op_code: u32,
) -> HANDLE
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier that owns the thread. Used for error context in diagnostic messages (currently commented out). |
| `tid` | `u32` | Thread identifier to open. Passed to `OpenThread`. |
| `process_name` | `&str` | Display name of the owning process. Used for error context in diagnostic messages (currently commented out). |
| `access` | `THREAD_ACCESS_RIGHTS` | The desired access rights for the handle. Typically one of `THREAD_QUERY_INFORMATION`, `THREAD_SET_LIMITED_INFORMATION`, or `THREAD_SET_INFORMATION`. |
| `internal_op_code` | `u32` | Numeric identifier for the access level being attempted, used to map errors to a human-readable handle name in `error_detail`. Values: `1` → `"r_handle"`, `2` → `"w_limited_handle"`, `3` → `"w_handle"`. |

## Return value

`HANDLE` — A valid thread handle on success, or `HANDLE::default()` (an invalid handle) on failure. The caller must check `is_invalid()` before using the returned handle.

## Remarks

This function is the building block used by [get_thread_handle](get_thread_handle.md) to open the non-required handle levels (`r_handle`, `w_limited_handle`, `w_handle`). Unlike the required `r_limited_handle` (whose failure causes `get_thread_handle` to return `None`), failure in `try_open_thread` is non-fatal — the returned invalid handle is stored in [ThreadHandle](ThreadHandle.md) and the caller simply avoids using that access level.

### Error handling

The function contains commented-out calls to `is_new_error` and `log_to_find` for both the `OpenThread` failure path and the invalid handle path. These are intentionally disabled in production because failures at these non-required access levels are expected and frequent (e.g., `THREAD_SET_INFORMATION` may be denied for protected processes even with SeDebugPrivilege). The `error_detail` inner function maps `internal_op_code` to a human-readable string for diagnostic purposes when the logging code is enabled.

### Inner function: error_detail

```rust
fn error_detail(internal_op_code: &u32) -> String
```

Maps the numeric `internal_op_code` to a handle field name string:

| Code | Returns |
|------|---------|
| `1` | `"r_handle"` |
| `2` | `"w_limited_handle"` |
| `3` | `"w_handle"` |
| other | `"UNKNOWN_OP_CODE"` |

### Visibility

This function is module-private (`fn`, not `pub fn`) and is only called by [get_thread_handle](get_thread_handle.md). It is marked `#[inline(always)]` because it is called three times per thread handle acquisition and the function body is small.

### Failure semantics

On failure, the function returns `HANDLE::default()`, which is a zeroed/invalid handle. The [ThreadHandle](ThreadHandle.md) struct's `Drop` implementation checks `is_invalid()` before calling `CloseHandle`, so storing an invalid handle is safe and will not cause a double-close or an error on cleanup.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [get_thread_handle](get_thread_handle.md) |
| **Callees** | `OpenThread` (Win32) |
| **Win32 API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |
| **Privileges** | Depends on `access` — `THREAD_QUERY_INFORMATION` requires process query rights; `THREAD_SET_INFORMATION` requires [SeDebugPrivilege](enable_debug_privilege.md) for protected processes |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Primary caller | [get_thread_handle](get_thread_handle.md) |
| Thread handle RAII wrapper | [ThreadHandle](ThreadHandle.md) |
| Process handle equivalent | [get_process_handle](get_process_handle.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*