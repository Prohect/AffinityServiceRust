# collect_group_block 函数 (config.rs)

从多行组块中收集成员，直到找到闭合花括号。支持单行 `{ a, b }` 和跨多行的组定义格式。

## 语法

```rust
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## 参数

`lines`

正在解析的配置文件的所有行切片。

`start_index`

开始扫描的行索引（即开花括号 `{` 之后的下一行）。

`first_line_content`

与开花括号出现在同一行的、`{` 字符之后的任何内容。在扫描后续行之前，会先从该内容中解析成员名称。

## 返回值

返回 `Option<(Vec<String>, Option<String>, usize)>`：

- **`Some((members, rule_suffix, next_index))`** — 成功找到闭合花括号 `}`。
  - `members` — 花括号内收集到的进程名称 `Vec<String>`。
  - `rule_suffix` — `Option<String>`，包含 `}:` 之后的规则定义部分（冒号后面的内容）。如果闭合花括号后没有冒号，则为 `None`。
  - `next_index` — 恢复解析的行索引（即包含 `}` 的那一行之后的行）。

- **`None`** — 未找到闭合花括号 `}`，表示组块未闭合。调用方应发出错误。

## 备注

当 [read_config](read_config.md) 遇到包含 `{` 但同一行没有匹配 `}` 的行时，会调用此函数。函数向后扫描后续行，通过 [collect_members](collect_members.md) 收集以冒号分隔的进程名称。

空行或以 `#`（注释）开头的行在收集成员时会被跳过。

找到闭合 `}` 时，该行中 `}` 之前的内容也会被收集为成员。`}` 之后的文本会检查是否以 `:` 开头——如果是，则将其余部分作为规则后缀返回，该后缀会传递给 [parse_and_insert_rules](parse_and_insert_rules.md)。

### 示例

配置中的多行组块：

```
browsers {
    chrome.exe
    firefox.exe
    msedge.exe
}:high:*p:0:0:none:none
```

当 `read_config` 遇到 `browsers {` 行时，会使用以下参数调用 `collect_group_block`：
- `lines` — 所有配置行
- `start_index` — `chrome.exe` 行的索引
- `first_line_content` — 空字符串（第一行 `{` 后没有内容）

函数返回：
- `members`: `["chrome.exe", "firefox.exe", "msedge.exe"]`
- `rule_suffix`: `Some("high:*p:0:0:none:none")`
- `next_index`: `}:high:*p:0:0:none:none` 行之后的行索引

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **行号** | L381–L411 |
| **可见性** | 私有 (`fn`) |
| **调用方** | [read_config](read_config.md) |
| **调用** | [collect_members](collect_members.md) |