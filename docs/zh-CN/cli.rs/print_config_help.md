# print_config_help 函数 (cli.rs)

将配置文件帮助模板打印到当前活动的日志输出。此函数遍历 [get_config_help_lines](get_config_help_lines.md) 返回的行，并通过 `log!()` 宏逐行记录。作为组合帮助输出的一部分调用时，它提供配置文件语法的快速参考。

## 语法

```rust
pub fn print_config_help()
```

## 参数

此函数不接受参数。

## 返回值

此函数没有返回值。

## 备注

### 实现

该函数是 [get_config_help_lines](get_config_help_lines.md) 的轻量封装：

```rust
pub fn print_config_help() {
    for line in get_config_help_lines() {
        log!("{}", line);
    }
}
```

模板向量中的每一行都传递给 `log!()` 宏，该宏根据全局 `USE_CONSOLE` 状态将输出路由到控制台（stdout）或日志文件。此函数本身不设置 `USE_CONSOLE`——它依赖调用方预先配置好输出目标。

### 控制台行为

当从 [print_help_all](print_help_all.md) 调用时，控制台输出已由该函数强制启用（`*get_use_console!() = true`）。如果在未启用控制台模式的情况下单独调用 `print_config_help`，模板行将被写入日志文件而非 stdout。实际上，此函数不会被单独调用——它始终作为 [print_help_all](print_help_all.md) 的第二部分被调用。

### 输出格式

输出由 24 行以注释符开头的行组成（每行以 `##` 开头），描述冒号分隔的配置规则格式、每个字段的取值选项、CPU 别名语法和组块语法。这些行逐行输出，每行作为单独的日志条目，这意味着写入日志文件时每行会带有时间戳前缀。写入控制台时，各行按顺序显示且不带时间戳（控制台日志记录器不添加时间戳前缀）。

### 与其他帮助函数的关系

| 函数 | 范围 | 调用方 |
|------|------|--------|
| [print_help](print_help.md) | 仅常用选项和模式 | `-help`、`--help`、`-?`、`/?`、`?` |
| [print_cli_help](print_cli_help.md) | 完整 CLI 参考，包含调试选项 | [print_help_all](print_help_all.md) |
| **print_config_help**（本函数） | 配置文件格式模板 | [print_help_all](print_help_all.md) |
| [print_help_all](print_help_all.md) | CLI 参考 + 配置模板组合输出 | `-helpall`、`--helpall` |

### 调用流程

此函数不会直接从 [main](../main.rs/main.md) 调用。调用链如下：

1. 用户传入 `-helpall` → [parse_args](parse_args.md) 设置 `cli.help_all_mode = true`。
2. [main](../main.rs/main.md) 检查 `cli.help_all_mode` 并调用 [print_help_all](print_help_all.md)。
3. [print_help_all](print_help_all.md) 设置 `USE_CONSOLE = true`，调用 [print_cli_help](print_cli_help.md)，输出一个空白分隔行，然后调用 `print_config_help()`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用方 | [print_help_all](print_help_all.md) |
| 被调用方 | [get_config_help_lines](get_config_help_lines.md)、`log!()` 宏 |
| API | 无 |
| 权限 | 不适用 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 配置帮助行提供者 | [get_config_help_lines](get_config_help_lines.md) |
| CLI + 配置组合帮助 | [print_help_all](print_help_all.md) |
| 详细 CLI 帮助 | [print_cli_help](print_cli_help.md) |
| 基本帮助输出 | [print_help](print_help.md) |
| 配置文件解析器 | [read_config](../config.rs/read_config.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |
| 入口点 | [main](../main.rs/main.md) |