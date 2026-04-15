# collect_members 函数 (config.rs)

将以冒号分隔的进程名称字符串拆分为单独的小写成员条目，并将它们追加到提供的列表中。这是配置解析器内部使用的辅助函数，用于从内联规则行和 `{ }` 分组块中提取进程名称。

## 语法

```AffinityServiceRust/src/config.rs#L242-249
fn collect_members(text: &str, members: &mut Vec<String>) {
    for item in text.split(':') {
        let item = item.trim().to_lowercase();
        if !item.is_empty() && !item.starts_with('#') {
            members.push(item);
        }
    }
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `text` | `&str` | 包含一个或多个以冒号（`:`）分隔的进程名称的字符串。也可能包含内联注释（以 `#` 开头的标记）和空白字符，两者都会被过滤掉。 |
| `members` | `&mut Vec<String>` | 可变引用，指向收集到的成员名称将被追加到的向量。调用方负责初始化此向量；`collect_members` 只追加，不会清空它。 |

## 返回值

此函数不返回值。结果通过调用方传入的 `members` 向量累积。

## 备注

### 处理步骤

1. 输入 `text` 按 `:` 分隔符拆分。
2. 每个生成的标记去除前后空白并转换为小写。
3. 去除空白后为空或以 `#` 开头（内联注释）的标记将被丢弃。
4. 所有剩余的标记被推入 `members` 向量。

### 去重

`collect_members` **不**执行去重。如果同一进程名称在 `text` 中多次出现，它将被多次添加到 `members` 中。去重是下游消费者的职责（例如，[`parse_and_insert_rules`](parse_and_insert_rules.md) 会检测并警告重复规则）。

### 大小写规范化

所有成员名称在插入前都被转为小写。这确保了运行时的不区分大小写匹配，因为 Windows 进程名称是不区分大小写的。

### 使用上下文

此函数在以下位置被调用：

- **[`collect_group_block`](collect_group_block.md)**：从多行 `{ }` 分组块中的每一行以及右花括号之前的内容中收集成员。
- **[`read_config`](read_config.md)**：从单行分组块（左花括号和右花括号出现在同一行）中收集成员。
- **[`sort_and_group_config`](sort_and_group_config.md)**：在自动分组遍历期间重新解析分组块时收集成员。

### 边界情况

| 输入 | 结果 |
|------|------|
| `""` （空字符串） | 不追加任何内容 |
| `"  "` （仅空白） | 不追加任何内容 |
| `"# comment"` | 不追加任何内容（注释被过滤） |
| `"game.exe"` | 追加 `["game.exe"]` |
| `"Game.EXE : app.exe"` | 追加 `["game.exe", "app.exe"]` |
| `"a.exe: : b.exe"` | 追加 `["a.exe", "b.exe"]`（空标记被跳过） |

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（`fn`，非 `pub fn`） |
| 调用方 | [`collect_group_block`](collect_group_block.md)、[`read_config`](read_config.md)、[`sort_and_group_config`](sort_and_group_config.md) |
| 被调用方 | `str::split`、`str::trim`、`str::to_lowercase`、`str::starts_with`、`Vec::push` |
| API | 仅标准库 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| collect_group_block | [collect_group_block](collect_group_block.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| read_config | [read_config](read_config.md) |
| sort_and_group_config | [sort_and_group_config](sort_and_group_config.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*