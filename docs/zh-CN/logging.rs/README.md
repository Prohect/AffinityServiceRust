# logging.rs 模块 (logging.rs)

`logging` 模块为应用程序提供集中式日志基础设施、错误去重和进程发现跟踪。它管理日志文件、控制台输出，并维护状态以在循环迭代之间抑制重复的错误消息。

## 概述

本模块通过以下几种机制处理应用程序的所有日志输出：

- **通用日志** — [`log_message`](log_message.md) 将带时间戳的消息写入日志文件，并可选地输出到控制台。
- **纯日志** — [`log_pure_message`](log_pure_message.md) 写入不带时间戳前缀的消息。
- **发现日志** — [`log_to_find`](log_to_find.md) 和 [`log_process_find`](log_process_find.md) 写入单独的 `.find.log` 文件，用于进程发现跟踪。
- **错误去重** — [`is_new_error`](is_new_error.md) 防止同一错误在循环迭代中被反复记录。

`log!` 宏是整个代码库使用的主要日志接口，它委托给 [`log_message`](log_message.md)。

## 项目列表

### 静态变量

| 名称 | 说明 |
| --- | --- |
| [FINDS_SET](FINDS_SET.md) | 跟踪已记录到发现日志的进程名称，避免重复。 |
| [USE_CONSOLE](USE_CONSOLE.md) | 控制日志输出是否同时写入控制台。 |
| [DUST_BIN_MODE](DUST_BIN_MODE.md) | 在 UAC 提升前抑制日志输出，避免写入即将被废弃的日志文件。 |
| [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) | 缓存当前本地时间，确保单次循环迭代内时间戳一致。 |
| [LOG_FILE](LOG_FILE.md) | 主日志文件句柄。 |
| [FIND_LOG_FILE](FIND_LOG_FILE.md) | 进程发现输出的发现日志文件句柄。 |
| [FINDS_FAIL_SET](FINDS_FAIL_SET.md) | 跟踪未找到的进程名称，用于去重。 |
| [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) | 按 PID 的去重错误条目映射，键为 (tid, name, operation, error_code)，外层映射按 PID 索引。 |

### 枚举

| 名称 | 说明 |
| --- | --- |
| [Operation](Operation.md) | 枚举所有在配置应用期间可能产生错误的 Windows API 操作。 |

### 结构体

| 名称 | 说明 |
| --- | --- |
| [ApplyFailEntry](ApplyFailEntry.md) | 错误去重的复合键，组合 tid、进程名称、操作和错误代码。 |

### 函数

| 名称 | 说明 |
| --- | --- |
| [is_new_error](is_new_error.md) | 检查给定的 pid/tid/操作/错误组合是否已被记录。 |
| [get_log_path](get_log_path.md) | 使用给定后缀在可执行文件旁构造日志文件路径。 |
| [log_message](log_message.md) | 将带时间戳的日志消息写入日志文件，并可选地输出到控制台。 |
| [log_pure_message](log_pure_message.md) | 写入不带时间戳前缀的日志消息。 |
| [log_to_find](log_to_find.md) | 将消息写入 `.find.log` 文件。 |
| [log_process_find](log_process_find.md) | 将发现的进程名称记录到发现日志（去重）。 |

## 错误去重

去重系统防止每次循环迭代中对同一进程/线程/操作组合反复记录相同的错误导致日志泛滥：

1. 每个错误表示为一个 [`ApplyFailEntry`](ApplyFailEntry.md)，键为 `(pid, tid, process_name, operation, error_code)`。
2. [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 将这些条目存储在两级映射中：PID → 失败条目集合。
3. [`is_new_error`](is_new_error.md) 仅在首次遇到特定错误组合时返回 `true`。
4. 进程退出清理通过 ETW 事件响应式处理——当进程停止时，主循环直接从映射中移除其条目。

## 回收站模式

当 [`DUST_BIN_MODE`](DUST_BIN_MODE.md) 启用时，日志输出被抑制。这用于 UAC 提升前阶段：由于进程将以提升权限重新启动，提升前写入的任何日志输出都会进入一个随即被废弃的日志文件。[`CliArgs`](../cli.rs/CliArgs.md) 中的 `skip_log_before_elevation` 标志控制此行为。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/logging.rs` |
| **调用方** | 所有模块通过 `log!` 宏；[`apply_config`](../main.rs/apply_config.md) 通过 apply 函数间接调用 |
| **关键依赖** | [`Operation`](Operation.md)、[`ApplyFailEntry`](ApplyFailEntry.md)、`chrono::Local`、`once_cell::sync::Lazy` |