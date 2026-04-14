# mask_from_cpusetids 函数 (winapi.rs)

将一组 Windows CPU 集合 ID 转换回 `usize` 类型的亲和性掩码，方法是在 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 缓存中查找每个 ID 的逻辑处理器索引并设置对应的位。

## 语法

```rust
#[allow(dead_code)]
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpuids` | `&[u32]` | 需要转换的 Windows CPU 集合 ID 切片。每个 ID 应为先前从 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 缓存或 Windows CPU 集合 API 获取的值。 |

## 返回值

一个 `usize` 位掩码，其中如果 `cpuids` 中有任何 CPU 集合 ID 映射到逻辑处理器索引 *N*，则第 *N* 位被设置。如果 `cpuids` 为空或没有 ID 与 CPU 集合缓存中的条目匹配，则返回 `0`。

**示例：**

| 输入 CPU 集合 ID | 查找到的逻辑索引 | 输出掩码 |
|-------------------|-----------------|----------|
| `[]` | — | `0x0` |
| 映射到 CPU 0、1 的 ID | 0, 1 | `0x3` |
| 映射到 CPU 0、2、4 的 ID | 0, 2, 4 | `0x15` |

## 备注

此函数是 [cpusetids_from_mask](cpusetids_from_mask.md) 的逆操作。两者共同构成亲和性掩码与 CPU 集合 ID 之间的往返转换：

```
mask → cpusetids_from_mask → cpusetids → mask_from_cpusetids → mask  (恒等变换)
```

### 算法

1. 如果输入切片为空，立即返回 `0`。
2. 锁定 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 互斥锁。
3. 遍历缓存中的每个 [CpuSetData](CpuSetData.md) 条目。
4. 对于每个 `id` 出现在 `cpuids` 中的条目，在累加掩码中设置 `logical_processor_index` 对应的位（通过 `idx < 64` 检查防止移位溢出）。
5. 返回累加的掩码。

### 64 处理器限制

逻辑处理器索引大于或等于 64 的会被静默跳过，因为 64 位 Windows 上的 `usize` 位掩码只能表示处理器 0–63。具有跨多个处理器组超过 64 个逻辑处理器的系统应使用基于 CPU 集合的 API（[cpusetids_from_indices](cpusetids_from_indices.md)、[indices_from_cpusetids](indices_from_cpusetids.md)）而非基于掩码的函数。

### 线程安全

该函数在查找期间持有 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 互斥锁。当 `MutexGuard` 在函数末尾离开作用域时，锁被释放。

### dead_code 注解

该函数标注了 `#[allow(dead_code)]`，因为当前代码库中可能未直接调用它，但作为完整 CPU 集合转换工具集的一部分予以保留。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **依赖** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)、[get_cpu_set_information](get_cpu_set_information.md) |
| **API** | 无（纯粹基于缓存数据的查找） |
| **特权** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 逆向转换（掩码 → CPU 集合 ID） | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 索引 → CPU 集合 ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| CPU 集合 ID → CPU 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| 按掩码过滤索引 | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU 集合拓扑数据 | [CpuSetData](CpuSetData.md) |
| 全局 CPU 集合缓存 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd