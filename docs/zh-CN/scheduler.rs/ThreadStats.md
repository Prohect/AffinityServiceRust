# ThreadStats struct (scheduler.rs)

每线程运行时跟踪数据。`ThreadStats` 存储线程的 CPU 周期计数、时间统计、句柄缓存、活跃连续计数以及理想处理器分配状态，供 [PrimeThreadScheduler](PrimeThreadScheduler.md) 在每次循环迭代中进行增量计算和 prime 线程选择。

## 语法

```rust
pub struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}
```

## 成员

`last_total_time`

类型：`i64`

上一次迭代结束时记录的线程总时间（内核时间 + 用户时间），单位为 100 纳秒间隔。用于与当前迭代的值做差，计算增量。

`cached_total_time`

类型：`i64`

当前迭代中从系统查询到的线程总时间快照。在 [`update_thread_stats`](../apply.rs/update_thread_stats.md) 阶段被持久化到 `last_total_time`，为下一次迭代的增量计算做准备。

`last_cycles`

类型：`u64`

上一次迭代结束时记录的线程 CPU 周期计数。通过 `QueryThreadCycleTime` 获取的累计值。

`cached_cycles`

类型：`u64`

当前迭代中查询到的线程 CPU 周期计数快照。与 `last_cycles` 的差值即为本迭代的增量周期数（delta cycles），是 prime 线程排序的核心指标。

`handle`

类型：`Option<`[ThreadHandle](../winapi.rs/ThreadHandle.md)`>`

缓存的线程句柄容器。`None` 表示尚未打开。当存在时，`r_limited_handle` 始终有效；使用其他句柄前应检查有效性。`ThreadHandle` 实现了 `Drop` trait，在 `ThreadStats` 被清理时自动关闭句柄。

`pinned_cpu_set_ids`

类型：`Vec<u32>`

当线程被提升为 prime 线程时，通过 `SetThreadSelectedCpuSets` 分配的 CPU 集 ID 列表。降级时用于恢复原始 CPU 集分配。

`active_streak`

类型：`u8`

活跃连续计数器（0–254）。当线程的增量周期数超过进入阈值（entry threshold，默认 0.42）时递增，低于保持阈值（keep threshold，默认 0.69）时重置为 0。[PrimeThreadScheduler](PrimeThreadScheduler.md) 要求 `active_streak >= min_active_streak` 才允许新线程被提升为 prime 状态，防止短暂活跃的线程被错误提升。

`start_address`

类型：`usize`

线程的起始地址，通过 `NtQueryInformationThread` 获取。用于在进程退出时通过 [`resolve_address_to_module`](../winapi.rs/resolve_address_to_module.md) 将地址解析为模块名称，以便在诊断报告中标识线程来源。

`original_priority`

类型：`Option<`[ThreadPriority](../priority.rs/ThreadPriority.md)`>`

线程被提升为 prime 之前的原始优先级。在线程降级时用于恢复其优先级。`None` 表示线程从未被提升过。

`last_system_thread_info`

类型：`Option<SYSTEM_THREAD_INFORMATION>`

最后一次从 [ProcessSnapshot](../process.rs/ProcessSnapshot.md) 快照中获取的系统线程信息。包含内核时间、用户时间、创建时间、等待时间、上下文切换次数、线程状态和等待原因等字段。在进程退出时用于 [`close_dead_process_handles`](PrimeThreadScheduler.md) 中的诊断报告输出。

`ideal_processor`

类型：[IdealProcessorState](IdealProcessorState.md)

理想处理器分配跟踪状态。记录当前和上一次分配的处理器组号与编号，以及是否已完成分配。

`process_id`

类型：`u32`

此线程所属进程的 PID。用于 `Debug` trait 实现中通过 `resolve_address_to_module` 解析起始地址时提供进程上下文。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **new** | `pub fn new(process_id: u32) -> Self` | 创建一个新的 `ThreadStats`，所有计数器归零，`handle` 和 `original_priority` 为 `None`。 |

## 备注

### 增量计算模式

`ThreadStats` 采用双缓冲模式进行增量计算：

1. **预取阶段**（[`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md)）：查询 `QueryThreadCycleTime` 和系统线程信息，将结果写入 `cached_cycles` 和 `cached_total_time`。
2. **选择阶段**：`cached_cycles - last_cycles` 计算出增量周期数，用于排序和 prime 线程选择。
3. **持久化阶段**（[`update_thread_stats`](../apply.rs/update_thread_stats.md)）：将 `cached_*` 值复制到 `last_*`，为下一次迭代做准备。

### 句柄生命周期

`handle` 字段缓存的 `ThreadHandle` 在首次需要时懒惰打开，此后在整个进程生命周期内复用。当进程退出后，[`close_dead_process_handles`](PrimeThreadScheduler.md) 清理 `ProcessStats` 时，`ThreadStats` 被 drop，其中的 `ThreadHandle` 通过 `Drop` trait 自动关闭所有内核句柄。

### Debug 实现

`ThreadStats` 提供自定义 `fmt::Debug` 实现，而非使用 `#[derive(Debug)]`。这是因为 `handle` 字段（`Option<ThreadHandle>`）和 `last_system_thread_info` 字段（`Option<SYSTEM_THREAD_INFORMATION>`）不适合直接输出。自定义实现会将 `start_address` 通过 `resolve_address_to_module` 解析为可读的模块名称。

### Default 实现

`Default::default()` 等价于 `ThreadStats::new(0)`，`process_id` 默认为 0。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/scheduler.rs |
| **行号** | L236–L300 |
| **依赖** | [ThreadHandle](../winapi.rs/ThreadHandle.md)、[ThreadPriority](../priority.rs/ThreadPriority.md)、[IdealProcessorState](IdealProcessorState.md)、`SYSTEM_THREAD_INFORMATION`（ntapi） |
| **消费者** | [PrimeThreadScheduler](PrimeThreadScheduler.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) |

## 另请参阅

- [PrimeThreadScheduler](PrimeThreadScheduler.md) — 拥有并管理 `ThreadStats` 实例
- [ProcessStats](ProcessStats.md) — 通过 `tid_to_thread_stats` 映射包含 `ThreadStats`
- [IdealProcessorState](IdealProcessorState.md) — 理想处理器分配跟踪
- [ThreadPriority 枚举](../priority.rs/ThreadPriority.md) — `original_priority` 的类型
- [ThreadHandle](../winapi.rs/ThreadHandle.md) — 缓存的线程句柄容器