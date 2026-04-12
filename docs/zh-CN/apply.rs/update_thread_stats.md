# update_thread_stats 函数 (apply.rs)

在每次循环迭代结束时更新缓存的线程统计数据，将周期和时间测量值持久化，以便在下一次迭代中计算增量。

## 语法

```rust
pub fn update_thread_stats(
    pid: u32,
    prime_scheduler: &mut PrimeThreadScheduler,
)
```

## 参数

`pid`

需要更新线程统计数据的进程 ID。

`prime_scheduler`

对 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，持有每个进程、每个线程的统计数据。通过调度器的 `pid_to_process_stats` 映射访问给定 `pid` 对应的 [ProcessStats](../scheduler.rs/ProcessStats.md) 条目。

## 返回值

此函数没有返回值。

## 备注

此函数是每次 apply-config 循环迭代的最后一步。它将每个 [ThreadStats](../scheduler.rs/ThreadStats.md) 条目中的 `cached_*` 字段复制到 `last_*` 字段，然后将缓存值清零。这为下一次迭代的增量计算建立基线。

对于进程 `tid_to_thread_stats` 映射中的每个线程：

- 若 `cached_cycles > 0`：将 `last_cycles` 设置为 `cached_cycles`，然后将 `cached_cycles` 清零。
- 若 `cached_total_time > 0`：将 `last_total_time` 设置为 `cached_total_time`，然后将 `cached_total_time` 清零。

`> 0` 的守卫条件确保在当前迭代中未被测量的线程（例如，在 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 中处于候选池之外的线程）保留其先前的基线值，而不会被重置为零。

此函数不调用任何 Windows API，也不执行 I/O 操作。它是纯粹的内存簿记操作。

### 调用序列

在 `main.rs` 中，典型的每进程应用序列以如下顺序结束：

1. [apply_priority](apply_priority.md) / [apply_affinity](apply_affinity.md) / [apply_process_default_cpuset](apply_process_default_cpuset.md) / [apply_io_priority](apply_io_priority.md) / [apply_memory_priority](apply_memory_priority.md)
2. [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)
3. [apply_prime_threads](apply_prime_threads.md) / [apply_ideal_processors](apply_ideal_processors.md)
4. **`update_thread_stats`** ← 完成本次迭代

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/apply.rs` |
| **行号** | L1327–L1340 |
| **调用者** | [main.rs](../main.rs/README.md) 中的 `apply_config()` |
| **依赖** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)、[ThreadStats](../scheduler.rs/ThreadStats.md) |
| **Windows API** | 无 |