# parse_mask 函数 (config.rs)

便捷函数，解析 CPU 规格字符串并返回对应的 `usize` 位掩码。这是 [`parse_cpu_spec`](parse_cpu_spec.md) 和 [`cpu_indices_to_mask`](cpu_indices_to_mask.md) 的简写组合。

## 语法

```AffinityServiceRust/src/config.rs#L895-898
#[allow(dead_code)]
pub fn parse_mask(s: &str) -> usize {
    let cpus = parse_cpu_spec(s);
    cpu_indices_to_mask(&cpus)
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `s` | `&str` | 任何 [`parse_cpu_spec`](parse_cpu_spec.md) 接受的格式的 CPU 规格字符串：范围（`"0-7"`）、单独索引（`"0;4;8"`）、十六进制位掩码（`"0xFF"`）、单个 CPU 索引（`"7"`）或特殊值 `"0"`（无 CPU）。 |

## 返回值

类型：`usize`

一个位掩码，其中每个被设置的位代表来自已解析规格的一个逻辑处理器索引。位 0 对应 CPU 0，位 1 对应 CPU 1，以此类推。当输入为空、`"0"` 或无法解析时返回 `0`。

### 示例

| 输入 | 中间 CPU 索引 | 输出（十六进制） |
|------|---------------|-----------------|
| `"0-3"` | `[0, 1, 2, 3]` | `0x0F` |
| `"0;2;4"` | `[0, 2, 4]` | `0x15` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0x00` |
| `""` | `[]` | `0x00` |

## 备注

- 此函数在源代码中标注了 `#[allow(dead_code)]`，表示它可能在当前代码库中未被主动调用，但作为测试、调试或未来使用的实用工具被保留。

- **64 核限制**：由于返回类型为 `usize` 且 [`cpu_indices_to_mask`](cpu_indices_to_mask.md) 仅对小于 64 的 CPU 索引设置位，因此已解析规格中任何 ≥ 64 的 CPU 索引将从结果位掩码中被静默丢弃。对于拥有超过 64 个逻辑处理器的系统，请直接使用 [`parse_cpu_spec`](parse_cpu_spec.md) 来处理完整的索引列表。

- 该函数为 `pub`，使其可供 crate 中的其他模块使用，尽管内部使用可能有限。

- 往返保真度：对于仅产生 0–63 范围内索引的输入，位掩码忠实地表示已解析的规格。但是，`parse_mask` 对于超过 64 核的规格是有损操作，且不保留原始字符串格式（范围 vs. 单独索引 vs. 十六进制表示法）。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | 当前未使用（`#[allow(dead_code)]`）；作为实用工具可用 |
| 被调用方 | [`parse_cpu_spec`](parse_cpu_spec.md)、[`cpu_indices_to_mask`](cpu_indices_to_mask.md) |
| API | 仅标准库 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*