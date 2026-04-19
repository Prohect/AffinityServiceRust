# apply_process_default_cpuset 函数 (apply.rs)

使用 Windows CPU 集合 API 为进程设置默认 CPU 集合，提供一种软 CPU 偏好设置，调度程序会遵循这种偏好而不会硬性限制线程执行。

## 语法

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

## 参数

`pid: u32`

目标进程的进程 ID。

`config: &ProcessLevelConfig`

包含 `cpu_set_cpus`（所需的 CPU 索引列表）和 `cpu_set_reset_ideal`（应用 CPU 集合后是否重置理想处理器的标志）的进程级配置。

`dry_run: bool`

如果为 `true`，将记录 `apply_config_result` 中将要进行的更改，而不调用任何 Windows API。

`process_handle: &ProcessHandle`

目标进程的句柄包装器。需要读取和写入句柄。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

进程线程映射的延迟访问器。仅当 `cpu_set_reset_ideal` 为 `true` 且应用了更改时才会评估，此时会转发到 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。

`apply_config_result: &mut ApplyConfigResult`

在操作期间生成的更改消息和错误消息的累加器。

## 返回值

无。结果累积到 `apply_config_result` 中。

## 备注

与通过 `SetProcessAffinityMask` 设置的硬性亲和性掩码不同，CPU 集合提供的是**软偏好**。Windows 调度程序会优先使用指定的 CPU，但在高负载下也可能在其他 CPU 上调度线程。这使得 CPU 集合成为在现代 Windows 上进行工作负载引导的首选机制。

### 算法

1. **提前退出** — 如果 `config.cpu_set_cpus` 为空或全局 CPU 集合信息缓存为空，则立即返回。
2. **试运行模式** — 如果 `dry_run` 为 `true`，记录预期的 CPU 集合并返回。
3. **转换索引** — 通过 `cpusetids_from_indices` 将配置的 CPU 索引转换为 Windows CPU 集合 ID。
4. **查询当前值** — 首先使用 `None` 缓冲区调用 `GetProcessDefaultCpuSets`：
   - 如果成功，表示进程未分配默认 CPU 集合，因此需要更改。
   - 如果以错误代码 **122**（`ERROR_INSUFFICIENT_BUFFER`）失败，表示进程已有 CPU 集合。使用正确大小的缓冲区进行第二次调用以检索当前集 ID 进行比较。
   - 任何其他错误都通过 [log_error_if_new](log_error_if_new.md) 记录。
5. **比较** — 如果当前 CPU 集合 ID 与目标匹配，则不采取任何操作。
6. **重置理想处理器（可选）** — 如果 `config.cpu_set_reset_ideal` 为 `true`，在应用新的 CPU 集合之前调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md) 并传入 `config.cpu_set_cpus`。这可以防止过时的理想处理器分配覆盖新的 CPU 偏好。
7. **应用** — 使用目标 CPU 集合 ID 调用 `SetProcessDefaultCpuSets`。
8. **记录** — 成功时，记录显示从旧索引到新索引过渡的更改消息。失败时，记录错误。

### 两阶段查询模式

`GetProcessDefaultCpuSets` API 使用常见的 Windows 模式，其中第一次调用确定所需缓冲区大小。错误代码 122（`ERROR_INSUFFICIENT_BUFFER`）是预期的条件，而不是真正的错误，会触发第二次调用并使用适当大小的缓冲区。

### 与亲和性掩码的交互

CPU 集合和亲和性掩码是独立的机制。进程可以同时具有硬性亲和性掩码和默认 CPU 集合。有效的调度取决于 Windows 内部逻辑，但通常亲和性掩码作为硬性约束优先，而 CPU 集合作为该约束内的提示。

## 要求

| | |
|---|---|
| **模块** | [apply.rs](README.md) |
| **调用方** | `main.rs` 强制循环 |
| **被调用方** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), `cpusetids_from_indices`, `indices_from_cpusetids`, `format_cpu_indices` |
| **Win32 API** | [GetProcessDefaultCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets), [SetProcessDefaultCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets), [GetLastError](https://learn.microsoft.com/zh-cn/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| **权限** | 需要目标进程的 `PROCESS_QUERY_LIMITED_INFORMATION`（读取）和 `PROCESS_SET_LIMITED_INFORMATION`（写入）访问权限 |
| **最低操作系统** | Windows 10 版本 1607（CPU 集合 API） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [apply_affinity](apply_affinity.md) | 硬性亲和性掩码的替代方案 |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | 在 CPU 集合更改后重新分配线程理想处理器 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `cpu_set_cpus` 和 `cpu_set_reset_ideal` 字段的配置结构体 |
| [ApplyConfigResult](ApplyConfigResult.md) | 更改和错误的累加器 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*