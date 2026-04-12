# apply_process_default_cpuset function (apply.rs)

Sets the default CPU Set for a process, providing a soft CPU preference via the Windows CPU Sets API. Unlike hard affinity masks, CPU Sets allow the scheduler to use other CPUs under contention.

## Syntax

```rust
pub fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process identifier of the target process.

`config`

Reference to a [ProcessConfig](../config.rs/ProcessConfig.md) containing the desired `cpu_set_cpus` indices and the `cpu_set_reset_ideal` flag.

`dry_run`

When `true`, records the intended change without calling any Windows APIs.

`process_handle`

Reference to a [ProcessHandle](../winapi.rs/ProcessHandle.md) providing read and write access to the target process.

`process`

Mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. Passed through to [reset_thread_ideal_processors](reset_thread_ideal_processors.md) when `cpu_set_reset_ideal` is enabled.

`apply_config_result`

Mutable reference to an [ApplyConfigResult](ApplyConfigResult.md) that collects change and error messages.

## Return value

This function does not return a value. Results are recorded in `apply_config_result`.

## Remarks

The function operates as follows:

1. **Guard checks** — Returns immediately if `config.cpu_set_cpus` is empty or if system CPU Set information is unavailable.
2. **Dry run** — If `dry_run` is `true`, a change message is recorded showing the target CPU set and the function returns.
3. **Convert CPU indices to CPU Set IDs** — Translates the logical CPU indices from `config.cpu_set_cpus` into Windows CPU Set IDs using `cpusetids_from_indices`.
4. **Query current CPU Sets** — Calls `GetProcessDefaultCpuSets` with a `None` buffer first:
   - If the call **succeeds**, the process has no default CPU set assigned, so a set operation is needed.
   - If the call **fails with error 122** (`ERROR_INSUFFICIENT_BUFFER`), this is the expected path — it means the process already has CPU sets assigned. The required buffer size is returned in `requiredidcount`. A second call retrieves the current CPU set IDs. The function then compares the current set to the target set; if they match, no action is taken.
   - If the call **fails with any other error**, the error is logged via [log_error_if_new](log_error_if_new.md) and the function continues without setting.
5. **Reset ideal processors** — If `config.cpu_set_reset_ideal` is `true` and a set operation is pending, calls [reset_thread_ideal_processors](reset_thread_ideal_processors.md) with `&config.cpu_set_cpus` *before* applying the new CPU set. This redistributes thread ideal processors across the new CPU set to avoid clumping.
6. **Apply new CPU Sets** — Calls `SetProcessDefaultCpuSets` with the target CPU set IDs. On success, a change message is recorded showing the old and new CPU indices. On failure, the error is logged.

### Error 122 (ERROR_INSUFFICIENT_BUFFER)

The initial `GetProcessDefaultCpuSets` call with a `None` buffer is expected to fail with error code 122 when the process already has default CPU sets assigned. This is the documented two-call query pattern for Windows APIs that return variable-length data. The error is intentionally suppressed and not logged.

### Change log format

```
CPU Set: [old_indices] -> [new_indices]
```

For example: `CPU Set: [0,1,2,3] -> [4,5,6,7]`

When setting for the first time (no previous CPU set): `CPU Set: [] -> [4,5,6,7]`

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L315–L418 |
| **Called by** | [apply_config](../main.rs/apply_config.md) in main.rs |
| **Calls** | [get_handles](get_handles.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), [log_error_if_new](log_error_if_new.md) |
| **Windows API** | [GetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets), [SetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets) |
| **See also** | [apply_affinity](apply_affinity.md), [ProcessConfig](../config.rs/ProcessConfig.md) |