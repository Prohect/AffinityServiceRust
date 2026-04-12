# log_to_find 函数 (logging.rs)

将消息写入 `.find.log` 发现日志文件，附带时间戳前缀。此函数用于记录进程发现相关的信息到独立的发现日志中。

## 语法

```rust
pub fn log_to_find(msg: &str)
```

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `msg` | `&str` | 要写入发现日志的消息文本。 |

## 返回值

无返回值。

## 备注

`log_to_find` 将带有时间戳前缀的消息写入 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 指向的 `.find.log` 文件。输出格式为：

```
[HH:MM:SS] message
```

时间戳从 [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md) 获取，确保同一循环迭代内的所有发现日志条目共享相同的时间戳。

此函数与 [`log_message`](log_message.md) 的主要区别在于：

- **输出目标不同** — `log_to_find` 写入 `.find.log` 文件，而 `log_message` 写入主日志文件。
- **不受 DUST_BIN_MODE 影响** — 发现日志独立于主日志的抑制控制。
- **不输出到控制台** — 消息仅写入发现日志文件，不会回显到 stdout。
- **时间戳格式不同** — 使用 `[HH:MM:SS]` 格式，而非主日志中的完整时间戳。

### 写入流程

1. 获取 [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md) 锁，读取缓存的当前时间。
2. 格式化时间戳为 `[HH:MM:SS]` 格式。
3. 获取 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 锁，将格式化后的消息写入文件。

### 线程安全

对 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 和 [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md) 的访问均通过 `Mutex` 同步。锁的持有时间仅限于必要的读取和写入操作。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L205–L212 |
| **调用方** | [`log_process_find`](log_process_find.md) |
| **使用的静态变量** | [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md)、[`FIND_LOG_FILE`](FIND_LOG_FILE.md) |

## 另请参阅

- [log_process_find 函数](log_process_find.md)
- [log_message 函数](log_message.md)
- [FIND_LOG_FILE 静态变量](FIND_LOG_FILE.md)
- [LOCAL_TIME_BUFFER 静态变量](LOCAL_TIME_BUFFER.md)
- [logging.rs 模块概述](README.md)