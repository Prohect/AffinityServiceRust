# log_process_find 函数 (logging.rs)

将 `-find` 模式下发现的进程名称记录到发现日志文件中。使用 [`FINDS_SET`](FINDS_SET.md) 进行去重，确保同一进程名称在整个会话中仅记录一次。

## 语法

```rust
fn log_process_find(process_name: &str)
```

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `process_name` | `&str` | 被发现的进程可执行文件名称（例如 `"game.exe"`）。 |

## 返回值

无。

## 备注

此函数是 `-find` 模式的核心输出机制。当主循环在每次迭代中扫描正在运行的进程时，匹配的进程名称被传递给 `log_process_find` 以记录发现结果。

### 算法

1. 获取 [`FINDS_SET`](FINDS_SET.md) 的锁。
2. 检查 `process_name` 是否已存在于集合中。
3. 如果已存在，立即返回——该进程已在本次会话中记录过。
4. 如果不存在，将 `process_name` 插入集合。
5. 释放 `FINDS_SET` 的锁。
6. 调用 [`log_to_find`](log_to_find.md) 写入格式化的发现条目。

### 输出格式

写入 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 的条目格式为：

```
[HH:MM:SS] find process.exe
```

其中时间戳来自 [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md)，通过 [`log_to_find`](log_to_find.md) 添加。

### 去重行为

[`FINDS_SET`](FINDS_SET.md) 在应用程序的整个生命周期内持续累积，不会被清除。这意味着：

- 每个唯一的进程名称仅在发现日志中出现一次。
- 如果一个进程退出后重新启动，它不会在同一会话中被再次记录。
- 发现日志因此提供了会话期间所有被检测到的唯一进程的简洁汇总。

### 线程安全

对 [`FINDS_SET`](FINDS_SET.md) 的访问通过 `Mutex` 同步。锁在检查和插入操作完成后、I/O 操作之前释放，以最小化锁持有时间。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L214–L223 |
| **调用方** | [`main`](../main.rs/main.md)（`-find` 模式循环） |
| **依赖** | [`FINDS_SET`](FINDS_SET.md)、[`log_to_find`](log_to_find.md) |

## 另请参阅

- [FINDS_SET 静态变量](FINDS_SET.md)
- [log_to_find 函数](log_to_find.md)
- [FIND_LOG_FILE 静态变量](FIND_LOG_FILE.md)
- [FINDS_FAIL_SET 静态变量](FINDS_FAIL_SET.md)
- [logging.rs 模块概述](README.md)