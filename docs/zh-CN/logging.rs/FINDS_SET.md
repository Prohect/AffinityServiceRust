# FINDS_SET 静态变量 (logging.rs)

跟踪已记录到 `.find.log` 文件的进程名称，防止在多次循环迭代中发现同一进程时产生重复条目。

## 语法

```rust
static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `HashSet<String>`。每个条目是一个已写入发现日志的进程名称（例如 `"game.exe"`）。

## 备注

当 [`log_process_find`](log_process_find.md) 被调用并传入一个进程名称时，它首先检查 `FINDS_SET` 是否已包含该名称。如果已包含，函数立即返回而不写入发现日志。如果不包含，则将该名称插入集合并向 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 写入新条目。

此去重非常重要，因为 [`main`](../main.rs/main.md) 中的主循环在每次迭代中都会发现正在运行的进程。如果没有 `FINDS_SET`，发现日志将在每个循环周期为每个找到的进程包含重复条目，使其难以查看哪些唯一进程在会话期间被检测到。

该集合在应用程序的整个生命周期内不会被清除——一旦进程名称被记录，它将永久保留在集合中。这意味着退出后又重新启动的进程在同一会话中不会被再次记录。

### 线程安全

所有对 `HashSet` 的访问都通过 `Mutex` 同步。锁被获取后执行检查并插入操作，然后在对发现日志文件进行任何 I/O 操作之前释放锁。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L11 |
| **使用方** | [`log_process_find`](log_process_find.md) |

## 另请参阅

- [log_process_find](log_process_find.md)
- [FIND_LOG_FILE 静态变量](FIND_LOG_FILE.md)
- [FINDS_FAIL_SET 静态变量](FINDS_FAIL_SET.md)
- [logging.rs 模块概述](README.md)