# apply_affinity function (apply.rs)

The `apply_affinity` function reads the current process affinity mask via `GetProcessAffinityMask` and, if it differs from the configured target mask, sets the new mask via `SetProcessAffinityMask`. On a successful affinity change, it also calls [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) to redistribute thread ideal processors across the new set of CPUs. As a side effect, the function writes the current (or newly applied) affinity mask into the caller-provided `current_mask` output parameter.

## Syntax

```AffinityServiceRust/src/apply.rs#L134-145
pub fn apply_affinity<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Used for error logging and passed through to `reset_thread_ideal_processors`. |
| `config` | `&ProcessLevelConfig` | The process-level configuration that contains `affinity_cpus` (a list of CPU indices) and the process `name` used in log messages. If `affinity_cpus` is empty, the function returns immediately without making any changes. |
| `dry_run` | `bool` | When `true`, the function records what *would* change in `apply_config_result` without calling any Windows APIs to modify state. Read operations are also skipped in dry-run mode (errors from `GetProcessAffinityMask` are suppressed). |
| `current_mask` | `&mut usize` | An output parameter. On successful query or set, `*current_mask` is updated to reflect the process's affinity mask. This value is consumed by downstream functions such as [`apply_prime_threads_promote`](apply_prime_threads_promote.md) to filter prime CPU indices against the current affinity. |
| `process_handle` | `&ProcessHandle` | A handle wrapper providing read and write access to the process. The function extracts `r_handle` (for `GetProcessAffinityMask`) and `w_handle` (for `SetProcessAffinityMask`) via [`get_handles`](get_handles.md). If either handle is unavailable, the function returns early. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A lazy closure that returns a map of thread IDs to their `SYSTEM_THREAD_INFORMATION` snapshots. The closure is only invoked when `reset_thread_ideal_processors` needs to redistribute ideal processors after a successful affinity change. This deferred evaluation avoids the cost of building the thread map when no affinity change occurs. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during execution. |

## Return value

This function does not return a value. All results are communicated through the `current_mask` output parameter and the `apply_config_result` accumulator.

## Remarks

- The target affinity mask is computed from `config.affinity_cpus` using `cpu_indices_to_mask`. If the resulting mask is `0` or matches the current process affinity mask, no change is applied.
- When the affinity mask is changed successfully, `*current_mask` is updated to the new target mask, and [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) is called with `dry_run: false` and `config.affinity_cpus` as the CPU list. This redistributes thread ideal processors across the new affinity set to prevent Windows from concentrating threads on a single CPU after a mask change.
- The function queries both the process affinity mask and the system affinity mask via `GetProcessAffinityMask`, but only the process mask is used for comparison. The system mask is discarded.
- Errors from `GetProcessAffinityMask` are logged through [`log_error_if_new`](log_error_if_new.md) only when `dry_run` is `false`. In dry-run mode, query failures are silently ignored and a synthetic change message is generated based on the configured target.
- Errors from `SetProcessAffinityMask` are logged through [`log_error_if_new`](log_error_if_new.md) with `Operation::SetProcessAffinityMask`. The `current_mask` is **not** updated when the set operation fails.
- Change messages are formatted as `"Affinity: {current:#X} -> {target:#X}"` showing the hexadecimal affinity masks.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows APIs | `GetProcessAffinityMask`, `SetProcessAffinityMask`, `GetLastError` |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` |
| Callees | [`get_handles`](get_handles.md), [`log_error_if_new`](log_error_if_new.md), [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md), `cpu_indices_to_mask` (config module), `error_from_code_win32` (error_codes module) |
| Privileges | Requires `PROCESS_QUERY_INFORMATION` or `PROCESS_QUERY_LIMITED_INFORMATION` for reading, and `PROCESS_SET_INFORMATION` for writing. These are encapsulated in the `ProcessHandle` handles. |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| reset_thread_ideal_processors | [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) |
| apply_process_default_cpuset | [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |

---
*Commit: b0df9da35213b050501fab02c3020ad4dbd6c4e0*