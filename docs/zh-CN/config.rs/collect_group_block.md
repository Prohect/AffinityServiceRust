# collect_group_block 函数 (config.rs)

从多行组块定义中收集进程名称，读取行直到遇到闭合的 `}` 大括号。返回累积的成员列表、闭合大括号后的规则后缀，以及要从哪里恢复解析的行索引。

## 语法

```rust
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## 参数

`lines: &[String]`

从配置文件中读取的所有行的完整集合。函数从 `start_index` 开始向后读取，寻找闭合的 `}` 大括号。

`start_index: usize`

从零开始的行索引，用于开始扫描。这应该是包含打开 `{` 大括号的行之后的下一行。函数读取 `lines[start_index]`、`lines[start_index + 1]`，依此类推，直到找到 `}` 或到达文件末尾。

`first_line_content: &str`

出现在打开 `{` 同一行上的任何内容。例如，给定 `group_name { proc1.exe: proc2.exe`，此参数将是 `"proc1.exe: proc2.exe"`。如果打开的大括号是其行上最后一个非空白字符，这将是空字符串或空白。此内容在读取后续行之前用于解析成员名称。

## 返回值

`Option<(Vec<String>, Option<String>, usize)>` — 成功时返回 `Some(...)`，如果到达文件末尾而未找到闭合的 `}` 则返回 `None`。

元组元素为：

| 索引 | 类型 | 描述 |
|-------|------|-------------|
| `.0` | `Vec<String>` | 收集的组成员名称，已小写并修剪。从 `first_line_content` 和所有后续行累积，包括包含 `}` 的行。 |
| `.1` | `Option<String>` | 闭合 `}` 之后的规则后缀。如果包含 `}` 的行在 `}` 后有 `:rule_fields...`，则 `:` 之后的文本作为 `Some(rule_fields)` 返回。如果 `}` 后没有 `:`，返回 `None`。 |
| `.2` | `usize` | 包含 `}` 的行之后的行索引。调用方应从该索引恢复解析。 |

## 备注

### 解析算法

1. 如果 `first_line_content` 非空且不以 `#` 开头，则将其传递给 [collect_members](collect_members.md) 以提取同一行上打开 `{` 之后出现的任何进程名称。
2. 然后函数从 `start_index` 开始迭代 `lines`：
   - 如果一行包含 `}`，则解析 `}` 之前的内容以获取成员，检查 `}` 之后的内容以获取规则后缀（前导 `:` 后的文本），然后函数成功返回。
   - 否则，整个行（如果非空且不是注释）通过 [collect_members](collect_members.md) 解析为成员。
3. 如果循环在没有找到 `}` 的情况下耗尽所有行，则函数返回 `None`，表示组块未关闭。

### 规则后缀提取

闭合 `}` 之后的立即文本确定组是否有关联的规则。解析器期望格式 `}:priority:affinity:...`。去除前导 `:`，剩余文本成为 `.1` 中返回的规则后缀。此后缀稍后由 [parse_and_insert_rules](parse_and_insert_rules.md) 在 `:` 上分割以创建实际配置条目。

如果 `}` 后没有 `:`（例如，闭合大括号单独在一行上），则 [read_config](read_config.md) 中的调用方将其视为错误——没有规则定义的组。

### 注释处理

组块内以 `#` 开头的行被静默忽略，不贡献成员。这允许用户在组内注释掉单个进程名称：

```
my_group {
    active_game.exe
    # disabled_game.exe
    another_game.exe
}:normal:*ecore:0:0:none:none:0:1
```

### 单行与多行

此函数仅用于多行组——打开 `{` 行不包含闭合 `}` 的情况。单行组（例如，`group { a: b }:rule`）由 [read_config](read_config.md) 内联处理，无需调用此函数。

### 错误情况

当函数返回 `None` 时，[read_config](read_config.md) 中的调用方将错误推入 [ConfigResult](ConfigResult.md)，格式为 `"Line {N}: Unclosed group '{name}' - missing }"`，然后跳过下一行。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有 (`fn`) — config 模块内部 |
| **调用方** | [read_config](read_config.md)（多行组解析），[sort_and_group_config](sort_and_group_config.md)（自动分组读取器） |
| **被调用方** | [collect_members](collect_members.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 成员名称解析器 | [collect_members](collect_members.md) |
| 收集成员的规则插入 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 主配置读取器 | [read_config](read_config.md) |
| 带错误报告的配置结果 | [ConfigResult](ConfigResult.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*