# filter_indices_by_mask 函数 (winapi.rs)

过滤一组逻辑 CPU 索引，仅保留在给定亲和性掩码中对应位已设置的索引。该函数用于将用户指定的 CPU 列表与进程的有效亲和性掩码求交集，确保仅针对进程可实际运行的处理器进行 CPU 集合或理想处理器分配。

## 语法

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpu_indices` | `&[u32]` | 待过滤的零基逻辑处理器索引切片（例如 `[0, 1, 4, 5, 6, 7]`）。值 ≥ 64 的索引会被静默排除，因为它们无法在 64 位平台的 `usize` 位掩码中表示。 |
| `affinity_mask` | `usize` | 位掩码，其中第 *N* 位被设置表示逻辑处理器 *N* 是允许的。通常通过 `GetProcessAffinityMask` 获取，或通过配置中的 CPU 规格经 [`cpu_indices_to_mask`](../config.rs/cpu_indices_to_mask.md) 构造。 |

## 返回值

一个 `Vec<u32>`，包含 `cpu_indices` 中在 `affinity_mask` 对应位已设置的子集。元素顺序与输入切片保持一致。

**示例：**

| `cpu_indices` | `affinity_mask`（二进制） | 结果 |
|---------------|--------------------------|--------|
| `[0, 1, 2, 3]` | `0b0101` (5) | `[0, 2]` |
| `[4, 5, 6, 7]` | `0b1111_0000` (0xF0) | `[4, 5, 6, 7]` |
| `[0, 1, 2]` | `0` | `[]` |
| `[]` | `0xFF` | `[]` |
| `[64, 65]` | `usize::MAX` | `[]` |

## 备注

该函数使用迭代器组合子（`filter` + `copied` + `collect`）实现简洁、高效的分配。每个索引通过表达式 `idx < 64 && ((1usize << idx) & affinity_mask) != 0` 进行测试，在位移之前先执行边界检查以避免未定义宽度的位移操作。

### 与 CPU 集合函数的关系

该函数在**亲和性掩码**层面操作，而非 CPU 集合 ID 层面。它与 CPU 集合转换函数互为补充：

- [cpusetids_from_indices](cpusetids_from_indices.md) 将索引转换为 CPU 集合 ID。
- [cpusetids_from_mask](cpusetids_from_mask.md) 将亲和性掩码转换为 CPU 集合 ID。
- `filter_indices_by_mask` 在转换**之前**通过亲和性掩码缩小索引列表。

### 64 处理器限制

`idx < 64` 的保护条件反映了 `GetProcessAffinityMask` 使用的单处理器组亲和性掩码模型。在拥有超过 64 个逻辑处理器（多处理器组）的系统上，索引 ≥ 64 的处理器无法在单个 `usize` 位掩码中表示，因此会被过滤掉。多处理器组的亲和性通过 CPU 集合 API 单独处理，该 API 使用不透明 ID 而非位掩码位置。

### 在 apply 模块中的使用

[apply_affinity](../apply.rs/apply_affinity.md) 和 [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) 函数调用 `filter_indices_by_mask` 将配置的 CPU 列表与进程的当前亲和性掩码求交集，确保理想处理器仅分配给进程可实际运行的 CPU。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用方** | [apply_affinity](../apply.rs/apply_affinity.md)、[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **依赖** | 无（纯计算） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CPU 索引 → CPU 集合 ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 亲和性掩码 → CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID → CPU 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合 ID → 亲和性掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| CPU 索引转位掩码工具 | [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) |
| 亲和性应用逻辑 | [apply_affinity](../apply.rs/apply_affinity.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd