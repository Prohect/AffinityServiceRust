# prefetch_all_thread_cycles 函数 (apply.rs)

为进程的顶级 CPU 消耗线程预取线程周期计数，为基于迟滞的 Prime 线程提升/降级算法建立基线测量。

此函数按内核 + 用户时间对线程排序并打开句柄，通过 `QueryThreadCycleTime` 查询其周期计数器，从缓存值计算周期增量，并将活动 streak 信息更新到 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)。它还解析线程起始地址以供后续的模块匹配使用。

## 语法

```AffinityServiceRust/src/apply.rs#L585-591
pub fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

正在测量的进程 ID。

`config: &ThreadLevelConfig`

进程的线程级配置。`name` 字段用于错误日志记录。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

返回线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 结构（来自 `NtQuerySystemInformation`）的映射的惰性访问器。

`prime_scheduler: &mut PrimeThreadScheduler`

调度器的可变引用，存储每线程周期/时间缓存、活跃 streak 和线程句柄。使用新测量值原地更新。

`apply_config_result: &mut ApplyConfigResult`

周期时间查询期间遇到的错误的累积器。

## 返回值

此函数不返回任何值。结果作为副作用存储在 `prime_scheduler` 中：

- `cached_cycles` — 每线程的当前原始周期计数。
- `cached_total_time` — 每线程的当前内核 + 用户时间。
- 通过 [`PrimeThreadScheduler::update_active_streaks`](../scheduler.rs/PrimeThreadScheduler.md) 更新活跃 streak。

## 备注

### 算法

1. **计算时间增量** — 对于进程中的每个线程，计算 `cached_total_time`（内核 + 用户）以及与 `last_total_time` 的增量。将结果存储在按时间增量降序排序的列表中。

2. **清理死线程** — 从不存在的线程中移除 `pid_to_process_stats` 中的条目。丢弃死线程的缓存线程句柄以避免句柄泄漏。

3. **限制候选计数** — 仅处理顶级线程，上限为 `min(cpu_count * 2, thread_count)`。这限制了具有数百或数千个线程的进程的开销。

4. **打开线程句柄** — 对于每个候选线程，如果 `thread_stats.handle` 中尚未缓存，则通过 [`get_thread_handle`](../winapi.rs/get_thread_handle.md) 打开线程句柄。句柄被缓存以便在迭代中重复使用。

5. **解析起始地址** — 如果 `thread_stats.start_address` 为零，则通过 [`get_thread_start_address`](../winapi.rs/get_thread_start_address.md) 查询线程起始地址。这用于 Prime 线程提升期间的模块前缀匹配。

6. **查询周期时间** — 调用 `QueryThreadCycleTime` 读取线程的 CPU 周期计数器。该值存储在 `cached_cycles` 中。

7. **计算周期增量** — 所有查询完成后，计算每个具有非零缓存周期的线程的 `cached_cycles - last_cycles`。具有零缓存周期的线程的 `active_streak` 重置为 0。

8. **更新活跃 streak** — 使用周期增量列表调用 `PrimeThreadScheduler::update_active_streaks`。其周期超过保持阈值的线程的 streak 递增；其他线程重置。

### 线程句柄缓存

线程句柄仅打开一次并存储在 `thread_stats.handle` 中。这避免了对每个轮询间隔重复调用 `OpenThread`。当线程从统计映射中清理时，句柄会自动释放。

### 候选池大小

候选池大小为 `cpu_count * 2`（基于 CPU 集合信息），确保跟踪足够的线程以处理变化而不会产生过多开销。池至少包含线程数减一。

### 平台说明

- `QueryThreadCycleTime` 返回 CPU 周期（而非墙钟时间），提供线程活动的高分辨率、与调度无关的度量。
- 函数优先使用完全访问读句柄（`r_handle`）而非限制句柄（`r_limited_handle`）。

## 要求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | [`apply_prime_threads`](apply_prime_threads.md)，主轮询循环 |
| **被调用方** | [`get_thread_handle`](../winapi.rs/get_thread_handle.md)，[`get_thread_start_address`](../winapi.rs/get_thread_start_address.md)，[`PrimeThreadScheduler::update_active_streaks`](../scheduler.rs/PrimeThreadScheduler.md)，[`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | [`QueryThreadCycleTime`](https://learn.microsoft.com/zh-cn/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) |
| **权限** | 目标线程上的 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION` |

## 另请参阅

| 主题 | 说明 |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | Prime 线程选择前调用此函数的编排器 |
| [apply_prime_threads_select](apply_prime_threads_select.md) | 使用此处计算的周期增量进行基于迟滞的选择 |
| [update_thread_stats](update_thread_stats.md) | 应用周期完成后将缓存值复制到 `last_*` 字段 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 存储所有每线程统计信息的调度器结构 |
| [ApplyConfigResult](ApplyConfigResult.md) | 更改和错误的累积器 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*