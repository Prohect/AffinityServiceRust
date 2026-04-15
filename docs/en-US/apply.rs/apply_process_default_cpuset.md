# apply_process_default_cpuset function (apply.rs)

The `apply_process_default_cpuset` function queries the current default CPU Set IDs assigned to a process via `GetProcessDefaultCpuSets` and, if they differ from the configured target, applies the new set via `SetProcessDefaultCpuSets`. When the `cpu_set_reset_ideal` configuration flag is enabled, the function also calls [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) to redistribute thread ideal processors across the new CPU set before applying the change. This function operates on CPU Set IDs (not affinity masks), which is the modern Windows mechanism for controlling process-to-CPU assignment without the limitations of legacy affinity masks.

## Syntax

```AffinityServiceRust/src/apply.rs#L297-308
pub fn apply_process_default_cpuset<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Used for error deduplication and log messages. |
| `config` | `&ProcessLevelConfig` | The process-level configuration containing `cpu_set_cpus` (a list of CPU indices to convert into CPU Set IDs), `cpu_set_reset_ideal` (a boolean controlling whether thread ideal processors are redistributed on change), and `name` (the human-readable config rule name used in log messages). If `cpu_set_cpus` is empty, the function returns immediately without making any changes. |
| `dry_run` | `bool` | When `true`, the function records what *would* change in `apply_config_result` without calling any Windows APIs to modify state. When `false`, the Windows APIs are called to apply the change. |
| `process_handle` | `&ProcessHandle` | A handle wrapper providing read and write access to the process. The function extracts `r_handle` (for `GetProcessDefaultCpuSets`) and `w_handle` (for `SetProcessDefaultCpuSets`) via [`get_handles`](get_handles.md). If either handle is unavailable, the function returns early. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A lazy closure that returns a map of thread IDs to their `SYSTEM_THREAD_INFORMATION` snapshots. The closure is only evaluated when [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) needs the thread data (i.e., when `cpu_set_reset_ideal` is enabled and a change is being applied). This deferred evaluation avoids the cost of building the thread map when it is not needed. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during execution. |

## Return value

This function does not return a value. All outcomes are communicated through the `apply_config_result` parameter.

## Remarks

- The function exits early without action if `config.cpu_set_cpus` is empty **or** if the global CPU set information (from `get_cpu_set_information()`) is empty. The latter condition ensures the function does not attempt to convert CPU indices to CPU Set IDs when no system CPU set information is available.
- The configured CPU indices are converted to Windows CPU Set IDs using `cpusetids_from_indices`. If the resulting ID list is empty after conversion, no change is applied.
- The query uses a two-call pattern for `GetProcessDefaultCpuSets`:
  1. **First call** with `None` buffer: If it succeeds, the process has no default CPU set assigned yet, and `toset` is set to `true`.
  2. If the first call fails with Win32 error code `122` (`ERROR_INSUFFICIENT_BUFFER`), a **second call** is made with a properly sized buffer to retrieve the current CPU Set IDs. The retrieved IDs are then compared against the target; `toset` is `true` only if they differ.
  3. If the first call fails with any other error code, the error is logged via [`log_error_if_new`](log_error_if_new.md) and the function does not attempt to set the CPU set.
- When `config.cpu_set_reset_ideal` is `true` and a change is needed, [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) is invoked **before** the CPU set is applied, using `config.cpu_set_cpus` as the target CPU list. This redistributes thread ideal processors in anticipation of the new CPU set assignment.
- On success, the change message is formatted as `"CPU Set: [<old>] -> [<new>]"` where `<old>` and `<new>` are formatted CPU index lists. When the process had no previous default CPU set, `<old>` is an empty list.
- On failure of `SetProcessDefaultCpuSets`, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::SetProcessDefaultCpuSets`.
- Current CPU Set IDs are decoded back to CPU indices using `indices_from_cpusetids` for the change message's "old" value.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows APIs | `GetProcessDefaultCpuSets`, `SetProcessDefaultCpuSets`, `GetLastError` |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that iterates matched processes |
| Callees | [`get_handles`](get_handles.md), [`log_error_if_new`](log_error_if_new.md), [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md), `cpusetids_from_indices`, `indices_from_cpusetids`, `get_cpu_set_information`, `format_cpu_indices`, `error_from_code_win32` |
| Privileges | Requires a process handle with `PROCESS_QUERY_LIMITED_INFORMATION` (read) and `PROCESS_SET_LIMITED_INFORMATION` (write). |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| reset_thread_ideal_processors | [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| winapi module | [`winapi.rs`](../winapi.rs/README.md) |

---
*Commit: b0df9da35213b050501fab02c3020ad4dbd6c4e0*