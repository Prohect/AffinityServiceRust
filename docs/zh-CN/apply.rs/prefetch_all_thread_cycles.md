# prefetch_all_thread_cycles 函数 (apply.rs)

`prefetch_all_thread_cycles` 函数打开进程中 CPU 消耗最高的线程的句柄，并通过 `QueryThreadCycleTime` 查询其硬件周期计数器。这将建立基线测量值，供后续主线程选择算法计算每线程周期增量时使用。该函数还会解析并缓存每个线程的起始地址，以便在提升阶段进行模块前缀匹配。查询周期后，它会更新滞后选择逻辑所使用的活跃连续计数器。

## 语法

```AffinityServiceRust/src/apply.rs#L584-594
pub fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于在调度器中查找线程统计信息，以及在日志消息中进行错误去重。 |
| `config` | `&ThreadLevelConfig` | 线程级别配置。`name` 字段用于错误日志消息，并传递给 `get_thread_handle` 用于句柄获取。 |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射，来自最近一次系统进程信息查询。`KernelTime` 和 `UserTime` 字段之和用于计算每个线程的总 CPU 时间，用于初始排序和增量计算。 |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态，存储每线程统计信息，包括缓存周期、上次周期、总时间值、活跃连续计数、线程句柄和起始地址。此函数读取并更新调度器中每线程统计信息的多个字段。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 错误消息累加器。此函数仅记录 `QueryThreadCycleTime` 失败；不产生变更消息。 |

## 返回值

此函数不返回值。所有结果通过修改 `prime_scheduler` 线程统计信息以及向 `apply_config_result` 追加错误消息来传递。

## 备注

### 算法

1. **计算时间增量**：对于 `threads` 中的每个线程，函数计算 `KernelTime + UserTime` 并存储在 `thread_stats.cached_total_time` 中。同时计算与上一周期 `last_total_time` 的增量。结果收集到一个固定容量的 `(tid, delta_time)` 元组列表中。

2. **按时间增量排序**：列表使用 `sort_unstable_by_key` 配合 `Reverse` 按时间增量降序排序，使 CPU 活动最高的线程排在最前面。

3. **限制候选数量**：仅保留前 `min(cpu_count * 2, thread_count) - 1 + 1` 个线程用于周期查询。这将打开的线程句柄数限制在大约 CPU 数量的两倍，足以涵盖主候选线程和一定数量的备选线程，而无需为可能拥有数百或数千个线程的进程中的每个线程打开句柄。

4. **打开句柄并查询周期**：对于每个候选线程：
   - 如果该线程在调度器中尚未缓存句柄，则调用 `get_thread_handle` 打开一个。句柄存储在 `thread_stats.handle` 中，以便在后续应用周期中重用。
   - 选择读取句柄（优先使用 `r_handle`，回退到 `r_limited_handle`）。
   - 如果 `thread_stats.start_address` 为 `0`（尚未解析），则调用 `get_thread_start_address` 进行填充。
   - 使用读取句柄调用 `QueryThreadCycleTime` 获取当前硬件周期计数。成功时，值存储在 `thread_stats.cached_cycles` 中。失败时，Win32 错误代码通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::QueryThreadCycleTime` 记录。

5. **计算周期增量**：在所有候选线程被查询后，函数遍历该进程的调度器线程统计信息，为每个 `cached_cycles` 非零的线程计算 `cached_cycles - last_cycles`（饱和减法）。`cached_cycles` 为零的线程（即句柄无法打开或周期查询失败的线程）其 `active_streak` 将被重置为 `0`。

6. **更新活跃连续计数**：计算出的周期增量传递给 `prime_scheduler.update_active_streaks`，该方法为具有正增量的线程递增连续计数器，为增量为零的线程重置计数器。活跃连续计数在 [`apply_prime_threads_select`](apply_prime_threads_select.md) 的滞后选择中用于要求持续活动后才能提升。

### 副作用

- 在 `prime_scheduler` 中为之前没有句柄的线程打开并缓存操作系统线程句柄。
- 解析并缓存线程起始地址，以便在提升阶段进行模块名称解析。
- 更新调度器中每线程统计信息的 `cached_total_time`、`cached_cycles` 和活跃连续计数器。
- 此函数**不会**对线程应用任何 CPU 集合、优先级或其他调度更改。它是一个纯粹的测量/预取阶段。

### 边界情况

- 如果 `threads` 为空或所有时间增量为零，函数在初始收集步骤后提前返回。
- 对于 `get_thread_handle` 返回 `None` 的线程（例如线程已退出或访问被拒绝），将被静默跳过。句柄获取函数会在内部记录自己的错误。
- 候选上限使用 `(cpu_count * 2).min(thread_count) - 1`，如果 `thread_count` 为 0 可能会下溢，但前面的空检查阻止了这一路径。
- `collections` 模块中的 `TIDS_CAPED` 和 `TIDS_FULL` 常量控制内部使用的固定大小列表的最大容量。

### 调用时机

此函数在每个进程应用管道中，在 [`apply_prime_threads`](apply_prime_threads.md) **之前**调用。它缓存的周期值由 `apply_prime_threads` 使用，用于计算选择/提升/降级管道的增量。管道完成后，[`update_thread_stats`](update_thread_stats.md) 将缓存的值提交到 `last_cycles` 和 `last_total_time`，并清除缓存以备下一周期使用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `QueryThreadCycleTime`、`GetLastError` |
| 调用方 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用方 | [`log_error_if_new`](log_error_if_new.md)、`winapi::get_thread_handle`、`winapi::get_thread_start_address`、`winapi::get_cpu_set_information`、`PrimeThreadScheduler::get_thread_stats`、`PrimeThreadScheduler::update_active_streaks`、`error_codes::error_from_code_win32` |
| 权限 | 需要具有 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION` 权限的线程句柄，用于 `QueryThreadCycleTime`。 |

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
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*