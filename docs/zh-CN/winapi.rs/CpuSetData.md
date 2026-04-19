# CpuSetData 结构体 (winapi.rs)

轻量级缓存记录，保存 `SYSTEM_CPU_SET_INFORMATION` 项中的关键字段。服务在启动时通过 `GetSystemCpuSetInformation` 枚举 CPU 集合拓扑，并将结果存储为 `Vec<CpuSetData>`，保存在静态 [CPU_SET_INFORMATION](README.md) 缓存中。所有后续的 CPU 索引 ↔ CPU 集合 ID 转换都在此缓存数据上执行，而不是重新查询操作系统。

## 语法

```rust
#[derive(Clone, Copy)]
pub struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `id` | `u32` | 由 Windows 分配的不透明 CPU 集合 ID（来自 `SYSTEM_CPU_SET_INFORMATION.CpuSet.Id`）。此值传递给 `SetProcessDefaultCpuSets` 和 `SetThreadSelectedCpuSets` 等 API。CPU 集合 ID **不是**连续的，也不对应于逻辑处理器索引。 |
| `logical_processor_index` | `u8` | 此 CPU 集合条目的从零开始的逻辑处理器索引（来自 `SYSTEM_CPU_SET_INFORMATION.CpuSet.LogicalProcessorIndex`）。这是亲和性掩码中使用的相同索引——亲和性掩码中的位 *N* 对应于 `logical_processor_index == N`。存储为 `u8`，支持每组最多 256 个逻辑处理器。 |

## 备注

- `CpuSetData` 派生 `Clone` 和 `Copy`，因为它是小型的（5 字节）、仅堆栈的值类型，没有堆分配或资源所有权。
- 结构体字段是**模块私有的**（没有 `pub` 可见性）。所有访问都通过转换函数进行，这些函数遍历缓存的 `Vec<CpuSetData>`：
  - [cpusetids_from_indices](cpusetids_from_indices.md) — 逻辑索引 → CPU 集合 ID
  - [cpusetids_from_mask](cpusetids_from_mask.md) — 亲和性掩码 → CPU 集合 ID
  - [indices_from_cpusetids](indices_from_cpusetids.md) — CPU 集合 ID → 逻辑索引
  - [mask_from_cpusetids](mask_from_cpusetids.md) — CPU 集合 ID → 亲和性掩码
  - [filter_indices_by_mask](filter_indices_by_mask.md) — 按亲和性掩码过滤索引
- `SYSTEM_CPU_SET_INFORMATION` 联合包含许多其他字段（例如 `Group`、`NumaNodeIndex`、`LastLevelCacheIndex`、`CoreIndex`、`EfficiencyClass`），这些字段未保存在 `CpuSetData` 中。只有 `Id` 和 `LogicalProcessorIndex` 对服务的 CPU 集合和亲和性操作是必要的。

### 拓扑缓存

静态 `CPU_SET_INFORMATION` 是一个 `Lazy<Mutex<Vec<CpuSetData>>>`，在首次访问时初始化一次。初始化调用 `GetSystemCpuSetInformation` 两次——首先确定所需缓冲区大小，然后填充它——并使用每个条目的 `Size` 字段作为步幅遍历变长条目。生成的 `Vec<CpuSetData>` 被 `Mutex` 锁定，初始化后永不修改。

### CPU 集合 ID 与逻辑索引

| 概念 | 示例 | 由...使用 |
|---------|---------|---------|
| 逻辑处理器索引 | `0`, `1`, `2`, … | 亲和性掩码、配置文件 `affinity_cpus` 列表 |
| CPU 集合 ID | `256`, `257`, `258`, …（不透明） | `SetProcessDefaultCpuSets`、`SetThreadSelectedCpuSets` |

配置文件使用对人类友好的逻辑索引。Windows CPU 集合 API 需要不透明 ID。`CpuSetData` 桥接了这个差距。

## 要求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [cpusetids_from_indices](cpusetids_from_indices.md)、[cpusetids_from_mask](cpusetids_from_mask.md)、[indices_from_cpusetids](indices_from_cpusetids.md)、[mask_from_cpusetids](mask_from_cpusetids.md)、[get_cpu_set_information](README.md) |
| **依赖** | `SYSTEM_CPU_SET_INFORMATION` (windows crate) |
| **Win32 API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation)（在缓存初始化时） |
| **特权** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概览 | [winapi.rs](README.md) |
| 索引 → CPU 集合 ID 转换 | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 掩码 → CPU 集合 ID 转换 | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID → 索引转换 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合 ID → 掩码转换 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU 集合应用到进程 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |

*文档记录于 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*