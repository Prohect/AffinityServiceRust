# cpusetids_from_indices 函数 (winapi.rs)

将逻辑 CPU 索引数组（0、1、2、……）转换为其对应的 Windows CPU Set ID。Windows CPU Set 使用不一定与逻辑处理器编号匹配的不透明标识符；此函数使用缓存的系统 CPU 集合信息执行转换。

## 语法

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | 要转换的逻辑处理器索引切片。每个值对应操作系统报告的从零开始的逻辑处理器编号。 |

## 返回值

返回一个 `List<[u32; CONSUMER_CPUS]>`（内联容量为 32 的 `SmallVec`），包含与给定逻辑索引对应的 CPU Set ID。如果 `cpu_indices` 为空，则返回空列表。不匹配系统 CPU 集合信息中任何条目的索引将被静默跳过。

## 备注

- 该函数获取 `CPU_SET_INFORMATION` 静态变量的锁，以遍历缓存的 `CpuSetData` 条目。每个条目的 `logical_processor_index` 与提供的索引使用 `slice::contains` 进行比较。
- 由于查找通过对 CPU 集合缓存和输入切片的线性扫描执行，性能为 O(n × m)，其中 n 是系统上的逻辑处理器数量，m 是 `cpu_indices` 的长度。对于典型的 CPU 数量（≤ 128 核心）和小型输入切片，这是可以接受的。
- 返回的 `SmallVec` 使用 `CONSUMER_CPUS`（32）的内联容量。如果生成超过 32 个 CPU Set ID，向量将溢出到堆分配。
- 此函数**不会**验证索引是否在可用逻辑处理器的范围内。超出范围的索引不会产生匹配输出。
- CPU 集合信息在进程启动时通过 `GetSystemCpuSetInformation` 查询一次，并在进程的整个生命周期内缓存。运行时 CPU 拓扑的变化（例如热添加处理器）不会被反映。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` — 在调用 `SetProcessDefaultCpuSets` 或 `SetThreadSelectedCpuSets` 之前，将配置中指定的 CPU 索引转换为 CPU Set ID 的规则应用逻辑。 |
| **被调用者** | [get_cpu_set_information](get_cpu_set_information.md)（间接通过 `CPU_SET_INFORMATION` 锁） |
| **Windows API** | 无直接调用；依赖 `GetSystemCpuSetInformation` 的缓存结果。 |
| **权限** | 无需特殊权限。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CpuSetData 结构体 | [CpuSetData](CpuSetData.md) |
| CPU_SET_INFORMATION 静态变量 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](../collections.rs/CONSUMER_CPUS.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
