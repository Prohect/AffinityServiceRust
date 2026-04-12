# log_pure_message 函数 (logging.rs)

将日志消息写入日志文件，不附加时间戳前缀。用于多行日志条目的延续行，使输出保持整洁对齐。

## 语法

```rust
pub fn log_pure_message(args: &str)
```

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `args` | `&str` | 要写入日志文件的消息文本。 |

## 返回值

无。

## 备注

`log_pure_message` 是 [`log_message`](log_message.md) 的精简版本，区别在于它不在消息前添加 `[HH:MM:SS]` 时间戳前缀。这在需要输出多行日志条目时非常有用——第一行通过 [`log_message`](log_message.md)（或 `log!` 宏）写入并携带时间戳，而后续的延续行通过 `log_pure_message` 写入，避免每行都重复显示时间戳，从而保持日志的可读性。

### DUST_BIN_MODE 检查

与 [`log_message`](log_message.md) 一样，`log_pure_message` 在执行任何 I/O 操作之前会检查 [`DUST_BIN_MODE`](DUST_BIN_MODE.md)。如果 `DUST_BIN_MODE` 为 `true`，函数立即返回，不写入任何内容。这确保在 UAC 提升前阶段，所有日志输出都被一致地抑制。

### 输出目标

消息仅写入 [`LOG_FILE`](LOG_FILE.md) 日志文件。与 [`log_message`](log_message.md) 不同，`log_pure_message` 不检查 [`USE_CONSOLE`](USE_CONSOLE.md) 标志，因此不会将消息回显到控制台。这是因为延续行通常是上下文信息，在交互式监控中不需要单独显示。

### 线程安全

函数通过 `Mutex` 获取对 [`LOG_FILE`](LOG_FILE.md) 的独占访问，确保多线程环境下写入操作不会交错。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L197–L203 |
| **读取** | [`DUST_BIN_MODE`](DUST_BIN_MODE.md)、[`LOG_FILE`](LOG_FILE.md) |

## 另请参阅

- [log_message 函数](log_message.md)
- [DUST_BIN_MODE 静态变量](DUST_BIN_MODE.md)
- [LOG_FILE 静态变量](LOG_FILE.md)
- [logging.rs 模块概述](README.md)