# filter_indices_by_mask 函数 (winapi.rs)

按亲和性位掩码过滤逻辑处理器索引列表，仅保留掩码中对应位已置位的索引。

## 语法

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
```

## 参数

`cpu_indices`

要过滤的从零开始的逻辑处理器索引切片。

`affinity_mask`

位掩码，其中每个置位的位表示一个允许的逻辑处理器。位位置 `n` 对应逻辑处理器索引 `n`。

## 返回值

返回 `Vec<u32>`，仅包含 `cpu_indices` 中在 `affinity_mask` 中对应位已置位的索引。元素顺序与输入切片保持一致。

## 备注

此函数执行简单的按位交集操作：对于 `cpu_indices` 中的每个索引，检查 `affinity_mask & (1 << index)` 是否非零，仅当结果非零时才将该索引包含在输出中。

当进程同时具有配置的 CPU 列表和已有的亲和性掩码约束时，此函数非常有用。例如，如果配置指定 CPU 为 `[0, 2, 4, 6]`，但进程已有亲和性掩码 `0b00110101`（CPU 0、2、4、5），则函数返回 `[0, 2, 4]` — 即配置集合与当前允许集合的交集。

该函数不修改全局 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md)，也不执行任何 Windows API 调用。它是一个纯计算辅助函数。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L428–L435 |
| **调用方** | [`apply_affinity`](../apply.rs/apply_affinity.md)、[`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md) |

## 另请参阅

- [cpusetids_from_indices](cpusetids_from_indices.md)
- [cpusetids_from_mask](cpusetids_from_mask.md)
- [mask_from_cpusetids](mask_from_cpusetids.md)
- [winapi.rs 模块概述](README.md)