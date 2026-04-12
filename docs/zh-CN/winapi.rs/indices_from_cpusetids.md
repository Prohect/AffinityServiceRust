# indices_from_cpusetids 函数 (winapi.rs)

将 CPU 集 ID 切片转换回对应的逻辑处理器索引。

## 语法

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
```

## 参数

`cpuids`

由 Windows CPU 集 API（例如 `GetProcessDefaultCpuSets`）返回的 CPU 集 ID 切片。每个 ID 是系统在 CPU 集枚举期间分配的不透明标识符。

## 返回值

返回 `Vec<u32>`，包含与提供的 CPU 集 ID 对应的从零开始的逻辑处理器索引。输出顺序与输入顺序一致。不匹配任何已知 CPU 集条目的 ID 将被静默忽略。

## 备注

此函数是 [`cpusetids_from_indices`](cpusetids_from_indices.md) 的逆操作。它在全局 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 向量（通过 [`get_cpu_set_information`](get_cpu_set_information.md) 访问）中查找每个 CPU 集 ID，并返回匹配的 [`CpuSetData`](CpuSetData.md) 条目的 `logical_processor_index` 字段。

返回的索引从零开始，直接对应于亲和性掩码中的位位置以及配置文件中使用的处理器编号。

此函数在查找期间获取 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 互斥锁。函数返回时释放锁。

### 转换函数族

| 函数 | 源 | 目标 |
| --- | --- | --- |
| [cpusetids_from_indices](cpusetids_from_indices.md) | 逻辑处理器索引 | CPU 集 ID |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 亲和性位掩码 | CPU 集 ID |
| **indices_from_cpusetids** | **CPU 集 ID** | **逻辑处理器索引** |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU 集 ID | 亲和性位掩码 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L392–L408 |
| **调用方** | [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md)、[`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) |
| **调用** | [`get_cpu_set_information`](get_cpu_set_information.md) |

## 另请参阅

- [cpusetids_from_indices](cpusetids_from_indices.md)
- [mask_from_cpusetids](mask_from_cpusetids.md)
- [CpuSetData 结构体](CpuSetData.md)
- [winapi.rs 模块概述](README.md)