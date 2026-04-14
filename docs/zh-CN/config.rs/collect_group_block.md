# collect_group_block 函数 (config.rs)

从多行组块中收集进程成员名称，从起始行索引向前扫描，直到找到闭合的 `}` 大括号。返回累积的成员名称、闭合大括号后的规则后缀以及恢复解析的行索引。如果块从未闭合（即到达文件末尾仍未遇到 `}`），则返回 `None`。

## 语法

```rust
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `lines` | `&[String]` | 配置文件的完整行向量，由 [read_config](read_config.md) 收集。函数从 `start_index` 开始向前读取此向量。 |
| `start_index` | `usize` | 开始扫描组成员的基于 0 的行索引。通常是包含开启 `{` 的行之后的下一行。 |
| `first_line_content` | `&str` | 与开启 `{` 出现在同一行上、大括号之后的文本。例如，如果配置行为 `group_name { game.exe: helper.exe`，则 `first_line_content` 为 `"game.exe: helper.exe"`。在函数开始扫描后续行之前，会先解析此内容中的成员。如果为空或以 `#` 开头，则被忽略。 |

## 返回值

返回 `Option<(Vec<String>, Option<String>, usize)>`：

| 变体 | 含义 |
|---------|---------|
| `Some((members, rule_suffix, next_index))` | 已找到闭合 `}`。`members` 是收集到的小写进程名称列表。`rule_suffix` 在大括号后找到 `}:rule_text` 形式的文本时为 `Some(suffix_string)`，否则为 `None`。`next_index` 是闭合大括号行之后的行索引（即 `i + 1`），调用方应使用此值恢复其主解析循环。 |
| `None` | 到达文件末尾仍未找到闭合 `}`。组块未闭合，调用方应记录错误。 |

## 备注

### 算法

1. **首行内容** — 如果 `first_line_content` 非空且不以 `#` 开头，则将其传递给 [collect_members](collect_members.md) 以从开启大括号行的剩余部分提取成员名称。
2. **行扫描** — 从 `start_index` 开始，对每一行进行修剪和检查：
   - 如果行包含 `}`，则通过 [collect_members](collect_members.md) 解析 `}` 之前的文本获取成员，检查 `}` 之后的文本是否有 `:` 前缀以提取规则后缀，然后返回 `Some(...)`。
   - 如果行非空且不以 `#` 开头，则将其传递给 [collect_members](collect_members.md) 以累积更多成员名称。
   - 跳过空行和注释行（以 `#` 开头）。
3. **未闭合块** — 如果循环遍历完 `lines` 仍未找到 `}`，函数返回 `None`。

### 规则后缀提取

在闭合 `}` 之后，同一行的剩余文本会被修剪。如果以 `:` 开头，则去除冒号后的其余部分成为规则后缀字符串。此后缀随后由调用方（[read_config](read_config.md)）按 `:` 分割并传递给 [parse_and_insert_rules](parse_and_insert_rules.md)。

如果 `}` 之后的文本不以 `:` 开头（或为空），则 `rule_suffix` 为 `None`，这会导致调用方报告该组的"缺少规则"错误。

### 示例：多行组

给定以下配置文件行（基于 0 的索引）：

```text
L0: my_group {
L1:     game.exe: helper.exe
L2:     launcher.exe
L3: }:high:0-7
```

调用方（[read_config](read_config.md)）在第 0 行检测到 `{` 并调用：

```text
collect_group_block(lines, 1, "")
```

函数执行过程：

1. 跳过 `first_line_content`（为空）。
2. 处理第 1 行：收集 `["game.exe", "helper.exe"]`。
3. 处理第 2 行：收集 `["launcher.exe"]`。
4. 处理第 3 行：找到 `}`，提取后缀 `"high:0-7"`。
5. 返回 `Some((["game.exe", "helper.exe", "launcher.exe"], Some("high:0-7"), 4))`。

### 示例：大括号后同行内容

```text
L0: my_group { game.exe: helper.exe
L1: launcher.exe
L2: }:high:0-7
```

此处调用方传入 `first_line_content = "game.exe: helper.exe"` 和 `start_index = 1`：

1. 解析 `first_line_content` → `["game.exe", "helper.exe"]`。
2. 处理第 1 行 → `["launcher.exe"]`。
3. 处理第 2 行 → 找到 `}`，后缀 = `"high:0-7"`。
4. 返回 `Some((["game.exe", "helper.exe", "launcher.exe"], Some("high:0-7"), 3))`。

### 块内注释处理

组块内以 `#` 开头的行会被完全跳过。成员行中的行内注释由 [collect_members](collect_members.md) 处理，它会在按 `:` 分割后跳过以 `#` 开头的段。

### 可见性

此函数具有 **crate 私有** 可见性（`fn`，而非 `pub fn`）。仅由 `config` 模块内的 [read_config](read_config.md) 和 [sort_and_group_config](sort_and_group_config.md) 调用。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | Crate 私有 |
| **调用方** | [read_config](read_config.md)、[sort_and_group_config](sort_and_group_config.md) |
| **被调用方** | [collect_members](collect_members.md) |
| **API** | 纯函数 — 无 I/O，无 Windows API 调用 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 成员名称分词器 | [collect_members](collect_members.md) |
| 主配置文件读取器（调用方） | [read_config](read_config.md) |
| 规则字段解析（使用成员列表） | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 自动分组工具（也使用组块） | [sort_and_group_config](sort_and_group_config.md) |
| 配置模块概览 | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd