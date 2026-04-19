# parse_mask 函数 (config.rs)

便捷的函数，用于解析 CPU 规格字符串并返回相应的位掩码。将 [parse_cpu_spec](parse_cpu_spec.md) 和 [cpu_indices_to_mask](cpu_indices_to_mask.md) 合并为单个调用。

## 语法

```rust
pub fn parse_mask(s: &str) -> usize
```

## 参数

`s: &str`

采用 [parse_cpu_spec](parse_cpu_spec.md) 接受的任何格式的 CPU 规格字符串：范围（`"0-7"`）、分号分隔的索引（`"0;4;8"`）、十六进制掩码（`"0xFF"`）或哨兵值 `"0"`（无 CPU）。

## 返回值

`usize` — 如果 CPU 索引 *N* 在解析的规格中存在，则位 *N* 被设置。当输入为空、`"0"` 或无法解析时返回 `0`。

## 备注

此函数是两步管道：

1. 输入字符串 `s` 被传递给 [parse_cpu_spec](parse_cpu_spec.md)，生成排序的 `List<[u32; CONSUMER_CPUS]>` CPU 索引列表。
2. 生成的索引列表被传递给 [cpu_indices_to_mask](cpu_indices_to_mask.md)，生成 `usize` 位掩码。

### 转换示例

| 输入 | 中间索引 | 输出掩码 |
|------|----------|---------|
| `"0-3"` | `[0, 1, 2, 3]` | `0xF` |
| `"0;2;4"` | `[0, 2, 4]` | `0x15` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0x0` |

### 64 核限制

由于 [cpu_indices_to_mask](cpu_indices_to_mask.md) 静默丢弃索引 ≥ 64，因此解析规格中任何超出处理器 63 的 CPU 索引都不会在返回的位掩码中表示。对于拥有超过 64 个逻辑处理器的系统，建议使用 [parse_cpu_spec](parse_cpu_spec.md) 直接处理的 CPU 索引列表。

### 死代码允许

该函数使用 `#[allow(dead_code)]` 注解，表明它在所有构建配置中可能没有活跃调用方，但作为公共实用工具被保留。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 公共 (`pub fn`) |
| **调用方** | 实用工具/外部消费者 |
| **被调用方** | [parse_cpu_spec](parse_cpu_spec.md), [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|------|------|
| 模块概述 | [config.rs](README.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 索引到位掩码转换 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 位掩码到索引转换 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| 显示格式化 | [format_cpu_indices](format_cpu_indices.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*