# logging 模块 (AffinityServiceRust)

`logging` 模块提供 AffinityServiceRust 的所有日志基础设施，包括带时间戳的文件和控制台输出、find 模式下的进程发现日志，以及错误报告的去重系统。它管理基于日期命名的日志文件创建，维护日志路由的全局状态（控制台与文件、dust-bin 抑制），并跟踪每个 PID 的操作失败，使重复错误在每个会话中仅记录一次。该模块公开了直接日志记录函数和用于锁保护访问其全局静态变量的便捷宏。

## 静态变量

| 静态变量 | 描述 |
|----------|------|
| [FINDS_SET](FINDS_SET.md) | 去重集合，记录当前会话中在 `-find` 模式下已记录的进程名称。 |
| [USE_CONSOLE](USE_CONSOLE.md) | 控制日志输出是发送到控制台（`true`）还是日志文件（`false`）的标志。 |
| [DUST_BIN_MODE](DUST_BIN_MODE.md) | 当为 `true` 时抑制所有日志输出的标志；用于 UAC 提升之前，以避免写入非特权进程无权拥有的文件。 |
| [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) | 缓存的 `DateTime<Local>`，用于日志时间戳和基于日期的日志文件命名。 |
| [LOG_FILE](LOG_FILE.md) | 主日志文件句柄，以追加模式打开在 `logs/YYYYMMDD.log`。 |
| [FIND_LOG_FILE](FIND_LOG_FILE.md) | find 模式日志文件句柄，以追加模式打开在 `logs/YYYYMMDD.find.log`。 |
| [FINDS_FAIL_SET](FINDS_FAIL_SET.md) | 用于失败查找操作的去重集合，防止重复记录相同的失败。 |
| [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) | 每 PID 的 [ApplyFailEntry](ApplyFailEntry.md) 记录映射，用于去重 Windows API 操作错误。 |

## 宏

| 宏 | 描述 |
|----|------|
| [log!](log.md) | 格式化参数并委托给 [log_message](log_message.md)，带时间戳前缀。 |
| [get_use_console!](get_use_console.md) | 返回 [USE_CONSOLE](USE_CONSOLE.md) 标志的 `MutexGuard<bool>`。 |
| [get_dust_bin_mod!](get_dust_bin_mod.md) | 返回 [DUST_BIN_MODE](DUST_BIN_MODE.md) 标志的 `MutexGuard<bool>`。 |
| [get_local_time!](get_local_time.md) | 返回 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 的 `MutexGuard<DateTime<Local>>`。 |
| [get_logger!](get_logger.md) | 返回 [LOG_FILE](LOG_FILE.md) 句柄的 `MutexGuard<File>`。 |
| [get_logger_find!](get_logger_find.md) | 返回 [FIND_LOG_FILE](FIND_LOG_FILE.md) 句柄的 `MutexGuard<File>`。 |
| [get_fail_find_set!](get_fail_find_set.md) | 返回 [FINDS_FAIL_SET](FINDS_FAIL_SET.md) 的 `MutexGuard<HashSet<String>>`。 |
| [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) | 返回 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 的 `MutexGuard<HashMap<u32, HashMap<ApplyFailEntry, bool>>>`。 |

## 枚举

| 枚举 | 描述 |
|------|------|
| [Operation](Operation.md) | 标识规则应用过程中可能失败的每个 Windows API 操作，用作失败去重的键。 |

## 结构体

| 结构体 | 描述 |
|--------|------|
| [ApplyFailEntry](ApplyFailEntry.md) | 用于失败去重的复合键：线程 ID、进程名称、操作和错误代码。 |

## 函数

| 函数 | 描述 |
|------|------|
| [is_new_error](is_new_error.md) | 如果此 PID/操作/错误组合之前未出现过，则返回 `true`，并将其注册以备将来去重。 |
| [purge_fail_map](purge_fail_map.md) | 从失败跟踪映射中移除不再运行的进程的过期条目。 |
| [get_log_path](get_log_path.md) | 构建日期前缀的日志文件路径（`logs/YYYYMMDD<suffix>.log`）。 |
| [log_message](log_message.md) | 将 `[HH:MM:SS]` 时间戳消息写入控制台或日志文件，遵循 dust-bin 模式。 |
| [log_pure_message](log_pure_message.md) | 将不带时间戳前缀的消息写入控制台或日志文件。 |
| [log_to_find](log_to_find.md) | 将带时间戳的消息写入 find 模式日志文件（或控制台）。 |
| [log_process_find](log_process_find.md) | 在 `-find` 模式下记录已发现的进程，通过 [FINDS_SET](FINDS_SET.md) 按会话去重。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 日志消息中使用的错误码转换 | [error_codes 模块](../error_codes.rs/README.md) |
| 生成操作错误的规则应用 | [apply 模块](../apply.rs/README.md) |
| 进程优先级 / I/O 优先级 / 内存优先级枚举 | [priority 模块](../priority.rs/README.md) |
| 服务主循环和 find 模式入口点 | [main 模块](../main.rs/README.md) |
| 控制日志行为的 CLI 标志 | [cli 模块](../cli.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd