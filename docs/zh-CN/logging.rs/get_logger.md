# get_logger! 宏 (logging.rs)

便捷宏，获取 [LOG_FILE](LOG_FILE.md) 静态变量上的互斥锁并返回 `MutexGuard<File>`。这提供了对主日志文件句柄的符合人体工程学的、受锁保护的写入访问，无需调用方拼写完整的 `LOG_FILE.lock().unwrap()` 表达式。

## 语法

```logging.rs
#[macro_export]
macro_rules! get_logger {
    () => {
        $crate::logging::LOG_FILE.lock().unwrap()
    };
}
```

## 返回值

返回 `std::sync::MutexGuard<'static, std::fs::File>`。该守卫在其生命周期内持有互斥锁。当守卫被丢弃时（通常在包含语句或块的末尾），锁被释放。

返回的守卫解引用为 `&File`（或 `&mut File`），因此可以直接与 `writeln!` 及其他 I/O 操作配合使用。

## 备注

- 该宏对 `lock()` 的结果调用 `.unwrap()`。如果互斥锁被污染（之前的持有者在持有锁时发生了 panic），这将导致 panic。在实践中，日志记录函数不会 panic，因此不预期会发生互斥锁污染。
- 锁仅在返回的 `MutexGuard` 的生命周期内被持有。调用方应避免在长时间运行的操作中或在获取其他日志互斥锁的调用之间持有该守卫，以防止死锁或不必要的竞争。
- 此宏标记有 `#[macro_export]`，将其放置在 crate 根级别。在 crate 内的任何模块中以 `get_logger!()` 调用。
- 该宏在内部由 [log_message](log_message.md) 和 [log_pure_message](log_pure_message.md) 使用，当 [USE_CONSOLE](USE_CONSOLE.md) 为 `false` 时将格式化输出写入主日志文件。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 展开为 | `$crate::logging::LOG_FILE.lock().unwrap()` |
| 依赖 | [LOG_FILE](LOG_FILE.md) |
| 使用者 | [log_message](log_message.md)、[log_pure_message](log_pure_message.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主日志文件句柄 | [LOG_FILE](LOG_FILE.md) |
| 查找模式日志文件访问宏 | [get_logger_find!](get_logger_find.md) |
| 带时间戳的日志写入函数 | [log_message](log_message.md) |
| 无时间戳的日志写入函数 | [log_pure_message](log_pure_message.md) |
| 控制台与文件路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd