# mask_from_cpusetids 函数 (winapi.rs)

将 CPU 集 ID 切片转换为亲和性位掩码，通过查找每个 ID 的逻辑处理器索引并设置对应的位。

## 语法

```rust
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## 参数

`cpuids`

要转换的 CPU 集 ID 切片。每个 ID 在 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 表中查找以找到其对应的逻辑处理器索引。

## 返回值

返回 `usize` 位掩码，其中每个置位的位对应一个逻辑处理器，该处理器的 CPU 集 ID 存在于输入切片中。如果映射到逻辑处理器索引 N 的 CPU 集 ID 在 `cpuids` 中被找到，则位 N 被置位。

## 备注

此函数执行 [`cpusetids_from_mask`](cpusetids_from_mask.md) 的逆操作。它锁定 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 互斥锁，遍历输入的 CPU 集 ID，对每个 ID 找到匹配的 [`CpuSetData`](CpuSetData.md) 条目以确定 `logical_processor_index`，然后通过按位或操作在结果掩码中设置对应的位。

在 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 中不匹配任何条目的 CPU 集 ID 将被静默忽略，不会影响结果掩码。

当需要将 CPU 集 API 表示转换回 `SetProcessAffinityMask` 等相关 API 所需的旧式亲和性掩码表示时，此函数非常有用。

### 转换函数族

| 函数 | 源 | 目标 |
| --- | --- | --- |
| [`cpusetids_from_indices`](cpusetids_from_indices.md) | 逻辑索引 | CPU 集 ID |
| [`cpusetids_from_mask`](cpusetids_from_mask.md) | 亲和性掩码 | CPU 集 ID |
| [`indices_from_cpusetids`](indices_from_cpusetids.md) | CPU 集 ID | 逻辑索引 |
| **mask_from_cpusetids** | **CPU 集 ID** | **亲和性掩码** |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L411–L426 |
| **调用方** | [`apply_affinity`](../apply.rs/apply_affinity.md)、[`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) |
| **调用** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## 另请参阅

- [cpusetids_from_mask](cpusetids_from_mask.md)
- [indices_from_cpusetids](indices_from_cpusetids.md)
- [CpuSetData 结构体](CpuSetData.md)
- [winapi.rs 模块概述](README.md)