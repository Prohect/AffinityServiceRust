# log_to_find 函数 (logging.rs)

将带时间戳的消息写入查找模式日志文件（或者在 [USE_CONSOLE](USE_CONSOLE.md) 激活时写入控制台）。此函数是 `-find` 模式诊断的主要输出机制，由 [log_process_find](log_process_find.md) 调用，也可从查找模式处理循环中直接调用。与 [log_message](log_message.md) 不同，此函数**不会**检查 [DUST_BIN_MODE](DUST_BIN_MODE.md)——查找模式的输出永远不会被抑制。

## 语法

```logging.rs
pub fn log_to_find(msg: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `msg` | `&str` | 要写入的消息字符串。函数在写入之前会添加 `[HH:MM:SS]` 时间戳前缀。 |

## 返回值

无（`()`）。I/O 错误会被静默忽略——函数使用 `let _ = writeln!(…)` 来丢弃任何写入失败。

## 备注

函数的行为取决于 [USE_CONSOLE](USE_CONSOLE.md) 标志：

- **控制台模式（`true`）：** 带时间戳的消息通过 `writeln!(stdout(), "[{}]{}", time_prefix, msg)` 写入 `stdout`。
- **文件模式（`false`）：** 带时间戳的消息通过 `writeln!(get_logger_find!(), "[{}]{}", time_prefix, msg)` 写入 [FIND_LOG_FILE](FIND_LOG_FILE.md) 句柄。

### 时间戳来源

时间戳前缀从缓存的 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 值派生，格式化为 `%H:%M:%S`（例如 `14:32:07`）。主服务循环在每次迭代开始时更新此缓冲区，因此同一趟处理中的所有查找模式日志条目共享相同的时间戳。

### 与 log_message 的区别

| 方面 | `log_to_find` | [log_message](log_message.md) |
|------|---------------|-------------------------------|
| 输出目标 | [FIND_LOG_FILE](FIND_LOG_FILE.md)（`logs/YYYYMMDD.find.log`） | [LOG_FILE](LOG_FILE.md)（`logs/YYYYMMDD.log`） |
| Dust-bin 抑制 | 否——始终写入 | 是——当 [DUST_BIN_MODE](DUST_BIN_MODE.md) 为 `true` 时被抑制 |
| 用途 | 查找模式进程发现诊断 | 通用服务诊断 |

### 输出格式

类似 `log_to_find("find notepad.exe")` 的调用会产生如下日志行：

```/dev/null/example.log#L1-1
[14:32:07]find notepad.exe
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 调用者 | [log_process_find](log_process_find.md)、[process_find](../main.rs/README.md) |
| 被调用者 | [get_local_time!](get_local_time.md)、[get_use_console!](get_use_console.md)、[get_logger_find!](get_logger_find.md) |
| 读取 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md)、[USE_CONSOLE](USE_CONSOLE.md)、[FIND_LOG_FILE](FIND_LOG_FILE.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 去重的进程发现日志记录 | [log_process_find](log_process_find.md) |
| 主日志写入函数 | [log_message](log_message.md) |
| 无时间戳的日志写入 | [log_pure_message](log_pure_message.md) |
| 查找模式日志文件句柄 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 控制台与文件路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| 用于日志前缀的缓存时间戳 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd