# format_cpu_indices 函数 (config.rs)

将 CPU 索引切片格式化为紧凑的、人类可读的字符串表示，尽可能使用范围表示法。

## 语法

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## 参数

`cpus`

待格式化的 CPU 索引切片（`&[u32]`）。输入无需预排序，函数内部会在格式化前自动排序。

## 返回值

返回一个 `String`，包含格式化后的 CPU 规格字符串。连续的索引会被折叠为使用短横线表示的范围（`start-end`），非连续的索引或范围之间以逗号分隔。若输入切片为空，则返回 `"0"`。

## 备注

此函数是 [parse_cpu_spec](parse_cpu_spec.md) 的显示端对应函数。`parse_cpu_spec` 将字符串规格转换为 CPU 索引向量，而 `format_cpu_indices` 则将索引向量转换回紧凑字符串。

格式化算法如下：

1. 若输入切片为空，返回字符串 `"0"`。
2. 将索引的副本按升序排序。
3. 遍历排序后的列表，检测连续值的区间。
4. 对每个区间，若跨越多个值则输出 `start-end`，否则仅输出单个值。
5. 各段之间以逗号分隔。

### 示例

| 输入 | 输出 |
| --- | --- |
| `[]` | `"0"` |
| `[0, 1, 2, 3]` | `"0-3"` |
| `[0, 2, 4]` | `"0,2,4"` |
| `[0, 1, 2, 5, 6, 7]` | `"0-2,5-7"` |
| `[3, 1, 2, 0]` | `"0-3"` |
| `[0, 1, 2, 4, 8, 9, 10]` | `"0-2,4,8-10"` |

请注意，输出使用逗号作为分隔符，而 [parse_cpu_spec](parse_cpu_spec.md) 接受分号作为分隔符。这是刻意设计的——逗号分隔格式用于显示和配置输出，分号分隔格式用于配置输入解析。

此函数被日志和报告子系统用于以可读方式显示 CPU 分配情况，也被 [sort_and_group_config](sort_and_group_config.md) 在生成分组配置输出时使用。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **行号** | L134–L164 |
| **可见性** | `pub` |
| **相关函数** | [parse_cpu_spec](parse_cpu_spec.md)、[cpu_indices_to_mask](cpu_indices_to_mask.md)、[mask_to_cpu_indices](mask_to_cpu_indices.md) |