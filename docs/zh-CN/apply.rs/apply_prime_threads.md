# apply_prime_threads 函数 (apply.rs)

`apply_prime_threads` 函数为单个进程编排主线程（prime thread）调度流水线。它通过对线程的周期计数增量进行排序来识别最耗 CPU 的线程，使用滞后机制选择最佳候选线程，通过 CPU Sets 将获胜者提升到指定的高性能 CPU 上并可选提升优先级，同时降级不再符合条件的线程。该函数还负责管理线程句柄的生命周期，关闭已退出进程的线程句柄。

## 语法

```AffinityServiceRust/src/apply.rs#L696-712
#[allow(clippy::too_many_arguments)]
pub fn apply_prime_threads(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &ProcessEntry,
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于调度器查找、错误去重和日志消息。 |
| `config` | `&ThreadLevelConfig` | 线程级配置，包含 `prime_threads_cpus`（指定给主线程的 CPU 索引）、`prime_threads_prefixes`（模块前缀匹配规则，带有可选的每前缀 CPU 覆盖）、`track_top_x_threads`（跟踪计数；负值禁用主线程调度）和 `name`（人类可读的配置规则名称）。 |
| `dry_run` | `bool` | 为 `true` 时，函数将预期变更记录到 `apply_config_result` 中，而不调用任何 Windows API 来修改状态。仅发出列出主 CPU 集合的合成变更消息。为 `false` 时，执行完整的选择/提升/降级流水线。 |
| `current_mask` | `&mut usize` | 当前进程亲和性掩码，由先前调用 [`apply_affinity`](apply_affinity.md) 确定。传递给 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 以根据实际亲和性掩码过滤主 CPU 索引。值为 `0` 表示没有活动的亲和性掩码，不进行过滤。 |
| `process` | `&ProcessEntry` | 进程快照条目，用于通过 `process.thread_count()` 获取线程数以确定候选池大小。 |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 从最近一次系统进程信息查询获得的线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射。用于构建候选列表和识别存活线程。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态，跨应用周期跟踪每线程统计信息（缓存周期、上次周期、活跃连续计数、固定的 CPU 集合 ID、句柄等）。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 在选择/提升/降级阶段产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过 `prime_core_scheduler` 的状态变更和 `apply_config_result` 累加器传达。

## 备注

### 门控条件

该函数根据两个标志决定是否运行主线程流水线：

- **`do_prime`**：当 `prime_threads_cpus` 或 `prime_threads_prefixes` 非空，**且** `track_top_x_threads >= 0` 时为 `true`。负数的 `track_top_x_threads` 值显式禁用主线程调度。
- **`has_tracking`**：当 `track_top_x_threads != 0` 时为 `true`。跟踪模式会记录每线程的系统线程信息（`last_system_thread_info`），即使在主调度禁用时也供外部使用。

如果 `do_prime` 和 `has_tracking` 都为 `false`，函数立即返回。在试运行模式中，当 `has_prime_cpus` 为 true 时，会记录形如 `"Prime CPUs: -> [<cpu_list>]"` 的变更消息，然后函数返回而不做进一步处理。

### 算法

1. **构建候选列表**：对于 `threads` 中的每个线程，函数在调度器中查找线程的缓存周期计数。只有 `cached_cycles > 0` 的线程（即被 [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) 成功预取周期的线程）才被包含。如果启用了 `has_tracking`，线程的 `last_system_thread_info` 也会被更新。列表按时间增量（缓存总时间减去上次总时间）降序排序。

2. **确定候选池大小**：池大小为 `max(prime_count * 4, cpu_count)`，上限为进程线程数。这确保了考虑足够多的候选线程，以应对可能短暂活动飙升的线程。先前被固定但已从顶部候选中脱落的线程也会被重新加入，以确保它们能被正确降级。

3. **计算周期增量**：对于每个候选线程，增量为 `cached_cycles - last_cycles`（饱和减法）。每个元组的第三个元素（`bool`）初始化为 `false`，在选择阶段会对符合主线程条件的线程设置为 `true`。

4. **选择**（[`apply_prime_threads_select`](apply_prime_threads_select.md)）：应用基于滞后的选择来标记顶部线程为主线程。已固定的线程获得较低的"保持"阈值；新候选必须超过较高的"进入"阈值并具有足够的活跃连续计数。

5. **提升**（[`apply_prime_threads_promote`](apply_prime_threads_promote.md)）：对于每个新选择的主线程，通过 `SetThreadSelectedCpuSets` 将其固定到指定 CPU，并可选地提升其线程优先级。

6. **降级**（[`apply_prime_threads_demote`](apply_prime_threads_demote.md)）：对于每个先前被固定但不再被选为主线程的线程，移除 CPU 集合固定（通过设置空 CPU 集合）并恢复其原始线程优先级。

7. **句柄清理**：在提升/降级周期之后，函数从候选列表构建存活线程 ID 集合。对于调度器状态中不在存活集合中的任何线程，其线程句柄被丢弃（通过 `ThreadHandle` 的 `Drop` 实现关闭底层操作系统句柄）。这防止了已退出进程的线程出现句柄泄漏。

### 边界情况

- 如果 `prime_threads_cpus` 为空但 `prime_threads_prefixes` 非空，函数仍会运行流水线。每个前缀条目可能携带自己的 `cpus` 覆盖，如果没有匹配到特定线程，该线程将不会被提升。
- 如果 `track_top_x_threads` 为负数，主调度被禁用，但如果绝对值触发了 `has_tracking`，线程跟踪仍可能发生。
- 未被预取周期的线程（例如，因为在 `prefetch_all_thread_cycles` 中句柄获取失败）的 `cached_cycles == 0`，将被完全排除在候选列表之外。
- 先前被固定但不再出现在顶部候选中的线程会被重新注入候选列表，使用其上次已知的周期增量。这确保降级阶段可以处理它们，即使其 CPU 活动已降至零。

## 要求

| 要求 | 值 |
|------|---|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | 无直接调用；委托给 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 和 [`apply_prime_threads_demote`](apply_prime_threads_demote.md)，它们调用 `SetThreadSelectedCpuSets`、`SetThreadPriority`、`GetThreadPriority` 和 `GetLastError`。 |
| 调用者 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用者 | [`apply_prime_threads_select`](apply_prime_threads_select.md)、[`apply_prime_threads_promote`](apply_prime_threads_promote.md)、[`apply_prime_threads_demote`](apply_prime_threads_demote.md)、`PrimeThreadScheduler::get_thread_stats`、`PrimeThreadScheduler::set_tracking_info`、`get_cpu_set_information`、`format_cpu_indices` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION`（写入）和 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION`（读取）的线程句柄。这些通过 `PrimeThreadScheduler` 缓存的句柄获取。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
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
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*