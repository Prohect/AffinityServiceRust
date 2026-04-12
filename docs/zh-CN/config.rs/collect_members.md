# collect_members 函数 (config.rs)

从冒号分隔的文本字符串中收集进程名称成员到向量中，过滤空条目和注释。

## 语法

```rust
fn collect_members(text: &str, members: &mut Vec<String>)
```

## 参数

`text`

包含冒号分隔进程名称的字符串切片。每个片段在插入前会被修剪空白并转为小写。空片段或以 `#`（注释）开头的片段将被跳过。

`members`

接收解析后成员名称的 `Vec<String>` 可变引用。新条目追加到已有内容之后。

## 返回值

此函数无返回值。结果通过 `members` 输出参数累积。

## 备注

此函数由配置解析器内部使用，用于从单行和多行组定义中提取进程名称。它以 `:` 分隔符拆分输入，这是组块中进程名称的标准分隔符。

每个提取的名称经过以下处理：
1. 修剪前后空白。
2. 转换为小写以实现不区分大小写的匹配。
3. 若为空或以 `#`（行内注释）开头则丢弃。

### 示例

给定输入文本 `"chrome.exe: Firefox.exe: # comment: edge.exe"`，函数将向 `members` 向量追加 `["chrome.exe", "firefox.exe", "edge.exe"]`。

此函数在解析组块（包括内联 `{ a: b }` 和多行形式）时由 [read_config](read_config.md) 调用，也在跨续行累积成员时由 [collect_group_block](collect_group_block.md) 调用。

## 要求

| 要求 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **可见性** | 私有 (`fn`) |
| **调用方** | [read_config](read_config.md)、[collect_group_block](collect_group_block.md) |
| **依赖** | 无 |