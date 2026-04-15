# apply_affinity 函数 (apply.rs)

`apply_affinity` 函数通过 `GetProcessAffinityMask` 读取当前进程亲和性掩码，如果与配置的目标掩码不同，则通过 `SetProcessAffinityMask` 设置新掩码。亲和性成功更改后，该函数还会调用 [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) 在新的 CPU 集合上重新分配线程理想处理器。作为副作用，该函数会将当前（或新应用的）亲和性掩码写入调用方提供的 `current_mask` 输出参数。

## 语法

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

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于错误日志记录，并传递给 `reset_thread_ideal_processors`。 |
| `config` | `&ProcessLevelConfig` | 进程级配置，包含 `affinity_cpus`（CPU 索引列表）和日志消息中使用的进程 `name`。如果 `affinity_cpus` 为空，函数将立即返回且不做任何更改。 |
| `dry_run` | `bool` | 当为 `true` 时，函数在 `apply_config_result` 中记录*将要*执行的更改，而不调用任何 Windows API 修改状态。在 dry-run 模式下读取操作也会被跳过（`GetProcessAffinityMask` 的错误将被抑制）。 |
| `current_mask` | `&mut usize` | 输出参数。查询或设置成功时，`*current_mask` 将被更新以反映进程的亲和性掩码。此值将被下游函数（如 [`apply_prime_threads_promote`](apply_prime_threads_promote.md)）使用，以根据当前亲和性筛选优选 CPU 索引。 |
| `process_handle` | `&ProcessHandle` | 提供对进程的读写访问的句柄包装器。函数通过 [`get_handles`](get_handles.md) 提取 `r_handle`（用于 `GetProcessAffinityMask`）和 `w_handle`（用于 `SetProcessAffinityMask`）。如果任一句柄不可用，函数将提前返回。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 返回线程 ID 到 `SYSTEM_THREAD_INFORMATION` 快照映射的惰性闭包。该闭包仅在亲和性成功更改后 `reset_thread_ideal_processors` 需要重新分配理想处理器时才被调用。这种延迟求值避免了在未发生亲和性更改时构建线程映射的开销。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行过程中产生的更改描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过 `current_mask` 输出参数和 `apply_config_result` 累加器传达。

## 备注

- 目标亲和性掩码通过 `cpu_indices_to_mask` 从 `config.affinity_cpus` 计算得出。如果生成的掩码为 `0` 或与当前进程亲和性掩码匹配，则不应用任何更改。
- 当亲和性掩码成功更改时，`*current_mask` 将被更新为新的目标掩码，并以 `dry_run: false` 和 `config.affinity_cpus` 作为 CPU 列表调用 [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md)。这会在新的亲和性集合上重新分配线程理想处理器，以防止 Windows 在掩码更改后将线程集中到单个 CPU 上。
- 该函数通过 `GetProcessAffinityMask` 同时查询进程亲和性掩码和系统亲和性掩码，但仅使用进程掩码进行比较。系统掩码被丢弃。
- `GetProcessAffinityMask` 的错误仅在 `dry_run` 为 `false` 时才通过 [`log_error_if_new`](log_error_if_new.md) 记录。在 dry-run 模式下，查询失败将被静默忽略，并根据配置的目标生成合成更改消息。
- `SetProcessAffinityMask` 的错误通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetProcessAffinityMask` 记录。当设置操作失败时，`current_mask` **不会**被更新。
- 更改消息格式为 `"Affinity: {current:#X} -> {target:#X}"`，显示十六进制亲和性掩码。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows API | `GetProcessAffinityMask`、`SetProcessAffinityMask`、`GetLastError` |
| 调用方 | `scheduler.rs` / `main.rs` 中的编排代码 |
| 被调用方 | [`get_handles`](get_handles.md)、[`log_error_if_new`](log_error_if_new.md)、[`reset_thread_ideal_processors`](reset_thread_ideal_processors.md)、`cpu_indices_to_mask`（config 模块）、`error_from_code_win32`（error_codes 模块） |
| 权限 | 读取需要 `PROCESS_QUERY_INFORMATION` 或 `PROCESS_QUERY_LIMITED_INFORMATION`，写入需要 `PROCESS_SET_INFORMATION`。这些权限封装在 `ProcessHandle` 句柄中。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| reset_thread_ideal_processors | [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) |
| apply_process_default_cpuset | [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |

---
*Commit: b0df9da35213b050501fab02c3020ad4dbd6c4e0*