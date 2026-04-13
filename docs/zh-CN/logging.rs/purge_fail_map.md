# purge_fail_map 函数 (logging.rs)

从按 PID 组织的操作失败跟踪映射（[PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md)）中移除过期条目。此函数实现了一种标记-清除垃圾回收策略：首先将所有条目标记为死亡，然后将属于当前仍在运行的进程的条目重新标记为存活，最后移除所有仍处于死亡状态的条目。这防止了在服务生命周期内随着进程的启动和停止，失败映射的无限增长。

## 语法

```logging.rs
#[inline]
pub fn purge_fail_map(pids_and_names: &[(u32, String)])
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pids_and_names` | `&[(u32, String)]` | 表示当前正在运行的进程的 `(pid, process_name)` 元组切片。每个需要保留其失败条目的 PID 必须以匹配的进程名称出现在此列表中。 |

## 返回值

此函数不返回值。

## 备注

### 算法

该函数对全局 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 映射执行三阶段标记-清除操作：

1. **全部标记为死亡：** 遍历每个内层 `HashMap<ApplyFailEntry, bool>`，将所有 `bool` 值（"存活"标志）设置为 `false`。
2. **重新标记为存活：** 对于 `pids_and_names` 中的每个 `(pid, name)` 对，在外层映射中查找该 PID。如果找到，并且该 PID 内层映射中的任何条目的 `process_name` 与 `name` 匹配，则将内层映射中的第一个条目标记为存活（`true`）。这确认该 PID 仍然与同一进程关联，其失败记录应被保留。
3. **清除：** 调用 `map.retain(…)` 移除内层映射中不包含任何存活条目的外层映射条目。这会丢弃所有已退出进程的失败跟踪状态。

### 设计原理

- 服务的轮询循环定期（通常每次迭代一次）使用当前运行进程的快照调用 `purge_fail_map`。这确保已终止进程的失败条目不会无限积累。
- 存活标志机制避免了在每次迭代中重建整个映射的需要。大多数条目只是被重新标记为存活，只有已退出进程的条目被移除。
- 该函数通过 [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) 宏获取 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 互斥锁，并在清除期间一直持有该锁。由于此操作会触及映射中的每个条目，锁的持有时间与被跟踪的 PID 数量及其失败条目成正比。

### 进程名称匹配

重新标记阶段会检查 PID 内层映射中至少有一个 [ApplyFailEntry](ApplyFailEntry.md) 的 `process_name` 字段与 `pids_and_names` 中的名称匹配。这可以防止 PID 重用：如果操作系统回收了一个 PID 并将其分配给不同的进程，旧的失败条目（具有先前进程的名称）将不会与新名称匹配，会保持死亡标记，并在清除阶段被移除。

### 与 is_new_error 的交互

[is_new_error](is_new_error.md) 负责插入新条目和执行去重检查，而 `purge_fail_map` 负责在进程退出后进行清理。两者共同构成了失败跟踪条目的完整生命周期：

- `is_new_error` → 插入条目，将其标记为存活，对新错误返回 `true`
- `purge_fail_map` → 将条目标记为死亡，重新标记存活的条目，移除死亡的条目

### 内联提示

该函数使用 `#[inline]` 注解，建议编译器在调用点进行内联。在实际使用中，该函数仅从主循环中的单一位置调用，因此此提示主要作为该函数较小且性能敏感的文档信号。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 读取 / 写入 | 通过 [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) 访问 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 调用方 | [main 模块](../main.rs/README.md) 中的主服务循环 |
| 被调用方 | *（无——仅操作内存数据结构）* |
| 相关插入逻辑 | [is_new_error](is_new_error.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 错误去重检查与插入 | [is_new_error](is_new_error.md) |
| 全局失败跟踪映射 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 失败条目键结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| Windows API 操作标识符 | [Operation](Operation.md) |
| logging 模块概述 | [logging 模块](README.md) |