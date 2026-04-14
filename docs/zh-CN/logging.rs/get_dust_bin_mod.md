# get_dust_bin_mod! 宏 (logging.rs)

便捷宏，用于锁定 [DUST_BIN_MODE](DUST_BIN_MODE.md) 互斥锁并返回 `MutexGuard<bool>`。这提供了对控制日志输出是否被抑制（例如在 UAC 提升之前）的全局标志的符合人体工程学的、受保护的访问。

## 语法

```logging.rs
#[macro_export]
macro_rules! get_dust_bin_mod {
    () => {
        $crate::logging::DUST_BIN_MODE.lock().unwrap()
    };
}
```

## 返回值

返回 `MutexGuard<'static, bool>`。解引用该守卫可读取当前值（`true` = 日志被抑制，`false` = 日志已启用）。可变解引用可更改该值：

```/dev/null/example.rs#L1-2
*get_dust_bin_mod!() = true;  // 抑制日志
*get_dust_bin_mod!() = false; // 重新启用日志
```

## 备注

- 该宏对 `Mutex::lock()` 的结果调用 `.unwrap()`。如果互斥锁被中毒（某个线程在持有锁时发生了 panic），这将导致 panic。在实践中，锁仅在单次读取或赋值期间被持有，因此中毒的可能性极低。
- 返回的 `MutexGuard` 在被丢弃之前会一直持有锁。调用者应避免在长时间运行的操作中或在获取其他日志相关互斥锁的调用之间持有该守卫，以防止死锁。
- 此宏通过 `#[macro_export]` 导出，使其在整个 crate 中可以通过 `get_dust_bin_mod!()` 使用，无需模块路径前缀。
- 主要消费者是 [log_message](log_message.md)，它在入口处检查 `*get_dust_bin_mod!()`，如果值为 `true` 则立即返回。主模块在由 `--skip-log-before-elevation` CLI 标志控制的提升前阶段写入此标志。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 后备静态变量 | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Crate 依赖 | `std::sync::Mutex`（通过 `DUST_BIN_MODE`） |
| 使用者 | [log_message](log_message.md)、[main](../main.rs/README.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 此宏的后备静态变量 | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| 控制台标志访问宏 | [get_use_console!](get_use_console.md) |
| 本地时间访问宏 | [get_local_time!](get_local_time.md) |
| 检查此标志的带时间戳日志函数 | [log_message](log_message.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd