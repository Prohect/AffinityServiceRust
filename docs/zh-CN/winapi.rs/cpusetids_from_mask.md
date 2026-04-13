# cpusetids_from_mask 函数 (winapi.rs)

将 `usize` 类型的亲和性掩码转换为 Windows CPU 集合 ID 向量，方法是将掩码中的每个位位置与 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 缓存进行对照测试。掩码中每个已设置的位如果对应一个已知的逻辑处理器索引，则会在输出中产生匹配的 CPU 集合 ID。

## 语法

```rust
#[allow(dead_code)]
pub fn cpusetids_from_mask(mask: usize) -> Vec<u32>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `mask` | `usize` | 一个位掩码，其中每个已设置的位代表一个逻辑处理器索引。位 0 对应 CPU 0，位 1 对应 CPU 1，依此类推。仅低 64 位有意义（逻辑处理器索引 0–63）。值为 `0` 时立即返回空结果。 |

## 返回值

一个 `Vec<u32>`，包含掩码中每个已设置位对应的逻辑处理器的 Windows CPU 集合 ID。输出中 ID 的顺序遵循 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 缓存的迭代顺序（即 `GetSystemCpuSetInformation` 返回的顺序）。如果 `mask` 为 `0` 或没有位对应已知处理器，则返回空向量。

**示例：**

| `mask`（二进制） | 已设置的逻辑 CPU | 输出（CPU 集合 ID） |
|-----------------|-----------------|---------------------|
| `0b0000` | （无） | `[]` |
| `0b0101` | 0, 2 | `[<CPU 0 的 ID>, <CPU 2 的 ID>]` |
| `0b1111` | 0, 1, 2, 3 | `[<CPU 0 的 ID>, <CPU 1 的 ID>, <CPU 2 的 ID>, <CPU 3 的 ID>]` |

## 备注

此函数是 [cpusetids_from_indices](cpusetids_from_indices.md) 的位掩码版本。`cpusetids_from_indices` 接受一个显式的 CPU 索引值列表，而 `cpusetids_from_mask` 则使用 `GetProcessAffinityMask` 及相关 Win32 API 所用的紧凑位掩码表示。

### 算法

1. 如果 `mask == 0`，立即返回空 `Vec`。
2. 锁定 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 互斥锁。
3. 遍历缓存中的每个 [CpuSetData](CpuSetData.md) 条目。
4. 对于每个 `logical_processor_index` 小于 64 的条目，测试对应的位 `(1usize << logical_processor_index) & mask` 是否非零。
5. 如果已设置，则将该条目的 `id` 推入结果向量。

### 64 处理器限制

该函数在进行位测试前会显式检查 `logical_processor_index < 64`。在拥有超过 64 个逻辑处理器（多处理器组）的系统上，索引 63 以上的处理器无法用单个 `usize` 掩码表示，因此会被静默排除。对于此类系统，应优先使用基于索引的函数（[cpusetids_from_indices](cpusetids_from_indices.md)）。

### dead_code 注解

该函数标注了 `#[allow(dead_code)]`，因为当前主应用代码路径未直接引用它。保留此函数是作为基于掩码的 CPU 集合工作流的实用工具，同时也是为了与 [mask_from_cpusetids](mask_from_cpusetids.md) 保持对称性。

### 线程安全

该函数在迭代期间持有 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 互斥锁。当 `MutexGuard` 在函数末尾超出作用域时，锁会被释放。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **被调用方** | [get_cpu_set_information](get_cpu_set_information.md)（通过 `CPU_SET_INFORMATION` 访问） |
| **依赖** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md), [CpuSetData](CpuSetData.md) |
| **API** | 无直接调用；转换由 [`GetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) 产生的掩码 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 基于索引的转换 | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 反向转换：CPU 集合 ID → 掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| 反向转换：CPU 集合 ID → 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| 按掩码过滤索引 | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU 集合拓扑缓存 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU 集合数据元素 | [CpuSetData](CpuSetData.md) |