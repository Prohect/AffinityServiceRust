# cpusetids_from_mask 函数 (winapi.rs)

将亲和性位掩码转换为 CPU 集 ID 向量，通过将每个置位的位位置映射到其对应的 CPU 集标识符。

## 语法

```rust
pub fn cpusetids_from_mask(mask: usize) -> Vec<u32>
```

## 参数

`mask`

亲和性位掩码，其中每个置位的位表示一个逻辑处理器。位 0 对应逻辑处理器 0，位 1 对应逻辑处理器 1，依此类推。

## 返回值

返回 `Vec<u32>`，包含与掩码中每个置位的位对应的 CPU 集 ID。向量按从最低位到最高位的顺序排列。如果某个置位的位不对应任何已知的 CPU 集条目，则静默跳过。

## 备注

此函数遍历提供的掩码中每个置位的位，根据位位置确定逻辑处理器索引，然后通过 [`get_cpu_set_information`](get_cpu_set_information.md) 在全局 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 表中查找对应的 CPU 集 ID。

当需要将旧式亲和性掩码表示（`SetProcessAffinityMask` 使用）转换为较新的 CPU 集表示（`SetProcessDefaultCpuSets` 和 `SetThreadSelectedCpuSets` 使用）时，此函数非常有用。

该函数获取 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 的锁来执行查找。每个 [`CpuSetData`](CpuSetData.md) 条目的 `logical_processor_index` 与掩码中置位的位位置进行比较。

### 转换函数族

| 函数 | 源 | 目标 |
| --- | --- | --- |
| [cpusetids_from_indices](cpusetids_from_indices.md) | `&[u32]` 索引 | `Vec<u32>` CPU 集 ID |
| **cpusetids_from_mask** | `usize` 掩码 | `Vec<u32>` CPU 集 ID |
| [indices_from_cpusetids](indices_from_cpusetids.md) | `&[u32]` CPU 集 ID | `Vec<u32>` 索引 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | `&[u32]` CPU 集 ID | `usize` 掩码 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L377–L390 |
| **调用方** | [`apply_affinity`](../apply.rs/apply_affinity.md) |
| **调用** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## 另请参阅

- [cpusetids_from_indices](cpusetids_from_indices.md)
- [mask_from_cpusetids](mask_from_cpusetids.md)
- [CpuSetData 结构体](CpuSetData.md)
- [CPU_SET_INFORMATION 静态变量](CPU_SET_INFORMATION.md)