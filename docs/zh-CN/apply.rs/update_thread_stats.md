# update_thread_stats 函数（apply.rs）

缓存当前周期时间和总时间作为"上次"值，供下次迭代计算增量使用。此函数必须在每个应用周期结束时调用，以确保下次迭代能准确计算线程 CPU 使用情况的增量。

## 语法

```AffinityServiceRust/src/apply.rs#L1312-1315
pub fn update_thread_stats(
    pid: u32,
    prime_scheduler: &mut PrimeThreadScheduler,
)
```

## 参数

`pid: u32`

需要更新线程统计信息的目标进程 ID。

`prime_scheduler: &mut PrimeThreadScheduler`

对 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，该调度器拥有每线程统计信息缓存。调度器的 `pid_to_process_stats` 映射表会根据给定的 `pid` 进行查询。

## 返回值

此函数不返回值。

## 说明

`update_thread_stats` 对给定进程下跟踪的每个线程执行两次转移操作：

1. **周期快照** — 将 `cached_cycles` 复制到 `last_cycles`，然后将 `cached_cycles` 清零。这为下次调用 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 准备了基线，后者将计算 `delta_cycles = cached_cycles - last_cycles`。

2. **总时间快照** — 将 `cached_total_time` 复制到 `last_total_time`，然后将 `cached_total_time` 清零。总时间（内核时间 + 用户时间）用于按 CPU 消耗对线程排序，并计算基于时间的增量。

仅当缓存值大于零时才进行更新。这避免了覆盖那些在当前周期未被测量的线程的有效 `last_*` 值（例如，在 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 中落在候选池之外的线程）。

如果 `pid` 不存在于 `pid_to_process_stats` 中，函数将静默返回，不产生任何影响。

### 调用顺序

在典型的应用周期中，调用顺序为：

1. [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) — 查询当前周期/时间值并将其存储在 `cached_*` 字段中。
2. [apply_prime_threads](apply_prime_threads.md) / [apply_ideal_processors](apply_ideal_processors.md) — 消耗缓存的增量用于线程选择和调度决策。
3. **`update_thread_stats`** — 将缓存值快照到 `last_*` 字段并清除缓存。

省略此调用会导致增量在多个周期中累积，从而导致 Prime 线程和理想处理器算法中的线程排名错误。

## 要求

| | |
|---|---|
| **模块** | [apply.rs](README.md) |
| **调用方** | 服务主循环（所有每进程应用函数完成后） |
| **被调用方** | 无（纯数据记账） |
| **API** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 说明 |
|---|---|
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | 填充 `cached_cycles` 和 `cached_total_time`，此函数对这些值进行快照 |
| [apply_prime_threads](apply_prime_threads.md) | 消耗周期增量用于 Prime 线程选择 |
| [apply_ideal_processors](apply_ideal_processors.md) | 消耗周期增量用于理想处理器分配 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 拥有每进程、每线程的统计信息映射表 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*