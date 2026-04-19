# cpusetids_from_indices 函数 (winapi.rs)

将一组逻辑 CPU 索引（从 0 开始的处理器编号）转换为对应的 Windows CPU 集合 ID。这种转换是必要的，因为 Windows CPU 集合 API 操作的是不透明的系统分配 ID，而不是用户友好的逻辑处理器索引。

## 语法

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | 要转换的从零开始的逻辑处理器索引切片。每个值代表一个逻辑处理器编号，如任务管理器或 `PROCESSOR_NUMBER::Number` 中看到的。 |

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 与输入索引对应的 CPU 集合 ID 的栈分配列表。如果某些索引与缓存的 CPU 集合拓扑中的任何条目不匹配，列表可能比输入短。如果 `cpu_indices` 为空，则返回空列表。

## 备注

- 函数获取 [CPU_SET_INFORMATION](README.md) 静态缓存的锁，并遍历所有缓存的 [CpuSetData](CpuSetData.md) 条目。对于 `logical_processor_index` 出现在输入切片中的每个条目，将其 `id` 附加到结果列表。
- CPU 集合拓扑在进程启动时通过 `GetSystemCpuSetInformation` 查询一次，并在服务生命周期内缓存。此函数不直接调用任何 Windows API — 它只读取缓存。
- 输出顺序遵循缓存 CPU 集合数据的迭代顺序，这与系统枚举顺序匹配（通常按逻辑处理器索引升序排列）。
- 结果使用 `List<[u32; CONSUMER_CPUS]>`，这是 `crate::collections` 中的栈分配固定容量列表，避免对最多 `CONSUMER_CPUS` 个逻辑处理器的系统进行堆分配。

### CPU 集合 ID 与逻辑索引

| 概念 | 示例 | 用途 |
|---------|---------|---------|
| 逻辑处理器索引 | `0`, `1`, `2`, ... | 配置文件、亲和性掩码、`PROCESSOR_NUMBER::Number` |
| CPU 集合 ID | `0x100`, `0x101`, ... | `SetProcessDefaultCpuSets`、`SetThreadSelectedCpuSets` 和其他 CPU 集合 API |

索引与 ID 之间的映射是特定于系统的，在启动时确定。同一物理核心在重新启动之间可能具有不同的 CPU 集合 ID。

### 与其他转换函数的关系

| 函数 | 方向 |
|----------|-----------|
| **cpusetids_from_indices** | 逻辑索引 → CPU 集合 ID |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 亲和性掩码 → CPU 集合 ID |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU 集合 ID → 逻辑索引 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU 集合 ID → 亲和性掩码 |
| [filter_indices_by_mask](filter_indices_by_mask.md) | 通过亲和性掩码过滤逻辑索引 |

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **被调用方** | [get_cpu_set_information](README.md)（读取缓存的 CPU 集合拓扑） |
| **Win32 API** | 不直接调用；依赖来自 [GetSystemCpuSetInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) 的缓存数据 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| CPU 集合拓扑缓存 | [CpuSetData](CpuSetData.md) |
| 反向转换 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| 基于掩码的转换 | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合应用 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*