# FIND_LOG_FILE 静态变量 (logging.rs)

全局互斥锁保护的查找模式日志文件句柄。此静态变量持有以追加模式打开的 `File`，路径为 `logs/YYYYMMDD.find.log`，其中日期前缀在初始化时从 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 确定。所有由 [log_to_find](log_to_find.md) 和 [log_process_find](log_process_find.md) 写入的查找模式日志输出，在非控制台模式下都会定向到此文件。

## 语法

```logging.rs
pub static FIND_LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。文件在首次访问时打开。 |
| 内层 | `Mutex<File>` | 提供从任意线程对底层文件句柄的同步写入访问。 |

## 备注

- 文件以 `OpenOptions::new().append(true).create(true)` 打开，如果文件不存在则创建它，如果已存在则将所有写入定位到文件末尾。这确保即使在同一天内多次服务重启，日志条目也不会因覆盖而丢失。
- 文件路径由 [get_log_path](get_log_path.md) 使用后缀 `".find"` 构建，生成形如 `logs/YYYYMMDD.find.log` 的路径。日期部分在初始化时从 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 的值派生；如果服务运行跨越午夜，查找日志将继续写入以启动日期命名的文件。
- 初始化器中的 `unwrap()` 会在 `logs/` 目录无法创建或文件无法打开时 panic。实际上，[get_log_path](get_log_path.md) 会调用 `create_dir_all` 确保目录在文件打开之前就已存在。
- [get_logger_find!](get_logger_find.md) 宏为 `FIND_LOG_FILE.lock().unwrap()` 提供了便捷的简写，返回 `MutexGuard<File>`。
- 此文件句柄与服务主操作日志 [LOG_FILE](LOG_FILE.md) 是分开的。这种分离使得 `-find` 模式的发现输出可以独立于标准服务诊断信息进行查看，而不会混杂在一起。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell` (`Lazy`)、`std::sync::Mutex`、`std::fs::File`、`std::fs::OpenOptions` |
| 初始化方式 | `Lazy` 在首次访问时（通常是第一次调用 [log_to_find](log_to_find.md) 时） |
| 访问宏 | [get_logger_find!](get_logger_find.md) |
| 路径构建器 | [get_log_path](get_log_path.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主日志文件句柄 | [LOG_FILE](LOG_FILE.md) |
| 查找模式日志函数 | [log_to_find](log_to_find.md) |
| 去重的进程发现日志 | [log_process_find](log_process_find.md) |
| 日志路径构建 | [get_log_path](get_log_path.md) |
| 用于日期前缀的缓存本地时间 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| 控制台与文件路由标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| logging 模块概述 | [logging 模块](README.md) |