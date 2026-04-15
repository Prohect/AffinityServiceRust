# mask_from_cpusetids 函数 (winapi.rs)

将一组 Windows CPU Set ID 转换回亲和性位掩码，其中每个置位的比特对应于该 CPU Set ID 所关联的逻辑处理器索引。

## 语法

```rust
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpuids` | `&[u32]` | 需要转换回位掩码表示的不透明 Windows CPU Set ID 切片。 |

## 返回值

返回一个 `usize` 位掩码，其中每个置位的比特表示与所提供的某个 CPU Set ID 对应的逻辑处理器索引。如果 `cpuids` 为空或在系统 CPU 集合信息缓存中未找到匹配项，则返回 `0`。

## 备注

- 此函数是 [cpusetids_from_mask](cpusetids_from_mask.md) 的逆操作。给定先前从 [cpusetids_from_indices](cpusetids_from_indices.md) 或 [cpusetids_from_mask](cpusetids_from_mask.md) 获取的 CPU Set ID，它可以重建相应的亲和性位掩码。
- 该函数获取全局静态变量 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 的互斥锁，并遍历所有缓存的 `CpuSetData` 条目，检查每个条目的 `id` 是否在提供的切片中。
- 输出掩码中仅表示逻辑处理器索引小于 64 的条目，因为 64 位 Windows 上 `usize` 为 64 位宽。`logical_processor_index` 大于或等于 64 的条目将被静默跳过。
- 此函数标记为 `#[allow(dead_code)]`，表示它可能保留供将来使用或仅在特定条件下使用。

### 算法

1. 如果 `cpuids` 为空，立即返回 `0`。
2. 将 `mask` 初始化为 `0`。
3. 锁定 `CPU_SET_INFORMATION` 缓存。
4. 对于缓存中的每个 `CpuSetData` 条目：
   - 如果 `cpuids` 包含该条目的 `id` **且**该条目的 `logical_processor_index` 小于 64，则通过 `mask |= 1 << idx` 在 `mask` 中设置相应的位。
5. 返回累积的 `mask`。

### 与其他转换函数的关系

| 方向 | 函数 |
|------|------|
| 索引 → CPU Set ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 掩码 → CPU Set ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set ID → 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU Set ID → 掩码 | **mask_from_cpusetids**（本函数） |

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **依赖** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)、[CpuSetData](CpuSetData.md) |
| **平台** | Windows（使用缓存的 `GetSystemCpuSetInformation` 数据） |
| **权限** | 无需特殊权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| filter_indices_by_mask | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU_SET_INFORMATION 静态变量 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CpuSetData 结构体 | [CpuSetData](CpuSetData.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
