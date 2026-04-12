# apply_prime_threads_select 函数 (apply.rs)

使用滞后选择顶部线程获得 prime 状态。此函数封装了调度器的 `select_top_threads_with_hysteresis` 方法，使用 CPU 集固定状态作为"当前是否为 prime"的判定谓词。

## 语法

```rust
fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

## 参数

`pid`

目标进程的进程标识符。

`prime_count`

可用的 prime 线程槽位数量（通常等于配置的 prime CPU 数量）。

`tid_with_delta_cycles`

可变元组切片 `(tid, delta_cycles, is_prime)`。输入时，所有条目的 `is_prime` 均为 `false`。输出时，被选为 prime 状态的线程的 `is_prime` 被设置为 `true`。`delta_cycles` 值表示自上次测量以来的 CPU 周期变化量。

`prime_core_scheduler`

[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，持有滞后算法和每线程统计信息。

## 返回值

此函数不返回值。结果通过就地设置 `tid_with_delta_cycles` 中的 `is_prime` 标志（第三个元素）写入。

## 备注

滞后机制防止线程在每次迭代中在 prime 和非 prime 状态之间快速翻转。选择使用来自 [ConfigConstants](../config.rs/ConfigConstants.md) 的两个阈值：

- **keep_threshold** — *已经是* prime 的线程，只要其周期增量至少为候选集中最大周期增量的 `keep_threshold`%，就保持 prime 状态。
- **entry_threshold** — *尚未成为* prime 的线程，必须超过最大周期增量的 `entry_threshold`%，*并且*连续 `min_active_streak` 次迭代中的 `active_streak` 达标，才能被提升。

传递给 `select_top_threads_with_hysteresis` 的"当前是否为 prime"谓词为：

```rust
|thread_stats| !thread_stats.pinned_cpu_set_ids.is_empty()
```

这意味着如果线程有任何已固定的 CPU 集 ID（即之前被 [apply_prime_threads_promote](apply_prime_threads_promote.md) 提升过），则被视为当前处于 prime 状态。

此函数由 [apply_prime_threads](apply_prime_threads.md) 在收集候选后、提升/降级阶段之前调用。

### 选择流程

1. [apply_prime_threads](apply_prime_threads.md) 从候选池构建 `tid_with_delta_cycles` 数组。
2. `apply_prime_threads_select` 使用滞后算法将前 `prime_count` 个线程标记为 prime。
3. [apply_prime_threads_promote](apply_prime_threads_promote.md) 将新选中的线程固定到 prime CPU。
4. [apply_prime_threads_demote](apply_prime_threads_demote.md) 取消固定失去 prime 状态的线程。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **行号** | L802–L816 |
| **调用者** | [apply_prime_threads](apply_prime_threads.md) |
| **调用** | [PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md) |
| **Windows API** | 无 |

## 另请参阅

- [apply_prime_threads](apply_prime_threads.md)
- [apply_prime_threads_promote](apply_prime_threads_promote.md)
- [apply_prime_threads_demote](apply_prime_threads_demote.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)
- [ConfigConstants](../config.rs/ConfigConstants.md)