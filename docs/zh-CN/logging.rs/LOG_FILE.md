# LOG_FILE 静态变量 (logging.rs)

常规日志文件的共享句柄。所有通过 [`log_message`](log_message.md) 和 [`log_pure_message`](log_pure_message.md) 写入的日志消息都输出到此文件。

## 语法

```rust
static LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(File::create(get_log_path("")).unwrap()));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `std::fs::File` 句柄，指向当天的常规日志文件。

- 文件路径格式：`logs/YYYYMMDD.log`
- 路径由 [`get_log_path`](get_log_path.md) 生成，传入空字符串后缀。
- 如果 `logs/` 目录不存在，[`get_log_path`](get_log_path.md) 会自动创建。

## 备注

`LOG_FILE` 在首次访问时通过 `once_cell::sync::Lazy` 延迟初始化。初始化过程调用 [`get_log_path("")`](get_log_path.md) 获取基于当前日期的文件路径，然后创建（或截断）该文件。

日志文件名使用本地日期，格式为 `YYYYMMDD.log`。例如，2024 年 3 月 15 日的日志文件为 `logs/20240315.log`。该文件位于可执行文件所在目录下的 `logs/` 子目录中。

### 写入流程

1. [`log_message`](log_message.md) 获取 `LOG_FILE` 的 `Mutex` 锁。
2. 使用 [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md) 中缓存的时间戳格式化消息。
3. 将带时间戳的消息写入文件。
4. 释放锁。

[`log_pure_message`](log_pure_message.md) 执行相同的流程，但不添加时间戳前缀。

### 与 FIND_LOG_FILE 的区别

`LOG_FILE` 是主日志文件，记录应用程序的所有常规日志消息（配置应用结果、错误、状态信息等）。而 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 是专用于 `-find` 模式的独立日志文件，仅记录进程发现信息。

### 线程安全

所有对文件句柄的访问都通过 `Mutex` 同步，确保多处日志调用不会交错写入。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L65 |
| **初始化依赖** | [`get_log_path`](get_log_path.md) |
| **使用方** | [`log_message`](log_message.md)、[`log_pure_message`](log_pure_message.md) |

## 另请参阅

- [FIND_LOG_FILE 静态变量](FIND_LOG_FILE.md)
- [get_log_path 函数](get_log_path.md)
- [log_message 函数](log_message.md)
- [log_pure_message 函数](log_pure_message.md)
- [DUST_BIN_MODE 静态变量](DUST_BIN_MODE.md)
- [logging.rs 模块概述](README.md)