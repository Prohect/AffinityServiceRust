# is_affinity_unset function (winapi.rs)

Checks whether a process has its default (all-CPU) affinity mask — that is, whether the process affinity mask equals the system affinity mask. This is used by `-find` mode to identify processes whose CPU affinity has not been explicitly configured.

## Syntax

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process. |
| `process_name` | `&str` | The name of the target process, used for diagnostic logging and for populating the find-fail set on access-denied errors. |

## Return value

Returns `true` if the process's current affinity mask is equal to the system affinity mask, meaning no custom affinity has been applied. Returns `false` in all other cases, including:

- The process handle could not be opened.
- The handle was opened but is invalid.
- `GetProcessAffinityMask` failed.
- The process has a custom affinity mask that differs from the system mask.

## Remarks

### Algorithm

1. Opens the target process with `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION` access via `OpenProcess`.
2. If the open fails, logs the error to the find log via [`log_to_find`](../logging.rs/log_to_find.md). If the error code is `5` (`ACCESS_DENIED`), the process name is added to the `FINDS_FAIL_SET` so it is excluded from future find iterations.
3. If the returned handle is invalid, logs a diagnostic and returns `false`.
4. Calls `GetProcessAffinityMask` to retrieve both the process affinity mask (`current_mask`) and the system affinity mask (`system_mask`).
5. Compares the two masks. Returns `true` only if they are equal.
6. Closes the process handle before returning.

### Access-denied handling

When `OpenProcess` or `GetProcessAffinityMask` returns error code `5` (`ACCESS_DENIED`), the process name is inserted into the `FINDS_FAIL_SET` global. This set is used by the find-mode logic to skip processes that are known to be inaccessible (e.g., protected processes, anti-cheat services), avoiding repeated failed attempts and log noise.

### Handle lifetime

The function opens and closes the process handle within its own scope. It does **not** use the [`ProcessHandle`](ProcessHandle.md) RAII wrapper because it only needs a single handle with specific combined access rights (`PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION`).

### Platform notes

- **Windows only.** Uses `OpenProcess`, `GetProcessAffinityMask`, `GetLastError`, and `CloseHandle` from the Win32 API.
- The system affinity mask represents all logical processors available to the process's processor group. On systems with more than 64 logical processors, this function only considers the primary processor group.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | Find-mode logic in `apply.rs` / `scheduler.rs` |
| **Callees** | `OpenProcess`, `GetProcessAffinityMask`, `GetLastError`, `CloseHandle` (Win32), [`log_to_find`](../logging.rs/log_to_find.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [GetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |
| **Privileges** | `SeDebugPrivilege` recommended for querying protected/elevated processes. |

## See Also

| Topic | Link |
|-------|------|
| get_process_handle | [get_process_handle](get_process_handle.md) |
| log_to_find | [log_to_find](../logging.rs/log_to_find.md) |
| error_from_code_win32 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| FINDS_FAIL_SET static | [statics](../logging.rs/statics.md#finds_fail_set) |
| logging module | [logging.rs](../logging.rs/README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
