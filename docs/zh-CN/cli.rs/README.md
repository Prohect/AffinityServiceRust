# cli 模块 (AffinityServiceRust)

`cli` 模块为 AffinityServiceRust Windows 服务提供命令行参数解析和帮助文本生成功能。它定义了保存所有从命令行参数派生的运行时配置的 `CliArgs` 结构体，并公开用于解析参数、显示用法信息和输出配置文件文档的函数。

## 函数

| 函数 | 描述 |
|------|------|
| [parse_args](parse_args.md) | 将命令行参数字符串切片解析为 `CliArgs` 实例。 |
| [print_help](print_help.md) | 打印简洁的帮助信息，显示常用选项和操作模式。 |
| [print_cli_help](print_cli_help.md) | 打印详细的帮助信息，包括所有参数、调试选项和使用示例。 |
| [get_config_help_lines](get_config_help_lines.md) | 返回包含配置文件参考模板的静态字符串切片向量。 |
| [print_config_help](print_config_help.md) | 将配置文件参考信息打印到当前日志输出。 |
| [print_help_all](print_help_all.md) | 同时打印详细的 CLI 帮助信息和完整的配置文件参考。 |

## 结构体

| 结构体 | 描述 |
|--------|------|
| [CliArgs](CliArgs.md) | 保存控制服务运行时行为的所有命令行选项和标志。 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| config 模块 | [config.rs 概述](../config.rs/README.md) |
| main 模块 | [main.rs 概述](../main.rs/README.md) |
| logging 模块 | [logging.rs 概述](../logging.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
