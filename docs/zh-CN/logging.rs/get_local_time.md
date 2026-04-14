# get_local_time! 宏 (logging.rs)

便捷宏，获取 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 上的互斥锁并返回 `MutexGuard<DateTime<Local>>`。这提供了对用于日志前缀和基于日期的日志文件命名的缓存本地时间戳的符合人体工程学的、线程安全的读写访问。

## 语法

```logging.rs
#[macro_export]
macro_rules! get_local_time {
    () => {
        $crate::logging::LOCAL_TIME_BUFFER.lock().unwrap()
    };
}
```

## 返回值

返回 `std::sync::MutexGuard<'static, DateTime<Local>>`。该守卫解引用为 `DateTime<Local>` 以读取缓存的时间戳。调用者也可以进行可变解引用（`*get_local_time!() = Local::now()`）来更新缓存时间。互斥锁在守卫被丢弃时释放。

## 备注

- 此宏通过 `#[macro_export]` 导出，使其在整个 crate 中可以通过 `get_local_time!()` 使用，无需 `use` 导入宏本身。它在内部通过完全限定路径 `$crate::logging::LOCAL_TIME_BUFFER` 引用静态变量。
- 主服务循环通过此宏在每次迭代开始时更新缓存时间，赋值一个新的 `Local::now()`。同一迭代内的所有后续日志调用随后读取相同的时间戳，确保相关日志行之间的 `[HH:MM:SS]` 前缀保持一致。
- 该宏对 `Mutex::lock()` 的结果调用 `.unwrap()`。如果互斥锁被污染（先前的持有者在持锁时发生了 panic），将导致 panic。在实际使用中，持有此锁的代码路径在正常条件下不会 panic。
- 调用者应注意锁的持有时间。在长时间操作期间持有返回的 `MutexGuard` 会阻塞其他线程读取或更新时间缓冲区。建议将守卫绑定到短生命周期的变量，或在时间值被消费后显式丢弃它。

### 使用模式

**读取缓存时间用于日志前缀：**

```logging.rs
let time_prefix = get_local_time!().format("%H:%M:%S").to_string();
```

**在服务循环顶部更新缓存时间：**

```logging.rs
*get_local_time!() = Local::now();
```

**读取日期组件用于日志文件命名（如 [get_log_path](get_log_path.md) 中所做）：**

```logging.rs
let time = get_local_time!();
let (year, month, day) = (time.year(), time.month(), time.day());
drop(time); // 在文件 I/O 之前释放锁
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `chrono`（`DateTime`、`Local`）、`std::sync::Mutex` |
| 底层静态变量 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| 调用者 | [log_message](log_message.md)、[log_to_find](log_to_find.md)、[get_log_path](get_log_path.md)、[main](../main.rs/README.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 缓存时间戳静态变量 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| 带时间戳的日志输出 | [log_message](log_message.md) |
| 日志文件路径构建 | [get_log_path](get_log_path.md) |
| 其他访问器宏 | [get_use_console!](get_use_console.md)、[get_dust_bin_mod!](get_dust_bin_mod.md)、[get_logger!](get_logger.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd