# print_help 函数 (cli.rs)

向活动日志输出打印简洁的帮助信息，涵盖最常用的命令行选项和操作模式。这是用户传递 `-help`、`--help`、`-?`、`/?` 或 `?` 时显示的默认帮助信息。如需包含调试选项在内的完整参考，请参阅 [print_cli_help](print_cli_help.md) 或 [print_help_all](print_help_all.md)。

## 语法

```rust
pub fn print_help()
```

## 参数

此函数不接受参数。

## 返回值

此函数没有返回值。

## 备注

### 强制控制台输出

该函数在输出任何内容之前，会通过 `*get_use_console!() = true` 将全局 `USE_CONSOLE` 静态变量设置为 `true`。这确保帮助文本写入控制台（stdout）而不是日志文件，无论是否显式传递了 `-console` 参数。这是预期行为，因为帮助输出始终用于交互式查看。

### 内容

帮助信息是一个包含原始字符串字面量的单次 `log!()` 宏调用。涵盖以下内容：

- **标题** — 服务的单行描述和用法概要。
- **常用选项** — 最常用的标志：
  - `-help` / `--help` — 显示基本帮助信息。
  - `-helpall` — 显示详细参考（委托给 [print_help_all](print_help_all.md)）。
  - `-console` — 将输出路由到控制台而非日志文件。
  - `-config <file>` — 指定配置文件（默认：`config.ini`）。
  - `-find` — 发现具有默认 CPU affinity 的进程，可选配合 `-blacklist <file>` 使用。
  - `-interval <ms>` — 设置轮询间隔（默认：5000 毫秒）。
  - `-noUAC` — 禁止自动 UAC 提升请求。
  - `-resolution <t>` — 设置系统定时器分辨率（例如 `5210` → 0.5210 毫秒）。
- **模式** — 各实用模式的单行描述：
  - `-validate` — 检查配置语法但不运行。
  - `-processlogs` — 分析发现模式日志以发现新进程。
  - `-dryrun` — 模拟更改但不实际应用。
  - `-convert` — 转换 Process Lasso 配置。
  - `-autogroup` — 将具有相同设置的规则自动分组。

### 与其他帮助函数的关系

| 函数 | 范围 | 调用者 |
|------|------|--------|
| **print_help**（本函数） | 仅常用选项和模式 | `-help`、`--help`、`-?`、`/?`、`?` |
| [print_cli_help](print_cli_help.md) | 包含调试选项的完整 CLI 参考 | 由 [print_help_all](print_help_all.md) 调用 |
| [print_config_help](print_config_help.md) | 配置文件格式模板 | 由 [print_help_all](print_help_all.md) 调用 |
| [print_help_all](print_help_all.md) | CLI 参考 + 配置模板的组合输出 | `-helpall`、`--helpall` |

### 调用流程

在 [main](../main.rs/main.md) 中，帮助模式检查在加载任何配置之前执行：

```rust
if cli.help_mode {
    print_help();
    return Ok(());
}
```

这意味着函数执行后程序即退出，不会接触配置文件、黑名单、权限或任何其他子系统。即使在配置文件不存在或格式错误的环境中调用也是安全的。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用者 | [main](../main.rs/main.md)（当 `cli.help_mode` 为 `true` 时） |
| 被调用者 | `get_use_console!()` 宏、`log!()` 宏 |
| API | 无 |
| 权限 | 不适用 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 详细 CLI 帮助 | [print_cli_help](print_cli_help.md) |
| 配置文件帮助 | [print_config_help](print_config_help.md) |
| 组合帮助输出 | [print_help_all](print_help_all.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |
| 参数解析器 | [parse_args](parse_args.md) |
| 入口点 | [main](../main.rs/main.md) |