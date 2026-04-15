# ThreadStats 类型 (scheduler.rs)

每线程统计信息和状态容器，由 `PrimeThreadScheduler` 使用，用于跟踪 CPU 周期活动、管理线程句柄、记录理想处理器分配，以及支持跨轮询迭代的基于滞后的主线程选择。每个 `ThreadStats` 实例对应于父进程中由其 TID 标识的单个操作系统线程。

## 语法

```rust
pub struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: List<[u32; CONSUMER_CPUS]>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `last_total_time` | `i64` | 此线程最近观测到的总（内核 + 用户）执行时间，以 100 纳秒为间隔。每个轮询迭代更新。 |
| `cached_total_time` | `i64` | 上一个轮询迭代的 `last_total_time` 值。用于计算每次迭代的时间增量。 |
| `last_cycles` | `u64` | 此线程最近观测到的 CPU 周期计数，由 `QueryThreadCycleTime` 或等效方法返回。 |
| `cached_cycles` | `u64` | 上一个轮询迭代的 `last_cycles` 值。差值 `last_cycles - cached_cycles` 给出用于主线程排名的每次迭代增量。 |
| `handle` | `Option<ThreadHandle>` | 可选的线程句柄容器。`None` 表示句柄尚未打开。当为 `Some` 时，其中的 `r_limited_handle` 始终有效；调用方在使用提升权限的句柄之前应检查 `is_valid_handle()`。`ThreadHandle` 的 `Drop` 实现会自动关闭操作系统句柄。 |
| `pinned_cpu_set_ids` | `List<[u32; CONSUMER_CPUS]>` | 此线程已被主线程引擎固定到的 CPU 集合 ID 的栈分配列表。当线程未被分配到任何主 CPU 集合时为空。容量受 `CONSUMER_CPUS` 常量限制。 |
| `active_streak` | `u8` | 此线程的周期增量连续超过进入阈值的轮询迭代次数计数器。由 `PrimeThreadScheduler::update_active_streaks` 使用以实现滞后机制——线程必须维持 `min_active_streak` 次迭代的活动才能被提升为主线程状态。上限为 254 以防止溢出。当周期低于保持阈值时重置为 0。 |
| `start_address` | `usize` | 线程的起始地址（入口点），用于通过 `resolve_address_to_module` 解析出发起的模块名称。对于基于前缀的线程过滤非常有用（例如，只提升从 `"game.dll"` 启动的线程）。 |
| `original_priority` | `Option<ThreadPriority>` | 线程首次被观测时其优先级级别的快照。存储此值以便服务能在线程失去主线程状态或进程退出时恢复原始优先级。如果尚未捕获则为 `None`。 |
| `last_system_thread_info` | `Option<SYSTEM_THREAD_INFORMATION>` | 来自 `NtQuerySystemInformation` 的此线程的最新 `SYSTEM_THREAD_INFORMATION` 快照。当进程退出且 `track_top_x_threads` 非零时，用于诊断报告。包含内核时间、用户时间、创建时间、等待原因、上下文切换次数和调度优先级。 |
| `ideal_processor` | `IdealProcessorState` | 跟踪此线程的当前和先前理想处理器分配。参见 [IdealProcessorState](IdealProcessorState.md)。 |
| `process_id` | `u32` | 父进程的 Windows PID。存储在此处以便自定义 `Debug` 实现可以在不需要外部上下文的情况下调用 `resolve_address_to_module`。 |

## 备注

- `ThreadStats` 实现了自定义的 `fmt::Debug` 特征，该特征使用 `resolve_address_to_module(process_id, start_address)` 将 `start_address` 解析为模块名称，使调试输出更具可读性。`handle` 和 `last_system_thread_info` 字段有意从调试输出中省略以保持简洁。
- `new(process_id)` 构造函数将所有数值字段初始化为零，所有 `Option` 字段初始化为 `None`，`pinned_cpu_set_ids` 初始化为空列表，`ideal_processor` 初始化为默认的 `IdealProcessorState`。
- `Default` 实现调用 `Self::new(0)`，适用于占位上下文，但这意味着 `process_id` 必须显式设置，或者必须通过 `PrimeThreadScheduler::get_thread_stats` 创建条目。
- `last_cycles` 和 `cached_cycles` 之间的增量是 `PrimeThreadScheduler::select_top_threads_with_hysteresis` 用于排名线程并决定哪些线程接收主 CPU 分配的主要指标。
- 线程句柄是延迟打开的。`handle` 字段保持为 `None`，直到 apply 引擎首次需要对线程调用 Win32 API（例如 `SetThreadAffinityMask`、`SetThreadIdealProcessorEx`、`SetThreadPriority`）。一旦打开，句柄在 `ThreadStats` 条目的整个生命周期中都会被重用，并在条目被丢弃时自动关闭。
- `CONSUMER_CPUS` 常量限制了每个线程可以不使用堆分配存储的 CPU 集合 ID 的最大数量，反映了典型消费级硬件的核心数量。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 调用方 | [PrimeThreadScheduler](PrimeThreadScheduler.md)、`apply` 模块（线程级 apply 函数） |
| 依赖 | [ThreadPriority](../priority.rs/ThreadPriority.md)、[IdealProcessorState](IdealProcessorState.md)、`winapi::ThreadHandle`、`winapi::resolve_address_to_module`、`collections::List`、`collections::CONSUMER_CPUS`、`ntapi::ntexapi::SYSTEM_THREAD_INFORMATION` |
| 平台 | Windows（依赖 NT 线程信息结构和 Win32 线程句柄） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ProcessStats | [ProcessStats](ProcessStats.md) |
| IdealProcessorState | [IdealProcessorState](IdealProcessorState.md) |
| ThreadPriority | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| format_100ns | [format_100ns](format_100ns.md) |
| format_filetime | [format_filetime](format_filetime.md) |
| scheduler 模块 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
