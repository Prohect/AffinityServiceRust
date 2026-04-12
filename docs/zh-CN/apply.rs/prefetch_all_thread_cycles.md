# prefetch_all_thread_cycles 函数 (apply.rs)

预取线程周期计数，用于 prime 线程选择。打开 CPU 消耗最高的线程（按内核+用户时间增量排序）的句柄，并通过 `QueryThreadCycleTime` 查询其周期计数器。这为基于滞后的 prime 线程提升/降级算法建立基线测量值。

## 语法

```rust
pub fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含进程规则配置。用于在记录错误和打开线程句柄时提供进程名称。

`process`

指向目标进程的 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用。提供来自系统快照的线程枚举及其内核和用户时间数据。

`prime_scheduler`

指向 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，保存每线程的缓存周期计数、总时间统计、起始地址、句柄和活跃连击计数器。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集错误消息。

## 返回值

此函数不返回值。它作为副作用更新 `prime_scheduler` 中的状态。

## 备注

### 算法

1. **收集时间增量** — 遍历 `process` 中的所有线程，计算每个线程的总 CPU 时间（内核 + 用户），缓存到调度器的 `cached_total_time` 中，并计算与 `last_total_time` 的增量。
2. **按时间增量排序** — 按时间增量降序排序线程，使最活跃的线程最先被处理。
3. **限制候选数量** — 仅处理前 N 个线程，其中 N = `逻辑 CPU 数量 × 2`（来自系统 CPU 集信息）。这避免了为空闲线程打开句柄。
4. **打开句柄** — 对于尚未持有打开句柄的每个候选线程，调用 [get_thread_handle](../winapi.rs/get_thread_handle.md) 获取 `ThreadHandle`。如果线程的 `start_address` 尚未解析，则通过 `get_thread_start_address` 查询。
5. **查询周期计数** — 对每个候选线程的读取句柄调用 `QueryThreadCycleTime`，将结果存储到 `cached_cycles` 中。失败时通过 [log_error_if_new](log_error_if_new.md) 记录错误。
6. **计算周期增量** — 查询完成后，通过 `cached_cycles` 减去 `last_cycles` 构建 `(tid, delta_cycles)` 元组列表。缓存周期为零的线程会将其 `active_streak` 重置为 0。
7. **更新活跃连击** — 使用周期增量列表调用 `prime_scheduler.update_active_streaks()`。活跃连击稍后将被 [apply_prime_threads_select](apply_prime_threads_select.md) 中的滞后选择算法使用。

### 句柄复用

预取期间打开的线程句柄存储在 `ThreadStats.handle` 中，并在各迭代间复用。它们仅在线程退出时（在 [apply_prime_threads](apply_prime_threads.md) 中检测到）或 `ThreadHandle` 被 drop 时关闭。

### 与 prime 线程调度的关系

此函数是 [apply_prime_threads](apply_prime_threads.md) 的前置步骤。必须首先调用此函数以填充 `cached_cycles` 和 `cached_total_time`，以便 prime 线程选择及提升/降级逻辑拥有准确、最新的周期数据。此处计算的周期增量构成滞后比较的基础，用于确定哪些线程被提升到 prime CPU。

### 错误处理

- 如果 `get_thread_handle` 失败，该线程将被静默跳过（错误已在 winapi 模块内部记录）。
- 如果 `QueryThreadCycleTime` 失败，通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::QueryThreadCycleTime` 记录错误，该线程的周期数据不会被更新。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **行号** | L597–L702 |
| **调用者** | [apply_config](../main.rs/apply_config.md)（main.rs） |
| **调用** | [get_thread_handle](../winapi.rs/get_thread_handle.md)、`get_thread_start_address`、`QueryThreadCycleTime`、[log_error_if_new](log_error_if_new.md)、`PrimeThreadScheduler::update_active_streaks` |
| **Windows API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread)（通过 get_thread_handle）、[QueryThreadCycleTime](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) |

## 另请参阅

- [apply_prime_threads](apply_prime_threads.md)
- [apply_prime_threads_select](apply_prime_threads_select.md)
- [update_thread_stats](update_thread_stats.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)