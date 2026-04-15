# apply_process_level function (main.rs)

Applies one-shot process-level settings to a single Windows process identified by its PID. This function obtains a process handle and then delegates to specialized apply helpers for each setting category: priority class, processor affinity (with thread ideal-processor reset), default CPU set, I/O priority, and memory priority. It is called once per process (unless continuous process-level apply is enabled via CLI flag).

## Syntax

```rust
fn apply_process_level(
    pid: u32,
    config: &ProcessLevelConfig,
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The Windows process identifier of the target process. |
| `config` | `&ProcessLevelConfig` | The process-level configuration block that describes the desired priority class, affinity mask, CPU set IDs, I/O priority, and memory priority for this process. |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A map of thread IDs to their `SYSTEM_THREAD_INFORMATION` snapshots. Used by affinity and CPU-set helpers to reset per-thread ideal processors after a process-wide affinity change. |
| `dry_run` | `bool` | When `true`, the function logs what it *would* do but does not call any Win32 APIs to mutate process state. |
| `apply_configs` | `&mut ApplyConfigResult` | Accumulator for changes and errors produced during the apply pass. Populated by each sub-function and later consumed by [`log_apply_results`](log_apply_results.md). |

## Return value

This function does not return a value. If the process handle cannot be obtained (e.g., insufficient privileges or the process has already exited), the function returns early without applying any settings. All outcomes—successes and failures—are recorded in the `apply_configs` accumulator.

## Remarks

- The function calls `get_process_handle` first. If this returns `None` (access denied, process exited, etc.), the entire function is a no-op.
- A local `current_mask` variable is initialized to `0` and passed to `apply_affinity`, which populates it with the current affinity mask if an affinity change is requested. This mask is used internally by affinity helpers to determine whether ideal-processor resets are necessary.
- The order of application is deterministic: priority → affinity → CPU set → I/O priority → memory priority. This order ensures that the process priority class is set before any thread-level side effects of affinity changes take place.
- Each sub-function (`apply_priority`, `apply_affinity`, `apply_process_default_cpuset`, `apply_io_priority`, `apply_memory_priority`) independently checks whether its corresponding config field is set to a `None` sentinel and skips itself when no change is requested.
- This function is **not** called on every polling iteration by default. Once a PID appears in `process_level_applied`, it is skipped unless the `-continuousProcessLevelApply` CLI flag is active.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main.rs` |
| Callers | [`apply_config`](apply_config.md) |
| Callees | `winapi::get_process_handle`, `apply::apply_priority`, `apply::apply_affinity`, `apply::apply_process_default_cpuset`, `apply::apply_io_priority`, `apply::apply_memory_priority` |
| Win32 API | Indirect — delegates to `apply` module functions that call `SetPriorityClass`, `SetProcessAffinityMask`, `SetProcessDefaultCpuSets`, `NtSetInformationProcess` |
| Privileges | `SeDebugPrivilege` (for opening handles to elevated/system processes) |

## See Also

| Reference | Link |
|-----------|------|
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| apply_config | [apply_config](apply_config.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| ProcessLevelConfig | [config module](../config.rs/README.md) |
| apply module | [apply module](../apply.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
