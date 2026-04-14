# CpuSetData 结构体 (winapi.rs)

表示系统 CPU 集合拓扑中的单个条目，将 Windows 分配的不透明 CPU Set ID 与其对应的逻辑处理器索引配对。此结构体是存储在 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 全局缓存中的元素类型，被模块中所有 CPU 集合转换函数使用。

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
|------|------|------|
| `id` | `u32` | Windows 分配的不透明 CPU Set ID。此值从 `SYSTEM_CPU_SET_INFORMATION::Anonymous.CpuSet.Id` 获取，是 `SetProcessDefaultCpuSets` 和 `SetThreadSelectedCpuSets` 等 API 所需的标识符。CPU Set ID **不是**顺序的，也**不**对应逻辑处理器编号。 |
| `logical_processor_index` | `u8` | 此 CPU 集合条目映射到的从零开始的逻辑处理器索引。从 `SYSTEM_CPU_SET_INFORMATION::Anonymous.CpuSet.LogicalProcessorIndex` 获取。这是在配置文件和亲和性掩码中使用的人类可读 CPU 编号（0、1、2、……）。`u8` 类型限制最多支持 256 个逻辑处理器。 |

## 备注

Windows CPU Set API 使用不透明的 32 位 ID 而非逻辑处理器索引进行操作。AffinityServiceRust 配置文件使用人类可读的 CPU 索引（例如 `0;1;4-7`），因此需要一个转换层。`CpuSetData` 通过为进程启动时 `GetSystemCpuSetInformation` 报告的每个 CPU 集合条目缓存 `(id, logical_processor_index)` 对来提供此映射。

### 初始化

`CpuSetData` 条目由 `extract_cpu_set_data` 辅助函数在 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 静态变量的延迟初始化期间构造。`GetSystemCpuSetInformation` 返回的原始 `SYSTEM_CPU_SET_INFORMATION` 缓冲区被逐条遍历，每个条目被简化为这个紧凑的双字段结构体。

### 可见性

两个字段都是模块私有的（没有 `pub` 修饰符）。外部代码通过转换函数间接访问数据：

- [cpusetids_from_indices](cpusetids_from_indices.md) — 通过匹配 `logical_processor_index` 查找 `id`
- [cpusetids_from_mask](cpusetids_from_mask.md) — 通过将 `logical_processor_index` 与位掩码进行测试来查找 `id`
- [indices_from_cpusetids](indices_from_cpusetids.md) — 通过匹配 `id` 查找 `logical_processor_index`
- [mask_from_cpusetids](mask_from_cpusetids.md) — 通过匹配 `id` 查找 `logical_processor_index` 并设置对应位

### 派生 trait

`CpuSetData` 派生了 `Clone` 和 `Copy`，使其可以低成本地按值传递和存储在向量中，无需间接引用。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **构造者** | `extract_cpu_set_data`（模块私有辅助函数） |
| **存储于** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| **使用者** | [cpusetids_from_indices](cpusetids_from_indices.md)、[cpusetids_from_mask](cpusetids_from_mask.md)、[indices_from_cpusetids](indices_from_cpusetids.md)、[mask_from_cpusetids](mask_from_cpusetids.md) |
| **API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 全局 CPU 集合缓存 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU 索引 → CPU Set ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| CPU Set ID → CPU 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| 亲和性掩码 → CPU Set ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU Set ID → 亲和性掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| 将 CPU 集合应用于进程 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd