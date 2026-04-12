# sort_and_group_config 函数 (config.rs)

自动将具有相同规则设置的进程分组，以减少配置文件中的重复内容。读取现有配置，识别共享相同规则参数的进程，并使用 `{ process1: process2: ... }:rule` 语法将其合并为命名组块。

## 语法

```rust
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## 参数

`in_file`

一个 `Option<String>`，指定要读取和分析的输入配置文件路径。如果为 `None`，将记录错误并立即返回。通常通过 `-in` CLI 参数提供。

`out_file`

一个 `Option<String>`，指定写入分组后配置的输出文件路径。如果为 `None`，将记录错误并立即返回。通常通过 `-out` CLI 参数提供。

## 返回值

此函数不返回值。结果直接写入输出文件。错误和进度信息通过日志子系统记录。

## 备注

当用户传递 `-autogroup` CLI 标志时调用此函数。其执行以下步骤：

1. 调用 [read_config](read_config.md) 解析输入配置文件为 [ConfigResult](ConfigResult.md)。
2. 对于每个 grade 层级，遍历所有 [ProcessConfig](ProcessConfig.md) 条目，从非名称字段（priority、affinity、cpuset、prime threads、IO priority、memory priority、ideal processor rules、grade）计算规范化的"规则键"字符串。
3. 将共享相同规则键的进程合并为单个组块。
4. 通过重新读取原始行，保留原始文件中的常量（`@MIN_ACTIVE_STREAK` 等）和别名（`*name = cpu_spec`）。
5. 输出紧凑的分组配置：单成员组写为普通规则，多成员组使用 `{ member1: member2 }:rule` 语法。
6. 每个组内的进程按字母顺序排序以保持一致性。

生成的输出是一个有效的配置文件，功能上与输入等效，但减少了冗余。当配置文件因自然增长而包含许多共享相同设置的独立规则时，此功能非常有用。

原始规则引用的 CPU 别名在解析期间被解析，然后在输出中作为字面 CPU 规格重新发出，除非原始别名定义从原始文件中保留。

### 示例

给定输入配置：

```
*p = 0-7
chrome.exe:high:*p:0:0:none:none
firefox.exe:high:*p:0:0:none:none
notepad.exe:normal:0:0:0:none:none
```

输出将对 chrome 和 firefox 进行分组：

```
*p = 0-7
{ chrome.exe: firefox.exe }:high:*p:0:0:none:none
notepad.exe:normal:0:0:0:none:none
```

### 错误处理

- 如果 `in_file` 或 `out_file` 为 `None`，记录错误消息并返回。
- 如果无法读取或解析输入文件，将报告来自 [read_config](read_config.md) 的错误。
- 如果无法创建或写入输出文件，将记录错误。

### 相关函数

- [read_config](read_config.md) — 解析输入配置文件
- [convert](convert.md) — 将 Process Lasso 配置转换为原生格式
- [parse_and_insert_rules](parse_and_insert_rules.md) — 解析单个规则字段
- [format_cpu_indices](format_cpu_indices.md) — 将 CPU 索引格式化为紧凑的范围字符串用于输出

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **可见性** | `pub` |
| **调用方** | `src/main.rs`（设置 `-autogroup` CLI 标志时） |
| **依赖** | [read_config](read_config.md)、[format_cpu_indices](format_cpu_indices.md)、[ConfigResult](ConfigResult.md)、[ProcessConfig](ProcessConfig.md) |