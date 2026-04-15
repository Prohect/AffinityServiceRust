# print_cli_help 函数 (cli.rs)

向日志输出打印详细的帮助消息，涵盖 AffinityServiceRust 支持的每个命令行参数、操作模式和调试/测试选项。这是 [print_help](print_help.md) 的扩展版本，在用户传入 `-helpall` 时被调用。

## 语法

```AffinityServiceRust/src/cli.rs#L139-141
pub fn print_cli_help() {
    // ...
}
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。输出通过 `log!` 宏写入当前日志目标。

## 备注

与 [print_help](print_help.md) 不同，此函数**不会**通过设置 `get_use_console!()` 来强制启用控制台模式。预期调用方在调用此函数之前已经启用了控制台输出（如 [print_help_all](print_help_all.md) 所做的那样）。

详细帮助文本按以下部分组织：

| 部分 | 内容 |
|------|------|
| **基本参数** | `-help`、`-console`、`-noUAC`、`-config`、`-find`、`-blacklist`、`-interval`、`-resolution` |
| **操作模式** | `-validate`、`-processlogs`、`-dryrun`、`-convert`、`-autogroup`、`-in`、`-out` |
| **调试与测试选项** | `-loop`、`-logloop`、`-noDebugPriv`、`-noIncBasePriority`、`-no_etw`、`-continuous_process_level_apply` |
| **调试** | 用于非管理员和管理员场景的快速启动调试命令，包括关于 UAC 和控制台会话限制的说明 |

每个参数条目记录了：
- 标志名称及其接受的别名（例如 `-noUAC | -nouac`）
- 标志是否需要后续的值参数
- 未指定时的默认值
- 有效范围或约束（例如，间隔最小值为 16 毫秒）

### 平台说明

帮助文本末尾的说明警告：当进程通过 UAC 提升时，会创建一个新的登录会话。来自提升会话的控制台输出无法在原始终端窗口中显示，因此应改用日志文件。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli.rs` |
| 调用方 | [print_help_all](print_help_all.md) |
| 被调用方 | `log!` 宏（见 [logging.rs](../logging.rs/README.md)） |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| print_help | [print_help](print_help.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| print_config_help | [print_config_help](print_config_help.md) |
| parse_args | [parse_args](parse_args.md) |
| CliArgs | [CliArgs](CliArgs.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*