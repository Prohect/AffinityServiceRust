# print_help_all 函数 (cli.rs)

为 AffinityServiceRust 打印完整的帮助输出，将命令行用法和配置文件格式文档组合成一个统一的参考。这是 `-helpall` / `--helpall` 命令行标志的处理程序。

## 语法

```rust
pub fn print_help_all()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。

## 说明

`print_help_all` 是 `-helpall` / `--helpall` 命令行标志的入口点。它按顺序执行三个操作：

1. **强制控制台输出** — 通过 `get_use_console!()` 将全局 `use_console` 标志设置为 `true`，确保所有后续输出都发送到交互控制台而不是日志文件。
2. **打印 CLI 帮助** — 调用 [print_cli_help](print_cli_help.md) 显示完整的命令行参数参考，包括基本参数、操作模式、调试/测试选项和示例调试命令。
3. **打印配置帮助** — 通过 `log!("")` 输出一个空白分隔行，然后调用 [print_config_help](print_config_help.md) 显示配置文件格式文档，涵盖术语、规则语法、CPU 规格格式、优先级级别、理想处理器语法和进程组语法。

### 控制台标志副作用

由于 `print_help_all` 强制设置 `use_console = true`，同一进程调用期间的任何后续日志记录也将输出到控制台。这是有意的——用户正在交互模式下运行，期望所有输出都在屏幕上。

### 与其他帮助函数的关系

| 函数 | 范围 | 设置控制台标志？ |
|----------|-------|-------------------|
| [print_help](print_help.md) | 简洁用法摘要 | 是 |
| [print_cli_help](print_cli_help.md) | 仅完整 CLI 参考 | 否 |
| [print_config_help](print_config_help.md) | 仅配置格式参考 | 否 |
| **print_help_all** | 完整 CLI + 配置格式 | 是 |

`print_help_all` 是唯一组合 `print_cli_help` 和 `print_config_help` 的函数。它也是仅有的两个帮助函数之一（另一个是 `print_help`），它会设置控制台输出标志，因为另外两个函数设计为由组合函数调用的构建块。

## 要求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用方** | `main`（当 [CliArgs](CliArgs.md)`.help_all_mode` 为 `true` 时） |
| **被调用方** | `get_use_console!` 宏，`log!` 宏，[print_cli_help](print_cli_help.md)，[print_config_help](print_config_help.md) |
| **Win32 API** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [cli.rs](README.md) |
| 基本帮助输出 | [print_help](print_help.md) |
| CLI 帮助（内部调用） | [print_cli_help](print_cli_help.md) |
| 配置帮助（内部调用） | [print_config_help](print_config_help.md) |
| 配置帮助行来源 | [get_config_help_lines](get_config_help_lines.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*