# apply_prime_threads_select 函数 (apply.rs)

使用基于迟滞的升级逻辑选择最高优先级的线程作为 Prime 线程。此函数是 Prime 线程调度管道的决策层——它根据 CPU 周期差值和连续活跃时长来确定哪些线程有资格获得 Prime CPU 绑定，但不执行任何实际的系统调用。

## 语法

```AffinityServiceRust/src/apply.rs#L794-800
pub fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

## 参数

`pid: u32`

正在评估其线程的进程 ID。

`prime_count: usize`

可以提升为 Prime 线程状态的线程最大数量。通常等于配置的 prime CPUs 数量 (`config.prime_threads_cpus.len()`)。

`tid_with_delta_cycles: &mut [(u32, u64, bool)]`

`(线程 ID、周期差值、是否 Prime 线程)` 元组的可变切片。进入时，所有条目的 `is_prime` 字段为 `false`。退出时，被选为 Prime 线程的线程其 `is_prime` 字段设置为 `true`。该切片应已填充候选线程及其当前调度间隔的周期差值。

`prime_core_scheduler: &mut PrimeThreadScheduler`

[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 实例，拥有线程统计信息、迟滞常数和 `select_top_threads_with_hysteresis` 算法。

## 返回值

此函数不返回值。结果直接写入 `tid_with_delta_cycles` 中每个元组的 `is_prime` 字段。

## 说明

此函数完全委托给 [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md)，传递一个谓词，该谓词在 `pinned_cpu_set_ids` 非空时将线程视为"当前已分配"。迟滞算法应用两个阈值：

- **保持阈值** - 已经是 Prime 线程的线程，如果其周期差值在所有候选线程的最大周期差值中处于或高于此百分比，则保持 Prime 线程状态。
- **进入阈值** - 当前不是 Prime 线程的线程必须超过此（更高）百分比，并且在被升级前保持至少 `min_active_streak` 个间隔的连续活跃时长。

这种双阈值方法防止了当多个线程具有相似的 CPU 利用率时在 Prime 线程状态和非 Prime 线程状态之间的快速翻转（抖动）。

此函数与 `apply_prime_threads_promote` 和 `apply_prime_threads_demote` 明确分离，以在 [`apply_prime_threads`](apply_prime_threads.md) 内维护清晰的 **选择 → 升级 → 降级** 管道。

### 谓词：`is_currently_assigned`

传递给迟滞选择器的闭包为：

```AffinityServiceRust/src/apply.rs#L801-803
|thread_stats| {
    !thread_stats.pinned_cpu_set_ids.is_empty()
}
```

如果线程之前已被升级阶段分配到过一个或多个 CPU 集合 ID，则被视为当前已分配（并且有资格享受较低的保持阈值）。

## 要求

| 要求 | 值 |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | [`apply_prime_threads`](apply_prime_threads.md) |
| **被调用方** | [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md) |
| **Win32 API** | 无 |
| **权限** | 无（无系统调用） |

## 参见

| 主题 | 描述 |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | 调用选择 → 升级 → 降级的编排器 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 将选定的 Prime 线程绑定到 CPU 并提升优先级 |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | 解除绑定并恢复失去 Prime 线程状态的线程 |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | 收集选择所消耗的周期数据 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 拥有迟滞逻辑和线程统计信息的调度器 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*