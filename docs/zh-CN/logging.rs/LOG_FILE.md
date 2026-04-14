# LOG_FILE 静态变量 (logging.rs)

全局互斥锁保护的主日志文件句柄。所有由 [log_message](log_message.md) 产生的带时间戳的日志输出以及 [log_pure_message](log_pure_message.md) 产生的无时间戳输出，在控制台模式未激活时都写入此文件。该文件在首次访问时以追加模式打开，如果 `logs/` 目录和文件本身不存在，则会自动创建。

## 语法

```logging.rs
pub static LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。文件在首次访问时打开。 |
| 内层 | `Mutex<File>` | 提供来自任何线程的同步写入访问，保护底层 `std::fs::File` 句柄。 |

## 备注

- 文件路径由 [get_log_path](get_log_path.md) 以空后缀确定，根据首次访问时 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 中缓存的日期生成 `logs/YYYYMMDD.log`。
- 文件以 `OpenOptions::new().append(true).create(true)` 方式打开，这意味着：
  - 如果文件已存在，新输出将追加到文件末尾。
  - 如果文件不存在，则会创建该文件。
- 由于 `Lazy` 初始化器对 `open` 的结果调用 `.unwrap()`，如果无法创建或打开日志文件，将在首次访问时引发 panic。在实际使用中，这仅在进程对其工作目录没有写入权限或磁盘已满时发生。
- 日志文件句柄在进程的整个生命周期内不会被显式关闭。它保持打开状态直到进程退出，届时操作系统会回收该句柄。这避免了高频日志记录期间反复打开/关闭的开销。
- 文件句柄**不会**在午夜自动轮换。文件名中的日期在静态变量首次初始化时固定。跨天的日志轮换由服务的重启或重新提升逻辑处理，该逻辑会创建一个新进程并进行全新的 `Lazy` 初始化。
- [get_logger!](get_logger.md) 宏提供了 `LOG_FILE.lock().unwrap()` 的简便写法，返回一个可直接写入的 `MutexGuard<File>`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell` (`Lazy`)、`std::sync::Mutex`、`std::fs::File`、`std::fs::OpenOptions` |
| 初始化依赖 | [get_log_path](get_log_path.md)、[LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| 写入方 | [log_message](log_message.md)、[log_pure_message](log_pure_message.md) |
| 访问器宏 | [get_logger!](get_logger.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 查找模式日志文件句柄 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 日志路径构造 | [get_log_path](get_log_path.md) |
| 带时间戳的日志写入 | [log_message](log_message.md) |
| 无时间戳的日志写入 | [log_pure_message](log_pure_message.md) |
| 控制台与文件路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| 日志抑制标志 | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| 用于文件命名的缓存时间戳 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd