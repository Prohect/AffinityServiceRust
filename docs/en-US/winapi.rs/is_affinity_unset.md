# is_affinity_unset function (winapi.rs)

Checks whether a process's CPU affinity mask equals the full system affinity mask, indicating that no custom affinity restriction has been applied. Used by the `-find` mode to identify processes that have not yet been configured with a specific CPU affinity.

## Syntax

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier to query. |
| `process_name` | `&str` | The process image name, used for error logging and for recording in the `-find` failure set when access is denied. |

## Return value

`bool` — `true` if the process's current affinity mask equals the system-wide affinity mask (meaning no affinity restriction is in effect). `false` if the process has a custom affinity mask, or if any API call fails during the check.

## Remarks

### Algorithm

The function performs the following steps:

1. **Open process** — Calls `OpenProcess` with `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION` access rights.
2. **Query affinity** — Calls `GetProcessAffinityMask` to obtain both the process's `current_mask` and the system-wide `system_mask`.
3. **Compare** — Returns `true` if `current_mask == system_mask`.
4. **Close handle** — The process handle is closed via `CloseHandle` before returning.

### Error handling

| Failure point | Behavior |
|---------------|----------|
| `OpenProcess` fails | Logs via `log_to_find`; if the Win32 error code is `5` (`ERROR_ACCESS_DENIED`), inserts `process_name` into the global fail-find set. Returns `false`. |
| `OpenProcess` returns an invalid handle | Logs via `log_to_find`. Returns `false`. |
| `GetProcessAffinityMask` fails | Logs via `log_to_find`; if `ERROR_ACCESS_DENIED`, inserts into the fail-find set. Returns `false`. |

The conservative `false` return on failure means that processes which cannot be queried are treated as "already configured," preventing them from appearing in `-find` mode output.

### Fail-find set

When an `ERROR_ACCESS_DENIED` (code 5) is encountered during either the `OpenProcess` or `GetProcessAffinityMask` calls, the process name is inserted into the global fail-find set (accessed via the `get_fail_find_set!()` macro). This set is used by the `-find` mode to track processes that the service cannot inspect due to insufficient privileges, allowing them to be reported separately.

### Handle management

This function opens and closes its own temporary process handle rather than reusing handles from [ProcessHandle](ProcessHandle.md). This is because `-find` mode operates as a one-shot scan rather than a persistent polling loop, so there is no benefit to caching the handle.

### System affinity mask

The `system_mask` output of `GetProcessAffinityMask` represents all logical processors available on the system (within the current processor group). On a system with 8 logical processors, this would be `0xFF`. A process whose `current_mask` equals `system_mask` has the default "use all CPUs" configuration.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | `-find` mode in `src/main.rs` |
| **Callees** | `OpenProcess` (Win32), `GetProcessAffinityMask` (Win32), `CloseHandle` (Win32), `GetLastError`, [error_from_code_win32](../error_codes.rs/error_from_code_win32.md), [log_to_find](../logging.rs/log_to_find.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) (`PROCESS_SET_INFORMATION \| PROCESS_QUERY_INFORMATION`), [GetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |
| **Privileges** | Requires `PROCESS_QUERY_INFORMATION` access; [SeDebugPrivilege](enable_debug_privilege.md) extends access to protected processes |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Affinity application function | [apply_affinity](../apply.rs/apply_affinity.md) |
| CPU set alternative to affinity | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| CPU Set ID ↔ mask conversions | [cpusetids_from_mask](cpusetids_from_mask.md), [mask_from_cpusetids](mask_from_cpusetids.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*