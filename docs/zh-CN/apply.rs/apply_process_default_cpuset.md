# apply_process_default_cpuset 函数 (apply.rs)

`apply_process_default_cpuset` 函数通过 `GetProcessDefaultCpuSets` 查询当前分配给进程的默认 CPU Set ID，如果与配置的目标不同，则通过 `SetProcessDefaultCpuSets` 应用新的集合。当 `cpu_set_reset_ideal` 配置标志启用时，该函数还会在应用更改之前调用 [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) 在新的 CPU 集合上重新分配线程的理想处理器。此函数操作的是 CPU Set ID（而非亲和性掩码），这是 Windows 用于控制进程到 CPU 分配的现代机制，不受传统亲和性掩码的限制。

## 语法

```AffinityServiceRust/src/apply.rs#L297-307
pub fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于错误去重和日志消息。 |
| `config` | `&ProcessLevelConfig` | 进程级配置，包含 `cpu_set_cpus`（要转换为 CPU Set ID 的 CPU 索引列表）、`cpu_set_reset_ideal`（控制是否在更改时重新分配线程理想处理器的布尔值）和 `name`（日志消息中使用的可读配置规则名称）。如果 `cpu_set_cpus` 为空，函数将立即返回而不做任何更改。 |
| `dry_run` | `bool` | 为 `true` 时，函数在 `apply_config_result` 中记录*将要*进行的更改，但不调用任何 Windows API 来修改状态。为 `false` 时，调用 Windows API 应用更改。 |
| `process_handle` | `&ProcessHandle` | 提供对进程的读写访问的句柄包装器。函数通过 [`get_handles`](get_handles.md) 提取 `r_handle`（用于 `GetProcessDefaultCpuSets`）和 `w_handle`（用于 `SetProcessDefaultCpuSets`）。如果任一句柄不可用，函数将提前返回。 |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射。当 `cpu_set_reset_ideal` 启用时，传递给 [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md)。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行期间产生的更改描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过 `apply_config_result` 参数传达。

## 备注

- 如果 `config.cpu_set_cpus` 为空**或者**全局 CPU 集合信息（来自 `get_cpu_set_information()`）为空，函数将提前退出且不执行任何操作。后一个条件确保在没有系统 CPU 集合信息可用时，函数不会尝试将 CPU 索引转换为 CPU Set ID。
- 配置的 CPU 索引通过 `cpusetids_from_indices` 转换为 Windows CPU Set ID。如果转换后的 ID 列表为空，则不应用更改。
- 查询使用 `GetProcessDefaultCpuSets` 的两次调用模式：
  1. **第一次调用**使用 `None` 缓冲区：如果成功，表示进程尚未分配默认 CPU 集合，`toset` 设为 `true`。
  2. 如果第一次调用因 Win32 错误码 `122`（`ERROR_INSUFFICIENT_BUFFER`）失败，则进行**第二次调用**，使用适当大小的缓冲区来获取当前的 CPU Set ID。然后将获取到的 ID 与目标进行比较；只有在它们不同时 `toset` 才为 `true`。
  3. 如果第一次调用因任何其他错误码失败，错误通过 [`log_error_if_new`](log_error_if_new.md) 记录，函数不会尝试设置 CPU 集合。
- 当 `config.cpu_set_reset_ideal` 为 `true` 且需要更改时，[`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) 在应用 CPU 集合**之前**被调用，使用 `config.cpu_set_cpus` 作为目标 CPU 列表。这将在新的 CPU 集合分配之前重新分配线程的理想处理器。
- 成功时，更改消息格式为 `"CPU Set: [<old>] -> [<new>]"`，其中 `<old>` 和 `<new>` 是格式化的 CPU 索引列表。当进程之前没有默认 CPU 集合时，`<old>` 为空列表。
- `SetProcessDefaultCpuSets` 失败时，错误通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetProcessDefaultCpuSets` 记录。
- 当前 CPU Set ID 通过 `indices_from_cpusetids` 解码回 CPU 索引，用于更改消息中的"旧值"。

## 要求

| 要求 | 值 |
|------|------|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows API | `GetProcessDefaultCpuSets`、`SetProcessDefaultCpuSets`、`GetLastError` |
| 调用方 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用方 | [`get_handles`](get_handles.md)、[`log_error_if_new`](log_error_if_new.md)、[`reset_thread_ideal_processors`](reset_thread_ideal_processors.md)、`cpusetids_from_indices`、`indices_from_cpusetids`、`get_cpu_set_information`、`format_cpu_indices`、`error_from_code_win32` |
| 权限 | 需要具有 `PROCESS_QUERY_LIMITED_INFORMATION`（读取）和 `PROCESS_SET_LIMITED_INFORMATION`（写入）的进程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| reset_thread_ideal_processors | [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| winapi 模块 | [`winapi.rs`](../winapi.rs/README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*