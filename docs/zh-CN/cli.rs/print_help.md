# print_help 函数 (cli.rs)

向控制台打印简洁的帮助消息，显示 AffinityServiceRust 最常用的命令行选项和操作模式。这是用户传递 `-help`、`--help`、`-?`、`/?` 或 `?` 时显示的默认帮助输出。

## 语法

```AffinityServiceRust/src/cli.rs#L131-L157
pub fn print_help()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。

## 备注

- **控制台激活**：`print_help` 在输出之前无条件地将全局 `use_console` 标志设为 `true`，确保帮助文本被写入控制台，即使服务配置为基于文件的日志记录。

- **输出目标**：文本通过 `log!` 宏输出，该宏遵循在函数体顶部设置的 `use_console` 标志。

- **内容章节**：打印的帮助消息分为两组：
  - **常用选项** — 涵盖 `-help`、`-helpall`、`-console`、`-config`、`-find`、`-interval`、`-noUAC` 和 `-resolution`。
  - **模式** — 涵盖 `-validate`、`-processlogs`、`-dryrun`、`-convert` 和 `-autogroup`。

- **与其他帮助函数的关系**：`print_help` 提供简要概述。有关包括调试和测试选项在内的完整 CLI 参考，请参见 [print_cli_help](print_cli_help.md)。有关 CLI + 配置文件参考的组合输出，请参见 [print_help_all](print_help_all.md)。

- 当 `CliArgs.help_mode` 为 `true` 时调用此函数。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli.rs` |
| 调用方 | `main.rs`（当传递 `-help` / `--help` / `-?` / `/?` / `?` 时） |
| 被调用方 | `log!` 宏、`get_use_console!` 宏 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| CliArgs 结构体 | [CliArgs](CliArgs.md) |
| parse_args 函数 | [parse_args](parse_args.md) |
| print_cli_help 函数 | [print_cli_help](print_cli_help.md) |
| print_help_all 函数 | [print_help_all](print_help_all.md) |
| config 模块 | [config.rs 概述](../config.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*