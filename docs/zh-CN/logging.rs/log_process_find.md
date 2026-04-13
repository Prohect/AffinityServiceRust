# log_process_find 函数 (logging.rs)

在 `-find` 模式下记录已发现的进程名称，按会话去重。对于给定的进程名称，首次调用时，该函数会通过 [log_to_find](log_to_find.md) 写入一条 `find <process_name>` 条目到查找模式日志中；后续使用相同名称的调用将被静默忽略。这确保查找日志包含一个干净的、唯一的在会话期间观察到的所有进程列表，而不会因轮询循环而产生重复。

## 语法

```logging.rs
#[inline]
pub fn log_process_find(process_name: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `process_name` | `&str` | 要记录的已发现进程的名称。此值会被插入到 [FINDS_SET](FINDS_SET.md) 中用于去重，并以 `"find <process_name>"` 的形式格式化到日志行中。 |

## 返回值

*(无)*

## 备注

- 该函数锁定 [FINDS_SET](FINDS_SET.md) 并调用 `HashSet::insert`。如果 `insert` 返回 `true`（名称之前不存在），函数会委托 [log_to_find](log_to_find.md) 输出格式为 `"find <process_name>"` 的消息。如果 `insert` 返回 `false`（名称已被记录），函数直接返回，不产生任何输出。
- 去重是按会话（按进程生命周期）进行的，而非按天。如果服务在同一个日历日内重新启动，新进程将以空的 [FINDS_SET](FINDS_SET.md) 开始，并会重新记录所有发现的进程。这是设计上的选择：每次运行都应产生一份独立的发现报告。
- 该函数标注了 `#[inline]`，提示编译器在调用点内联展开。由于函数体较小（一次锁获取、一次条件日志调用），内联可以避免在热轮询路径中的函数调用开销。
- 进程名称按原样存储在 `FINDS_SET` 中，不进行小写转换或规范化。调用方（通常是 [process_find](../main.rs/README.md)）负责以预期格式提供名称。
- 由于 [log_to_find](log_to_find.md) 内部会检查 [USE_CONSOLE](USE_CONSOLE.md)，输出目标（控制台或 `logs/YYYYMMDD.find.log`）由该标志决定。时间戳前缀 `[HH:MM:SS]` 由 `log_to_find` 添加。

### 输出示例

对于首次发现 `notepad.exe`，查找日志会收到如下一行：

```/dev/null/example.log#L1-1
[09:15:42]find notepad.exe
```

在同一会话中使用 `"notepad.exe"` 再次调用不会产生任何输出。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 调用方 | [process_find](../main.rs/README.md) |
| 被调用方 | [log_to_find](log_to_find.md) |
| 读取 | [FINDS_SET](FINDS_SET.md)（锁定并插入） |
| 遵循 | [USE_CONSOLE](USE_CONSOLE.md)（间接，通过 `log_to_find`） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 成功查找的去重集合 | [FINDS_SET](FINDS_SET.md) |
| 带时间戳的查找模式日志写入器 | [log_to_find](log_to_find.md) |
| 查找模式日志文件句柄 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 失败查找的去重集合 | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| main 中的查找模式入口点 | [process_find](../main.rs/README.md) |
| logging 模块概述 | [logging 模块](README.md) |