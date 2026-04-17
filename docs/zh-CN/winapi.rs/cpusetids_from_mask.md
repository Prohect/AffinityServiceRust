# cpusetids_from_mask 函数 (winapi.rs)

将亲和性位掩码转换为 Windows CPU Set ID 列表。掩码中每个置位的比特对应一个逻辑处理器索引；该函数从系统 CPU 集合信息缓存中查找匹配的 CPU Set ID。

## 语法

```rust
pub fn cpusetids_from_mask(mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `mask` | `usize` | 一个位掩码，其中每个置位的比特代表一个逻辑处理器索引（比特 0 = CPU 0，比特 1 = CPU 1，以此类推）。最多支持 64 个逻辑处理器。 |

## 返回值

返回一个 `List<[u32; CONSUMER_CPUS]>`（`SmallVec`），包含与掩码中置位比特对应的 CPU Set ID。如果 `mask` 为 `0`，则返回空列表。

## 备注

- 此函数是 [cpusetids_from_indices](cpusetids_from_indices.md) 的位掩码版本，后者接受显式的索引切片。
- Windows CPU Set ID 是不透明的标识符，与逻辑处理器索引不直接对应。此函数通过查询系统 CPU 集合信息缓存（[CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)）来桥接这一差异。
- 该函数获取全局 `CPU_SET_INFORMATION` 静态变量的互斥锁，并遍历所有 CPU Set 条目，检查每个条目的 `logical_processor_index` 是否与位掩码匹配。
- 仅考虑小于 64 的逻辑处理器索引，因为 64 位 Windows 上的 `usize` 为 64 位宽。
- 此函数标记为 `#[allow(dead_code)]`，表示它可能保留供将来使用或有条件地使用。

### 算法

1. 如果 `mask` 为 `0`，立即返回空列表。
2. 锁定 `CPU_SET_INFORMATION` 缓存。
3. 对于缓存中的每个 `CpuSetData` 条目：
   - 如果该条目的 `logical_processor_index` 小于 64 **且**对应的比特在 `mask` 中已置位，则将该条目的 `id` 推入结果列表。
4. 返回收集到的 CPU Set ID。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **依赖** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)、[CpuSetData](CpuSetData.md) |
| **类型** | `List<[u32; CONSUMER_CPUS]>`，来自 [collections](../collections.rs/README.md) |
| **平台** | Windows（使用 `GetSystemCpuSetInformation` 数据） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU_SET_INFORMATION 静态变量 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CpuSetData 结构体 | [CpuSetData](CpuSetData.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
