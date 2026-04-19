# collect_members 函数 (config.rs)

从文本片段中解析冒号分隔的进程名列表，并将其追加到累加器向量中。内部用于从单行和多行组块定义中提取组成员名称。

## 语法

```rust
fn collect_members(text: &str, members: &mut Vec<String>)
```

## 参数

`text: &str`

包含冒号分隔进程名的文本片段。通常是组定义中 `{` 和 `}` 之间的内容，或者是多行组块内的单个行。会修剪每个项周围的空格。以 `#` 开头的项被视为注释并被跳过。

`members: &mut Vec<String>`

**\[in, out\]** 用于追加已解析进程名的累加器向量。保留现有条目；新名称被推送到末尾。所有名称都规范化为小写。

## 返回值

此函数不返回值。结果通过 `members` 输出参数传达。

## 备注

### 解析规则

1. 输入 `text` 按 `:`（冒号）字符分割。
2. 每个生成的片段会修剪首尾空格。
3. 片段转换为小写，以便下游进行不区分大小写的匹配。
4. 空片段和以 `#` 开头的片段会被丢弃。
5. 所有保留的片段被推送到 `members`。

### 分隔符选择

冒号 `:` 作为分隔符被使用，因为主规则语法已经将 `:` 用作字段分隔符。在组块内（`{` 和 `}` 之间），进程名之间用冒号分隔——而不是逗号或分号——以保持与外部配置行格式的一致性。

### 无去重

`collect_members` **不**检查重复名称。如果同一进程名在输入中出现多次，或在多次调用中出现，它将出现在 `members` 中多次。如果需要去重，由调用方或后续阶段如 [sort_and_group_config](sort_and_group_config.md) 处理。

### 示例

给定输入文本 `"game.exe: helper.exe: # comment: tool.EXE"`，该函数将 `["game.exe", "helper.exe", "tool.exe"]` 追加到 `members`。注释片段被跳过，`tool.EXE` 被转换为小写。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有 (`fn`) —— 配置模块内部 |
| **调用方** | [read_config](read_config.md)（内联组解析），[collect_group_block](collect_group_block.md)（多行组解析），[sort_and_group_config](sort_and_group_config.md)（自动分组工具） |
| **被调用方** | 无（仅使用 `str` 标准库方法） |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 多行组块收集器 | [collect_group_block](collect_group_block.md) |
| 消费组成员的规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 主配置读取器 | [read_config](read_config.md) |
| 自动分组工具 | [sort_and_group_config](sort_and_group_config.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*