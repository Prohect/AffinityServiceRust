# purge_fail_map 函数 (logging.rs)

从应用失败跟踪映射中删除过期条目以防止内存无限增长。由主轮询循环在枚举运行进程后定期调用。

## 语法

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, &str)])
```

## 参数

`pids_and_names: &[(u32, &str)]`

代表当前运行进程的 `(pid, process_name)` 元组切片。只有 PID **和** 进程名称与此切片中的元组匹配的条目才能在清理后保留。

## 返回值

此函数不返回值。它就地修改全局 `PID_MAP_FAIL_ENTRY_SET`。

## 备注

### 算法

清理遵循标记-清扫模式：

1. **标记全部为死亡：** 遍历全局 `PID_MAP_FAIL_ENTRY_SET` 映射中的每个条目，将所有 `alive` 标志设置为 `false`。
2. **重新标记存活：** 对于 `pids_and_names` 中的每个 `(pid, name)`，如果 PID 存在于映射中且至少有一个条目的 `process_name` 与 `name` 匹配，则第一个条目的 `alive` 标志被重新设置为 `true`。
3. **清扫：** 对外层映射调用 `retain`，删除内层映射中没有 `alive == true` 条目的任何 PID。

### PID 重用感知

因为 Windows PID 可以被回收，重新标记步骤除了 PID 之外还会检查 `process_name` 字段。如果 PID 被不同的进程重用，其条目将不会匹配新名称，过期条目将被清除。这防止错误去重错误地抑制意外接收到回收 PID 的新进程的错误。

### 调用频率

此函数在主循环每次迭代后调用一次，此时已经枚举了完整的进程列表。它在整个操作期间获取 `PID_MAP_FAIL_ENTRY_SET` 互斥锁，因此不应在已经持有此锁的上下文中调用。

### 与 is_new_error 的关系

[is_new_error](is_new_error.md) 负责添加条目并处理每个 PID 的进程名称变更（检测到名称不匹配时清除内层映射），而 `purge_fail_map` 处理互补的情况，即删除已完全退出的进程的条目。二者共同确保映射的大小与当前运行的受监控进程数量保持一致。

## 要求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用方** | 主轮询循环（进程枚举后） |
| **被调用方** | `get_pid_map_fail_entry_set!()` 宏 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [logging.rs](README.md) |
| 错误去重检查 | [is_new_error](is_new_error.md) |
| 失败键 | [ApplyFailEntry](ApplyFailEntry.md) |
| 操作枚举 | [Operation](Operation.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*