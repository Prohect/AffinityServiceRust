# update_thread_stats 函数 (apply.rs)

将存储在 [ThreadStats](../scheduler.rs/ThreadStats.md) 中的缓存周期计数和时间计数器提交，以便下一次轮询迭代能够计算正确的增量。此函数是每进程线程级应用管线的最后一步——必须在 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 和 [apply_prime_threads](apply_prime_threads.md) 完成之后、下一个周期开始之前调用。

## 语法

```AffinityServiceRust/src/apply.rs#L1327-1340
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
| `pid` | `u32` | 需要提交线程统计数据的进程标识符。用作 `prime_scheduler.pid_to_process_stats` 的键。如果此 pid 不存在对应条目（例如该进程从未被跟踪），函数将立即返回。 |
| `prime_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 持有每进程、每线程统计数据的调度器状态。此函数会修改给定进程中每个被跟踪线程的 [ThreadStats](../scheduler.rs/ThreadStats.md) 条目。 |

## 返回值

无 (`()`)。

## 备注

### 双缓冲提交模式

主力线程管线对周期计数和 CPU 时间采用双缓冲策略：

| 字段 | 写入者 | 读取者 | 提交者 |
|------|--------|--------|--------|
| `cached_cycles` | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | [apply_prime_threads](apply_prime_threads.md)（增量 = `cached_cycles - last_cycles`） | `update_thread_stats` → `last_cycles` |
| `cached_total_time` | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | [apply_prime_threads](apply_prime_threads.md)（增量 = `cached_total_time - last_total_time`） | `update_thread_stats` → `last_total_time` |

在一次轮询周期中：

1. [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 将最新的计数器值写入 `cached_*` 字段。
2. [apply_prime_threads](apply_prime_threads.md) 通过 `cached_*` 减去 `last_*` 计算增量。
3. `update_thread_stats` 将 `cached_*` 提升为 `last_*`，并将 `cached_*` 置零。

这种分离确保步骤 2 中的增量计算始终将*当前*快照与*上一次*快照进行比较，即使中间函数读取或修改了缓存值也不受影响。

### 零值保护

只有非零的 `cached_*` 值才会被提交。如果某个线程在当前周期中未被查询其周期计数或总时间（因为超出了 [prefetch 计数器限制](prefetch_all_thread_cycles.md#counter-limit)，或者 `QueryThreadCycleTime` 失败），则保留其先前的 `last_*` 值。这可以防止一次查询遗漏将基线重置为零，否则在下一次成功查询时会产生人为偏大的增量，从而可能错误地将低活跃度线程提升为主力线程。

提交后，`cached_*` 字段被设为 `0`，表示尚未为下一周期采集到新数据。

### 幂等性

在同一轮询周期内多次调用此函数是无害的——第二次调用时 `cached_*` 值均为 `0`，因此会跳过所有条目。但是，这样做会导致*下一个*周期对同一 `last_*` 基线计算两次增量，实际上将报告的增量减半。因此调用者必须确保每个进程每个周期恰好调用 `update_thread_stats` 一次。

### 未跟踪进程时的空操作

如果 `pid` 在 `prime_scheduler.pid_to_process_stats` 中不存在，`if let Some(ps)` 守卫会导致立即返回。这是安全且预期的行为，适用于拥有 [ProcessConfig](../config.rs/ProcessConfig.md) 但不使用主力线程或理想处理器功能的进程（即 `track_top_x_threads == 0` 且 `prime_threads_cpus` 为空）。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| 被调用者 | 无——仅对 [ThreadStats](../scheduler.rs/ThreadStats.md) 进行纯 Rust 字段赋值 |
| Win32 API | 无 |
| 权限 | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 周期时间预取（填充缓存值） | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 主力线程编排（消费缓存值） | [apply_prime_threads](apply_prime_threads.md) |
| 每线程统计模型 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 调度器状态 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 线程级应用编排 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| apply 模块概述 | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd