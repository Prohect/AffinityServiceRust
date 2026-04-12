# IdealProcessorState struct (scheduler.rs)

跟踪单个线程的理想处理器分配状态，包括当前分配和上一次分配的处理器组与编号，用于检测分配变化并避免冗余的 Windows API 调用。

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

`current_group`

当前分配的处理器组编号（类型 `u16`）。对应 Windows `PROCESSOR_NUMBER.Group` 字段。在多处理器组系统中，此值标识线程的理想处理器所在的处理器组。

`current_number`

当前分配的处理器组内编号（类型 `u8`）。对应 Windows `PROCESSOR_NUMBER.Number` 字段。标识处理器组内的具体逻辑处理器。

`previous_group`

上一次循环迭代中分配的处理器组编号（类型 `u16`）。用于与 `current_group` 比较，判断理想处理器是否发生了变化。

`previous_number`

上一次循环迭代中分配的处理器组内编号（类型 `u8`）。用于与 `current_number` 比较，判断理想处理器是否发生了变化。

`is_assigned`

指示此线程是否已被分配了理想处理器（类型 `bool`）。当为 `false` 时，表示此线程尚未通过 `SetThreadIdealProcessorEx` 设置过理想处理器。在 [select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md#select_top_threads_with_hysteresis) 的滞后选择中，此字段用于判断线程是否为"当前已分配"的线程。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **new** | `pub fn new() -> Self` | 创建一个默认状态：所有组和编号字段为 `0`，`is_assigned` 为 `false`。 |

## 备注

`IdealProcessorState` 作为 [ThreadStats](ThreadStats.md) 的 `ideal_processor` 字段内嵌使用。每次循环迭代中，[`apply_ideal_processors`](../apply.rs/README.md) 函数会：

1. 读取 `current_group` / `current_number` 作为"上一次的分配"。
2. 根据规则计算新的理想处理器。
3. 将旧值保存到 `previous_group` / `previous_number`。
4. 将新值写入 `current_group` / `current_number`。
5. 仅当新旧值不同时才调用 `SetThreadIdealProcessorEx`。

这种 current/previous 双缓冲设计避免了对已处于正确状态的线程进行冗余系统调用，减少了不必要的内核态切换。

该结构体派生了 `Clone` 和 `Copy`，可以按值传递。同时实现了 `Default` trait，`default()` 等效于 `new()`。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/scheduler.rs |
| **行号** | L206–L234 |
| **包含于** | [ThreadStats](ThreadStats.md) 的 `ideal_processor` 字段 |
| **消费者** | [`apply_ideal_processors`](../apply.rs/README.md)、[`apply_prime_threads_promote`](../apply.rs/apply_prime_threads.md) |

## 另请参阅

- [ThreadStats](ThreadStats.md)
- [PrimeThreadScheduler](PrimeThreadScheduler.md)
- [scheduler.rs 模块概述](README.md)
- [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) Windows API