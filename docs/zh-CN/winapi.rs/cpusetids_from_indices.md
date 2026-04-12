# cpusetids_from_indices 函数 (winapi.rs)

将逻辑处理器索引切片转换为对应的 Windows CPU 集 ID。

## 语法

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
```

## 参数

`cpu_indices`

要转换的从零开始的逻辑处理器索引切片。这些索引对应于亲和性掩码中的位位置以及配置文件中使用的处理器编号。

## 返回值

返回 `Vec<u32>`，包含与给定处理器索引对应的 Windows CPU 集 ID。不匹配任何已知 CPU 集条目的索引将被静默跳过。

## 备注

此函数针对全局 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 静态变量（通过 [`get_cpu_set_information`](get_cpu_set_information.md) 访问）执行查找。对于 `cpu_indices` 中的每个索引，在 [`CpuSetData`](CpuSetData.md) 向量中搜索 `logical_processor_index` 匹配的条目，并收集对应的 `id` 值。

此转换是必要的，因为 Windows CPU 集 API（`SetProcessDefaultCpuSets`、`SetThreadSelectedCpuSets`）需要 CPU 集 ID 而非处理器索引。配置文件为了用户方便使用从零开始的索引来指定处理器，因此此函数充当两者之间的桥梁。

逆操作由 [`indices_from_cpusetids`](indices_from_cpusetids.md) 提供。

### 相关转换

| 函数 | 源 | 目标 |
| --- | --- | --- |
| **cpusetids_from_indices** | 处理器索引 | CPU 集 ID |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 亲和性掩码 | CPU 集 ID |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU 集 ID | 处理器索引 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU 集 ID | 亲和性掩码 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L357–L374 |
| **调用方** | [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md)、[`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) |
| **调用** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## 另请参阅

- [cpusetids_from_mask](cpusetids_from_mask.md)
- [indices_from_cpusetids](indices_from_cpusetids.md)
- [CpuSetData 结构体](CpuSetData.md)
- [winapi.rs 模块概述](README.md)