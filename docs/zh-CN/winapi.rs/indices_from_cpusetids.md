# indices_from_cpusetids 函数 (winapi.rs)

将一组 CPU 集合 ID 转换回对应的逻辑处理器索引。这是 [cpusetids_from_indices](cpusetids_from_indices.md) 的逆操作，用于读取 CPU 集合分配以显示或与用户配置的 CPU 索引列表进行比较。

## 语法

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | 要转换的 CPU 集合 ID 切片。这些是 Windows CPU 集合 API 或之前调用 [cpusetids_from_indices](cpusetids_from_indices.md) / [cpusetids_from_mask](cpusetids_from_mask.md) 获得的不透明系统分配标识符。 |

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 与输入 CPU 集合 ID 对应的逻辑处理器索引的栈分配列表，按升序排序。如果 `cpuids` 为空，返回空列表。

## 说明

函数遍历在启动时通过 `GetSystemCpuSetInformation` 填充的缓存 [CPU_SET_INFORMATION](README.md) 拓扑数据。对于每个 `id` 字段匹配输入切片中值的缓存条目，条目的 `logical_processor_index` 被追加到结果列表。

### 排序顺序

返回的列表在返回前通过 `indices.sort()` 显式按升序排序。这提供了稳定、确定性的顺序，无论 CPU 集合在系统拓扑缓存或输入切片中出现的顺序如何。

### 与其他转换函数的关系

| 函数 | 方向 |
|----------|-----------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | 索引 → CPU 集合 ID |
| **indices_from_cpusetids** | CPU 集合 ID → 索引 |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 亲和性掩码 → CPU 集合 ID |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU 集合 ID → 亲和性掩码 |

### 锁获取

函数在迭代期间获取 `CPU_SET_INFORMATION` 静态变量的 `Mutex` 锁。锁持有时间为 O(N × M)，其中 N 是缓存的 CPU 集合条目数，M 是 `cpuids` 的长度。在消费级系统（≤64 个 CPU）上，这可以忽略不计。

### 未匹配的 ID

`cpuids` 中不匹配缓存拓扑数据中任何条目的 CPU 集合 ID 会被静默忽略。不报告错误。如果拓扑缓存过时或传递了无效 ID，可能会发生这种情况。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md)、诊断日志 |
| **被调用方** | [get_cpu_set_information](README.md) |
| **Win32 API** | 无直接调用；依赖来自 [GetSystemCpuSetInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) 的缓存数据 |
| **特权** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 前向转换（索引 → ID） | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 基于掩码的转换 | [cpusetids_from_mask](cpusetids_from_mask.md)、[mask_from_cpusetids](mask_from_cpusetids.md) |
| 按掩码过滤索引 | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU 集合拓扑缓存 | [CpuSetData](CpuSetData.md) |
| 模块概述 | [winapi.rs](README.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*