# purge_fail_map 函数 (logging.rs)

移除 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 中已不再运行的进程的陈旧错误条目，防止映射无限增长并正确处理 PID 重用。

## 语法

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, String)])
```

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `pids_and_names` | `&[(u32, String)]` | 当前正在运行的进程列表，每个元素包含 PID 和对应的进程名称。 |

## 返回值

无返回值。

## 备注

`purge_fail_map` 在每次主循环迭代中由 [`main`](../main.rs/main.md) 调用，用于维护 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 映射的健康状态。如果不进行定期清理，随着进程的启动和终止，映射将持续积累条目，导致内存无限增长。

### 算法

清理操作分为三个步骤：

1. **标记全部死亡** — 将映射中所有现有条目的 `alive` 标志设置为 `false`。
2. **标记运行中存活** — 遍历 `pids_and_names` 列表，对于在映射中存在的 PID，将其对应条目的 `alive` 标志设置回 `true`。
3. **移除死亡条目** — 移除所有 `alive` 标志仍为 `false` 的 PID 条目。

这种三阶段标记-清除策略确保只有当前正在运行的进程的错误条目被保留。

### PID 重用处理

操作系统可能会将已终止进程的 PID 分配给新进程。由于 [`is_new_error`](is_new_error.md) 在检测到进程名称不匹配时已经清除旧条目，`purge_fail_map` 主要负责移除完全不再存在的 PID。两个函数协同工作，确保 PID 重用场景下的正确行为。

### 调用时机

此函数在每次循环迭代的进程快照获取之后调用，传入当前活跃的进程列表。这确保映射与系统当前状态保持同步。

### 线程安全

函数内部获取 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 的 `Mutex` 锁，在整个标记-清除操作期间持有锁以保证原子性。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L151–L172 |
| **调用方** | [`main`](../main.rs/main.md) |

## 另请参阅

- [event_trace.rs 模块概述](../event_trace.rs/README.md) — 响应式进程监控
- [PID_MAP_FAIL_ENTRY_SET 静态变量](PID_MAP_FAIL_ENTRY_SET.md)
- [is_new_error 函数](is_new_error.md)
- [ApplyFailEntry 结构体](ApplyFailEntry.md)
- [Operation 枚举](Operation.md)
- [logging.rs 模块概述](README.md)
