# LOCAL_TIME_BUFFER 静态变量 (logging.rs)

一致时间显示的共享时间戳缓冲区。存储当前本地时间，确保同一循环迭代内的所有日志条目共享相同的时间戳，而非各自独立获取时间。

## 语法

```rust
static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `chrono::DateTime<Local>` 值。初始化时设置为 `Local::now()`，之后由主循环在每次迭代开始时更新。

## 备注

`LOCAL_TIME_BUFFER` 的设计目的是保证同一循环迭代中产生的所有日志条目具有一致的时间戳。如果每条日志消息都独立调用 `Local::now()` 获取当前时间，那么同一批处理中的日志条目将显示略有不同的时间，这会使日志分析变得困难。

### 更新机制

[`main`](../main.rs/main.md) 中的主循环在每次迭代开始时将 `LOCAL_TIME_BUFFER` 更新为当前本地时间。在该迭代的整个处理过程中，所有通过 [`log_message`](log_message.md) 写入的日志条目都使用此缓存的时间戳，而非重新获取系统时间。

### 时间戳格式

[`log_message`](log_message.md) 从 `LOCAL_TIME_BUFFER` 读取时间并格式化为 `[HH:MM:SS]` 前缀，附加在每条日志消息之前。由于同一循环中的所有消息共享相同的 `DateTime<Local>` 值，它们将显示完全相同的时间戳。

### 线程安全

对 `DateTime<Local>` 值的访问通过 `Mutex` 同步。主循环写入时短暂获取锁更新值，日志函数读取时同样短暂获取锁获取当前缓存的时间。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L64 |
| **设置方** | [`main`](../main.rs/main.md)（每次循环迭代更新） |
| **读取方** | [`log_message`](log_message.md)、[`log_to_find`](log_to_find.md)、[`log_process_find`](log_process_find.md) |

## 另请参阅

- [log_message 函数](log_message.md)
- [DUST_BIN_MODE 静态变量](DUST_BIN_MODE.md)
- [logging.rs 模块概述](README.md)