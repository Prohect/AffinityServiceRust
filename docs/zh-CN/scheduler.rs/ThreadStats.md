# ThreadStats 结构体 (scheduler.rs)

[PrimeThreadScheduler](PrimeThreadScheduler.md) 使用的线程级统计数据与状态，用于跟踪 CPU 周期增量、活跃连续计数、线程句柄、CPU 集合绑定、理想处理器分配以及原始优先级，以实现带滞后算法的主力线程调度。

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

| 成员 | 类型 | 描述 |
|------|------|------|
| `last_total_time` | `i64` | 该线程最近观测到的总时间（内核时间 + 用户时间），单位为 100 纳秒间隔。每个调度周期从 `SYSTEM_THREAD_INFORMATION` 更新。与 `cached_total_time` 配合用于计算增量。 |
| `cached_total_time` | `i64` | 上一个调度周期的总时间值。增量（`last_total_time - cached_total_time`）表示上一个间隔内消耗的 CPU 时间。 |
| `last_cycles` | `u64` | 该线程最近观测到的 CPU 周期计数，通过 `QueryThreadCycleTime` 获取。与 `cached_cycles` 配合用于计算每个间隔的周期增量，以对线程进行排名。 |
| `cached_cycles` | `u64` | 上一个调度周期的 CPU 周期计数。增量（`last_cycles - cached_cycles`）是 [select_top_threads_with_hysteresis](PrimeThreadScheduler.md) 用于对线程进行排名以确定主力核心晋升的主要指标。 |
| `handle` | `Option<`[ThreadHandle](../winapi.rs/ThreadHandle.md)`)>` | 线程句柄容器。`None` 表示句柄尚未打开。当为 `Some` 时，`r_limited_handle` 始终有效。其他句柄（`r_handle`、`w_limited_handle`、`w_handle`）在使用前应通过 `is_valid_handle()` 进行检查。当 `ThreadStats` 被销毁时，句柄会通过 `ThreadHandle` 的 `Drop` 实现自动关闭。 |
| `pinned_cpu_set_ids` | `Vec<u32>` | 通过 `SetThreadSelectedCpuSets` 当前分配给该线程的 CPU 集合 ID。空向量表示线程继承其进程级别的默认 CPU 集合。当线程被晋升为主力状态时，它会被绑定到性能核心的 CPU 集合 ID；当被降级时，该向量被清空以恢复默认行为。 |
| `active_streak` | `u8` | 该线程连续超过入场阈值的调度间隔数。必须达到 [ConfigConstants](../config.rs/ConfigConstants.md)`.min_active_streak` 后线程才有资格晋升为主力线程。当周期数降至保持阈值以下时重置为 `0`。上限为 `254`。 |
| `start_address` | `usize` | 线程入口点的起始地址，通过 `NtQueryInformationThread(ThreadQuerySetWin32StartAddress)` 获取。用于诊断日志记录和通过 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) 进行模块名解析。 |
| `original_priority` | `Option<`[ThreadPriority](../priority.rs/ThreadPriority.md)`)>` | 线程晋升前的优先级。在线程首次被晋升到主力核心时捕获，以便在降级时恢复。`None` 表示线程从未被晋升或已完全降级。 |
| `last_system_thread_info` | `Option<SYSTEM_THREAD_INFORMATION>` | 来自 [ProcessSnapshot](../process.rs/ProcessSnapshot.md) 的最后一次 `SYSTEM_THREAD_INFORMATION` 的缓存副本。由 [drop_process_by_pid](PrimeThreadScheduler.md) 用于生成详细的退出报告，包括内核时间、用户时间、创建时间、上下文切换次数、优先级、线程状态和等待原因。 |
| `ideal_processor` | [IdealProcessorState](IdealProcessorState.md) | 跟踪该线程当前和先前的理想处理器分配。由 `apply_ideal_processors` 用于检测变更并避免冗余的 `SetThreadIdealProcessorEx` 调用。 |
| `process_id` | `u32` | 拥有该线程的进程的 PID。存储用于调试格式化（模块名解析）并传递给 `resolve_address_to_module`。 |

## 备注

### 线程生命周期

`ThreadStats` 实例在首次通过 [PrimeThreadScheduler::get_thread_stats](PrimeThreadScheduler.md) 访问时创建，所有数值字段初始化为零，向量为空，可选字段为 `None`。拥有它的 `PrimeThreadScheduler` 会逐步填充各字段：

1. **周期预取** — `apply.rs` 中的 `prefetch_all_thread_cycles` 打开线程句柄（填充 `handle`），查询 `QueryThreadCycleTime`（更新 `last_cycles` / `cached_cycles`），并记录 `start_address` 和 `last_system_thread_info`。
2. **连续计数更新** — [update_active_streaks](PrimeThreadScheduler.md) 根据相对于阈值的周期增量递增或重置 `active_streak`。
3. **选择** — [select_top_threads_with_hysteresis](PrimeThreadScheduler.md) 读取 `active_streak` 并检查当前分配状态以决定主力晋升。
4. **晋升** — `apply_prime_threads_promote` 将线程绑定到性能核心（填充 `pinned_cpu_set_ids`），保存 `original_priority`，并可选地提升线程优先级。
5. **降级** — `apply_prime_threads_demote` 清空 `pinned_cpu_set_ids`，恢复 `original_priority`，并重置 CPU 集合分配。
6. **清理** — 当拥有该线程的进程退出时，[drop_process_by_pid](PrimeThreadScheduler.md) 记录最终统计数据，销毁所有 `ThreadHandle`（关闭操作系统句柄），并移除进程条目。

### 周期增量计算

双字段模式（`last_*` / `cached_*`）实现了一个简单的双缓冲方案。在每个调度周期开始时，`cached_cycles` 被设置为之前的 `last_cycles`，然后 `last_cycles` 用 `QueryThreadCycleTime` 的当前值进行更新。增量（`last_cycles - cached_cycles`）表示一个调度间隔内消耗的周期数，是线程排名的主要排序键。

### 调试格式化

`ThreadStats` 实现了自定义的 `fmt::Debug`，通过 `resolve_address_to_module(process_id, start_address)` 将 `start_address` 解析为模块名，以提供人类可读的输出。`handle` 字段被排除在调试输出之外。

### 线程安全性

`ThreadStats` 默认不是 `Send` 或 `Sync` 的，因为它包含 `Option<ThreadHandle>`（它包装了原始 `HANDLE` 值）。所有访问都通过 `PrimeThreadScheduler` 进行中介，而该调度器只在主服务循环线程上被访问。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 调用方 | [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[update_thread_stats](../apply.rs/update_thread_stats.md) |
| 依赖 | [ThreadHandle](../winapi.rs/ThreadHandle.md)、[ThreadPriority](../priority.rs/ThreadPriority.md)、[IdealProcessorState](IdealProcessorState.md)、`SYSTEM_THREAD_INFORMATION` (ntapi) |
| API | `QueryThreadCycleTime`、`SetThreadSelectedCpuSets`、`SetThreadIdealProcessorEx` |
| 权限 | `SeDebugPrivilege`（用于跨安全边界打开线程句柄） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 调度器概述 | [scheduler.rs 概述](README.md) |
| 理想处理器状态 | [IdealProcessorState](IdealProcessorState.md) |
| 进程级统计 | [ProcessStats](ProcessStats.md) |
| 主力线程调度器 | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| 线程句柄包装器 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| 线程优先级枚举 | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| 滞后算法常量 | [ConfigConstants](../config.rs/ConfigConstants.md) |