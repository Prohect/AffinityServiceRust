# format_cpu_indices 函数 (config.rs)

将 CPU 索引切片格式化为紧凑的、人类可读的字符串表示形式，在连续索引允许的情况下使用范围表示法。这是 [`parse_cpu_spec`](parse_cpu_spec.md) 的逆向显示操作，在整个应用程序的日志记录和诊断中使用。

## 语法

```AffinityServiceRust/src/config.rs#L129-159
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpus` | `&[u32]` | 要格式化的 CPU 索引值切片。切片不需要排序或去重——函数在格式化前会在内部创建排序后的副本。 |

## 返回值

类型：`String`

CPU 索引的紧凑字符串表示。连续索引使用短横线表示法折叠为范围，非连续值用逗号分隔。

| 输入 | 输出 |
|------|------|
| `[]`（空） | `"0"` |
| `[0, 1, 2, 3]` | `"0-3"` |
| `[0, 2, 4]` | `"0,2,4"` |
| `[0, 1, 2, 5, 6, 7]` | `"0-2,5-7"` |
| `[3]` | `"3"` |
| `[0, 1, 2, 8]` | `"0-2,8"` |

## 备注

### 排序

函数将输入复制到本地 `List<[u32; CONSUMER_CPUS]>` 中，对其排序，然后遍历排序后的列表以检测连续序列。原始输入切片不会被修改。

### 空输入约定

空切片返回字符串 `"0"`，这是配置文件中"不更改"或"未设置"的约定。这与 [`parse_cpu_spec`](parse_cpu_spec.md) 的行为一致，后者将空字符串和字面量 `"0"` 都视为空 CPU 列表。

### 算法

格式化算法对排序后的列表执行一次线性扫描：

1. 将 `start` 和 `end` 初始化为第一个元素。
2. 当下一个元素等于 `end + 1` 时，通过推进 `end` 来扩展当前范围。
3. 当检测到间隔（或列表已耗尽）时，输出累积的范围：
   - 如果 `start == end`，输出单个数字（例如 `"3"`）。
   - 如果 `start < end`，输出范围（例如 `"0-3"`）。
4. 输出的范围之间用逗号分隔。

### 与 parse_cpu_spec 的关系

`format_cpu_indices` 和 [`parse_cpu_spec`](parse_cpu_spec.md) 对于范围子集的 CPU 规格形成了往返转换对。给定排序且去重的输入，`parse_cpu_spec(format_cpu_indices(cpus))` 会重现原始列表。但是，十六进制掩码规格（`0xFF`）不会被 `format_cpu_indices` 重现——始终使用范围表示。

### 使用场景

此函数在日志记录和诊断输出中被调用，以可读形式显示 CPU 集合——例如，打印规则适用于哪些 CPU 时，或通过 [`sort_and_group_config`](sort_and_group_config.md) 生成自动分组的配置文件时。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 调用方 | 日志工具、[`sort_and_group_config`](sort_and_group_config.md)、诊断输出 |
| 被调用方 | `List::sort` |
| 依赖 | [`collections.rs`](../collections.rs/README.md) 中的 `List`、`CONSUMER_CPUS` |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*