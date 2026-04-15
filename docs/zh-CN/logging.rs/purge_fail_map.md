# purge_fail_map 函数 (logging.rs)

从每个 PID 的应用失败跟踪映射（[`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set)）中移除过时的条目。此函数实现了标记-清除垃圾回收策略：将所有条目标记为"已死"，重新将属于当前运行进程的条目标记为"存活"，然后移除所有仍然标记为"已死"的条目。这可以防止失败跟踪映射随着进程的启动和停止而无限增长。

## 语法

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, &str)])
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pids_and_names` | `&[(u32, &str)]` | 表示当前正在运行的进程的 `(pid, process_name)` 元组切片，这些进程应保留在失败跟踪映射中。不匹配此切片中任何元组的条目被视为过时条目，将被移除。 |

## 返回值

此函数不返回值。

## 备注

### 算法

该函数实现了三阶段的标记-清除方法：

1. **全部标记为已死。** 遍历每个 PID 的失败集合中的每个条目，将其 `alive` 标志设置为 `false`。

2. **重新标记为存活。** 对于 `pids_and_names` 中的每个 `(pid, name)`：
   - 在失败映射中查找该 PID。
   - 如果该 PID 存在失败集合，**且**该集合中至少有一个条目的 `process_name` 与提供的 `name` 匹配，则将该集合中的第一个条目标记为存活（`true`）。名称检查确保 PID 重用（即新进程获得了与已终止的不同名称进程相同的 PID）不会错误地保留过时条目。

3. **清除。** 调用 `HashMap::retain` 移除所有**没有**任何失败条目的 `alive` 标志设置为 `true` 的 PID 条目。这将移除不再运行或其进程名称已更改的 PID 的条目。

### 锁定

函数通过 `get_pid_map_fail_entry_set!()` 宏获取 `PID_MAP_FAIL_ENTRY_SET` 互斥锁，并在整个清除操作期间持有该锁。这确保了标记和清除阶段之间的一致性。

### 与 is_new_error 的交互

此函数与 [`is_new_error`](is_new_error.md) 互补。`is_new_error` 在遇到新的失败时向失败映射**添加**条目，而 `purge_fail_map` **移除**不再相关的条目。它们共同实现了一个有界的错误去重系统：

- `is_new_error` 确保每个唯一失败只记录一次。
- `purge_fail_map` 确保跟踪数据不会无限累积。

### 调用频率

此函数通常在每次调度循环迭代中调用一次，在获取进程快照之后，传入与配置规则匹配的当前活动进程列表。这确保了已退出进程的失败跟踪数据被及时清理。

### 边界情况

- 如果 `pids_and_names` 为空，失败映射中的所有条目都会被标记为已死，并在清除阶段被随后移除。
- 如果失败映射中存在某个 PID，但进程名称与 `pids_and_names` 中的名称不匹配（例如 PID 重用），则该 PID 的条目**不会**被重新标记为存活，将被清除。当稍后为占用该 PID 的新进程调用 `is_new_error` 时，它将清除所有名称不匹配的过时条目并重新开始。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **调用方** | `scheduler.rs` — 主调度循环的清理阶段 |
| **被调用方** | `get_pid_map_fail_entry_set!()` 宏 → `PID_MAP_FAIL_ENTRY_SET.lock()` |
| **静态变量** | [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) |
| **平台** | 平台无关的逻辑（数据结构在上下文中是 Windows 特有的） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| is_new_error 函数 | [is_new_error](is_new_error.md) |
| ApplyFailEntry 结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation 枚举 | [Operation](Operation.md) |
| PID_MAP_FAIL_ENTRY_SET 静态变量 | [statics](statics.md#pid_map_fail_entry_set) |
| logging 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
