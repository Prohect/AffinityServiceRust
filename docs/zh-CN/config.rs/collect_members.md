# collect_members 函数 (config.rs)

将以冒号分隔的文本字符串拆分为各个成员名称，对每个条目进行空白裁剪和小写转换，然后将结果追加到现有向量中。此函数是内联规则行和多行组块用于从配置文本中提取进程名称的共享分词器。

## 语法

```rust
fn collect_members(text: &str, members: &mut Vec<String>)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `text` | `&str` | 以冒号分隔的成员名称字符串（例如 `"game.exe: helper.exe: launcher.exe"`）。每个片段在添加前会进行裁剪和小写转换。裁剪后为空的片段或以 `#`（注释）开头的片段将被跳过。 |
| `members` | `&mut Vec<String>` | 用于追加解析出的成员名称的输出向量。向量中已有的条目将被保留；新名称被推入末尾。如果需要去重，由调用方负责处理。 |

## 返回值

此函数没有返回值。结果通过可变引用传递的 `members` 向量进行累积。

## 备注

### 解析规则

1. 输入 `text` 按 `:` 字符进行拆分。
2. 每个结果片段会进行前后空白裁剪，然后转换为小写。
3. 以下情况的片段将被**跳过**：
   - 裁剪后为空。
   - 以 `#` 开头（视为内联注释）。
4. 所有保留下来的片段作为 `String` 值被推入 `members`。

### 大小写规范化

所有成员名称通过 `to_lowercase()` 转换为小写。这确保了整个服务中的进程名称匹配不区分大小写，因为 Windows 进程名称本身就是不区分大小写的。

### 注释处理

`#` 检查允许在组块内使用内联注释。例如，在多行组块中：

```
my_group {
    game.exe: helper.exe
    # 这一行是注释，会被调用方跳过
    launcher.exe: updater.exe
}:high:0-7
```

单行内以 `#` 开头的片段也会被过滤。例如，`"game.exe: # not a process"` 只会产生 `["game.exe"]`。

### 不进行去重

`collect_members` 不检查重复名称。如果同一进程名称在多次调用中出现多次（例如在组块的不同行上），它将在 `members` 中出现多次。如果需要去重，则由下游处理——例如，[sort_and_group_config](sort_and_group_config.md) 在排序后调用 `dedup()` 来对成员列表去重。

### 可见性

此函数具有 **crate 私有**可见性（`fn`，而非 `pub fn`）。它仅在 `config` 模块内由 [read_config](read_config.md)（用于内联组解析）和 [collect_group_block](collect_group_block.md)（用于多行组块）调用。

### 使用场景

`collect_members` 通常在以下两种场景之一中被调用：

1. **内联组** — 当 [read_config](read_config.md) 遇到像 `{ a: b: c }:rule` 这样的单行组时，它提取 `{` 和 `}` 之间的文本并传递给 `collect_members`。
2. **多行组** — [collect_group_block](collect_group_block.md) 对 `{ ... }` 块内的每个非空、非注释行调用 `collect_members`，在各行之间累积所有成员。

### 示例

| 输入 `text` | 追加的结果条目 |
|-------------|---------------|
| `"game.exe: helper.exe"` | `["game.exe", "helper.exe"]` |
| `"  GAME.EXE : Helper.EXE "` | `["game.exe", "helper.exe"]` |
| `"single.exe"` | `["single.exe"]` |
| `""` | *（无）* |
| `"# comment"` | *（无）* |
| `"a.exe: # comment: b.exe"` | `["a.exe", "b.exe"]` |

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | Crate 私有 |
| **调用方** | [read_config](read_config.md)、[collect_group_block](collect_group_block.md)、[sort_and_group_config](sort_and_group_config.md) |
| **被调用方** | 无（仅使用标准库字符串操作） |
| **API** | 纯函数 — 无 I/O，无 Windows API 调用 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 多行组块收集器 | [collect_group_block](collect_group_block.md) |
| 主配置文件解析器 | [read_config](read_config.md) |
| 规则字段解析（消费成员列表） | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 自动分组工具 | [sort_and_group_config](sort_and_group_config.md) |
| 模块概述 | [config 模块](README.md) |