# filter_indices_by_mask 函数 (winapi.rs)

将逻辑 CPU 索引列表筛选为仅包含亲和性掩码中对应位被设置的那些索引。用于将配置的 CPU 索引列表与进程的实际亲和性取交集，确保 Prime 线程绑定和 CPU 集合操作仅针对进程允许使用的处理器。

## 语法

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `cpu_indices` | `&[u32]` | 要筛选的逻辑 CPU 索引（从零开始）切片。每个值代表一个逻辑处理器编号（例如，`0`、`1`、`5`、`12`）。 |
| `affinity_mask` | `usize` | 位掩码，其中位 *N* 被设置表示逻辑处理器 *N* 被允许。通常来自 `GetProcessAffinityMask` 或从配置中构造。 |

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 栈分配的列表，仅包含 `cpu_indices` 中对应位在 `affinity_mask` 中被设置的索引。元素顺序与输入顺序匹配。如果没有索引通过筛选，返回空列表。

## 说明

每个索引的筛选条件为：

```
idx < 64 && ((1usize << idx) & affinity_mask) != 0
```

- **64 位限制：** 索引 ≥ 64 被静默排除，因为 64 位 Windows 上的 `usize` 只能通过位位置表示处理器 0-63。具有超过 64 个逻辑处理器的系统通过处理器组处理，这些由 CPU 集合 API 单独处理。
- **无去重：** 如果输入 `cpu_indices` 包含重复值，则输出也将包含任何通过筛选的重复值。
- **不排序：** 输出保留原始输入顺序。如果需要排序输出，调用方必须单独对结果排序。
- 函数使用迭代器链（`filter` + `copied` + `collect`）且不执行堆分配 — 结果被收集到基于栈的 `List<[u32; CONSUMER_CPUS]>` 中。

### 典型用法

此函数由 [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) 调用，以确保 Prime 线程 CPU 索引仅限于进程当前亲和性掩码内的处理器。例如，如果配置指定 Prime CPU 为 `[4, 5, 6, 7]`，但进程亲和性掩码为 `0x3F`（CPU 0-5），则函数返回 `[4, 5]`。

### 示例

| `cpu_indices` | `affinity_mask` | 结果 |
|---------------|-----------------|--------|
| `[0, 2, 4]` | `0x15`（位 0、2、4）| `[0, 2, 4]` |
| `[0, 2, 4]` | `0x05`（位 0、2）| `[0, 2]` |
| `[0, 1, 2]` | `0x00` | `[]`（空） |
| `[64, 65]` | `0xFFFFFFFFFFFFFFFF` | `[]`（索引 ≥ 64 被排除） |

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)，其他需要交集配置的索引与进程亲和性的应用函数 |
| **被调用方** | 无（纯计算） |
| **Win32 API** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 索引 → CPU 集合 ID 转换 | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 掩码 → CPU 集合 ID 转换 | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID → 索引转换 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合 ID → 掩码转换 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| Prime 线程提升 | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*