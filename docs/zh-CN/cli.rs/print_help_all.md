# print_help_all 函数 (cli.rs)

打印完整帮助信息，包括 CLI 选项和配置文件格式说明。

## 语法

```rust
pub fn print_help_all()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。直接输出到控制台。

## 备注

`print_help_all` 是由 `-helpall` / `--helpall` 命令行参数触发的帮助输出函数。它将控制台输出模式设置为 `true`，然后依次调用 [`print_cli_help`](print_cli_help.md) 和 [`print_config_help`](print_config_help.md)，在两者之间插入一个空行分隔。

该函数提供最全面的帮助输出，涵盖：

1. **CLI 详细帮助** — 通过 [`print_cli_help`](print_cli_help.md) 输出所有命令行选项，包括基本参数、操作模式和调试/测试选项
2. **配置文件格式** — 通过 [`print_config_help`](print_config_help.md) 输出配置文件的语法说明和字段格式

### 调用时机

在 [`main`](../main.rs/main.md) 函数中，当 [`CliArgs`](CliArgs.md) 的 `help_all_mode` 为 `true` 时调用此函数，随后程序立即退出。

### 与 print_help 的区别

| 函数 | 触发参数 | 内容范围 |
| --- | --- | --- |
| [`print_help`](print_help.md) | `-help`、`--help`、`-?`、`/?`、`?` | 仅常用选项 |
| **print_help_all** | `-helpall`、`--helpall` | CLI 完整选项 + 配置文件格式 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/cli.rs |
| **源码行** | L237–L242 |
| **调用者** | [`main`](../main.rs/main.md) |
| **调用** | [`print_cli_help`](print_cli_help.md)、[`print_config_help`](print_config_help.md) |

## 另请参阅

- [cli.rs 模块概述](README.md)
- [print_help](print_help.md) — 基本帮助
- [print_cli_help](print_cli_help.md) — 详细 CLI 帮助
- [print_config_help](print_config_help.md) — 配置格式帮助
- [CliArgs](CliArgs.md) — `help_all_mode` 字段