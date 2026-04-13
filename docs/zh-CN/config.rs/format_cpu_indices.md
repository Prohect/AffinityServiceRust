# format_cpu_indices 函数 (config.rs)

将一组 CPU 索引切片格式化为紧凑、易读的范围字符串。连续的索引会折叠为 `start-end` 范围，非连续的索引以逗号分隔。此函数在整个服务中用于日志消息、配置文件生成和诊断输出。

## 语法

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpus` | `&[u32]` | 要格式化的 CPU 索引切片。无需排序或去重——函数内部会创建一个排序后的副本。可以为空。 |

## 返回值

返回一个包含格式化 CPU 索引表示的 `String`。

| 输入 | 输出 |
|------|------|
| `&[]` | `"0"` |
| `&[0, 1, 2, 3]` | `"0-3"` |
| `&[0, 2, 4]` | `"0,2,4"` |
| `&[0, 1, 2, 5, 6, 7, 12]` | `"0-2,5-7,12"` |
| `&[3]` | `"3"` |

空切片将产生字符串 `"0"`，在配置文件格式中表示"无 CPU"/"无更改"。

## 备注

### 算法

1. 如果输入切片为空，立即返回 `"0"`。
2. 创建输入的排序副本。
3. 遍历排序后的列表，在索引连续时（`sorted[i+1] == sorted[i] + 1`）扩展当前范围。
4. 当发现间隔时，输出累积的范围：
   - 单个索引 → `"N"`
   - 两个或更多连续索引 → `"start-end"`
5. 范围之间以逗号分隔（无空格）。

### 与 parse_cpu_spec 的反向关系

`format_cpu_indices` 是 [parse_cpu_spec](parse_cpu_spec.md) 的显示对应函数。往返转换 `format_cpu_indices(parse_cpu_spec(s))` 会生成原始规格字符串的规范化表示，去除重复并排序索引。请注意，十六进制位掩码输入（例如 `"0xFF"`）会被规范化为范围表示法（例如 `"0-7"`）。

### 空值等于零的约定

配置文件格式使用 `"0"` 表示"无 CPU 规格"（即不更改 CPU 亲和性/CPU 集）。为空切片返回 `"0"` 保留了此约定，使生成的配置文件在重新解析时语法有效且语义正确。

### 在生成文件中的输出

此函数被 [convert](convert.md) 和 [sort_and_group_config](sort_and_group_config.md) 在向输出配置文件写入 CPU 规格时调用。紧凑的范围表示法使生成的文件保持简洁，尤其对于多核心系统（例如 `"0-63,128-191"` 而非列出 128 个单独索引）。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用方** | [convert](convert.md)、[sort_and_group_config](sort_and_group_config.md)、apply 模块日志格式化 |
| **依赖项** | 无（纯函数） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CPU 规格字符串解析器（反向操作） | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU 索引转位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 位掩码转 CPU 索引 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| config 模块概览 | [README](README.md) |