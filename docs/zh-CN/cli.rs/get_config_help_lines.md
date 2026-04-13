# get_config_help_lines 函数 (cli.rs)

返回一个包含配置文件帮助模板的静态字符串切片向量。这些行描述了 AffinityServiceRust 配置文件的格式和语法，以注释块的形式适合嵌入到转换或自动分组后的配置文件顶部。该模板提供了冒号分隔规则格式、字段含义、CPU 别名语法和分组语法的快速参考。

## 语法

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

## 参数

此函数不接受参数。

## 返回值

返回一个 `Vec<&'static str>`，包含帮助模板行。每个元素是一行文本，以 `##` 为前缀，写入配置文件时形成注释块。该向量包含 24 行，涵盖：

- 头部横幅 (`## ===...===`)。
- 标题行 (`## AffinityServiceRust Configuration File`)。
- 文档指引 (`## Full documentation: docs/cli.md and docs/config.md`)。
- 冒号分隔规则格式概要 (`## Format: process:priority:affinity:cpuset:prime:io:memory:ideal:grade`)。
- 各字段描述（process、priority、affinity、cpuset、prime、io、memory、ideal、grade）及示例值。
- CPU 别名示例 (`*a`、`*p`、`*e`)。
- 分组语法概要 (`{ proc1: proc2 }:priority:affinity...`)。
- 结尾横幅。

## 备注

### 用途

此函数的存在是为了提供一个可复用的帮助模板，可用于：

1. **打印到控制台** — 通过 [print_config_help](print_config_help.md) 遍历返回的行并逐行记录。
2. **嵌入生成的文件** — [convert](../config.rs/convert.md) 和 [sort_and_group_config](../config.rs/sort_and_group_config.md) 函数可以将这些行添加到输出文件开头，以便用户在文件内直接参考配置语法。

通过返回 `Vec<&'static str>` 而非直接打印，该函数让调用者完全控制这些行的渲染方式和输出位置。

### 静态生命周期

返回向量中的所有字符串都具有 `'static` 生命周期，因为它们是嵌入二进制文件中的字符串字面量。这意味着该向量可以被存储、多次遍历或跨函数边界传递，无需担心生命周期问题。

### 注释前缀约定

每行以 `##`（双井号）开头。在 AffinityServiceRust 配置文件格式中，以 `#` 或 `##` 开头的行被视为注释。双井号约定将自动生成的帮助注释与用户编写的单井号注释区分开来，便于在不影响用户标注的情况下剥离或更新模板。

### 内容摘要

该模板记录了进程规则的九个冒号分隔字段：

| 字段 | 位置 | 示例值 |
|------|------|--------|
| process | 1 | `game.exe` |
| priority | 2 | `none`、`idle`、`below normal`、`normal`、`above normal`、`high`、`real time` |
| affinity | 3 | `0-7`、`0;4;8`、`0xFF`、`*alias` |
| cpuset | 4 | `*p`、`*e`、`*alias` |
| prime | 5 | `?10*pN01`、`*p@module.dll` |
| io | 6 | `none`、`very low`、`low`、`normal`、`high` |
| memory | 7 | `none`、`very low`、`low`、`medium`、`below normal`、`normal` |
| ideal | 8 | `*alias[@prefix]`、`0` |
| grade | 9 | `1`（每次循环）、`5`（每 5 次循环） |

### 典型用法

```rust
// 打印到控制台
for line in get_config_help_lines() {
    log!("{}", line);
}

// 嵌入输出文件
let mut output = String::new();
for line in get_config_help_lines() {
    output.push_str(line);
    output.push('\n');
}
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用者 | [print_config_help](print_config_help.md)、[print_help_all](print_help_all.md)（通过 `print_config_help` 间接调用）、[convert](../config.rs/convert.md)、[sort_and_group_config](../config.rs/sort_and_group_config.md) |
| 被调用者 | 无（纯函数） |
| API | 无 |
| 权限 | 不适用 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 将配置帮助行打印到日志输出 | [print_config_help](print_config_help.md) |
| 组合 CLI + 配置帮助 | [print_help_all](print_help_all.md) |
| 配置文件解析器 | [read_config](../config.rs/read_config.md) |
| Process Lasso 配置转换器 | [convert](../config.rs/convert.md) |
| 自动分组工具 | [sort_and_group_config](../config.rs/sort_and_group_config.md) |
| 进程规则结构体 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |