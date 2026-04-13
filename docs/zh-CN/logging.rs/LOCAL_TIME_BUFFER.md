# LOCAL_TIME_BUFFER 静态变量 (logging.rs)

缓存的 `DateTime<Local>` 值，用于日志时间戳和基于日期的日志文件命名。此全局静态变量保存当前本地时间，由主服务循环定期更新。所有日志函数从此缓冲区读取时间，而不是各自独立调用 `Local::now()`，从而确保在单个循环迭代中时间戳的一致性并避免冗余的系统调用。

## 语法

```logging.rs
pub static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。值在首次访问时创建。 |
| 中层 | `Mutex<…>` | 提供内部可变性和线程安全访问，支持从主线程和任何日志调用点访问。 |
| 内层 | `DateTime<Local>` | 一个 `chrono::DateTime<Local>`，表示缓存的本地时间戳。首次访问时初始化为 `Local::now()`。 |

## 备注

- 主服务循环在每次迭代开始时更新此缓冲区，以使同一趟处理中输出的所有日志消息共享相同的时间戳。这避免了在同一逻辑操作中毫秒级差异产生的消息显示不同秒数的视觉不一致性。
- 缓存的时间也被 [get_log_path](get_log_path.md) 用于派生日志文件名的日期部分（`YYYYMMDD`）。由于缓冲区仅在循环顶部更新，因此即使循环跨越午夜，日志文件也不会在迭代中途被意外轮转。
- 访问通过 `Mutex` 保护。便捷宏 [get_local_time!](get_local_time.md) 获取此锁并返回 `MutexGuard<DateTime<Local>>` 以便于使用：

```logging.rs
#[macro_export]
macro_rules! get_local_time {
    () => {
        $crate::logging::LOCAL_TIME_BUFFER.lock().unwrap()
    };
}
```

- [log_message](log_message.md)、[log_to_find](log_to_find.md) 和 [get_log_path](get_log_path.md) 都通过 `get_local_time!` 宏读取此缓冲区。日志行前缀使用的格式为 `%H:%M:%S`（例如 `[14:32:07]`），而 `get_log_path` 提取年、月、日组件用于文件命名。
- 初始值（首次访问时的 `Local::now()`）仅对第一个日志文件的创建有意义。服务循环开始后，缓冲区会在每次迭代中被覆盖。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell` (`Lazy`)、`chrono` (`DateTime`、`Local`)、`std::sync::Mutex` |
| 更新者 | [main 模块](../main.rs/README.md) 中的主服务循环 |
| 读取者 | [log_message](log_message.md)、[log_to_find](log_to_find.md)、[get_log_path](get_log_path.md) |
| 宏访问器 | [get_local_time!](get_local_time.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主日志文件句柄 | [LOG_FILE](LOG_FILE.md) |
| 查找模式日志文件句柄 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 日志文件路径构建 | [get_log_path](get_log_path.md) |
| 带时间戳的日志写入 | [log_message](log_message.md) |
| 控制台与文件路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| logging 模块概述 | [logging 模块](README.md) |