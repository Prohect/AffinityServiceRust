# log_apply_results 函数 (main.rs)

格式化并输出单次配置应用过程中对一个进程所产生的变更和错误的日志。错误会被转发到 find-log 接收器，而成功应用的变更则以对齐的多行格式写入主日志。

## 语法

```rust
fn log_apply_results(pid: &u32, name: &String, result: ApplyConfigResult)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `&u32` | 刚刚完成配置的 Windows 进程 ID。用作日志前缀的一部分以进行标识。 |
| `name` | `&String` | 进程的可执行文件名（例如 `"game.exe"`）。与 PID 一起显示在日志前缀中。 |
| `result` | `ApplyConfigResult` | 由 `apply_process_level` 和/或 `apply_thread_level` 产生的累积结果结构体，包含人类可读的变更描述和错误字符串的向量。该结构体由本函数消费。 |

## 返回值

本函数不返回值。

## 备注

- 当 `result.is_empty()` 返回 `true` 时（即未记录任何变更和错误），本函数不执行任何操作。
- `result.errors` 中的所有字符串都会被转发到 `log_to_find`，该函数将其写入 `.find.log` 文件，以供 `-processLogs` 模式后续分析。
- 第一条变更字符串以 `"{pid}::{name}::{change}"` 的格式化前缀进行记录。后续的变更字符串会缩进对齐到第一条变更文本的位置，同时考虑前缀宽度和日志子系统预置的 10 字符时间戳前缀（例如 `[04:55:16]`）。
- 对齐逻辑将填充计算为 `prefix_length - first_change_length + 10`，确保单个进程的多行输出在日志文件中形成视觉上分组的块。
- 本函数获取 `result` 的所有权，并在处理完成后将其丢弃。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用者 | [apply_config](apply_config.md)、[main](main.md) 中的线程级应用循环 |
| 被调用者 | `logging::log_to_find`、`logging::log_message`、`logging::log_pure_message`、`ApplyConfigResult::is_empty` |
| API | 无（仅内部日志） |
| 权限 | 无 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply_config | [apply_config](apply_config.md) |
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| main | [main](main.md) |
| logging 模块 | [logging](../logging.rs/README.md) |

---
Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
