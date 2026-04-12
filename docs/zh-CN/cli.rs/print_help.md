# print_help 函数 (cli.rs)

打印基本帮助信息，显示常用命令行选项和操作模式。

## 语法

```rust
pub fn print_help()
```

## 参数

此函数没有参数。

## 返回值

此函数不返回值。输出直接打印到控制台。

## 备注

调用此函数时，会自动将控制台输出模式设为启用（`USE_CONSOLE = true`），确保帮助文本显示在终端而非日志文件中。

帮助信息分为两个部分：

### 常用选项

| 参数 | 描述 |
| --- | --- |
| `-help` \| `--help` | 显示基本帮助信息 |
| `-helpall` | 显示详细选项和调试功能 |
| `-console` | 输出到控制台而非日志文件 |
| `-config <file>` | 指定配置文件（默认：`config.ini`） |
| `-find` | 查找使用默认亲和性的进程（配合 `-blacklist <file>`） |
| `-interval <ms>` | 检查间隔（毫秒，默认：5000） |
| `-noUAC` | 禁用 UAC 提权请求 |
| `-resolution <t>` | 计时器分辨率，如 5210 表示 0.5210ms（默认：0，不设置） |

### 操作模式

| 参数 | 描述 |
| --- | --- |
| `-validate` | 验证配置文件语法，不实际运行 |
| `-processlogs` | 处理 `-find` 模式的日志以发现新进程 |
| `-dryrun` | 显示将要执行的更改，但不实际应用 |
| `-convert` | 转换 Process Lasso 配置（`-in <file> -out <file>`） |
| `-autogroup` | 自动将相同设置的规则分组（`-in <file> -out <file>`） |

此函数输出的是简洁版帮助。如需查看包含调试选项的完整帮助，请使用 [`print_help_all`](print_help_all.md)。

当用户通过 `-help`、`--help`、`-?`、`/?` 或 `?` 参数触发帮助模式时，[`main`](../main.rs/main.md) 会调用此函数并立即退出。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/cli.rs |
| **源码行** | L121–L147 |
| **调用者** | [`main`](../main.rs/main.md)（当 `cli.help_mode == true`） |
| **触发参数** | `-help` \| `--help` \| `-?` \| `/?` \| `?` |

## 另请参阅

- [print_cli_help](print_cli_help.md) — 包含调试选项的详细 CLI 帮助
- [print_config_help](print_config_help.md) — 配置文件格式帮助
- [print_help_all](print_help_all.md) — 完整帮助（CLI + 配置格式）
- [parse_args](parse_args.md) — 命令行参数解析
- [CliArgs](CliArgs.md) — 解析后的命令行参数结构体