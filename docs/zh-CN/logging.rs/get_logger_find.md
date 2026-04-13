# get_logger_find! 宏 (logging.rs)

便捷宏，锁定 [FIND_LOG_FILE](FIND_LOG_FILE.md) 静态变量的互斥锁并返回 `MutexGuard<File>`。这提供了对查找模式日志文件句柄的符合人体工程学的、短生命周期的访问，用于写入查找模式日志条目。

## 语法

```logging.rs
#[macro_export]
macro_rules! get_logger_find {
    () => {
        $crate::logging::FIND_LOG_FILE.lock().unwrap()
    };
}
```

## 返回值

返回 `std::sync::MutexGuard<'static, std::fs::File>` —— 一个 RAII 守卫，解引用为底层 `File` 句柄。锁在守卫被丢弃时释放（通常在包含语句或代码块的末尾）。

## 备注

- 此宏是 `FIND_LOG_FILE.lock().unwrap()` 的简单封装。它对锁的结果调用 `unwrap()`，这意味着如果互斥锁被污染（即先前的持有者在持锁期间发生了 panic），将会导致 panic。在实际使用中，这不会发生，因为日志代码路径不会 panic。
- 返回的 `MutexGuard<File>` 实现了 `DerefMut` 到 `File`，因此调用者可以直接将其与 `writeln!` 和其他 I/O 写入宏一起使用。
- 该宏标注了 `#[macro_export]`，使其在整个 crate 中可以通过 `crate::get_logger_find!()` 使用，无需 `use` 导入 `logging` 模块。
- [log_to_find](log_to_find.md) 是此宏的主要消费者。当 [USE_CONSOLE](USE_CONSOLE.md) 为 `false` 时，它将带时间戳的消息写入查找模式日志文件。
- 锁的作用域应尽可能短，以避免在耗时操作或其他可能导致竞争或死锁的锁获取期间持有互斥锁。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 封装对象 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 使用者 | [log_to_find](log_to_find.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 查找模式日志文件静态变量 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 主日志文件访问宏 | [get_logger!](get_logger.md) |
| 查找模式日志写入函数 | [log_to_find](log_to_find.md) |
| 控制台与文件路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| logging 模块概述 | [logging 模块](README.md) |