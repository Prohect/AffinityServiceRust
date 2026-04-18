# filter_indices_by_mask 函数 (winapi.rs)

过滤逻辑 CPU 索引切片，仅保留给定亲和性位掩码允许的索引。此函数用于确保 CPU Set 分配不包含超出进程当前亲和性掩码范围的处理器。

## 语法

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | 需要过滤的从零开始的逻辑处理器索引切片。 |
| `affinity_mask` | `usize` | 位掩码，每个置位的位代表一个允许的逻辑处理器（位 0 = CPU 0，位 1 = CPU 1，依此类推）。 |

## 返回值

返回 `List<[u32; CONSUMER_CPUS]>`（内联容量为 32 的 `SmallVec`），仅包含 `cpu_indices` 中对应位在 `affinity_mask` 中被置位的索引。输入索引的顺序被保留。

## 备注

- 该函数执行简单的按位检查：对于 `cpu_indices` 中的每个索引，验证该索引小于 64 且 `(1usize << idx) & affinity_mask` 非零。大于等于 64 的索引会被静默排除，因为 64 位 Windows 上 `usize` 为 64 位宽。

- 此函数仅对索引和位掩码进行操作——它**不会**查询 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 缓存或调用任何 Windows API。它是一个 CPU 本地计算，没有锁定或副作用。

- 返回的 `SmallVec` 使用 `CONSUMER_CPUS` (32) 的内联容量。如果过滤结果超过 32 个条目，向量将溢出到堆分配。

- 典型用例：当配置规则指定了期望的 CPU 索引，但目标进程具有受限的亲和性掩码（例如由父进程或作业对象设置），此函数将两个集合取交集，以便仅分配有效的 CPU。

### 算法

1. 遍历 `cpu_indices` 中的每个 `idx`。
2. 对每个 `idx` 检查：
   - `idx < 64`（适合位掩码宽度），**且**
   - `(1usize << idx) & affinity_mask != 0`（对应位已置位）。
3. 通过 `Iterator::filter` 和 `Iterator::copied` 将通过检查的索引收集到结果列表中。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` — 在将 CPU 索引转换为 CPU Set ID 之前，用于将 CPU 索引限制在进程允许的亲和性掩码范围内。 |
| **被调用者** | 无（纯计算）。 |
| **类型** | `List<[u32; CONSUMER_CPUS]>`，来自 [collections](../collections.rs/README.md) |
| **平台** | 平台无关的逻辑，但仅在使用亲和性掩码的 Windows 上有意义。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| is_affinity_unset | [is_affinity_unset](is_affinity_unset.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](../collections.rs/CONSUMER_CPUS.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
