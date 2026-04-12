# log_message 函数 (logging.rs)

主日志函数，将带时间戳的消息写入日志文件，并可选地输出到控制台。整个代码库通过 `log!` 宏调用此函数。

## 语法

```rust
pub fn log_message(args: &str)
```

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `args` | `&str` | 要记录的格式化消息字符串。通常由 `log!` 宏通过 `format_args!` 生成。 |

## 返回值

无返回值。

## 备注

`log_message` 是应用程序的核心日志函数，`log!` 宏是对它的简单封装。每次调用时，函数执行以下步骤：

1. **检查 DUST_BIN_MODE** — 如果 [`DUST_BIN_MODE`](DUST_BIN_MODE.md) 为 `true`，函数立即返回，不产生任何输出。这用于在 UAC 提升前抑制日志，避免写入即将被废弃的日志文件。
2. **获取时间戳** — 从 [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md) 读取缓存的本地时间。该时间戳由 [`main`](../main.rs/main.md) 在每次循环迭代开始时更新，确保同一循环内的所有日志条目共享相同的时间戳。
3. **格式化消息** — 将时间戳与消息内容组合为 `[HH:MM:SS] message` 格式。
4. **写入日志文件** — 将格式化后的消息写入 [`LOG_FILE`](LOG_FILE.md) 句柄指向的日志文件。
5. **控制台输出** — 如果 [`USE_CONSOLE`](USE_CONSOLE.md) 为 `true`，同时通过 `println!` 将消息输出到 stdout。

### 与 log! 宏的关系

整个代码库使用 `log!` 宏而非直接调用 `log_message`。`log!` 宏接受与 `println!` 相同的格式化参数，内部通过 `format!` 将参数转换为字符串后传递给 `log_message`。

### 输出格式

日志文件中的每一行格式如下：

```
[HH:MM:SS] 消息内容
```

时间戳使用 24 小时制，精确到秒。

### 线程安全

函数内部访问多个 `Mutex` 保护的静态变量（`DUST_BIN_MODE`、`LOCAL_TIME_BUFFER`、`LOG_FILE`、`USE_CONSOLE`）。每个锁被短暂持有后立即释放，以最小化竞争。

### 与其他日志函数的区别

| 函数 | 时间戳 | 目标文件 | 用途 |
| --- | --- | --- | --- |
| **log_message** | ✓ | `.log` | 主日志输出 |
| [`log_pure_message`](log_pure_message.md) | ✗ | `.log` | 多行日志延续行 |
| [`log_to_find`](log_to_find.md) | ✓ | `.find.log` | 发现模式日志 |
| [`log_process_find`](log_process_find.md) | ✓ | `.find.log` | 发现模式进程记录（去重） |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L185–L195 |
| **调用方** | 所有模块通过 `log!` 宏 |
| **依赖** | [`DUST_BIN_MODE`](DUST_BIN_MODE.md)、[`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md)、[`LOG_FILE`](LOG_FILE.md)、[`USE_CONSOLE`](USE_CONSOLE.md) |

## 另请参阅

- [log_pure_message 函数](log_pure_message.md)
- [log_to_find 函数](log_to_find.md)
- [DUST_BIN_MODE 静态变量](DUST_BIN_MODE.md)
- [LOCAL_TIME_BUFFER 静态变量](LOCAL_TIME_BUFFER.md)
- [USE_CONSOLE 静态变量](USE_CONSOLE.md)
- [LOG_FILE 静态变量](LOG_FILE.md)
- [logging.rs 模块概述](README.md)