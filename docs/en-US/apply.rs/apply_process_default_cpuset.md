# apply_process_default_cpuset function (apply.rs)

Sets the default CPU set for a process using the Windows CPU Sets API, providing a soft CPU preference that the scheduler respects without hard-limiting thread execution.

## Syntax

```AffinityServiceRust/src/apply.rs#L298-308
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

`pid: u32`

The process ID of the target process.

`config: &ProcessLevelConfig`

The process-level configuration containing `cpu_set_cpus` (the desired CPU indices) and `cpu_set_reset_ideal` (whether to reset ideal processors after applying the CPU set).

`dry_run: bool`

If `true`, records what would change in `apply_config_result` without calling any Windows APIs.

`process_handle: &ProcessHandle`

Handle wrapper for the target process. Both read and write handles are required.

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

Lazy accessor for the process's thread map. Only evaluated if `cpu_set_reset_ideal` is `true` and a change is applied, in which case it is forwarded to [reset_thread_ideal_processors](reset_thread_ideal_processors.md).

`apply_config_result: &mut ApplyConfigResult`

Accumulator for change messages and error messages produced during the operation.

## Return value

None. Results are accumulated into `apply_config_result`.

## Remarks

Unlike hard affinity masks set via `SetProcessAffinityMask`, CPU sets provide a **soft preference**. The Windows scheduler prefers the specified CPUs but may schedule threads on other CPUs under load. This makes CPU sets the preferred mechanism for workload steering on modern Windows.

### Algorithm

1. **Early exit** — Returns immediately if `config.cpu_set_cpus` is empty or if the global CPU set information cache is empty.
2. **Dry run** — If `dry_run` is `true`, records the intended CPU set and returns.
3. **Convert indices** — Translates the configured CPU indices to Windows CPU Set IDs via `cpusetids_from_indices`.
4. **Query current** — Calls `GetProcessDefaultCpuSets` with `None` buffer first:
   - If it succeeds, the process has no default CPU set assigned, so a change is needed.
   - If it fails with error code **122** (`ERROR_INSUFFICIENT_BUFFER`), the process already has a CPU set. A second call with a correctly sized buffer retrieves the current set IDs for comparison.
   - Any other error is logged via [log_error_if_new](log_error_if_new.md).
5. **Compare** — If the current CPU set IDs match the target, no action is taken.
6. **Reset ideal (optional)** — If `config.cpu_set_reset_ideal` is `true`, calls [reset_thread_ideal_processors](reset_thread_ideal_processors.md) with `config.cpu_set_cpus` *before* applying the new CPU set. This prevents stale ideal processor assignments from overriding the new CPU preference.
7. **Apply** — Calls `SetProcessDefaultCpuSets` with the target CPU set IDs.
8. **Log** — On success, records a change message showing the transition from old to new CPU indices. On failure, logs the error.

### Two-pass query pattern

The `GetProcessDefaultCpuSets` API uses a common Windows pattern where the first call determines the required buffer size. Error code 122 (`ERROR_INSUFFICIENT_BUFFER`) is an expected condition, not a true error, and triggers the second call with an appropriately sized buffer.

### Interaction with affinity masks

CPU sets and affinity masks are independent mechanisms. A process can have both a hard affinity mask and a default CPU set. The effective scheduling depends on Windows internal logic, but in general the affinity mask takes precedence as a hard constraint while the CPU set acts as a hint within that constraint.

## Requirements

| | |
|---|---|
| **Module** | [apply.rs](README.md) |
| **Callers** | `main.rs` enforcement loop |
| **Callees** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), `cpusetids_from_indices`, `indices_from_cpusetids`, `format_cpu_indices` |
| **Win32 API** | [GetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets), [SetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets), [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| **Privileges** | Requires `PROCESS_QUERY_LIMITED_INFORMATION` (read) and `PROCESS_SET_LIMITED_INFORMATION` (write) access to the target process |
| **Minimum OS** | Windows 10 version 1607 (CPU Sets API) |

## See Also

| Topic | Description |
|---|---|
| [apply_affinity](apply_affinity.md) | Hard affinity mask alternative |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | Redistributes thread ideal processors after CPU set changes |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | Configuration struct with `cpu_set_cpus` and `cpu_set_reset_ideal` fields |
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for changes and errors |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*