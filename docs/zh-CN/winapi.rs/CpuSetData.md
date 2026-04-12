# CpuSetData 结构体 (winapi.rs)

存储 CPU 集 ID 及其对应的逻辑处理器索引，构成 Windows CPU 集标识符与应用程序中使用的从零开始的处理器索引之间的映射。

## 语法

```rust
pub struct CpuSetData {
    pub id: u32,
    pub logical_processor_index: u8,
}
```

## 成员

`id`

由 `GetSystemCpuSetInformation` 返回的 Windows CPU 集标识符。这是 `SetProcessDefaultCpuSets` 和 `SetThreadSelectedCpuSets` 等 API 使用的不透明 ID。

`logical_processor_index`

此 CPU 集对应的从零开始的逻辑处理器索引。该值直接映射到亲和性掩码中的位位置以及配置文件中使用的处理器编号。

## 备注

`CpuSetData` 条目在启动时通过调用 `GetSystemCpuSetInformation` 收集到全局静态变量 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 中。生成的向量提供了 CPU 集 ID（CPU 集 API 使用）与逻辑处理器索引（亲和性掩码和面向用户的配置使用）之间的权威映射。

该结构体设计上尽量精简——仅提取本模块转换函数所需的两个字段。Windows API 返回的完整 `SYSTEM_CPU_SET_INFORMATION` 结构包含额外字段（处理器组、NUMA 节点、缓存信息等），但本应用程序不需要这些信息。

使用此数据的转换函数包括：

- [`cpusetids_from_indices`](cpusetids_from_indices.md) — 通过匹配 `logical_processor_index` 查找 `id`
- [`cpusetids_from_mask`](cpusetids_from_mask.md) — 为掩码中每个置位的位位置查找 `id`
- [`indices_from_cpusetids`](indices_from_cpusetids.md) — 通过匹配 `id` 查找 `logical_processor_index`
- [`mask_from_cpusetids`](mask_from_cpusetids.md) — 从 `logical_processor_index` 值构建位掩码

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **行号** | L63–L66 |
| **存储于** | [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) |
| **使用方** | [`cpusetids_from_indices`](cpusetids_from_indices.md)、[`cpusetids_from_mask`](cpusetids_from_mask.md)、[`indices_from_cpusetids`](indices_from_cpusetids.md)、[`mask_from_cpusetids`](mask_from_cpusetids.md) |

## 另请参阅

- [CPU_SET_INFORMATION 静态变量](CPU_SET_INFORMATION.md)
- [get_cpu_set_information 函数](get_cpu_set_information.md)
- [winapi.rs 模块概述](README.md)