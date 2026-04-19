# apply_process_level function (main.rs)

Opens a process handle for the given PID and applies all process-level settings — priority class, CPU affinity mask, default CPU set, IO priority, and memory priority — in a single pass. This is the process-level wrapper called once per process when it first matches a configuration rule (or every iteration if continuous apply is enabled).

## Syntax

```rust
fn apply_process_level<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## Parameters

`pid: u32`

The process identifier of the target process.

`config: &ProcessLevelConfig`

The [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) containing the desired process-level settings (priority, affinity CPUs, CPU set CPUs, IO priority, memory priority). The `config.name` field is used when opening the process handle for error reporting.

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

A lazily-evaluated closure that returns the process's thread map, keyed by thread ID. This is shared with the thread-level path via a `OnceCell` in the caller ([`apply_config`](apply_config.md)). The thread map is needed by [`apply_affinity`](../apply.rs/apply_affinity.md) (to reset ideal processors after an affinity change) and [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md).

`dry_run: bool`

When **true**, all downstream `apply_*` functions record what *would* change without calling any Windows API. When **false**, changes are applied to the live process.

`apply_configs: &mut ApplyConfigResult`

Accumulator for change descriptions and error messages. See [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md).

## Return value

This function does not return a value. All outcomes (changes applied, errors encountered) are recorded in `apply_configs`.

## Remarks

The function follows a fixed order of operations:

1. **Open handle** — Calls [`get_process_handle`](../winapi.rs/get_process_handle.md) to obtain a [`ProcessHandle`](../winapi.rs/ProcessHandle.md). If the handle cannot be opened (e.g., access denied, process exited), the function returns immediately with no effect and no error recorded.
2. **Apply priority** — Delegates to [`apply_priority`](../apply.rs/apply_priority.md).
3. **Apply affinity** — Delegates to [`apply_affinity`](../apply.rs/apply_affinity.md). A local `current_mask` variable is passed to capture the process's current affinity mask for downstream use.
4. **Apply CPU set** — Delegates to [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md).
5. **Apply IO priority** — Delegates to [`apply_io_priority`](../apply.rs/apply_io_priority.md).
6. **Apply memory priority** — Delegates to [`apply_memory_priority`](../apply.rs/apply_memory_priority.md).
7. **Drop handle** — The `ProcessHandle` is explicitly dropped after all operations complete.

Each `apply_*` function independently checks whether its corresponding config field is set to `None` and short-circuits if so. This means a config that only specifies priority and affinity will not touch IO or memory priority.

### Thread enumeration cost

The `threads` closure is only invoked if an `apply_*` function actually needs thread information (e.g., `apply_affinity` resets ideal processors, or `apply_process_default_cpuset` redistributes threads). The `OnceCell` backing ensures the thread enumeration happens at most once across both process-level and thread-level application.

## Requirements

| | |
|---|---|
| **Module** | [`src/main.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/main.rs) |
| **Callers** | [`apply_config`](apply_config.md) |
| **Callees** | [`get_process_handle`](../winapi.rs/get_process_handle.md), [`apply_priority`](../apply.rs/apply_priority.md), [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md), [`apply_io_priority`](../apply.rs/apply_io_priority.md), [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| **Win32 API** | None directly (delegated to callees) |
| **Privileges** | `SeDebugPrivilege` (for opening handles to elevated processes) |

## See Also

| Topic | Link |
|-------|------|
| Thread-level counterpart | [`apply_thread_level`](apply_thread_level.md) |
| Combined caller | [`apply_config`](apply_config.md) |
| Apply engine overview | [apply.rs](../apply.rs/README.md) |
| Configuration struct | [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| Result accumulator | [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*