# apply_prime_threads 函数 (apply.rs)

`apply_prime_threads` 函数为单个进程编排主线程（prime-thread）调度管线。它通过按周期计数增量排序来识别 CPU 使用最密集的线程，使用滞后机制选择顶部候选线程，通过 CPU Sets 将优胜者提升到指定的高性能 CPU 并可选地提升优先级，同时降级不再符合条件的线程。

## 语法

```AffinityServiceRust/src/apply.rs#L696-712
#[allow(clippy::too_many_arguments)]
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

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 目标进程的进程 ID。用于调度器查找、错误去重和日志消息。 |
| `config` | `&ThreadLevelConfig` | 线程级别配置，包含 `prime_threads_cpus`（为主线程指定的 CPU 索引列表）、`prime_threads_prefixes`（模块前缀匹配规则，可选包含每个前缀的 CPU 覆盖配置）、`track_top_x_threads`（跟踪数量；负值禁用主线程调度）以及 `name`（人类可读的配置规则名称）。 |
| `dry_run` | `bool` | 当为 `true` 时，函数将*预期*的更改记录到 `apply_config_result` 中，而不调用任何 Windows API 来修改状态。仅发出列出主 CPU 集的合成变更消息。当为 `false` 时，执行完整的选择/提升/降级管线。 |
| `current_mask` | `&mut usize` | 当前进程亲和性掩码，由先前调用 [`apply_affinity`](apply_affinity.md) 确定。传递给 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 以根据实时亲和性掩码过滤主 CPU 索引。值为 `0` 表示没有活动的亲和性掩码，不进行过滤。 |
| `process` | `&'a ProcessEntry` | 进程快照条目（绑定到生命周期 `'a` 的不可变借用），用于通过 `process.thread_count()` 获取线程计数以确定候选池大小。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 按需返回线程信息映射引用的惰性闭包。闭包仅在函数实际需要枚举线程时才被调用，从而在满足提前退出条件（例如 dry-run 模式或禁用主线程调度）时避免构建映射的开销。返回的映射键为线程 ID，值为来自最近系统进程信息查询的 `SYSTEM_THREAD_INFORMATION` 快照。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态，跨应用周期跟踪每个线程的统计信息（缓存周期数、上次周期数、活跃连续次数、固定的 CPU Set ID、句柄等）。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 选择/提升/降级阶段产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过 `prime_core_scheduler` 的状态变更和 `apply_config_result` 累加器传达。

## 备注

### 门控条件

该函数根据两个标志决定是否运行主线程管线：

- **`do_prime`**：当 `prime_threads_cpus` 或 `prime_threads_prefixes` 非空，**且** `track_top_x_threads >= 0` 时为 `true`。负的 `track_top_x_threads` 值会显式禁用主线程调度。
- **`has_tracking`**：当 `track_top_x_threads != 0` 时为 `true`。跟踪模式即使在主线程调度被禁用时也会记录每个线程的系统线程信息（`last_system_thread_info`）以供外部使用。

如果 `do_prime` 和 `has_tracking` 都为 `false`，函数立即返回。在 dry-run 模式下，如果 `has_prime_cpus` 为 true，则记录形如 `"Prime CPUs: -> [<cpu_list>]"` 的变更消息，然后函数返回，不做进一步处理。

### 算法

1. **构建候选列表**：对于 `threads()` 中的每个线程（调用惰性闭包），函数在调度器中查找该线程的缓存周期计数。仅包含 `cached_cycles > 0` 的线程（即由 [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) 成功预取周期的线程）。如果启用了 `has_tracking`，还会更新线程的 `last_system_thread_info`。列表按时间增量（缓存总时间减去上次总时间）降序排序。

2. **确定候选池大小**：池大小为 `max(prime_count * 4, cpu_count)`，上限为进程线程数。这确保考虑足够的候选线程以应对可能短暂出现活动峰值的线程。先前被固定但已退出顶部候选的线程也会被重新添加，以确保它们能被正确降级。

3. **计算周期增量**：对于每个候选线程，增量为 `cached_cycles - last_cycles`（饱和减法）。每个元组的第三个元素（`bool`）初始化为 `false`，将由选择阶段为符合主线程条件的线程设置为 `true`。

4. **选择**（[`apply_prime_threads_select`](apply_prime_threads_select.md)）：应用基于滞后的选择来标记顶部线程为主线程。已被固定的线程获得较低的"保持"阈值；新候选线程必须超过较高的"进入"阈值并具有足够的活跃连续次数。

5. **提升**（[`apply_prime_threads_promote`](apply_prime_threads_promote.md)）：对于每个新选择的主线程，通过 `SetThreadSelectedCpuSets` 将其固定到指定的 CPU，并可选地提升其线程优先级。

6. **降级**（[`apply_prime_threads_demote`](apply_prime_threads_demote.md)）：对于先前被固定但不再被选为主线程的每个线程，移除 CPU Set 固定（通过设置空的 CPU Set）并恢复其原始线程优先级。

### 边缘情况

- 如果 `prime_threads_cpus` 为空但 `prime_threads_prefixes` 非空，函数仍会运行管线。每个前缀条目可能携带自己的 `cpus` 覆盖配置，如果没有任何前缀匹配特定线程，该线程将不会被提升。
- 如果 `track_top_x_threads` 为负数，主线程调度被禁用，但如果其绝对值触发 `has_tracking`，线程跟踪仍可能发生。
- 未被预取周期的线程（例如，因为在 `prefetch_all_thread_cycles` 中句柄获取失败）的 `cached_cycles == 0`，将被完全排除在候选列表之外。

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | 不直接调用；委托给 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 和 [`apply_prime_threads_demote`](apply_prime_threads_demote.md)，它们调用 `SetThreadSelectedCpuSets`、`SetThreadPriority`、`GetThreadPriority` 和 `GetLastError`。 |
| 调用方 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用方 | [`apply_prime_threads_select`](apply_prime_threads_select.md)、[`apply_prime_threads_promote`](apply_prime_threads_promote.md)、[`apply_prime_threads_demote`](apply_prime_threads_demote.md)、`PrimeThreadScheduler::get_thread_stats`、`PrimeThreadScheduler::set_tracking_info`、`get_cpu_set_information`、`format_cpu_indices` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION`（写入）和 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION`（读取）的线程句柄。这些通过 `PrimeThreadScheduler` 缓存的句柄获取。 |

## 另请参阅

| 参考 | 链接 |
|-----------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| apply_prime_threads_promote | [`apply_prime_threads_promote`](apply_prime_threads_promote.md) |
| apply_prime_threads_demote | [`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| apply_ideal_processors | [`apply_ideal_processors`](apply_ideal_processors.md) |
| update_thread_stats | [`update_thread_stats`](update_thread_stats.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |
| ProcessEntry | [`process.rs/ProcessEntry`](../process.rs/ProcessEntry.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*