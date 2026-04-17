# indices_from_cpusetids 函数 (winapi.rs)

将 Windows CPU Set ID 数组转换回对应的逻辑处理器索引。此函数是 [cpusetids_from_indices](cpusetids_from_indices.md) 的逆操作，用于从操作系统读取 CPU Set 分配并将其转换为面向用户的处理器编号。

## 语法

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpuids` | `&[u32]` | 需要转换回逻辑处理器索引的 Windows CPU Set ID 切片。这些是由 `GetProcessDefaultCpuSets` 或 `GetThreadSelectedCpuSets` 等 API 返回的不透明标识符。 |

## 返回值

返回一个 `List<[u32; CONSUMER_CPUS]>`（内联容量为 32 的 `SmallVec`），其中包含与给定 CPU Set ID 对应的逻辑处理器索引。返回的列表按**升序排列**。如果 `cpuids` 为空，则返回空列表。不匹配系统 CPU Set 信息缓存中任何条目的 CPU Set ID 将被静默跳过。

## 备注

- 该函数获取 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 静态变量的锁，并对所有缓存的 [CpuSetData](CpuSetData.md) 条目执行线性扫描。对于 `id` 出现在输入 `cpuids` 切片中的每个条目，其对应的 `logical_processor_index`（转换为 `u32`）将被推入结果列表。
- 收集所有匹配的索引后，通过 `indices.sort()` 进行排序，以确保无论 CPU Set 缓存或输入切片的顺序如何，结果都是确定性的升序排列。
- 查找复杂度为 O(n × m)，其中 n 是系统 CPU Set 的数量，m 是 `cpuids` 的长度。对于典型的 CPU 数量和较小的输入切片，这是可以接受的。
- 返回的 `SmallVec` 使用 `CONSUMER_CPUS`（32）的内联容量。如果产生的索引超过 32 个，向量将溢出到堆分配。
- CPU Set 信息在进程启动时通过 `GetSystemCpuSetInformation` 查询一次，并在进程生命周期内缓存。运行时拓扑更改不会被反映。

### 算法

1. 如果 `cpuids` 为空，立即返回空列表。
2. 锁定 `CPU_SET_INFORMATION` 缓存。
3. 对于缓存中的每个 `CpuSetData` 条目：
   - 如果该条目的 `id` 包含在 `cpuids` 中，则将该条目的 `logical_processor_index`（转换为 `u32`）推入结果列表。
4. 将结果列表按升序排序。
5. 返回排序后的列表。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` — 读取当前 CPU Set 分配以进行比较和日志记录。 |
| **被调用者** | [get_cpu_set_information](get_cpu_set_information.md)（间接通过 `CPU_SET_INFORMATION` 锁） |
| **Windows API** | 无直接调用；依赖于 `GetSystemCpuSetInformation` 的缓存结果。 |
| **权限** | 无需特殊权限。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CpuSetData 结构体 | [CpuSetData](CpuSetData.md) |
| CPU_SET_INFORMATION 静态变量 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](../collections.rs/CONSUMER_CPUS.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
