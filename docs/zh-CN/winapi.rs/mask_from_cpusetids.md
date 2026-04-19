# mask_from_cpusetids 函数 (winapi.rs)

将一组 CPU 集合 ID 转换回处理器亲和性位掩码。这是 [cpusetids_from_mask](cpusetids_from_mask.md) 的逆操作，将不透明的 Windows CPU 集合 标识符映射到位掩码中的位置位。

## 语法

```rust
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `cpuids` | `&[u32]` | 要转换的 CPU 集合 ID 切片。每个 ID 必须对应于从 [get_cpu_set_information](../winapi.rs/README.md) 获得的系统 CPU 集合拓扑中的条目。 |

## 返回值

`usize` — 位掩码，其中如果提供的任何 CPU 集合 ID 映射到逻辑处理器索引 *N*，则位 *N* 被设置。如果输入切片为空，返回 `0`。

### 示例

| 输入 CPU 集合 ID | 逻辑处理器索引 | 输出掩码 |
|-------------------|---------------------------|-------------|
| `[]` | *(无)* | `0x0` |
| `[256]` | `[0]` | `0x1` |
| `[256, 258, 260]` | `[0, 2, 4]` | `0x15` |

*(实际的 CPU 集合 ID 值是特定于系统的；以上是示例。)*

## 说明

- 函数获取 `CPU_SET_INFORMATION` 互斥锁的锁，并遍历所有缓存的 [CpuSetData](CpuSetData.md) 条目。对于每个在其 `id` 字段中找到于输入切片中的条目，相应的位（`1 << logical_processor_index`）在结果掩码中被设置。
- 逻辑处理器索引 ≥ 64 被静默跳过，因为它们无法在 64 位系统上的单个 `usize` 位掩码中表示。这与 Windows 亲和性掩码每组处理器 64 个的限制一致。
- 函数执行 O(C × N) 查找，其中 C 是 CPU 集合条目数，N 是输入切片的长度，对每个缓存条目使用 `slice::contains`。对于典型消费者系统（≤ 64 CPU）的小输入大小，这是高效的。
- 输入中不与缓存拓扑中任何条目匹配的 CPU 集合 ID 被静默忽略 — 未识别的 ID 不会设置任何位。
- 此函数当前标记为 `#[allow(dead_code)]`，但可供任何需要 CPU 集合 ID 转换回亲和性掩码表示的组件使用。

### 与其他转换函数的关系

| 函数 | 方向 |
|----------|-----------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | CPU 索引 → CPU 集合 ID |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 亲和性掩码 → CPU 集合 ID |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU 集合 ID → CPU 索引 |
| **mask_from_cpusetids** | **CPU 集合 ID → 亲和性掩码** |
| [filter_indices_by_mask](filter_indices_by_mask.md) | CPU 索引 × 掩码 → 筛选后的索引 |

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | 可供一般使用；当前标记为 `#[allow(dead_code)]` |
| **被调用方** | [get_cpu_set_information](../winapi.rs/README.md)（获取缓存的 CPU 集合拓扑） |
| **Win32 API** | 无直接调用；依赖来自 [GetSystemCpuSetInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) 的缓存数据 |
| **特权** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 反向：掩码到 CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| 索引到 CPU 集合 ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| CPU 集合 ID 到索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合拓扑缓存 | [CpuSetData](CpuSetData.md) |
| 亲和性检查工具 | [is_affinity_unset](is_affinity_unset.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*