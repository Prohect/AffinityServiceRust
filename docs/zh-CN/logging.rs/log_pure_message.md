# log_pure_message 函数 (logging.rs)

将消息写入控制台或主日志文件，**不带**时间戳前缀。与 [log_message](log_message.md) 在每一行前添加 `[HH:MM:SS]` 时间戳不同，`log_pure_message` 原样输出消息字符串。此函数用于续行、横幅以及其他添加时间戳会显得多余或影响视觉效果的输出场景。

## 语法

```logging.rs
pub fn log_pure_message(args: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `args` | `&str` | 要写入的消息字符串。通过 `writeln!` 原样写入并在末尾追加换行符。 |

## 返回值

无 (`()`)。

## 备注

- 输出目标由 [USE_CONSOLE](USE_CONSOLE.md) 标志决定：
  - 当为 `true` 时，消息通过 `writeln!(stdout(), "{}", args)` 写入 `stdout`。
  - 当为 `false` 时，消息通过 `writeln!(get_logger!(), "{}", args)` 写入主日志文件 [LOG_FILE](LOG_FILE.md)。
- 与 [log_message](log_message.md) 不同，此函数**不会**检查 [DUST_BIN_MODE](DUST_BIN_MODE.md)。传递给 `log_pure_message` 的消息始终会被输出，即使在提升前阶段正常日志被抑制时也是如此。需要抑制语义的调用者应自行检查 `DUST_BIN_MODE`，或者改用 [log_message](log_message.md) / [log!](log.md) 宏。
- `writeln!` 的写入错误会被静默忽略（`Result` 被绑定到 `let _ = …`）。这防止了 I/O 故障——如磁盘已满或管道断开——将 panic 传播到服务循环中。
- 该函数在单次调用中依次获取两个互斥锁：首先是 [USE_CONSOLE](USE_CONSOLE.md)（通过 [get_use_console!](get_use_console.md)），然后是 `stdout` 或 [LOG_FILE](LOG_FILE.md)（通过 [get_logger!](get_logger.md)）。`USE_CONSOLE` 守卫在文件写入之前就被释放，因为 `if *get_use_console!()` 临时值在分支体执行前已经被求值并释放。

### 典型使用场景

- **启动横幅：** 多行服务标识输出，其中仅第一行携带时间戳。
- **续行输出：** 由 [log_message](log_message.md) 产生的带时间戳头部行之后的补充细节行。
- **结构化输出块：** 配置转储、规则列表或其他格式化的块，每行添加时间戳会损害可读性。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 调用者 | [main](../main.rs/README.md)、[apply 模块](../apply.rs/README.md) |
| 被调用者 | [get_use_console!](get_use_console.md)、[get_logger!](get_logger.md) |
| 读取 | [USE_CONSOLE](USE_CONSOLE.md)、[LOG_FILE](LOG_FILE.md) |
| **不**读取 | [DUST_BIN_MODE](DUST_BIN_MODE.md)、[LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 带时间戳的日志输出 | [log_message](log_message.md) |
| 便捷日志宏（带时间戳） | [log!](log.md) |
| 查找模式日志输出 | [log_to_find](log_to_find.md) |
| 控制台与文件路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| 日志抑制标志（此处不检查） | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| 主日志文件句柄 | [LOG_FILE](LOG_FILE.md) |
| logging 模块概述 | [logging 模块](README.md) |