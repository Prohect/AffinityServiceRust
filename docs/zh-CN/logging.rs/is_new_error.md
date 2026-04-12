# is_new_error 函数 (logging.rs)

检查给定的错误组合是否为首次出现。用于错误去重，防止同一错误在每次循环迭代中被反复记录到日志。

## 语法

```rust
fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool
```

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `pid` | `u32` | 发生错误的进程 ID。 |
| `tid` | `u32` | 发生错误的线程 ID。启用每线程级别的去重。 |
| `process_name` | `&str` | 进程名称（例如 `"game.exe"`）。用于检测 PID 重用。 |
| `operation` | [`Operation`](Operation.md) | 失败的 Windows API 操作类型。 |
| `error_code` | `u32` | Windows API 返回的错误代码。 |

## 返回值

返回 `bool`：

- `true` — 该错误组合是首次出现，调用方应将其记录到日志。
- `false` — 该错误组合已被记录过，调用方应跳过日志记录。

## 备注

`is_new_error` 是错误去重系统的核心查询函数。它被 [`apply.rs`](../apply.rs/README.md) 中的 `log_error_if_new` 调用，用于决定是否将错误消息写入日志。

### 算法

1. **构造键** — 使用传入参数创建一个 [`ApplyFailEntry`](ApplyFailEntry.md)，包含 `pid`、`tid`、`process_name`、`operation` 和 `error_code`。
2. **获取锁** — 锁定 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 全局映射。
3. **检查 PID 条目** — 在映射中查找该 `pid` 对应的子映射。
4. **检测 PID 重用** — 如果该 PID 已存在条目，但其中记录的进程名称与当前 `process_name` 不同，说明操作系统已将该 PID 分配给了新进程。此时清除该 PID 下的所有旧条目。
5. **查找重复** — 在该 PID 的子映射中查找是否已存在相同的 `ApplyFailEntry` 键。
   - 如果存在，返回 `false`（非新错误）。
   - 如果不存在，将新条目插入映射并返回 `true`（新错误）。

### PID 重用处理

Windows 会回收已终止进程的 PID。如果一个进程退出后，新进程获得了相同的 PID，旧进程的错误记录不应阻止新进程的错误被记录。`is_new_error` 通过比较进程名称来检测这种情况——当同一 PID 关联的进程名称发生变化时，自动清除该 PID 下的所有历史条目。

### 线程安全

该函数通过 `Mutex` 锁定 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 进行所有操作。锁在函数返回前释放。

### 与 purge_fail_map 的关系

`is_new_error` 负责**添加**和**查询**条目，而 [`purge_fail_map`](purge_fail_map.md) 负责**移除**陈旧条目。两者共同维护 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 的生命周期，防止内存无限增长。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L105–L149 |
| **使用方** | [`log_error_if_new`](../apply.rs/log_error_if_new.md)（`apply.rs`） |
| **依赖** | [`ApplyFailEntry`](ApplyFailEntry.md)、[`Operation`](Operation.md)、[`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) |

## 另请参阅

- [ApplyFailEntry 结构体](ApplyFailEntry.md)
- [Operation 枚举](Operation.md)
- [PID_MAP_FAIL_ENTRY_SET 静态变量](PID_MAP_FAIL_ENTRY_SET.md)
- [purge_fail_map 函数](purge_fail_map.md)
- [logging.rs 模块概述](README.md)