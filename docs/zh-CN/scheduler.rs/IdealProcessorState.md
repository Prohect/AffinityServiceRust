# IdealProcessorState 结构体 (scheduler.rs)

跟踪单个线程的理想处理器分配状态，记录当前和上一次的处理器组及编号，以支持重新分配决策。

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

| 成员 | 类型 | 描述 |
|------|------|------|
| `current_group` | `u16` | 线程当前理想调度所在的处理器组索引。对应 `PROCESSOR_NUMBER` 结构的 `Group` 字段。 |
| `current_number` | `u8` | 线程当前理想调度所在 `current_group` 中的逻辑处理器编号。对应 `PROCESSOR_NUMBER` 结构的 `Number` 字段。 |
| `previous_group` | `u16` | 最近一次重新分配之前，线程理想调度所在的处理器组索引。用于检测线程的理想处理器是否实际发生了变化。 |
| `previous_number` | `u8` | 最近一次重新分配之前，线程理想调度所在 `previous_group` 中的逻辑处理器编号。 |
| `is_assigned` | `bool` | 指示调度器是否已为该线程显式分配了理想处理器。当为 `false` 时，`current_*` 和 `previous_*` 字段处于默认零值状态，并不代表有意的分配。 |

## 备注

`IdealProcessorState` 作为 `ideal_processor` 字段嵌入在每个 [ThreadStats](ThreadStats.md) 实例中。它使 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 函数能够跟踪线程的理想处理器是否已设置，并在决定是否调用 `SetThreadIdealProcessorEx` 时将当前分配与上一次分配进行比较。

`previous_*` 字段在将新值写入 `current_*` 之前更新，允许调用方检测并记录处理器重新分配的转换过程。这对于诊断调度不稳定性特别有用——如果线程的理想处理器每个时钟周期都在变化，可能表明 [PrimeThreadScheduler](PrimeThreadScheduler.md) 中的滞后算法阈值需要调整。

### 默认状态

调用 `IdealProcessorState::new()` 或 `IdealProcessorState::default()` 返回一个所有数值字段为 `0`、`is_assigned` 为 `false` 的状态。组 0、处理器 0 作为默认值**并不**意味着分配到了该处理器——必须首先检查 `is_assigned` 标志。

### 平台说明

处理器组和编号的值对应于 `SetThreadIdealProcessorEx` 和 `GetThreadIdealProcessorEx` 所使用的 Windows `PROCESSOR_NUMBER` 结构。在只有单个处理器组的系统上，`current_group` 和 `previous_group` 将始终为 `0`。

## 要求

| | |
|---|---|
| **模块** | `scheduler.rs` |
| **嵌入于** | [ThreadStats](ThreadStats.md) |
| **使用方** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **平台 API** | `SetThreadIdealProcessorEx`、`GetThreadIdealProcessorEx`（`windows` crate） |

## 另请参阅

| 主题 | 描述 |
|------|------|
| [ThreadStats](ThreadStats.md) | 包含 `IdealProcessorState` 实例的线程统计结构体。 |
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | 管理线程到处理器分配的核心调度器。 |
| [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) | 在应用理想处理器规则时读写 `IdealProcessorState` 的函数。 |
| [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) | 对 Windows `SetThreadIdealProcessorEx` API 的底层封装。 |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd