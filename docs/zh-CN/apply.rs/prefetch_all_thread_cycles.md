# prefetch_all_thread_cycles 函数 (apply.rs)

查询进程中 CPU 消耗最高的线程的周期计时，建立驱动基于滞后机制的主力线程提升和降级算法的基线测量值。此函数打开线程句柄，通过 `QueryThreadCycleTime` 读取周期计数器，并更新 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 中缓存的周期/时间值，以便下游函数可以计算每个线程的 CPU 增量。

## 语法

```AffinityServiceRust/src/apply.rs#L602-612
pub fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用作调度器按进程统计映射表的键，以及用于获取线程句柄。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 该进程的已解析配置。当线程句柄打开或周期时间查询失败时，`name` 字段用于错误消息。 |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | 目标进程的快照条目。提供线程列表以及来自最近一次 `NtQuerySystemInformation` 快照的每线程内核/用户时间数据。 |
| `prime_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 持有每线程 [ThreadStats](../scheduler.rs/ThreadStats.md) 的调度器状态。此函数将 `cached_total_time`、`cached_cycles`、`start_address` 和线程句柄写入统计条目。函数末尾还会调用 `update_active_streaks` 来维护滞后算法使用的连续活跃计数器。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 错误消息累加器。此函数不记录变更——仅在 `QueryThreadCycleTime` 或句柄获取失败时记录错误。 |

## 返回值

无 (`()`)。结果写入 `prime_scheduler`（缓存的周期/时间值、线程句柄、活跃连续计数）并将错误累积到 `apply_config_result` 中。

## 备注

### 算法

该函数分三个阶段运行：

**阶段 1 — 收集并按内核+用户时间增量排序。**
从 `process.get_threads()` 收集所有线程到一个 `Vec<(tid, delta_time)>` 中，其中 `delta_time` 是当前总 CPU 时间（`KernelTime + UserTime`）与线程 [ThreadStats](../scheduler.rs/ThreadStats.md) 中存储的 `last_total_time` 之间的差值。当前总时间也会写入 `cached_total_time`，以便稍后由 [update_thread_stats](update_thread_stats.md) 提交。该向量按时间增量降序排列，以便最活跃的线程优先处理。

**阶段 2 — 打开句柄并查询周期计数器。**
函数按排序后的线程列表进行迭代，上限为 `min(cpu_count * 2, thread_count) - 1`，其中 `cpu_count` 是系统 CPU 集合信息中的条目数。对于每个线程：

1. 如果该线程在其 [ThreadStats](../scheduler.rs/ThreadStats.md) 中尚未有打开的句柄，则通过 [get_thread_handle](../winapi.rs/get_thread_handle.md) 获取一个 [ThreadHandle](../winapi.rs/ThreadHandle.md)。该句柄会持久存储在 `thread_stats.handle` 中，以便在轮询周期间复用。
2. 选择最佳可用的读取句柄（`r_handle`，回退到 `r_limited_handle`）。
3. 如果线程的 `start_address` 为 `0`，则通过 [get_thread_start_address](../winapi.rs/get_thread_start_address.md) 解析。起始地址稍后由 [apply_prime_threads_promote](apply_prime_threads_promote.md) 和 [apply_ideal_processors](apply_ideal_processors.md) 用于解析线程的起始模块名以进行前缀匹配。
4. 调用 `QueryThreadCycleTime` 读取线程的累积周期计数。成功时，值写入 `cached_cycles`。失败时，错误通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::QueryThreadCycleTime` 记录。

**阶段 3 — 计算周期增量并更新活跃连续计数。**
所有句柄查询完成后，函数从调度器中该 pid 的所有具有非零 `cached_cycles` 的线程统计构建一个 `Vec<(tid, delta_cycles)>`，计算 `delta_cycles = cached_cycles - last_cycles`。`cached_cycles` 为零（未被查询或查询失败）的线程的 `active_streak` 会重置为 `0`。增量向量传递给 `prime_scheduler.update_active_streaks(pid, &tid_with_delta_cycles)`，该方法递增或重置[滞后选择算法](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm)使用的每线程连续计数器。

### 线程句柄缓存

与 [reset_thread_ideal_processors](reset_thread_ideal_processors.md) 在单次调用内打开和关闭线程句柄不同，`prefetch_all_thread_cycles` 将打开的句柄存储在 `thread_stats.handle` 中以便在轮询周期间复用。这避免了对持续活跃的线程反复打开和关闭句柄的开销。不再存在的线程的过期句柄由 [apply_prime_threads](apply_prime_threads.md) 在提升/降级流水线完成后清理。

### 计数器限制

迭代上限 `min(cpu_count * 2, thread_count) - 1` 确保函数查询的线程数量与系统核心数成比例。在 16 核系统上，这意味着最多查询 31 个线程（CPU 最活跃的那些）。这限制了对拥有数百或数千个线程的进程进行周期时间查询的开销，同时仍然覆盖了比可提升为主力线程状态更多的候选线程。

### 与 apply_prime_threads 的关系

在每个轮询周期中，`prefetch_all_thread_cycles` 必须在 [apply_prime_threads](apply_prime_threads.md) *之前*调用。预取操作将 `cached_cycles` 和 `cached_total_time` 填充到调度器中；`apply_prime_threads` 读取这些缓存值来计算增量并驱动选择算法。两者完成后，[update_thread_stats](update_thread_stats.md) 将缓存值提交到 `last_cycles` 和 `last_total_time` 以供下一次迭代使用。

### 活跃连续计数跟踪

[ThreadStats](../scheduler.rs/ThreadStats.md) 中的活跃连续计数器跟踪线程在多少个连续轮询周期中显示了非零周期活动。[滞后选择算法](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm)使用此计数器来防止短暂的 CPU 突发立即将线程提升为主力线程状态——线程必须在至少 `min_active_streak` 个连续周期内维持活动（在 [ConfigConstants](../config.rs/ConfigConstants.md) 中配置）才有资格被提升。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| 被调用者 | [get_thread_handle](../winapi.rs/get_thread_handle.md)、[get_thread_start_address](../winapi.rs/get_thread_start_address.md)、[get_cpu_set_information](../winapi.rs/get_cpu_set_information.md)、[log_error_if_new](log_error_if_new.md)、[PrimeThreadScheduler::get_thread_stats](../scheduler.rs/PrimeThreadScheduler.md)、[PrimeThreadScheduler::update_active_streaks](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | [`QueryThreadCycleTime`](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) |
| 权限 | `THREAD_QUERY_LIMITED_INFORMATION`（用于 `QueryThreadCycleTime` 和 `NtQueryInformationThread`）。`SeDebugPrivilege` 用于跨进程线程访问。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主力线程编排（消费预取的数据） | [apply_prime_threads](apply_prime_threads.md) |
| 周期完成后提交缓存计数器 | [update_thread_stats](update_thread_stats.md) |
| 基于滞后机制的线程选择 | [apply_prime_threads_select](apply_prime_threads_select.md) |
| 调度器状态和连续计数跟踪 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 每线程统计模型 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 线程句柄包装器 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| apply 模块概览 | [apply](README.md) |