# apply_prime_threads 函数 (apply.rs)

通过识别 CPU 密集型线程并将它们固定到指定的"prime"CPU 上，以改进缓存局部性和性能，来协调进程的 Prime 线程调度。这是 Prime 线程子系统的顶级入口点，协调选择、提升和降级阶段。

## 语法

```AffinityServiceRust/src/apply.rs#L708-718
pub fn apply_prime_threads<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

### `pid`

目标进程的进程 ID。

### `config`

对包含 Prime 线程调度设置的 [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) 的引用，包括 `prime_threads_cpus`、`prime_threads_prefixes`、`track_top_x_threads` 和 `ideal_processor_rules`。

### `dry_run`

如果为 `true`，函数会记录 `apply_config_result` 中的预期更改，而不会调用任何 Windows API。仅记录初始 prime CPU 描述。

### `current_mask`

对当前进程亲和性掩码的可变引用。传递给 [`apply_prime_threads_promote`](apply_prime_threads_promote.md)，它使用此掩码将 prime CPU 集合与进程亲和性进行过滤。

### `process`

对目标进程 [`ProcessEntry`](../process.rs/ProcessEntry.md) 的引用。用于获取线程总数以进行候选池大小调整。

### `threads`

返回 `HashMap<u32, SYSTEM_THREAD_INFORMATION>` 引用（将线程 ID 映射到其系统线程信息）的闭包。传递给降级阶段以枚举活动线程。

### `prime_core_scheduler`

对 [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，该调度器跨迭代维护每线程状态（周期计数、固定 CPU 集合、活动连续计数、句柄）。

### `apply_config_result`

对 [`ApplyConfigResult`](ApplyConfigResult.md) 的可变引用，该结构累积更改和错误消息。

## 返回值

此函数不返回值。结果累积在 `apply_config_result` 中。

## 备注

### 算法

Prime 线程算法分四个阶段进行：

1. **候选构建** — 收集具有非零缓存周期计数的线程，并按总 CPU 时间增量（内核 + 用户时间）降序排序。候选池大小为 `max(prime_count × 4, cpu_count)`，上限为总线程数。之前固定的线程如果跌出顶级候选者，会重新添加以确保它们可以被正确降级。

2. **选择** — [`apply_prime_threads_select`](apply_prime_threads_select.md) 使用基于迟滞的选择，通过 [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md) 确定哪些线程有资格成为 Prime 线程。这防止了 Prime 线程和非 Prime 线程状态之间的快速翻转。

3. **提升** — [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 通过 `SetThreadSelectedCpuSets` 将新选中的 Prime 线程固定到指定的 CPU，并可选择提升其线程优先级。

4. **降级** — [`apply_prime_threads_demote`](apply_prime_threads_demote.md) 从不再符合资格的线程中移除 CPU 集合固定，并恢复其原始线程优先级。

### 前提条件

必须在调用此函数之前调用 [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md)，以填充缓存的周期计数并打开线程句柄。周期数据从 `PrimeThreadScheduler` 的每线程统计信息中消耗。

### 跟踪模式

当 `track_top_x_threads` 为非零值时，函数启用跟踪模式，该模式通过 `last_system_thread_info` 存储每个线程的 `SYSTEM_THREAD_INFORMATION` 快照。对于 `track_top_x_threads` 的负值会禁用 Prime 线程调度阶段，但仍允许跟踪。

### 早期退出条件

如果以下情况，函数立即返回：
- `prime_threads_cpus` 和 `prime_threads_prefixes` 都为空，**且** `track_top_x_threads` 为零。
- 在 `dry_run` 模式下，仅记录一个描述 prime CPU 的单个更改消息。

### 候选池大小调整

候选池有意相对于 prime 槽位数进行过采样（`prime_count × 4`），以便为迟滞算法提供足够的线程活动级别上下文。这确保降级候选者始终可见。

## 要求

| 要求 | 值 |
|---|---|
| 模块 | `apply` |
| 配置类型 | [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| 调用方 | 服务主循环（每进程强制） |
| 被调用方 | [`apply_prime_threads_select`](apply_prime_threads_select.md)、[`apply_prime_threads_promote`](apply_prime_threads_promote.md)、[`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| 前置条件 | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| Windows API | 无直接调用（委托给子函数） |
| 权限 | `THREAD_SET_INFORMATION`、`THREAD_QUERY_INFORMATION`（通过子函数） |

## 另请参阅

| 主题 | 链接 |
|---|---|
| Prime 线程选择 | [apply_prime_threads_select](apply_prime_threads_select.md) |
| Prime 线程提升 | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| Prime 线程降级 | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| 周期时间预取 | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 迟滞调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 线程级配置 | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| 结果累积器 | [ApplyConfigResult](ApplyConfigResult.md) |
| 模块概述 | [apply.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*