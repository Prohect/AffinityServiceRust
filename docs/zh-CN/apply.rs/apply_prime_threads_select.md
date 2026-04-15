# apply_prime_threads_select 函数 (apply.rs)

`apply_prime_threads_select` 函数使用基于滞后（hysteresis）的算法选择最优线程作为 prime 线程。它委托给 `PrimeThreadScheduler::select_top_threads_with_hysteresis`，该方法对进入和保持阈值进行差异化处理，以防止线程在连续的应用周期中快速地在 prime 和非 prime 状态之间翻转。已经固定到 CPU 集合的线程（即当前为 prime 状态）使用更宽松的保持阈值进行评估，而非 prime 线程必须超过更严格的进入阈值并满足最低活跃连续次数要求才能被提升。

## 语法

```AffinityServiceRust/src/apply.rs#L793-802
pub fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。传递给调度器用于查找每个进程的线程统计信息。 |
| `prime_count` | `usize` | 可被选为 prime 的最大线程数。此值等于 `config.prime_threads_cpus` 中的 CPU 数量，即可用于固定的专用高性能核心数量。 |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | 可变的元组切片，每个候选线程对应一个元组。每个元组包含：线程 ID（`u32`）、自上次测量以来的增量周期计数（`u64`）和一个布尔选择标志（`bool`）。输入时，所有元素的布尔值为 `false`。输出时，被选为 prime 的线程的布尔值被设置为 `true`。调用者（[`apply_prime_threads`](apply_prime_threads.md)）应预先按增量周期降序排列该切片。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 维护每个进程、每个线程统计信息（活跃连续次数、固定的 CPU 集合 ID、周期历史记录）的 prime 线程调度器。该函数在此调度器上调用 `select_top_threads_with_hysteresis`，读取并更新线程统计信息。 |

## 返回值

此函数不返回值。选择结果通过修改 `tid_with_delta_cycles` 切片中每个元组的 `is_prime` 布尔值（第三个元素）来传达。

## 备注

### 滞后算法

该函数完全委托给 `PrimeThreadScheduler::select_top_threads_with_hysteresis`，传递闭包 `|thread_stats| !thread_stats.pinned_cpu_set_ids.is_empty()` 作为"当前是否为 prime"的谓词。当线程已经固定了 CPU 集合 ID（即在上一个周期中被提升）时，此闭包返回 `true`，使调度器应用更宽松的**保持阈值**。未被固定的线程必须超过更严格的**进入阈值**，并且其 `active_streak` 计数达到或超过配置的最低值才能被选中。

滞后机制确保：
- 已经在 prime 核心上运行的线程即使其周期计数暂时低于进入阈值也会留在那里。这避免了不必要的降级后重新提升的抖动。
- 线程必须展示持续的高 CPU 活动（通过活跃连续次数衡量）才能被提升，防止短暂的峰值触发随即被撤销的提升。

### 选择限制

最多 `prime_count` 个线程被标记为 prime。如果符合条件的线程数超过 prime 槽位数，则只选择增量周期最高的前 `prime_count` 个线程。

### 与提升/降级的交互

此函数是由 [`apply_prime_threads`](apply_prime_threads.md) 编排的 prime 线程管道中三个阶段中的第一个：

1. **选择**（`apply_prime_threads_select`）— 在 `tid_with_delta_cycles` 切片中将线程标记为 prime 或非 prime。
2. **提升**（[`apply_prime_threads_promote`](apply_prime_threads_promote.md)）— 将新选中的 prime 线程固定到 CPU 并提升其优先级。
3. **降级**（[`apply_prime_threads_demote`](apply_prime_threads_demote.md)）— 取消固定并恢复失去 prime 状态的线程的优先级。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | 无（纯逻辑；无操作系统调用） |
| 调用者 | [`apply_prime_threads`](apply_prime_threads.md) |
| 被调用者 | `PrimeThreadScheduler::select_top_threads_with_hysteresis` |
| 权限 | 无 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概览 | [`README`](README.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_promote | [`apply_prime_threads_promote`](apply_prime_threads_promote.md) |
| apply_prime_threads_demote | [`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*