# is_new_error 函数 (logging.rs)

跟踪操作失败以避免重复的错误消息刷屏日志。仅在给定的 PID/TID/进程名称/操作/错误码组合首次出现时返回 `true`，允许调用者仅对每个唯一失败进行一次条件日志记录或处理。

## 语法

```rust
pub fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 与失败关联的进程标识符。用作失败跟踪映射中的顶层键。 |
| `tid` | `u32` | 与失败关联的线程标识符。与 `process_name`、`operation` 和 `error_code` 组合形成唯一的失败键。对于非线程特定的进程级操作，使用 `0`。 |
| `process_name` | `&str` | 与失败关联的进程名称。既用作失败键的一部分，也用于过时条目检测（参见备注）。 |
| `operation` | [`Operation`](Operation.md) | 失败的 Windows API 操作。[`Operation`](Operation.md) 枚举的每个变体代表一个不同的 API 调用或句柄获取步骤。 |
| `error_code` | `u32` | 失败操作返回的 Win32 错误码或自定义标识符。当没有上下文错误码时使用 `0`，或者如果需要区分共享相同操作但有不同原因的失败，使用自定义值。 |

## 返回值

如果这是该特定 `(pid, tid, process_name, operation, error_code)` 组合**首次**被记录，则返回 `true`。如果相同的组合已经存在于失败跟踪映射中，则返回 `false`。

## 备注

### 数据结构

该函数使用全局 [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) 静态变量，其类型为：

```text
Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>
```

- **外层键**是 `pid`。
- **内层键**是一个 [`ApplyFailEntry`](ApplyFailEntry.md) 结构体，包含 `(tid, process_name, operation, error_code)`。
- **内层值**是一个 `bool` "存活"标志，由 [`purge_fail_map`](purge_fail_map.md) 用于检测和移除过时条目。

### 算法

1. 从提供的参数构造一个 [`ApplyFailEntry`](ApplyFailEntry.md)。
2. 锁定 `PID_MAP_FAIL_ENTRY_SET` 互斥量。
3. 在外层映射中查找 `pid`：
   - **如果找到** — 在内层 `HashMap` 中搜索匹配的条目：
     - **如果存在匹配的条目** — 将其标记为存活（`true`）并返回 `false`（不是新错误）。
     - **如果不存在匹配的条目** — 检查现有条目的 `process_name` 是否与新条目相同。如果不同（表示 PID 被不同的进程重用），则在插入新条目之前**清空整个内层映射**。插入新条目并将 `alive = true`，然后返回 `true`。
   - **如果未找到** — 创建一个仅包含新条目（`alive = true`）的新内层映射，将其插入到 `pid` 键下，然后返回 `true`。

### PID 重用检测

当一个 PID 被不同的进程重用时（例如，原始进程退出后新进程被分配了相同的 PID），现有的失败条目会变得过时。该函数通过比较第一个现有条目的 `process_name` 与新条目的 `process_name` 来检测此情况。如果两者不同，则在插入新条目之前清空内层映射。这可以防止误报（即由于先前具有相同 PID 的不相关进程有相同的操作失败，而导致真正的新错误被抑制）。

### 存活标志

每个条目存储一个 `alive` 标志（内层 `HashMap` 中的 `bool` 值）。当 `is_new_error` 找到匹配条目时，它将 `alive` 设置为 `true`。配套函数 [`purge_fail_map`](purge_fail_map.md) 使用此标志实现标记-清除垃圾回收方案：它首先将所有条目标记为已死（`false`），然后将当前正在运行的进程的条目重新标记为存活，最后移除所有仍然为死状态的条目。

### 线程安全

该函数通过 `get_pid_map_fail_entry_set!()` 宏获取 `PID_MAP_FAIL_ENTRY_SET` 互斥量。锁在查找和插入操作的整个持续期间被持有。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **调用方** | [`get_process_handle`](../winapi.rs/get_process_handle.md)、[`get_thread_handle`](../winapi.rs/get_thread_handle.md)、`apply.rs` 规则应用逻辑 |
| **被调用方** | `get_pid_map_fail_entry_set!()` 宏（锁定 [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set)） |
| **静态变量** | [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) |
| **依赖** | [`ApplyFailEntry`](ApplyFailEntry.md)、[`Operation`](Operation.md) |
| **平台** | 与平台无关的逻辑（数据为平台特定） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| purge_fail_map | [purge_fail_map](purge_fail_map.md) |
| ApplyFailEntry 结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation 枚举 | [Operation](Operation.md) |
| PID_MAP_FAIL_ENTRY_SET | [statics](statics.md#pid_map_fail_entry_set) |
| get_process_handle | [get_process_handle](../winapi.rs/get_process_handle.md) |
| get_thread_handle | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| logging 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
