# cpusetids_from_indices 函数 (winapi.rs)

将一组逻辑 CPU 索引（0, 1, 2, …）转换为对应的 Windows CPU 集合 ID，方法是在 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 全局缓存中查找每个索引。这是从配置文件应用 CPU 集合规则时所使用的主要转换函数，用户在配置中指定人类可读的 CPU 索引。

## 语法

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpu_indices` | `&[u32]` | 需要转换的从零开始的逻辑处理器索引切片。这些索引对应于任务管理器中可见的 CPU 编号或配置文件中指定的编号（例如 `0;1;4-7`）。允许重复值，但会在输出中产生重复的 ID。 |

## 返回值

一个 `Vec<u32>`，包含与所提供的逻辑处理器索引对应的 Windows CPU 集合 ID。输出顺序遵循 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 缓存的迭代顺序，**而非**输入索引的顺序。如果某个输入索引在缓存中没有匹配的条目（例如索引超出逻辑处理器数量），则该索引会被静默地从结果中省略。

如果 `cpu_indices` 为空，则立即返回空的 `Vec`。

## 备注

### 转换机制

该函数获取 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 的互斥锁，然后遍历缓存中的每个 [CpuSetData](CpuSetData.md) 条目。对于 `logical_processor_index` 出现在输入切片中的每个条目，将该条目的 `id` 字段推入结果向量。

这实际上是一个查找连接：对于缓存中的每个 `(id, logical_processor_index)` 对，如果 `logical_processor_index ∈ cpu_indices`，则输出 `id`。

### 性能考虑

当前实现执行 O(n × m) 扫描，其中 n 是缓存大小（逻辑处理器数量），m 是输入切片长度，因为它对每个缓存条目调用 `cpu_indices.contains()`。对于典型的桌面和服务器处理器数量（≤ 256 个逻辑处理器）和典型的规则大小（≤ 64 个 CPU），这可以忽略不计。

### 排序

输出的 CPU 集合 ID 按缓存中的存储顺序出现（与 `GetSystemCpuSetInformation` 返回的顺序一致）。这通常按逻辑处理器索引升序排列，但调用者不应依赖任何特定的排序。消费 CPU 集合 ID 的 API（如 `SetProcessDefaultCpuSets`）将它们视为无序集合。

### 空输入快速路径

如果输入切片为空，函数会立即返回空的 `Vec`，而不获取互斥锁，从而避免不必要的同步。

### 用法示例

假设系统中 CPU 0 的集合 ID 为 256，CPU 1 的集合 ID 为 257，CPU 2 的集合 ID 为 258：

| 输入 | 输出 |
|------|------|
| `&[0, 2]` | `vec![256, 258]` |
| `&[1]` | `vec![257]` |
| `&[99]` | `vec![]`（无此 CPU） |
| `&[]` | `vec![]` |

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用者** | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **被调用** | [get_cpu_set_information](get_cpu_set_information.md)（获取 `Mutex<Vec<CpuSetData>>` 锁） |
| **依赖** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md)、[CpuSetData](CpuSetData.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 反向操作：CPU 集合 ID → 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| 亲和性掩码 → CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID → 亲和性掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU 集合拓扑缓存 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU 集合缓存访问器 | [get_cpu_set_information](get_cpu_set_information.md) |
| 将 CPU 集合应用于进程 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd