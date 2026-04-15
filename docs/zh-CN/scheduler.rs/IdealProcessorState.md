# IdealProcessorState 类型 (scheduler.rs)

跟踪单个线程的当前和先前理想处理器分配。理想处理器是 Windows 调度提示，告知内核线程应优先在哪个逻辑处理器上运行。此结构体同时记录当前分配和先前分配，以便应用引擎能够检测变更并避免冗余的 `SetThreadIdealProcessorEx` 调用。

## 语法

```rust
#[derive(Debug, Clone, Copy)]
pub struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
```

## 成员

| 字段 | 类型 | 描述 |
|------|------|------|
| `current_group` | `u16` | 当前分配的理想处理器所在的处理器组编号。在具有 64 个或更少逻辑处理器的系统上，只有组 0。 |
| `current_number` | `u8` | `current_group` 中从零开始的处理器编号，即线程当前的理想处理器。 |
| `previous_group` | `u16` | 先前分配的理想处理器所在的处理器组。用于检测自上次迭代以来是否发生了重新分配。 |
| `previous_number` | `u8` | `previous_group` 中在最近一次更改之前线程的理想处理器编号。 |
| `is_assigned` | `bool` | 指示服务是否曾为此线程分配过理想处理器。当为 `false` 时，`current_*` 和 `previous_*` 字段包含默认值（全部为零），不代表实际分配。 |

## 备注

- 所有字段通过 `IdealProcessorState::new()` 和 `Default` 实现初始化为零/`false`。
- 该结构体派生了 `Clone` 和 `Copy`，使得跨迭代的快照和比较操作成本很低。
- previous/current 对使应用引擎能够实现变更检测：如果 `(current_group, current_number)` 等于 `(previous_group, previous_number)` 且 `is_assigned` 为 `true`，则无需进行 Win32 调用，因为分配没有变化。
- 处理器组在具有超过 64 个逻辑处理器的系统上相关（例如双路服务器硬件或高核心数工作站）。在典型的消费级硬件上，`current_group` 和 `previous_group` 始终为 `0`。
- 此结构体作为 `ideal_processor` 字段嵌入在 [`ThreadStats`](ThreadStats.md) 中。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 使用者 | [`ThreadStats`](ThreadStats.md)、`apply::apply_ideal_processors` |
| Win32 API | 对应 `SetThreadIdealProcessorEx` / `GetThreadIdealProcessorEx` 使用的 `PROCESSOR_NUMBER` 结构体 |
| 权限 | 无（纯数据结构体） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| ThreadStats | [ThreadStats](ThreadStats.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ProcessStats | [ProcessStats](ProcessStats.md) |
| apply_thread_level | [apply_thread_level](../main.rs/apply_thread_level.md) |
| scheduler 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
