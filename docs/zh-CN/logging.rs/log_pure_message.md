# log_pure_message 函数 (logging.rs)

将消息写入主日志文件或标准输出，**不带**时间戳前缀。用于续行、横幅或结构化输出等场景，在这些场景中 [`log_message`](log_message.md) 添加的 `[HH:MM:SS]` 前缀是不合适的。

## 语法

```rust
pub fn log_pure_message(args: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `args` | `&str` | 要写入的消息字符串。通过 `writeln!` 自动追加尾部换行符。 |

## 返回值

此函数不返回值。来自 `writeln!` 的写入错误会被静默忽略。

## 备注

- 与 [`log_message`](log_message.md) 不同，此函数**不会**检查 [`DUST_BIN_MODE`](statics.md#dust_bin_mode) 标志。通过 `log_pure_message` 发送的消息始终会被写入，不受丢弃模式的影响。

- 输出目标由 [`USE_CONSOLE`](statics.md#use_console) 标志决定：
  - 当为 `true` 时，消息通过 `writeln!(stdout(), ...)` 写入 `stdout`。
  - 当为 `false` 时，消息通过 `writeln!(get_logger!(), ...)` 写入主日志文件。

- 不会添加时间戳前缀——原始 `args` 字符串直接写入。这是与 [`log_message`](log_message.md) 的关键区别，后者会从 [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer) 格式化当前时间作为 `[HH:MM:SS]` 前缀。

- 写入错误（例如管道断开、磁盘已满）通过 `let _ = writeln!(...)` 丢弃。此函数不会将 I/O 错误传播给调用者。

### 与其他日志函数的比较

| 函数 | 时间戳 | 丢弃模式检查 | 输出目标 |
|------|--------|-------------|----------|
| [`log_message`](log_message.md) | `[HH:MM:SS]` 前缀 | 是 — 当 `DUST_BIN_MODE` 为 `true` 时跳过 | 主日志 / stdout |
| **log_pure_message** | 无 | 否 — 始终写入 | 主日志 / stdout |
| [`log_to_find`](log_to_find.md) | `[HH:MM:SS]` 前缀 | 否 — 始终写入 | find 日志 / stdout |

### 锁定行为

此函数每次调用最多获取两个互斥锁：

1. `USE_CONSOLE` — 检查控制台模式标志。
2. `LOG_FILE`（通过 `get_logger!()`）或 `stdout`（无需额外锁）。

调用者应避免在调用此函数时持有其他日志相关的锁，以防止潜在的死锁。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **调用方** | `main.rs`、`scheduler.rs` — 用于不应带时间戳的横幅行和结构化输出。 |
| **被调用方** | `get_use_console!()` 宏、`get_logger!()` 宏、`std::io::stdout`、`writeln!` |
| **读取的静态变量** | [`USE_CONSOLE`](statics.md#use_console)、[`LOG_FILE`](statics.md#log_file) |
| **平台** | Windows（日志文件路径采用 Windows 目录约定） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| log_message 函数 | [log_message](log_message.md) |
| log_to_find 函数 | [log_to_find](log_to_find.md) |
| log_process_find 函数 | [log_process_find](log_process_find.md) |
| 日志静态变量 | [statics](statics.md) |
| logging 模块概述 | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
