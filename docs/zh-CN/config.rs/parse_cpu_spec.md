# parse_cpu_spec 函数 (config.rs)

将 CPU 规格字符串解析为已排序的 CPU 索引列表。支持多种输入格式，包括十六进制位掩码、闭区间范围、单个 CPU 索引及其组合。

## 语法

```rust
pub fn parse_cpu_spec(s: &str) -> Vec<u32>
```

## 参数

`s`

包含待解析 CPU 规格的字符串切片。前后空白字符会被自动去除。支持以下格式：

| 格式 | 示例 | 结果 |
| --- | --- | --- |
| 空字符串或 `"0"` | `""`, `"0"` | `[]`（空向量） |
| 十六进制位掩码（传统格式，≤64 核） | `"0xFF"`, `"0x0F"` | `[0, 1, 2, 3, 4, 5, 6, 7]`, `[0, 1, 2, 3]` |
| 闭区间范围 | `"0-7"` | `[0, 1, 2, 3, 4, 5, 6, 7]` |
| 分号分隔的单个 CPU | `"0;4;8"` | `[0, 4, 8]` |
| 多段范围 | `"0-7;64-71"` | `[0, 1, 2, 3, 4, 5, 6, 7, 64, 65, 66, 67, 68, 69, 70, 71]` |

## 返回值

返回 `Vec<u32>`，包含从规格字符串解析出的已排序、已去重的 CPU 索引列表。当输入为空字符串、`"0"` 或包含无法解析的十六进制掩码时，返回空向量。

## 备注

这是整个配置系统中最基础的 CPU 规格解析器。所有其他接受 CPU 规格的函数最终都会委托给 `parse_cpu_spec`（直接调用或通过 [resolve_cpu_spec](resolve_cpu_spec.md) 间接调用）。

**十六进制位掩码格式**属于传统格式，仅支持最多 64 个核心。通过 `0x` 或 `0X` 前缀识别。掩码中每个置位的比特对应一个 CPU 索引。例如，`0x0F`（二进制 `00001111`）映射到 CPU `[0, 1, 2, 3]`。转换由内部函数 [mask_to_cpu_indices](mask_to_cpu_indices.md) 执行。

**范围和单个索引格式**使用分号（`;`）作为段之间的分隔符。每个段可以是单个 CPU 编号或使用短横线（`-`）表示的闭区间范围。跨段的重复 CPU 索引会被自动去除。最终结果始终按升序排列。

此函数**不会**解析 CPU 别名（以 `*` 为前缀的名称）。别名解析由 [resolve_cpu_spec](resolve_cpu_spec.md) 负责，该函数在内部对非别名规格调用 `parse_cpu_spec`。

### 示例

```text
"0-3"       → [0, 1, 2, 3]
"0;2;4"     → [0, 2, 4]
"0x0F"      → [0, 1, 2, 3]
"0-7;64-71" → [0, 1, 2, 3, 4, 5, 6, 7, 64, 65, 66, 67, 68, 69, 70, 71]
""          → []
"0"         → []
```

## 要求

| 条目 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **可见性** | `pub` |
| **源码行** | L70–L118 |
| **被调用方** | [resolve_cpu_spec](resolve_cpu_spec.md)、[parse_alias](parse_alias.md)、[parse_mask](parse_mask.md)、[convert](convert.md) |
| **调用** | [mask_to_cpu_indices](mask_to_cpu_indices.md) |

## 另请参阅

- [resolve_cpu_spec](resolve_cpu_spec.md) — 增加 CPU 别名解析的包装函数
- [mask_to_cpu_indices](mask_to_cpu_indices.md) — 将位掩码转换为 CPU 索引列表
- [cpu_indices_to_mask](cpu_indices_to_mask.md) — 反向操作（索引转位掩码）
- [format_cpu_indices](format_cpu_indices.md) — 将 CPU 索引格式化为紧凑字符串