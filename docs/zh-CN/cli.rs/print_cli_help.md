# print_cli_help 函数 (cli.rs)

打印详细的命令行帮助信息，包含所有选项和调试功能。

## 语法

```rust
pub fn print_cli_help()
```

## 参数

此函数没有参数。

## 返回值

此函数不返回值。直接将帮助文本输出到当前日志目标。

## 备注

`print_cli_help` 输出完整的命令行参数参考，覆盖所有可用选项，包括不在基本帮助 [`print_help`](print_help.md) 中显示的调试和测试选项。

输出内容按以下分类组织：

### 基本参数

| 参数 | 描述 |
| --- | --- |
| `-help` \| `--help` | 打印基本帮助信息 |
| `-?` \| `/?` \| `?` | 打印基本帮助信息 |
| `-helpall` \| `--helpall` | 打印此详细帮助及调试选项 |
| `-console` | 使用控制台作为输出而非日志文件 |
| `-noUAC` \| `-nouac` | 禁用 UAC 提权请求 |
| `-config <file>` | 指定配置文件（默认：`config.ini`） |
| `-find` | 查找亲和性与系统默认相同的进程 |
| `-blacklist <file>` | `-find` 模式的黑名单文件 |
| `-interval <ms>` | 设置检查间隔（默认 5000，最小 16） |
| `-resolution <t>` | 计时器分辨率，如 5210 → 0.5210ms（默认 0，不设置） |

### 运行模式

| 参数 | 描述 |
| --- | --- |
| `-validate` | 验证配置文件语法后退出 |
| `-processlogs` | 处理 `-find` 模式的日志，发现新进程并搜索路径 |
| `-dryrun` | 模拟变更，不实际应用 |
| `-convert` | 转换 Process Lasso 格式的配置文件 |
| `-autogroup` | 将设置相同的规则自动分组为命名组块 |
| `-in <file>` | 输入文件（`-convert`）或日志目录（`-processlogs`） |
| `-out <file>` | 输出文件 |

### 调试与测试选项

| 参数 | 描述 |
| --- | --- |
| `-loop <count>` | 运行指定循环次数（默认：无限） |
| `-logloop` | 在每次循环开始时记录消息 |
| `-noDebugPriv` | 不请求 SeDebugPrivilege |
| `-noIncBasePriority` | 不请求 SeIncreaseBasePriorityPrivilege |

输出还包含快速调试命令示例：

- **非管理员调试：** `AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini`
- **管理员调试：** 不使用 `-console`，运行后检查 `logs/YYYYMMDD.log`

> **注意：** 当通过 UAC 提权运行时，`-console` 输出会进入新会话，无法在当前会话中显示。此时应使用日志文件。

与 [`print_help`](print_help.md) 不同，此函数**不会**设置控制台模式（`USE_CONSOLE`）。它由 [`print_help_all`](print_help_all.md) 调用，后者负责在调用前设置控制台模式。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/cli.rs |
| **源码行** | L149–L197 |
| **调用者** | [`print_help_all`](print_help_all.md) |

## 另请参阅

- [cli.rs 模块概述](README.md)
- [print_help](print_help.md) — 基本帮助（常用选项）
- [print_help_all](print_help_all.md) — 完整帮助（CLI + 配置格式）
- [print_config_help](print_config_help.md) — 配置文件格式帮助
- [CliArgs](CliArgs.md) — 解析后的命令行参数结构体