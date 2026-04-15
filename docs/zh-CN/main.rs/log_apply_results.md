# log_apply_results 函数 (main.rs)

格式化并输出单个进程的一次配置应用过程中产生的变更和错误的日志信息。错误会转发到 find-log 接收器，而成功应用的变更则以对齐的多行格式写入主日志。

## 语法

```rust
fn log_apply_results(pid: &u32, name: &String, result: ApplyConfigResult)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `&u32` | 刚完成配置的 Windows 进程 ID。作为日志前缀的一部分用于标识。 |
| `name` | `&String` | 进程的可执行文件名（例如 `"game.exe"`）。在日志前缀中与 PID 一起显示。 |
| `result` | `ApplyConfigResult` | 来自 `apply_process_level` 和/或 `apply_thread_level` 的累积结果结构体，包含人类可读的变更描述向量和错误字符串。由此函数消费。 |

## 返回值

此函数没有返回值。

## 备注

- 当 `result.is_empty()` 返回 `true`（即没有记录任何变更和错误）时，该函数不执行任何操作。
- `result.errors` 中的所有字符串都会转发到 `log_to_find`，后者将其写入 `.find.log` 文件，供后续 `-processLogs` 模式分析使用。
- 第一个变更字符串以格式化前缀 `"{pid}::{name}::{change}"` 进行记录。后续变更字符串会进行缩进对齐，使其与第一个变更文本对齐，同时考虑前缀宽度和日志子系统预置的 10 字符时间戳前缀（例如 `[04:55:16]`）。
- 对齐逻辑将填充计算为 `prefix_length - first_change_length + 10`，确保单个进程的多行输出在日志文件中呈现为视觉分组的块。
- 此函数获取 `result` 的所有权，并在处理完毕后将其丢弃。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用者 | [apply_config](apply_config.md)、[main](main.md) 中的线程级应用循环 |
| 被调用者 | `logging::log_to_find`、`logging::log_message`、`logging::log_pure_message`、`ApplyConfigResult::is_empty` |
| API | 无（仅内部日志记录） |
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
Commit: `7221ea0694670265d4eb4975582d8ed2ae02439d`
