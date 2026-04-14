# get_fail_find_set! 宏 (logging.rs)

便捷宏，锁定 [FINDS_FAIL_SET](FINDS_FAIL_SET.md) 互斥锁并返回 `MutexGuard<HashSet<String>>`。此宏消除了获取全局查找失败去重集合锁时的样板代码，提供了在整个代码库中使用的简洁一致的访问器。

## 语法

```logging.rs
#[macro_export]
macro_rules! get_fail_find_set {
    () => {
        $crate::logging::FINDS_FAIL_SET.lock().unwrap()
    };
}
```

## 返回值

返回 `std::sync::MutexGuard<'static, HashSet<String>>` —— 一个 RAII 守卫，解引用为底层 `HashSet<String>`，并在被丢弃时释放锁。

## 备注

- 该宏对 `lock()` 的结果调用 `.unwrap()`。如果互斥锁被污染（即某个线程在持有锁时发生了 panic），该宏将 panic。在 AffinityServiceRust 中，这被视为不可恢复的状态——被污染的互斥锁表明存在编程错误或灾难性故障，此时崩溃比静默数据损坏更为合适。
- 返回的守卫在其整个生命周期内持有互斥锁。调用方应尽量缩小守卫的作用域，以避免阻塞其他线程。在实际使用中，查找失败检查频率较低且为单线程操作，因此竞争可以忽略不计。
- 此宏访问的是 [FINDS_FAIL_SET](FINDS_FAIL_SET.md)，**而非** [FINDS_SET](FINDS_SET.md)。尽管命名相似，`FINDS_SET` 跟踪成功的查找，而 `FINDS_FAIL_SET` 跟踪失败的查找。在 [log_process_find](log_process_find.md) 中直接使用 `.lock().unwrap()` 访问 `FINDS_SET`。
- `#[macro_export]` 属性将此宏放置在 crate 根级别，因此调用方使用 `crate::get_fail_find_set!()` 而非 `crate::logging::get_fail_find_set!()` 来导入。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging`（通过 `#[macro_export]` 导出到 crate 根级别） |
| 底层静态变量 | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| 调用方 | [process_find](../main.rs/README.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 失败查找去重的被守卫静态变量 | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| 成功查找去重集合 | [FINDS_SET](FINDS_SET.md) |
| 查找模式进程日志记录 | [log_process_find](log_process_find.md) |
| 其他访问器宏 | [get_use_console!](get_use_console.md)、[get_dust_bin_mod!](get_dust_bin_mod.md)、[get_local_time!](get_local_time.md)、[get_logger!](get_logger.md)、[get_logger_find!](get_logger_find.md)、[get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd