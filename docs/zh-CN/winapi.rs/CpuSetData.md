# CpuSetData 结构体 (winapi.rs)

持有一个 CPU Set ID 及其对应的逻辑处理器索引。这是从 Windows `SYSTEM_CPU_SET_INFORMATION` 结构体中提取的数据的内部表示，用于在面向用户的逻辑处理器索引和 Windows CPU Set API 所需的不透明 CPU Set ID 之间进行转换。

## 语法

```rust
#[derive(Clone, Copy)]
pub struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
```

## 成员

| 字段 | 类型 | 描述 |
|-------|------|-------------|
| `id` | `u32` | 由 Windows 分配的不透明 CPU Set ID。此值传递给 `SetProcessDefaultCpuSets` 和 `SetThreadSelectedCpuSets` 等 API。 |
| `logical_processor_index` | `u8` | 与此 CPU Set 对应的从零开始的逻辑处理器索引。这是面向用户的索引（0、1、2、……），映射到物理或逻辑核心。 |

## 备注

- 两个字段均为**模块私有**（没有 `pub` 可见性）。外部代码仅通过转换函数 [cpusetids_from_indices](cpusetids_from_indices.md)、[cpusetids_from_mask](cpusetids_from_mask.md)、[indices_from_cpusetids](indices_from_cpusetids.md) 和 [mask_from_cpusetids](mask_from_cpusetids.md) 间接访问 `CpuSetData`。

- 该结构体派生了 `Clone` 和 `Copy`，使其可以低成本地按值传递并存储在 `Vec<CpuSetData>` 中，无需担心引用生命周期问题。

- 实例由 `extract_cpu_set_data` 辅助函数创建，该函数在 `unsafe` 块中从原始 `SYSTEM_CPU_SET_INFORMATION` 结构体的联合体字段中读取数据。

- 全局静态变量 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 存储一个 `Vec<CpuSetData>`，在初始化时填充一次，随后的所有 CPU Set 查找都复用该数据。

- `logical_processor_index` 字段存储为 `u8`，最多支持 256 个逻辑处理器。这涵盖了单个处理器组内绝大多数消费级和服务器级系统。

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `winapi.rs` |
| 填充者 | `extract_cpu_set_data`（unsafe，模块私有） |
| 存储于 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 静态变量 |
| 平台 | 仅限 Windows |
| 底层 API | `GetSystemCpuSetInformation`（通过 `SYSTEM_CPU_SET_INFORMATION`） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| CPU_SET_INFORMATION 静态变量 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
