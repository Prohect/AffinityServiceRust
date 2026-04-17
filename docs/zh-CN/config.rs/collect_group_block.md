# collect_group_block 函数 (config.rs)

从配置文件中的多行 `{ … }` 分组块中收集进程名称成员，从起始行索引开始向前扫描，直到找到右花括号 `}`。返回收集到的成员名称、右花括号之后出现的任何规则后缀，以及继续解析的下一行索引。

## 语法

```AffinityServiceRust/src/config.rs#L390-418
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `lines` | `&[String]` | 从配置文件读取的完整行列表。函数从 `start_index` 开始向前读取，寻找右花括号 `}`。 |
| `start_index` | `usize` | 开始扫描分组成员和右花括号的 `lines` 中从 0 开始的索引。这通常是包含左花括号 `{` 的行之后的下一行。 |
| `first_line_content` | `&str` | 与左花括号 `{` 出现在同一行上、位于花括号之后的任何内容。如果非空且不是注释，则在扫描后续行之前先从中解析成员名称。 |

## 返回值

类型：`Option<(Vec<String>, Option<String>, usize)>`

当找到右花括号 `}` 时返回 `Some((members, rule_suffix, next_line_index))`，如果到达文件末尾仍未遇到右花括号（未闭合的分组）则返回 `None`。

| 元组元素 | 类型 | 描述 |
|----------|------|------|
| `members` | `Vec<String>` | 从花括号内收集的小写进程名称，通过 [`collect_members`](collect_members.md) 提取。 |
| `rule_suffix` | `Option<String>` | 右花括号行上 `}:` 之后的规则字符串（即 `}` 之后第一个 `:` 后面的所有内容）。如果右花括号之后没有 `:`，则为 `None`。 |
| `next_line_index` | `usize` | 包含右花括号的行之后的行索引；调用方应从此索引继续解析。 |

## 备注

### 扫描算法

1. 如果 `first_line_content` 非空且不以 `#` 开头，则将其传递给 [`collect_members`](collect_members.md) 以提取与左花括号在同一行上的所有进程名称。
2. 然后函数从 `start_index` 开始遍历 `lines`：
   - 如果行中包含 `}`，则收集花括号之前的所有内容作为成员（如果非空且不是注释），提取 `}` 之后的规则后缀，然后返回。
   - 否则，将非空、非注释行传递给 [`collect_members`](collect_members.md) 以累积更多成员名称。
3. 如果循环到达 `lines` 末尾仍未找到 `}`，则函数返回 `None`，表示分组块未闭合。

### 规则后缀提取

在右花括号行上，`}` 之后的文本被修剪并检查是否有前导 `:`。如果找到，冒号被去除，其余部分作为 `Some(rule_string)` 返回。如果没有冒号（例如 `}` 之后仅有空白或无内容），则后缀返回 `None`，这在 [`read_config`](read_config.md) 的调用方中被视为错误。

### 注释处理

修剪后以 `#` 开头的行被视为注释并完全跳过。此外，在行内，由 [`collect_members`](collect_members.md) 产生的以 `#` 开头的令牌也会被过滤掉——这提供了行内注释支持。

### 单行分组

左花括号 `{` 和右花括号 `}` 都出现在同一行上的单行分组（例如 `{ a.exe: b.exe }:normal:0-7`）**不**由此函数处理。它们在 [`read_config`](read_config.md) 中被检测并在调用 `collect_group_block` 之前内联解析。此函数仅在左花括号 `{` 所在行不同时包含右花括号 `}` 时才被调用。

### 边界情况

| 场景 | 行为 |
|------|------|
| 左花括号后紧跟下一行的 `}` | 仅返回从 `first_line_content` 收集的成员。 |
| 空分组块（`{ }` 跨多行，其中只有注释/空行） | 返回空的 `members` 向量；调用方发出警告。 |
| 嵌套花括号 | 不支持。块内的 `}` 会立即终止收集，不考虑上下文。 |
| 到达文件末尾仍未遇到 `}` | 返回 `None`。调用方推送一条关于未闭合分组的错误。 |

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（`fn`，非 `pub fn`） |
| 调用方 | [`read_config`](read_config.md)、[`sort_and_group_config`](sort_and_group_config.md) |
| 被调用方 | [`collect_members`](collect_members.md)、`str::find`、`str::trim`、`str::starts_with`、`str::strip_prefix` |
| API | 仅标准库 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| collect_members | [collect_members](collect_members.md) |
| read_config | [read_config](read_config.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| sort_and_group_config | [sort_and_group_config](sort_and_group_config.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*