# log_message 函数 (logging.rs)

将带时间戳的日志消息写入控制台或主日志文件。每条消息以当前时间的 `[HH:MM:SS]` 格式作为前缀，时间从缓存的 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 中读取。当 [USE_CONSOLE](USE_CONSOLE.md) 为 `true` 时，输出路由到 `stdout`；为 `false` 时，路由到 [LOG_FILE](LOG_FILE.md) 句柄。如果 [DUST_BIN_MODE](DUST_BIN_MODE.md) 处于激活状态，函数会立即返回而不产生任何输出。

## 语法

```logging.rs
pub fn log_message(args: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `args` | `&str` | 要记录的消息文本。此字符串在 `[HH:MM:SS]` 时间戳前缀之后追加在同一行中。`writeln!` 会自动添加尾部换行符。 |

## 返回值

无（`()`）。

## 备注

该函数按以下顺序执行步骤：

1. **Dust-bin 检查：** 通过 [get_dust_bin_mod!](get_dust_bin_mod.md) 获取 [DUST_BIN_MODE](DUST_BIN_MODE.md) 锁。如果标志为 `true`，则立即返回——消息被静默丢弃。这可以防止在 UAC 提升之前进行日志记录。
2. **时间戳格式化：** 通过 [get_local_time!](get_local_time.md) 获取 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 锁，并将缓存的 `DateTime<Local>` 格式化为 `%H:%M:%S`（例如 `14:32:07`）。格式化后的字符串存储在局部 `String` 中。
3. **输出路由：** 通过 [get_use_console!](get_use_console.md) 获取 [USE_CONSOLE](USE_CONSOLE.md) 锁。
   - 如果为 `true`，通过 `writeln!(stdout(), ...)` 将 `[{time_prefix}]{args}\n` 写入 `stdout`。
   - 如果为 `false`，通过 `writeln!(get_logger!(), ...)` 将相同的格式化行写入 [LOG_FILE](LOG_FILE.md) 句柄。

### 输出格式

```/dev/null/example.log#L1-2
[14:32:07]Applied 3 rules to PID 1234
[14:32:07]Set priority class to "high" for notepad.exe
```

时间戳用方括号括起，消息正文之前没有空格。每次调用产生恰好一行输出（以 `\n` 结尾）。

### 错误处理

来自 `writeln!` 的写入错误被静默忽略（`Result` 绑定到 `let _ = …`）。这一设计选择防止了日志目标的故障（例如磁盘已满）导致服务崩溃。即使日志不可用，服务也会继续运行。

### 锁获取顺序

该函数在单次调用中最多获取三个互斥锁，顺序如下：

1. `DUST_BIN_MODE` —— 读取标志后立即释放。
2. `LOCAL_TIME_BUFFER` —— 格式化时间戳字符串后释放。
3. `USE_CONSOLE` 以及 `LOG_FILE`（通过 `get_logger!`）或 `stdout` —— 写入完成后释放。

由于锁是顺序获取的（从不嵌套），因此此函数内部不存在死锁风险。调用方在调用 `log_message` 时应避免持有上述任何锁。

### 与 log! 宏的关系

此函数通常不会被直接调用。大多数调用点使用 [log!](log.md) 宏，该宏通过 `format!()` 格式化其参数并将结果 `&str` 传递给 `log_message`。当调用方已经拥有预格式化的 `&str` 并希望避免额外的 `format!` 分配时，直接调用是有用的。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 调用方 | [log!](log.md) 宏（主要）、[log_error_if_new](../apply.rs/log_error_if_new.md)，以及 crate 中的各种调用点 |
| 被调用方 | [get_dust_bin_mod!](get_dust_bin_mod.md)、[get_local_time!](get_local_time.md)、[get_use_console!](get_use_console.md)、[get_logger!](get_logger.md) |
| 读取 | [DUST_BIN_MODE](DUST_BIN_MODE.md)、[LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md)、[USE_CONSOLE](USE_CONSOLE.md)、[LOG_FILE](LOG_FILE.md) |
| 标准库依赖 | `std::io::Write`、`std::io::stdout` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 便捷日志宏 | [log!](log.md) |
| 无时间戳的日志写入 | [log_pure_message](log_pure_message.md) |
| 查找模式带时间戳日志 | [log_to_find](log_to_find.md) |
| 日志抑制标志 | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| 控制台路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| 主日志文件句柄 | [LOG_FILE](LOG_FILE.md) |
| 缓存时间戳 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd