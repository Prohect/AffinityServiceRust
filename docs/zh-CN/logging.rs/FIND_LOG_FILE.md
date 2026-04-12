# FIND_LOG_FILE 静态变量 (logging.rs)

Find 模式专用的日志文件句柄。当应用程序以 `-find` 模式运行时，进程发现结果将写入此独立的 `.find.log` 文件，与常规日志分离以便于分析。

## 语法

```rust
static FIND_LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(File::create(get_log_path(".find")).unwrap()));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `File` 句柄。文件路径通过 [`get_log_path`](get_log_path.md) 生成，格式为 `logs/YYYYMMDD.find.log`。

## 备注

`FIND_LOG_FILE` 在首次访问时通过 `Lazy` 延迟初始化。初始化时调用 [`get_log_path`](get_log_path.md) 并传入后缀 `".find"` 来生成文件路径，然后创建（或截断）该文件。

该文件句柄被以下函数使用：

- [`log_to_find`](log_to_find.md) — 将带时间戳的消息写入发现日志。
- [`log_process_find`](log_process_find.md) — 将发现的进程名称写入发现日志（通过 [`FINDS_SET`](FINDS_SET.md) 去重）。

### 文件路径

日志文件存储在可执行文件旁的 `logs/` 目录下，文件名格式为：

```
logs/YYYYMMDD.find.log
```

例如，2024 年 1 月 15 日的发现日志为 `logs/20240115.find.log`。如果 `logs/` 目录不存在，[`get_log_path`](get_log_path.md) 会自动创建它。

### 与 LOG_FILE 的关系

应用程序维护两个独立的日志文件：

- [`LOG_FILE`](LOG_FILE.md) — 常规日志输出（错误、状态信息、配置应用结果等）。
- `FIND_LOG_FILE` — 仅用于 `-find` 模式的进程发现记录。

这种分离使用户可以单独查看发现日志，而不必从大量常规日志消息中筛选进程发现信息。

### 线程安全

所有对文件句柄的写入操作都通过 `Mutex` 同步。写入方在执行 I/O 操作时短暂获取锁，写入完成后立即释放。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L67 |
| **初始化依赖** | [`get_log_path`](get_log_path.md) |
| **使用方** | [`log_to_find`](log_to_find.md)、[`log_process_find`](log_process_find.md) |

## 另请参阅

- [LOG_FILE 静态变量](LOG_FILE.md)
- [get_log_path 函数](get_log_path.md)
- [log_to_find 函数](log_to_find.md)
- [log_process_find 函数](log_process_find.md)
- [logging.rs 模块概述](README.md)