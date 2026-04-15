# print_help_all 函数 (cli.rs)

将详细的 CLI 帮助和完整的配置文件参考同时打印到控制台。这是 `-helpall` / `--helpall` 命令行标志的处理程序，为用户提供所有可用选项和配置文件语法的完整文档。

## 语法

```AffinityServiceRust/src/cli.rs#L267-L271
pub fn print_help_all() {
    *get_use_console!() = true;
    print_cli_help();
    log!("");
    print_config_help();
}
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。

## 备注

`print_help_all` 是 [print_cli_help](print_cli_help.md) 和 [print_config_help](print_config_help.md) 的组合，中间以空行分隔。在打印之前，它通过 `get_use_console!()` 宏将全局 `use_console` 标志强制设为 `true`，确保组合帮助文本始终以交互方式显示，而不是写入日志文件。

输出分为两大部分：

1. **CLI 选项** — 由 `print_cli_help` 生成，涵盖所有命令行参数、运行模式以及调试/测试选项，并附有用法示例。
2. **配置文件参考** — 由 `print_config_help` 生成，涵盖术语、配置格式、CPU 规格格式、优先级级别、理想处理器语法和进程分组语法。

当用户在命令行中传入 `-helpall` 或 `--helpall` 时，将调用此函数。[CliArgs](CliArgs.md) 上的 `help_all_mode` 标志由 [parse_args](parse_args.md) 设置，主模块随后调用 `print_help_all` 作为响应。

### 激活方式

该标志可通过以下命令行标记触发：

| 标记 | 效果 |
|------|------|
| `-helpall` | 设置 `cli.help_all_mode = true` |
| `--helpall` | 设置 `cli.help_all_mode = true` |

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli`（`src/cli.rs`） |
| 调用方 | `main`（当 `cli.help_all_mode` 为 `true` 时） |
| 被调用方 | [print_cli_help](print_cli_help.md)、[print_config_help](print_config_help.md)、`get_use_console!()`、`log!()` |
| 平台 | Windows（通过 `log!` 宏进行控制台输出） |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| print_help | [print_help](print_help.md) |
| print_cli_help | [print_cli_help](print_cli_help.md) |
| print_config_help | [print_config_help](print_config_help.md) |
| get_config_help_lines | [get_config_help_lines](get_config_help_lines.md) |
| CliArgs | [CliArgs](CliArgs.md) |
| parse_args | [parse_args](parse_args.md) |
| config 模块 | [config.rs 概述](../config.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
