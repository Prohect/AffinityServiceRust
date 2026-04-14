# parse_mask 函数 (config.rs)

便捷包装函数，将 CPU 规格字符串直接解析为 `usize` 位掩码。等效于依次调用 [parse_cpu_spec](parse_cpu_spec.md) 和 [cpu_indices_to_mask](cpu_indices_to_mask.md)，合并为一步完成。

## 语法

```rust
pub fn parse_mask(s: &str) -> usize
```

## 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `s` | `&str` | 任何 [parse_cpu_spec](parse_cpu_spec.md) 所接受格式的 CPU 规格字符串：十六进制位掩码（`"0xFF"`）、闭区间范围（`"0-7"`）、分号分隔的索引（`"0;4;8"`）或混合格式（`"0-3;8;12-15"`）。字符串 `"0"` 和空字符串均产生掩码 `0`。 |

## 返回值

返回一个 `usize` 位掩码，其中第 *N* 位被设置表示解析后的规格中包含 CPU 索引 *N*。由于在 64 位平台上 `usize` 无法表示 ≥ 64 的 CPU 索引，这些索引会被静默丢弃。

在以下情况下返回 `0`：

- `s` 为空字符串、仅包含空白字符或字面量 `"0"`。
- `s` 为十六进制前缀但后续值无法解析。
- 所有解析得到的 CPU 索引均 ≥ 64。

## 备注

### 实现

该函数是对现有工具函数的两行组合：

```rust
pub fn parse_mask(s: &str) -> usize {
    let cpus = parse_cpu_spec(s);
    cpu_indices_to_mask(&cpus)
}
```

不执行额外的验证、错误报告或别名解析。如需支持别名解析，请使用 [resolve_cpu_spec](resolve_cpu_spec.md)。

### dead_code 标注

该函数在源代码中带有 `#[allow(dead_code)]` 属性，表明它可能并非在所有构建配置中都被调用。它作为公共工具函数提供，供外部使用者或未来使用。

### 64 核限制

由于输出是 `usize` 位掩码（在 x86-64 Windows 上为 64 位），此函数无法表示超过 64 个 CPU。对于拥有 64 个以上逻辑处理器的系统，调用者应直接使用 `Vec<u32>` 索引（通过 [parse_cpu_spec](parse_cpu_spec.md)）或使用 CPU 集合 API 路径（[ProcessConfig](ProcessConfig.md)`.cpu_set_cpus`）。

### 与 format_cpu_indices 的关系

[format_cpu_indices](format_cpu_indices.md) 将 CPU 索引切片转换为显示字符串，[parse_cpu_spec](parse_cpu_spec.md) 将字符串转换为索引列表，而 `parse_mask` 提供了从字符串直接转换为 Windows 兼容的 CPU 亲和性位掩码的快捷路径。

### 示例

| 输入 | 中间索引 | 输出掩码 |
|------|----------|----------|
| `"0-3"` | `[0, 1, 2, 3]` | `0x0F` |
| `"0;4;8"` | `[0, 4, 8]` | `0x111` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0` |
| `""` | `[]` | `0` |
| `"0-3;63"` | `[0, 1, 2, 3, 63]` | `0x800000000000000F` |

## 要求

| | |
|---|---|
| **模块** | `config`（`src/config.rs`） |
| **可见性** | `pub`（带 `#[allow(dead_code)]`） |
| **调用者** | 保留供外部使用者和未来使用 |
| **被调用函数** | [parse_cpu_spec](parse_cpu_spec.md)、[cpu_indices_to_mask](cpu_indices_to_mask.md) |
| **API** | 纯函数 ─ 无 I/O，无 Windows API 调用 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CPU 规格字符串解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU 索引转位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 位掩码转 CPU 索引（逆操作） | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| 紧凑范围显示格式化器 | [format_cpu_indices](format_cpu_indices.md) |
| 进程级配置（CPU 亲和性掩码用途） | [ProcessConfig](ProcessConfig.md) |
| 模块概述 | [config 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd