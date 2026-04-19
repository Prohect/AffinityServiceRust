# log_process_find 函数 (logging.rs)

将发现的进程名记录到查找日志文件中，进行去重处理，使得每个进程名在每次会话中最多记录一次。

## 语法

```rust
#[inline]
pub fn log_process_find(process_name: &str)
```

## 参数

`process_name: &str`

发现的进程的可执行文件名（例如 `"notepad.exe"`）。

## 返回值

该函数不返回值。

## 备注

此函数是查找模式进程日志记录的公共入口点。它获取全局 `FINDS_SET` 的锁，并尝试插入 `process_name`。如果插入成功（名称尚不存在），函数将使用格式化消息 `"find {process_name}"` 委托给 [log_to_find](log_to_find.md)。如果名称已被记录，则该调用为无操作。

### 去重范围

`FINDS_SET` 静态变量在程序启动时初始化一次，并且在进程生命周期内永远不会被清除。这意味着去重跨越整个服务会话——在 1 分钟时发现的一个进程名，在 60 分钟时不会再次被记录，即使进程在此期间重新启动。该集合仅在服务本身重启时重置，届时也会按日期轮转日志文件。

### 输出格式

写入查找日志的消息格式为：

```
[HH:MM:SS]find notepad.exe
```

时间戳前缀由 [log_to_find](log_to_find.md) 添加，而不是由此函数添加。

### 线程安全

该函数获取 `FINDS_SET` 的 `Mutex` 锁。锁仅持有 `HashSet::insert` 调用期间；后续的 [log_to_find](log_to_find.md) 调用发生在锁释放之后（在 `if` 条件结束时隐式 drop）。

### 控制台模式

当 `USE_CONSOLE` 为 `true` 时，查找日志输出通过 [log_to_find](log_to_find.md) 重定向到 stdout。这通常在工具使用 `-find` CLI 标志交互式运行时发生。

## 要求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用方** | 主轮询循环（查找模式路径），[scheduler](../scheduler.rs/README.md) |
| **被调用方** | [log_to_find](log_to_find.md) |
| **静态变量** | `FINDS_SET` (`Lazy<Mutex<HashSet<String>>>`) |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [logging.rs](README.md) |
| 查找日志写入器 | [log_to_find](log_to_find.md) |
| 通用日志函数 | [log_message](log_message.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*