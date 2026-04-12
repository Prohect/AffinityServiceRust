# cli.rs 模块 (cli.rs)

`cli` 模块负责命令行参数解析和帮助文本生成。它将用户提供的命令行参数转换为结构化的 [`CliArgs`](CliArgs.md) 配置，并提供多层级的帮助信息输出。

## 概述

本模块是应用程序启动流程的第一步。[`main`](../main.rs/main.md) 函数在执行任何业务逻辑之前，首先调用 [`parse_args`](parse_args.md) 解析命令行参数，然后根据解析结果决定进入哪种运行模式（帮助、转换、验证、主循环等）。

帮助系统分为三个层级：

- **基本帮助** — [`print_help`](print_help.md)：常用选项和运行模式概览
- **详细 CLI 帮助** — [`print_cli_help`](print_cli_help.md)：包含所有命令行选项及调试技巧
- **配置格式帮助** — [`print_config_help`](print_config_help.md)：配置文件语法模板
- **完整帮助** — [`print_help_all`](print_help_all.md)：CLI 帮助 + 配置格式帮助的合集

## 项目

### 结构体

| 名称 | 描述 |
| --- | --- |
| [CliArgs](CliArgs.md) | 存储解析后的命令行参数，包含 21 个字段。 |

### 函数

| 名称 | 描述 |
| --- | --- |
| [parse_args](parse_args.md) | 解析命令行参数字符串数组，填充 `CliArgs` 结构体。 |
| [print_help](print_help.md) | 打印基本帮助信息（常用选项和运行模式）。 |
| [print_cli_help](print_cli_help.md) | 打印详细 CLI 帮助，包含所有选项和调试命令示例。 |
| [get_config_help_lines](get_config_help_lines.md) | 返回配置文件帮助模板行，用于 `-convert` 输出嵌入。 |
| [print_config_help](print_config_help.md) | 打印配置文件格式帮助。 |
| [print_help_all](print_help_all.md) | 打印完整帮助（CLI + 配置格式）。 |

## 运行模式

`CliArgs` 中的布尔字段控制应用程序进入不同的运行模式：

| 模式 | 字段 | CLI 参数 | 描述 |
| --- | --- | --- | --- |
| 帮助 | `help_mode` | `-help` | 显示基本帮助后退出 |
| 完整帮助 | `help_all_mode` | `-helpall` | 显示完整帮助后退出 |
| 转换 | `convert_mode` | `-convert` | 转换 Process Lasso 配置后退出 |
| 自动分组 | `autogroup_mode` | `-autogroup` | 自动分组相同配置的规则后退出 |
| 查找 | `find_mode` | `-find` | 在主循环中启用未配置进程发现 |
| 验证 | `validate_mode` | `-validate` | 验证配置文件语法后退出 |
| 处理日志 | `process_logs_mode` | `-processlogs` | 处理 find 日志发现新进程后退出 |
| 试运行 | `dry_run` | `-dryrun` | 执行一次循环但不实际应用更改 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/cli.rs` |
| **调用者** | [`main`](../main.rs/main.md) |
| **依赖** | `crate::log`（日志宏）、`crate::get_use_console`（控制台输出开关） |