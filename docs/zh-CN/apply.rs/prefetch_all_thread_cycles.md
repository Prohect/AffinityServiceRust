# prefetch_all_thread_cycles 函数 (apply.rs)

`prefetch_all_thread_cycles` 函数打开进程中 CPU 使用率最高的线程的句柄，并通过 `QueryThreadCycleTime` 查询其硬件周期计数器。这将建立基准测量值，供后续的主线程选择算法使用，以计算每个线程的周期增量。该函数还会解析并缓存每个线程的起始地址，以便在提升阶段进行模块前缀匹配。查询周期后，它会更新滞后选择逻辑所使用的活跃连续计数器。

## 语法

```AffinityServiceRust/src/apply.rs#L584-595
pub fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于在调度器中查找线程统计信息以及在日志消息中进行错误去重。 |
| `config` | `&ThreadLevelConfig` | 线程级别配置。`name` 字段用于错误日志消息，并传递给 `get_thread_handle` 用于句柄获取。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 一个惰性闭包，返回一个线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射引用，数据来自最近的系统进程信息查询。该闭包在需要线程数据时被调用一次。`KernelTime` 和 `UserTime` 字段相加计算每个线程的总 CPU 时间，用于初始排序和增量计算。 |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态，存储每个线程的统计信息，包括缓存的周期数、上次周期数、总时间值、活跃连续次数、线程句柄和起始地址。此函数读取并更新调度器中每个线程统计信息的多个字段。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 错误消息的累加器。此函数仅记录 `QueryThreadCycleTime` 的失败信息；不产生变更消息。 |

## 返回值

此函数不返回值。所有结果通过对 `prime_scheduler` 线程统计信息的修改以及追加到 `apply_config_result` 的错误消息来传递。

## 备注

### 算法

1. **计算时间增量**：对于 `threads` 中的每个线程，函数计算 `KernelTime + UserTime` 并存储到 `thread_stats.cached_total_time` 中。它还会计算与上一个周期 `last_total_time` 之间的增量。结果被收集到一个固定容量的 `(tid, delta_time)` 元组列表中。

2. **死线程清理**：计算时间增量后，函数从调度器的 `tid_to_thread_stats` 映射中移除不再出现在活跃线程列表中的线程。死线程持有的句柄会被显式释放以防止句柄泄漏。此清理在管道的较早阶段执行，以确保句柄及时释放。

3. **按时间增量排序**：列表使用 `sort_unstable_by_key` 配合 `Reverse` 按时间增量降序排列，使 CPU 使用最活跃的线程排在最前面。

4. **限制候选数量**：仅保留前 `min(cpu_count * 2, thread_count).saturating_sub(1) + 1` 个线程用于周期查询。`saturating_sub(1)` 可防止线程数为零时的算术下溢。这将打开的线程句柄数量限制在大约 CPU 数量的两倍，足以覆盖主候选线程和一定余量的备选线程，而无需为可能拥有数百或数千个线程的进程打开所有线程的句柄。

5. **打开句柄并查询周期**：对于每个候选线程：
   - 如果调度器中尚未有缓存的句柄，则调用 `get_thread_handle` 打开一个句柄。该句柄存储在 `thread_stats.handle` 中，供后续应用周期复用。
   - 选择读取句柄（优先使用 `r_handle`，其次使用 `r_limited_handle`）。
   - 如果 `thread_stats.start_address` 为 `0`（尚未解析），则调用 `get_thread_start_address` 填充该值。
   - 使用读取句柄调用 `QueryThreadCycleTime` 获取当前硬件周期计数。成功时，值存储到 `thread_stats.cached_cycles` 中。失败时，Win32 错误码通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::QueryThreadCycleTime` 记录。

6. **计算周期增量**：所有候选线程查询完毕后，函数遍历进程的调度器线程统计信息，对每个 `cached_cycles` 非零的线程计算 `cached_cycles - last_cycles`（饱和减法）。`cached_cycles` 为零的线程（即句柄无法打开或周期查询失败的线程）的 `active_streak` 被重置为 `0`。

7. **更新活跃连续次数**：计算得到的周期增量传递给 `prime_scheduler.update_active_streaks`，该方法对具有正增量的线程递增连续计数器，对增量为零的线程重置计数器。活跃连续次数在 [`apply_prime_threads_select`](apply_prime_threads_select.md) 的滞后选择中使用，要求线程在提升前保持持续的活跃度。

### 副作用

- 为之前没有句柄的线程在 `prime_scheduler` 中打开并缓存操作系统线程句柄。
- 解析并缓存线程起始地址，以便在提升阶段进行模块名称解析。
- 更新调度器每个线程统计信息中的 `cached_total_time`、`cached_cycles` 和活跃连续计数器。
- 此函数**不会**对线程应用任何 CPU 集合、优先级或其他调度变更。它是一个纯粹的测量/预取阶段。

### 边界情况

- 如果 `threads` 为空或所有时间增量为零，函数在初始收集步骤后提前返回。
- 对于 `get_thread_handle` 返回 `None` 的线程（例如，线程已退出或访问被拒绝），将被静默跳过。句柄获取函数在内部记录自己的错误。
- 候选数量上限使用 `(cpu_count * 2).min(thread_count).saturating_sub(1)`，在 `thread_count` 为 0 时可安全避免算术下溢。
- `collections` 模块中的 `TIDS_CAPED` 和 `TIDS_FULL` 常量控制内部使用的固定大小列表的最大容量。

### 调用时机

此函数在每个进程的应用管道中于 [`apply_prime_threads`](apply_prime_threads.md) **之前**调用。它缓存的周期值被 `apply_prime_threads` 使用，以计算选择/提升/降级管道的增量。管道完成后，[`update_thread_stats`](update_thread_stats.md) 将缓存的值提交到 `last_cycles` 和 `last_total_time` 中，并清除缓存以供下一个周期使用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `QueryThreadCycleTime`、`GetLastError` |
| 调用者 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用者 | [`log_error_if_new`](log_error_if_new.md)、`winapi::get_thread_handle`、`winapi::get_thread_start_address`、`winapi::get_cpu_set_information`、`PrimeThreadScheduler::get_thread_stats`、`PrimeThreadScheduler::update_active_streaks`、`error_codes::error_from_code_win32` |
| 权限 | 需要具有 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION` 权限的线程句柄才能调用 `QueryThreadCycleTime`。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| update_thread_stats | [`update_thread_stats`](update_thread_stats.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*