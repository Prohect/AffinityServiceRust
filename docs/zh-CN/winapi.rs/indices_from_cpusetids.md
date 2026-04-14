# indices_from_cpusetids 函数 (winapi.rs)

将一组 Windows CPU 集合 ID 转换回对应的逻辑处理器索引。这是 [cpusetids_from_indices](cpusetids_from_indices.md) 的逆操作，用于将系统返回的 CPU 集合分配读取回配置文件所使用的人类友好索引表示。

## 语法

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpuids` | `&[u32]` | 需要转换的不透明 Windows CPU 集合 ID 切片。这些值通常由 `GetProcessDefaultCpuSets` 或 `GetThreadSelectedCpuSets` 等 API 返回。 |

## 返回值

一个已排序的 `Vec<u32>`，包含与输入 CPU 集合 ID 对应的逻辑处理器索引。如果 `cpuids` 为空，则返回空向量。任何在 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 缓存中不匹配的 CPU 集合 ID 会被静默跳过。

## 备注

### 算法

1. 如果输入切片为空，立即返回空 `Vec`（快速路径）。
2. 获取 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 的互斥锁。
3. 遍历缓存中的每个 [CpuSetData](CpuSetData.md) 条目。
4. 对于每个 `id` 包含在 `cpuids` 中的条目，将其 `logical_processor_index`（转换为 `u32`）推入结果向量。
5. 返回前对结果向量进行排序。

### 排序

与按缓存迭代顺序返回 ID 的 [cpusetids_from_indices](cpusetids_from_indices.md) 不同，`indices_from_cpusetids` 会显式对输出进行排序。这确保了无论 CPU 集合条目在系统缓存中的出现顺序或输入中 ID 的顺序如何，CPU 索引始终以稳定的升序排列。

### 与其他转换函数的对称性

| 从 | 到 | 函数 |
|----|----|------|
| CPU 索引 | CPU 集合 ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 亲和性掩码 | CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| **CPU 集合 ID** | **CPU 索引** | **indices_from_cpusetids**（本函数） |
| CPU 集合 ID | 亲和性掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |

### 锁竞争

该函数在迭代期间持有 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 的互斥锁。由于缓存在初始化后是只读的，竞争通常可以忽略不计，但执行批量转换的调用者应注意每次调用都会独立获取和释放锁。

### 未匹配的 ID

如果输入中的某个 CPU 集合 ID 在系统缓存中不存在（例如，因为它来自不同的机器或拓扑在热添加后发生了变化），该 ID 会被静默排除在输出之外。不会记录错误或警告。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | 公共（`pub fn`） |
| **调用方** | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **被调用方** | [get_cpu_set_information](get_cpu_set_information.md)（通过 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)） |
| **依赖** | `once_cell::sync::Lazy`、`std::sync::Mutex` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 逆操作（索引 → ID） | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 掩码转 CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID 转掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU 集合拓扑缓存 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| 缓存元素类型 | [CpuSetData](CpuSetData.md) |
| CPU 集合应用逻辑 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetProcessDefaultCpuSets (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd