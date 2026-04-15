# update_thread_stats 函数 (apply.rs)

`update_thread_stats` 函数将当前应用周期中收集的缓存周期计数和总时间测量值提交到每个线程统计信息的持久化字段 `last_cycles` 和 `last_total_time` 中，然后将缓存值重置为零。这为下一个应用周期的增量计算建立基准线。该函数应在每个进程的应用管道结束时调用，即在所有主线程选择、提升和降级逻辑消费完缓存值之后。

## 语法

```AffinityServiceRust/src/apply.rs#L1311-1324
pub fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) {
    if let Some(ps) = prime_scheduler.pid_to_process_stats.get_mut(&pid) {
        for ts in ps.tid_to_thread_stats.values_mut() {
            if ts.cached_cycles > 0 {
                ts.last_cycles = ts.cached_cycles;
                ts.cached_cycles = 0;
            }
            if ts.cached_total_time > 0 {
                ts.last_total_time = ts.cached_total_time;
                ts.cached_total_time = 0;
            }
        }
    }
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 需要提交线程统计信息的进程 ID。用于在调度器的 `pid_to_process_stats` 映射中查找对应的 `ProcessStats` 条目。如果该 PID 不存在对应条目，则函数不执行任何操作。 |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态，包含每个进程、每个线程的统计信息。函数遍历给定 PID 的所有线程统计信息并更新其 `last_*` / `cached_*` 字段。 |

## 返回值

此函数没有返回值。

## 备注

### 提交语义

函数使用守卫模式：仅当缓存值严格大于零时才进行提交。这确保在当前周期中未能测量周期数或总时间的线程（例如，因为在 [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) 中句柄获取失败）保留其先前的 `last_*` 基准值，而不是被重置为零。

提交后，`cached_cycles` 和 `cached_total_time` 字段被设置为 `0`。这确保如果某个线程在*下一个*周期中未被测量，其增量将被正确计算（增量计算使用 `cached_cycles.saturating_sub(last_cycles)`，因此缓存值为 `0` 将产生增量 `0`）。

### 调用顺序

此函数必须在以下所有消费缓存测量值的函数**之后**调用：

1. [`apply_prime_threads`](apply_prime_threads.md) — 使用 `cached_cycles` 和 `cached_total_time` 构建候选列表并计算增量。
2. [`apply_prime_threads_select`](apply_prime_threads_select.md) — 读取从缓存值计算的周期增量。
3. [`apply_prime_threads_promote`](apply_prime_threads_promote.md) — 在变更消息中记录 `delta_cycles`。
4. [`apply_prime_threads_demote`](apply_prime_threads_demote.md) — 对相同的选择结果进行操作。
5. [`apply_ideal_processors`](apply_ideal_processors.md) — 使用 `cached_cycles - last_cycles` 进行线程排名。

如果在这些函数之前调用 `update_thread_stats`，所有增量都将为零，实际上会禁用主线程算法。

### 边界情况

- 如果 `pid` 不在 `pid_to_process_stats` 中（例如，该进程从未在调度器中注册或已被清理），`if let Some` 守卫会使函数立即返回，不会产生错误。
- 如果某个线程的 `cached_cycles` 为 `0`（因为 `QueryThreadCycleTime` 失败或从未对该线程调用），`last_cycles` 保留其先前的值。这意味着下一个周期该线程的增量仍将基于最近一次成功的测量值。
- 该函数不会移除陈旧的线程条目。线程清理由 [`apply_prime_threads`](apply_prime_threads.md) 单独处理，它会为不再出现在活动线程快照中的线程释放句柄。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | 无（纯数据操作；无操作系统调用） |
| 调用者 | `scheduler.rs` / `main.rs` 中运行每进程应用管道的编排代码 |
| 被调用者 | 无（直接读写 `PrimeThreadScheduler` 的字段） |
| 权限 | 无 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_ideal_processors | [`apply_ideal_processors`](apply_ideal_processors.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*