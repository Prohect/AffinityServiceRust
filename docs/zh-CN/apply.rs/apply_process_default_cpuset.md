# apply_process_default_cpuset 函数 (apply.rs)

通过 Windows `SetProcessDefaultCpuSets` API 为进程应用软性 CPU 集合偏好。与硬性亲和性掩码（无条件限制*所有*线程）不同，默认 CPU 集合建立了一组首选的逻辑处理器，线程将在这些处理器上调度，除非在线程级别通过 `SetThreadSelectedCpuSets` 进行覆盖。这使得 CPU 集合成为引导工作负载的首选机制，同时不会阻止个别线程（如主力线程）被固定到其他处理器上。

## 语法

```AffinityServiceRust/src/apply.rs#L315-323
pub fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用于日志记录以及作为错误去重映射中的键。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 与此进程匹配的已解析配置规则。`cpu_set_cpus` 字段提供所需的 CPU 索引；`cpu_set_reset_ideal` 标志控制在 CPU 集合更改后是否重新分配线程的理想处理器。 |
| `dry_run` | `bool` | 当为 `true` 时，函数将预期的更改记录到 `apply_config_result` 中，但不调用任何 Windows API。 |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | 为目标进程打开的 OS 句柄。通过 [get_handles](get_handles.md) 获取最佳可用的读取和写入 `HANDLE`。 |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | 对缓存的进程/线程快照的可变引用。当设置了 `cpu_set_reset_ideal` 时，转发给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 此调用期间产生的变更描述和错误消息的累加器。 |

## 返回值

无 (`()`)。

## 备注

### CPU 集合标识符

Windows CPU 集合由不透明的 32 位 ID 标识（与逻辑处理器索引不同）。该函数使用 [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md) 将 `config.cpu_set_cpus` 中面向用户的 CPU 索引转换为 API 所需的系统 CPU 集合 ID。如果系统级 CPU 集合信息为空（例如在不支持 CPU 集合的较旧 Windows 版本上），函数会立即返回而不做任何更改。

### 先查询后设置策略

该函数在决定是否设置之前执行两阶段查询：

1. **第一次查询** —— 调用 `GetProcessDefaultCpuSets`，缓冲区传入 `None`。如果成功，则进程当前*没有*默认 CPU 集合，因此 `toset` 被设置为 `true`。
2. **第二次查询** —— 如果第一次查询失败并返回 `ERROR_INSUFFICIENT_BUFFER`（Win32 错误 122），函数将分配所报告大小的缓冲区并再次查询以获取当前的 CPU 集合 ID。如果当前集合已与目标匹配，则不执行写入。

第一次查询中 122 以外的任何错误代码都被视为真正的失败，并通过 [log_error_if_new](log_error_if_new.md) 记录。

### 理想处理器重置

当 `config.cpu_set_reset_ideal` 为 `true` 且即将写入更改时，函数会在应用新的 CPU 集合*之前*使用 `config.cpu_set_cpus` 调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。这会在新的 CPU 集合上重新分配线程的理想处理器，使 OS 调度器将线程均匀分布，而不是将它们聚集在更改前碰巧作为理想处理器的那些处理器上。

### 变更消息格式

成功时变更消息遵循以下模式：

`"CPU Set: [0,1,2] -> [4,5,6]"`

其中左侧显示从先前活跃的 CPU 集合 ID 解码得到的 CPU 索引（通过 [indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md)），右侧显示配置中的索引。当进程之前没有 CPU 集合时，左侧为空列表 `[]`。

### 模拟运行行为

在模拟运行模式下，函数无条件地将目标 CPU 集合记录为变更，不查询当前状态，生成如下消息：

`"CPU Set: -> [4,5,6]"`

### 边界情况

- 如果 `config.cpu_set_cpus` 为空，函数立即返回——它永远不会*清除*现有的 CPU 集合。
- 如果 `cpusetids_from_indices` 返回空向量（给定索引未找到匹配的 CPU 集合 ID），则跳过写入。
- 该函数不会修改进程亲和性掩码；CPU 集合和亲和性掩码分别由 [apply_affinity](apply_affinity.md) 和本函数独立约束。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 被调用者 | [get_handles](get_handles.md)、[cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md)、[indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md)、[get_cpu_set_information](../winapi.rs/get_cpu_set_information.md)、[format_cpu_indices](../config.rs/format_cpu_indices.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md)、[log_error_if_new](log_error_if_new.md) |
| Win32 API | `GetProcessDefaultCpuSets`、`SetProcessDefaultCpuSets` |
| 权限 | `PROCESS_QUERY_LIMITED_INFORMATION`（读取）、`PROCESS_SET_LIMITED_INFORMATION`（写入） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 硬性 CPU 亲和性掩码 | [apply_affinity](apply_affinity.md) |
| 理想处理器重新分配 | [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| 线程级 CPU 集合固定（主力线程） | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| CPU 集合 ID 转换 | [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md)、[indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md) |
| 配置模型 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| apply 模块概述 | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd