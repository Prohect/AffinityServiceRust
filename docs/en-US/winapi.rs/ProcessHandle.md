# ProcessHandle struct (winapi.rs)

RAII container that holds up to four Windows process handles opened at different access levels. When the `ProcessHandle` is dropped, all valid handles are automatically closed via `CloseHandle`. The two limited handles (`r_limited_handle` and `w_limited_handle`) are always valid when the struct exists; the full-access handles (`r_handle` and `w_handle`) are `Option`-wrapped and may be `None` if the caller lacks sufficient privileges to open the process at that access level.

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
| `r_limited_handle` | `HANDLE` | Handle opened with `PROCESS_QUERY_LIMITED_INFORMATION`. Always valid when the struct exists. Used for lightweight queries such as reading CPU sets and process times. |
| `r_handle` | `Option<HANDLE>` | Handle opened with `PROCESS_QUERY_INFORMATION`. `Some` when the caller has sufficient access; `None` otherwise. Required for operations such as `GetProcessAffinityMask` and `NtQueryInformationProcess` with higher information classes. |
| `w_limited_handle` | `HANDLE` | Handle opened with `PROCESS_SET_LIMITED_INFORMATION`. Always valid when the struct exists. Used for setting CPU sets via `SetProcessDefaultCpuSets`. |
| `w_handle` | `Option<HANDLE>` | Handle opened with `PROCESS_SET_INFORMATION`. `Some` when the caller has sufficient access; `None` otherwise. Required for operations such as `SetPriorityClass`, `SetProcessAffinityMask`, and `NtSetInformationProcess`. |

## Remarks

### Construction

`ProcessHandle` instances are created exclusively by [get_process_handle](get_process_handle.md). That function opens all four access levels in sequence; the two limited handles are mandatory (failure returns `None` from the function), while the two full-access handles degrade gracefully to `None`. This design allows AffinityServiceRust to apply as many settings as the current privilege level permits without failing entirely when access is restricted.

### Drop behavior

The `Drop` implementation closes handles in the following order:

1. `r_handle` (if `Some`)
2. `w_handle` (if `Some`)
3. `r_limited_handle` (always)
4. `w_limited_handle` (always)

Each `CloseHandle` call is wrapped in `unsafe` and its result is intentionally discarded — there is no meaningful recovery action if closing a handle fails.

### Handle access level mapping

| Handle | Win32 Access Flag | Typical operations |
|--------|-------------------|--------------------|
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | `GetProcessDefaultCpuSets`, `GetProcessInformation` (memory priority) |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | `GetProcessAffinityMask`, `NtQueryInformationProcess` (IO priority) |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | `SetProcessDefaultCpuSets` |
| `w_handle` | `PROCESS_SET_INFORMATION` | `SetPriorityClass`, `SetProcessAffinityMask`, `NtSetInformationProcess` (IO / memory priority) |

### Usage in the apply module

The [apply module](../apply.rs/README.md) receives a `&ProcessHandle` reference and selects the appropriate handle for each operation via the helper function `get_handles`, which returns the best available read and write handles (preferring full-access over limited).

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Constructed by** | [get_process_handle](get_process_handle.md) |
| **Consumed by** | [apply_priority](../apply.rs/apply_priority.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| **Win32 API** | `OpenProcess`, `CloseHandle` |
| **Privileges** | `SeDebugPrivilege` recommended for full-access handles on protected processes |

## See Also

| Topic | Link |
|-------|------|
| Opens and returns a ProcessHandle | [get_process_handle](get_process_handle.md) |
| Thread handle RAII container | [ThreadHandle](ThreadHandle.md) |
| Rule application entry point (process level) | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Logging operations for handle failures | [Operation enum](../logging.rs/README.md) |
| Error code formatting | [error_codes module](../error_codes.rs/README.md) |