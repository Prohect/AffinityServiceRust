# apply_config_process_level function (main.rs)

Applies process-level settings for a single managed process. This function is called once per process lifetime (one-shot) and handles priority class, CPU affinity, CPU set, IO priority, and memory priority. It opens a handle to the target process and delegates to the individual apply functions in the `apply` module.

## Syntax

```rust
fn apply_config_process_level(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process identifier (PID) of the target process.

`config`

A reference to the [ProcessConfig](../config.rs/ProcessConfig.md) that contains the desired settings for this process. Fields such as `priority`, `affinity_cpus`, `cpu_set_cpus`, `io_priority`, and `memory_priority` are read from this structure.

`process`

A mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. This is mutated by `apply_affinity` and `apply_process_default_cpuset` to update cached thread ideal processor state and CPU set assignments.

`dry_run`

When `true`, the function simulates changes and records what would be applied in `apply_config_result` without making any actual Win32 API calls. When `false`, settings are applied to the live process.

`apply_config_result`

A mutable reference to an [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) accumulator. Each sub-function appends its changes and errors to this structure. The caller inspects the result after the call to log changes and errors.

## Return value

This function does not return a value. If the process handle cannot be obtained (for example, because the process exited or access was denied), the function returns early without applying any settings.

## Remarks

The function calls the following apply functions in order:

1. [apply_priority](../apply.rs/apply_priority.md) — Sets the process priority class (e.g., High, Above Normal).
2. [apply_affinity](../apply.rs/apply_affinity.md) — Sets the hard CPU affinity mask and resets thread ideal processors if the affinity changed.
3. [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) — Assigns the process default CPU set (soft CPU preference).
4. [apply_io_priority](../apply.rs/apply_io_priority.md) — Sets the process IO priority via `NtSetInformationProcess`.
5. [apply_memory_priority](../apply.rs/apply_memory_priority.md) — Sets the process memory priority via `SetProcessInformation`.

The ordering matters: affinity is set before CPU set because `apply_affinity` tracks `current_mask`, which may be referenced downstream. Priority is set first because some priority changes require specific handle access rights, and early failure avoids unnecessary work.

A process handle is obtained via [get_process_handle](../winapi.rs/get_process_handle.md), which requests both query and set access rights. If the handle cannot be opened (e.g., the process has exited, or the caller lacks `SeDebugPrivilege`), the function returns immediately and no settings are applied.

This function is designed to be called exactly once per process. The caller (`main`) tracks which PIDs have already been processed in a `HashSet<u32>` named `process_level_applied` and skips subsequent calls for the same PID. If the process exits and a new process with the same PID is created, the ETW monitor removes the PID from the applied set, allowing re-application.

In dry-run mode (`-dryrun` CLI flag), all sub-functions record their intended changes in `apply_config_result.changes` without invoking any Win32 APIs. This is useful for validating configuration before deployment.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main` |
| Callers | [main](main.md) (polling loop, ETW pending queue) |
| Callees | [get_process_handle](../winapi.rs/get_process_handle.md), [apply_priority](../apply.rs/apply_priority.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| API | `OpenProcess` (via `get_process_handle`), `SetPriorityClass`, `SetProcessAffinityMask`, `SetProcessDefaultCpuSets`, `NtSetInformationProcess`, `SetProcessInformation` |
| Privileges | `SeDebugPrivilege` (recommended), `SeIncreaseBasePriorityPrivilege` (for High/Realtime priority) |

## See Also

| Topic | Link |
|-------|------|
| Thread-level settings (per-iteration) | [apply_config_thread_level](apply_config_thread_level.md) |
| Process configuration structure | [ProcessConfig](../config.rs/ProcessConfig.md) |
| Apply result accumulator | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| Main entry point and polling loop | [main](main.md) |