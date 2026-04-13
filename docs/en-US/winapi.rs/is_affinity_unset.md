# is_affinity_unset function (winapi.rs)

Checks whether a process's CPU affinity mask equals the system-wide default (all logical processors enabled). This is used by the `-find` mode to identify processes that have not yet had a custom affinity applied, helping users discover which processes are still running with default settings.

## Syntax

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process. |
| `process_name` | `&str` | The image name of the target process (e.g., `"game.exe"`). Used for diagnostic logging and to populate the find-fail set when access is denied. |

## Return value

| Value | Meaning |
|-------|---------|
| `true` | The process's current affinity mask is equal to the system affinity mask — meaning all CPUs are enabled and no custom affinity has been set. |
| `false` | The process has a custom affinity mask (a subset of CPUs), **or** the process could not be opened, **or** the affinity query failed. |

## Remarks

### Algorithm

1. Opens the process with `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION` access via `OpenProcess`.
2. If the open fails, logs the error to `log_to_find`. If the error code is `5` (access denied), inserts `process_name` into the `FINDS_FAIL_SET` so the find-mode report can note this process as inaccessible.
3. Calls `GetProcessAffinityMask` to retrieve both the process affinity mask (`current_mask`) and the system affinity mask (`system_mask`).
4. Returns `true` if and only if `current_mask == system_mask`.
5. Closes the process handle before returning.

### Handle management

Unlike most other functions in this module, `is_affinity_unset` does **not** use the [ProcessHandle](ProcessHandle.md) RAII wrapper. It opens a single combined-access handle directly via `OpenProcess` and closes it manually with `CloseHandle` at the end of the function. This is because the function is a standalone query used only in `-find` mode and does not participate in the main apply loop's handle lifecycle.

### Error behavior

The function returns `false` on any error, treating inaccessible or failed-to-query processes as "already configured" to avoid false positives in find-mode output. Specific error handling:

| Scenario | Behavior |
|----------|----------|
| `OpenProcess` fails | Logs error via `log_to_find`; if error 5 (access denied), adds to fail set; returns `false` |
| `OpenProcess` returns an invalid handle | Logs `[INVALID_HANDLE]` via `log_to_find`; returns `false` |
| `GetProcessAffinityMask` fails | Logs error via `log_to_find`; if error 5, adds to fail set; returns `false` |

### Access-denied tracking

When the error code is `5` (`ERROR_ACCESS_DENIED`), the process name is inserted into the global `FINDS_FAIL_SET` (accessed via the `get_fail_find_set!()` macro). This set is used by the find-mode report to list processes that could not be inspected, typically because `SeDebugPrivilege` is not held or the process is protected.

### System affinity mask

The system affinity mask returned by `GetProcessAffinityMask` reflects the set of all logical processors available to the calling process. On a single-processor-group system (≤ 64 CPUs), this is typically `(1 << cpu_count) - 1`. When a process has not had `SetProcessAffinityMask` called on it, its process mask equals the system mask.

### Comparison with affinity in the apply module

The [apply_affinity](../apply.rs/apply_affinity.md) function uses a [ProcessHandle](ProcessHandle.md) to get and set affinity during the main service loop. `is_affinity_unset` operates independently and is called only during `-find` mode discovery, not during the apply cycle.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [process_find](../main.rs/process_find.md) (via the `-find` CLI mode) |
| **Callees** | `OpenProcess`, `GetProcessAffinityMask`, `GetLastError`, `CloseHandle` (Win32); `log_to_find`, `get_fail_find_set!()` |
| **API** | [`GetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask), [`OpenProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) |
| **Privileges** | `SeDebugPrivilege` recommended; without it, protected processes return access-denied |

## See Also

| Topic | Link |
|-------|------|
| Affinity application logic | [apply_affinity](../apply.rs/apply_affinity.md) |
| Find mode entry point | [process_find](../main.rs/process_find.md) |
| Process handle RAII wrapper | [ProcessHandle](ProcessHandle.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |
| CPU indices to bitmask utility | [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) |
| Error code formatting | [error_codes module](../error_codes.rs/README.md) |
| GetProcessAffinityMask (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |