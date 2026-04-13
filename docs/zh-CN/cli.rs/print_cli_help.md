# print_cli_help 函数 (cli.rs)

打印详细的 CLI 参考信息，涵盖所有命令行选项，包括基本参数、运行模式、调试/测试选项以及实用的调试示例。此函数提供了补充简洁 [print_help](print_help.md) 输出的完整参考。它不会被 CLI 标志直接调用——而是由 [print_help_all](print_help_all.md) 作为组合帮助输出的前半部分在内部调用。

## 语法

```rust
pub fn print_cli_help()
```

## 参数

此函数不接受参数。

## 返回值

此函数没有返回值。

## 备注

### 控制台行为

与 [print_help](print_help.md) 不同，此函数**不会**自行设置全局 `USE_CONSOLE` 静态变量。它依赖其调用者（[print_help_all](print_help_all.md)）在调用前已将 `USE_CONSOLE` 设置为 `true`。如果在未启用控制台模式的情况下单独调用，输出将写入日志文件而非 stdout。

### 内容

帮助信息是一个包含原始字符串字面量的 `log!()` 宏调用，分为四个部分：

- **基本参数** — 所有常用标志及其描述：
  - `-help` / `--help` / `-?` / `/?` / `?` — 打印基本帮助信息。
  - `-helpall` / `--helpall` — 打印详细帮助（即此输出）。
  - `-console` — 将输出路由到控制台而非日志文件。
  - `-noUAC` / `-nouac` — 禁用 UAC 提升请求。
  - `-config <file>` — 指定配置文件（默认：`config.ini`）。
  - `-find` — 发现具有默认（未管理）CPU affinity 的进程。
  - `-blacklist <file>` — 为 `-find` 模式指定黑名单文件。
  - `-interval <ms>` — 设置轮询间隔（默认：5000，最小：16）。
  - `-resolution <t>` — 设置系统定时器分辨率（例如 `5210` → 0.5210 ms；0 表示不设置）。

- **运行模式** — 每种实用模式的详细描述：
  - `-validate` — 验证配置文件语法和未定义的别名，然后退出。
  - `-processlogs` — 处理 find 模式日志以发现新进程并通过 Everything 搜索解析路径（`-config`、`-blacklist`、`-in`、`-out`）。
  - `-dryrun` — 模拟更改而不实际应用（显示将会发生什么）。
  - `-convert` — 从 `-in <file>` 转换 Process Lasso 配置到 `-out <file>`。
  - `-autogroup` — 将具有相同设置的规则自动分组为命名组块（`-in <file>` `-out <file>`）。
  - `-in <file>` — `-convert` / `-processlogs` 的输入文件/目录（`-processlogs` 默认为 `logs`）。
  - `-out <file>` — `-convert` / `-processlogs` 的输出文件（默认：`new_processes_results.txt`）。

- **调试与测试选项** — 用于开发和故障排除的标志：
  - `-loop <count>` — 运行固定次数的轮询迭代后退出（默认：无限）。
  - `-logloop` — 在每次循环迭代开始时记录带时间戳的消息。
  - `-noDebugPriv` — 启动时不请求 `SeDebugPrivilege`。
  - `-noIncBasePriority` — 启动时不请求 `SeIncreaseBasePriorityPrivilege`。

- **调试示例** — 两个实用的命令行示例：
  - 使用 `-console -noUAC -logloop -loop 3 -interval 2000 -config test.ini` 的非管理员快速调试命令。
  - 管理员调试工作流（不使用 `-console`），写入 `logs/YYYYMMDD.log`，并附有说明 UAC 提升会启动新会话，控制台输出在该会话中不可见。

### 与其他帮助函数的关系

| 函数 | 范围 | 调用者 |
|------|------|--------|
| [print_help](print_help.md) | 仅常用选项和模式 | `-help`、`--help`、`-?`、`/?`、`?` |
| **print_cli_help**（本函数） | 完整 CLI 参考，包括调试选项 | [print_help_all](print_help_all.md) |
| [print_config_help](print_config_help.md) | 配置文件格式模板 | [print_help_all](print_help_all.md) |
| [print_help_all](print_help_all.md) | CLI 参考 + 配置模板的组合输出 | `-helpall`、`--helpall` |

### 调用流程

此函数不会直接从 [main](../main.rs/main.md) 调用。调用链为：

1. 用户传入 `-helpall` → [parse_args](parse_args.md) 设置 `cli.help_all_mode = true`。
2. [main](../main.rs/main.md) 检查 `cli.help_all_mode` 并调用 [print_help_all](print_help_all.md)。
3. [print_help_all](print_help_all.md) 设置 `USE_CONSOLE = true`，调用 `print_cli_help()`，然后调用 [print_config_help](print_config_help.md)。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用者 | [print_help_all](print_help_all.md) |
| 被调用者 | `log!()` 宏 |
| API | 无 |
| 权限 | 不适用 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 基本帮助输出 | [print_help](print_help.md) |
| 配置文件帮助 | [print_config_help](print_config_help.md) |
| 组合帮助输出 | [print_help_all](print_help_all.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |
| 参数解析器 | [parse_args](parse_args.md) |
| 入口点 | [main](../main.rs/main.md) |