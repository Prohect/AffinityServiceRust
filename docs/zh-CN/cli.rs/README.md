# cli 模块 (AffinityServiceRust)

`cli` 模块负责 AffinityServiceRust 的命令行参数解析和帮助文本输出。它定义了 [CliArgs](CliArgs.md) 结构体，用于承载所有运行时选项——轮询间隔、模式标志、文件路径、权限开关和调试参数——并提供 [parse_args](parse_args.md) 函数从原始参数向量中填充该结构体。多个帮助打印函数提供从基本用法摘要到包含配置文件语法的完整参考等不同详细程度的输出。

`CliArgs` 中的所有模式标志默认为 `false`，所有文件路径默认为合理的值（`config.ini`、`logs`、`new_processes_results.txt`），因此在常见情况下服务可以零参数启动。解析器是一个简单的线性扫描，不依赖外部 crate——每个标志以区分大小写的方式匹配（少数标志提供了大小写变体别名，例如 `-noUAC`/`-nouac`），带值标志会消耗下一个参数。

## 结构体

| 结构体 | 描述 |
|--------|------|
| [CliArgs](CliArgs.md) | 包含所有命令行参数及默认值的容器。在服务生命周期内以引用方式传递。 |

## 函数

| 函数 | 描述 |
|------|------|
| [parse_args](parse_args.md) | 将原始参数切片解析为 [CliArgs](CliArgs.md) 结构体。未知标志会被静默忽略。 |
| [print_help](print_help.md) | 打印涵盖常用选项和运行模式的简洁帮助信息。 |
| [print_cli_help](print_cli_help.md) | 打印详细的 CLI 参考，包括调试和测试选项。 |
| [get_config_help_lines](get_config_help_lines.md) | 返回 `Vec<&'static str>`，包含适合嵌入转换后配置文件的配置文件帮助模板行。 |
| [print_config_help](print_config_help.md) | 将配置帮助模板行打印到活动日志输出。 |
| [print_help_all](print_help_all.md) | 将详细的 CLI 帮助和配置帮助模板合并在一次输出中打印。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 消费 CliArgs 的入口点 | [main](../main.rs/main.md) |
| 配置文件解析 | [config.rs](../config.rs/README.md) |
| 进程优先级枚举 | [priority.rs](../priority.rs/README.md) |
| 日志基础设施（控制台 vs. 文件） | [logging.rs](../logging.rs/README.md) |