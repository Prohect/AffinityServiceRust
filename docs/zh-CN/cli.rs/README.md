# cli 模块 (AffinityServiceRust)

`cli` 模块实现了 AffinityServiceRust 的命令行参数解析和帮助文本打印。它定义了 [CliArgs](CliArgs.md) 结构体，用于捕获所有运行时选项——轮询间隔、运行模式、调试标志、文件路径和权限控制——并提供 [parse_args](parse_args.md) 从进程参数中填充该结构体。该模块还暴露了几个帮助打印函数，用于记录基本用法、详细的 CLI 选项和配置文件格式。

## 结构体

| 名称 | 描述 |
|------|-------------|
| [CliArgs](CliArgs.md) | 从命令行参数填充的运行时配置。包含轮询间隔、模式标志、文件路径、权限切换和调试选项。 |

## 函数

| 名称 | 描述 |
|------|-------------|
| [parse_args](parse_args.md) | 将字符串切片形式的命令行参数解析为 [CliArgs](CliArgs.md) 实例。 |
| [print_help](print_help.md) | 打印简洁的用法摘要，涵盖最常见的选项和运行模式。 |
| [print_cli_help](print_cli_help.md) | 打印详细的 CLI 帮助，包括所有基本参数、运行模式、调试和测试选项。 |
| [get_config_help_lines](get_config_help_lines.md) | 返回一个 `Vec<&'static str>`，包含配置文件格式文档模板。 |
| [print_config_help](print_config_help.md) | 通过迭代 [get_config_help_lines](get_config_help_lines.md) 来打印配置文件格式帮助。 |
| [print_help_all](print_help_all.md) | 通过 [print_cli_help](print_cli_help.md) 和 [print_config_help](print_config_help.md) 打印组合的完整帮助——CLI 选项后跟配置文件格式。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 配置解析 | [config.rs](../config.rs/README.md) |
| 主入口点 | [main.rs](../main.rs/README.md) |
| 日志基础设施 | [logging.rs](../logging.rs/README.md) |
| 优先级枚举 | [priority.rs](../priority.rs/README.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*