# get_use_console! 宏 (logging.rs)

便捷宏，获取 [USE_CONSOLE](USE_CONSOLE.md) 静态变量的互斥锁并返回 `MutexGuard<bool>`。这提供了从任何调用点对控制台路由标志的符合人体工程学的、panic-on-poison 访问方式，无需调用方拼写完整的 `lock().unwrap()` 链。

## 语法

```logging.rs
#[macro_export]
macro_rules! get_use_console {
    () => {
        $crate::logging::USE_CONSOLE.lock().unwrap()
    };
}
```

## 返回值

返回 `std::sync::MutexGuard<'static, bool>`。解引用该守卫可读取当前标志值（`true` = 控制台输出，`false` = 文件输出）。守卫也可以进行可变解引用来更改标志，不过在实际使用中该标志仅在启动时设置一次。

## 备注

- 该宏展开为 `$crate::logging::USE_CONSOLE.lock().unwrap()`，这意味着如果互斥锁被污染（即某个线程在持有锁时发生了 panic），它将 **panic**。在 AffinityServiceRust 中这是可以接受的，因为被污染的日志互斥锁表明存在不可恢复的状态。
- 返回的 `MutexGuard` 在其生命周期内持有锁。调用方应避免跨长时间操作持有该守卫，以防止阻塞需要检查控制台标志的其他线程。
- 由于 `#[macro_export]` 将宏放置在 crate 根级别，因此可以在 crate 内的任何模块中以 `get_use_console!()` 调用，无需 `use` 导入。
- 日志函数中的典型使用模式：

```logging.rs
if *get_use_console!() {
    let _ = writeln!(stdout(), "[{}]{}", time_prefix, args);
} else {
    let _ = writeln!(get_logger!(), "[{}]{}", time_prefix, args);
}
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging`（通过 `#[macro_export]` 导出到 crate 根级别） |
| 底层静态变量 | [USE_CONSOLE](USE_CONSOLE.md) |
| 使用者 | [log_message](log_message.md)、[log_pure_message](log_pure_message.md)、[log_to_find](log_to_find.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 控制台路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| dust-bin 模式访问宏 | [get_dust_bin_mod!](get_dust_bin_mod.md) |
| 本地时间访问宏 | [get_local_time!](get_local_time.md) |
| 日志文件访问宏 | [get_logger!](get_logger.md) |
| 查找日志文件访问宏 | [get_logger_find!](get_logger_find.md) |
| logging 模块概述 | [logging 模块](README.md) |