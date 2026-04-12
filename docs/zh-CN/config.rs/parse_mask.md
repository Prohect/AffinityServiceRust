# parse_mask 函数 (config.rs)

解析 CPU 规格字符串并直接转换为位掩码表示。

## 语法

```rust
pub fn parse_mask(s: &str) -> usize
```

## 参数

`s`

CPU 规格字符串。支持 [parse_cpu_spec](parse_cpu_spec.md) 接受的所有格式，包括十六进制掩码、范围和分号分隔的单个 CPU 索引。

## 返回值

返回一个 `usize` 位掩码，其中每个置位的位对应解析出的规格中的一个 CPU 索引。例如，CPU `[0, 1, 2, 3]` 生成掩码 `0x0F`。仅索引小于 64 的 CPU 会被表示在掩码中。

## 备注

这是一个便捷封装函数，将 [parse_cpu_spec](parse_cpu_spec.md) 和 [cpu_indices_to_mask](cpu_indices_to_mask.md) 合并为单次调用。它先将输入字符串解析为 CPU 索引列表，然后将这些索引转换为位掩码。

由于返回类型为 `usize`，此函数仅能表示 CPU 0–63。在拥有超过 64 个逻辑处理器的系统上，编号更高的 CPU 将被静默排除在返回的掩码之外。对于超过 64 核的系统，建议直接通过 [parse_cpu_spec](parse_cpu_spec.md) 使用 CPU 索引向量。

此函数在源码中标记为 `#[allow(dead_code)]`，表明它可能保留用于将来使用或外部工具调用。

### 示例

| 输入 | 解析后的 CPU | 返回的掩码 |
| --- | --- | --- |
| `"0-3"` | `[0, 1, 2, 3]` | `0x0F` |
| `"0;2;4"` | `[0, 2, 4]` | `0x15` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0x00` |

## 要求

| 条目 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **定义位置** | 第 860 行 |
| **可见性** | `pub` |
| **调用** | [parse_cpu_spec](parse_cpu_spec.md)、[cpu_indices_to_mask](cpu_indices_to_mask.md) |

## 另请参阅

- [parse_cpu_spec](parse_cpu_spec.md) — 将 CPU 规格字符串解析为索引向量
- [cpu_indices_to_mask](cpu_indices_to_mask.md) — 将 CPU 索引切片转换为位掩码
- [mask_to_cpu_indices](mask_to_cpu_indices.md) — 逆向操作：位掩码转 CPU 索引