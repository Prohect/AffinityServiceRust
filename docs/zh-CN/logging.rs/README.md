# logging 模块 (AffinityServiceRust)

`logging` 模块为 AffinityServiceRust 提供了日志基础设施，包括基于日期的日志轮转、可选的控制台输出、查找模式发现日志和错误去重。所有日志输出都由一个禁用模式标志（在 UAC 提升期间抑制日志记录）和一个控制台标志（将输出重定向到 stdout 而不是文件）控制。日志文件写入 `logs/` 目录，文件名格式为 `YYYYMMDD.log` 和 `YYYYMMDD.find.log`。

## 静态变量

| 名称 | 类型 | 描述 |
|------|------|-------------|
| `FINDS_SET` | `Lazy<Mutex<HashSet<String>>>` | 查找模式进程发现的去重集合。每个进程名在每次会话中最多记录一次。 |
| `USE_CONSOLE` | `Lazy<Mutex<bool>>` | 当为 `true` 时，所有日志输出写入 stdout 而不是日志文件。用于 CLI/交互模式。 |
| `DUST_BIN_MODE` | `Lazy<Mutex<bool>>` | 当为 `true` 时，抑制所有日志记录。在 UAC 提升期间启用，以避免从临时提升进程写入文件。 |
| `LOCAL_TIME_BUFFER` | `Lazy<Mutex<DateTime<Local>>>` | 缓存的本地时间戳，由外部每次 tick 更新。用于日志前缀和基于日期的日志轮转。 |
| `LOG_FILE` | `Lazy<Mutex<File>>` | 主日志文件句柄，以追加模式打开，路径为 `logs/YYYYMMDD.log`。 |
| `FIND_LOG_FILE` | `Lazy<Mutex<File>>` | 查找模式日志文件句柄，以追加模式打开，路径为 `logs/YYYYMMDD.find.log`。 |
| `FINDS_FAIL_SET` | `Lazy<Mutex<HashSet<String>>>` | 查找模式错误的去重跟踪。 |
| `PID_MAP_FAIL_ENTRY_SET` | `Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>` | 每个 PID 的 [ApplyFailEntry](ApplyFailEntry.md) 到存活标志的映射。由 [is_new_error](is_new_error.md) 和 [purge_fail_map](purge_fail_map.md) 用于错误去重。 |

## 宏

| 名称 | 描述 |
|------|------|
| `log!()` | 格式化参数并委托给 [log_message](log_message.md)。用法：`log!("value: {}", x)`。 |
| `get_use_console!()` | 锁定并返回 `USE_CONSOLE` 互斥锁保护对象。 |
| `get_dust_bin_mod!()` | 锁定并返回 `DUST_BIN_MODE` 互斥锁保护对象。 |
| `get_local_time!()` | 锁定并返回 `LOCAL_TIME_BUFFER` 互斥锁保护对象。 |
| `get_logger!()` | 锁定并返回 `LOG_FILE` 互斥锁保护对象。 |
| `get_logger_find!()` | 锁定并返回 `FIND_LOG_FILE` 互斥锁保护对象。 |
| `get_fail_find_set!()` | 锁定并返回 `FINDS_FAIL_SET` 互斥锁保护对象。 |
| `get_pid_map_fail_entry_set!()` | 锁定并返回 `PID_MAP_FAIL_ENTRY_SET` 互斥锁保护对象。 |

## 枚举

| 名称 | 描述 |
|------|------|
| [Operation](Operation.md) | 枚举服务执行的所有 Windows API 操作，用作错误去重的键。 |

## 结构体

| 名称 | 描述 |
|------|------|
| [ApplyFailEntry](ApplyFailEntry.md) | 表示唯一失败的复合键：线程 ID、进程名、操作和错误代码。 |

## 函数

| 名称 | 描述 |
|------|------|
| [is_new_error](is_new_error.md) | 如果给定 PID 的失败组合之前未见过，返回 `true`。跟踪每个 PID 的错误历史。 |
| [purge_fail_map](purge_fail_map.md) | 从错误去重映射中删除过期条目，仅保留当前运行的进程。 |
| [get_log_path](get_log_path.md) | 构建 `logs/` 目录下的日期戳日志文件路径。 |
| [log_message](log_message.md) | 将带有时间戳的 `[HH:MM:SS]msg` 行写入日志文件或控制台。 |
| [log_pure_message](log_pure_message.md) | 不带时间戳前缀将行写入日志文件或控制台。 |
| [log_to_find](log_to_find.md) | 将带有时间戳的行写入查找模式日志文件或控制台。 |
| [log_process_find](log_process_find.md) | 在查找模式下记录发现的进程名，每会话去重。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 应用模块（主要消费者） | [apply.rs](../apply.rs/README.md) |
| 错误日志辅助函数 | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| 配置类型 | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) |
| 主服务循环 | [main.rs](../main.rs/README.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*