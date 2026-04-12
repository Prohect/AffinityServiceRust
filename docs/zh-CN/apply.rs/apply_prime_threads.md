# apply_prime_threads 函数 (apply.rs)

Prime 线程调度的主编排函数。识别进程中 CPU 使用率最高的线程，并通过 CPU 集将其固定到指定的"prime"CPU 上，以提升缓存局部性和调度可预测性。

## 语法

```rust
pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含 prime 线程设置，包括 `prime_threads_cpus`、`prime_threads_prefixes` 和 `track_top_x_threads`。

`dry_run`

为 `true` 时，记录将要进行的更改但不调用任何 Windows API。

`current_mask`

指向进程当前 CPU 亲和性掩码的可变引用。传递给 [apply_prime_threads_promote](apply_prime_threads_promote.md) 以根据活跃亲和性过滤 prime CPU。

`process`

目标进程的 [ProcessEntry](../process.rs/ProcessEntry.md) 可变引用。用于枚举存活线程及其调度信息。

`prime_core_scheduler`

[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，维护跨迭代的每线程统计数据、周期计数和固定状态。

`apply_config_result`

[ApplyConfigResult](ApplyConfigResult.md) 的可变引用，收集更改和错误消息。

## 返回值

此函数不返回值。结果记录在 `apply_config_result` 中。

## 备注

### 算法

Prime 线程调度管线按以下阶段进行：

1. **前置检查** — 如果未配置 prime CPU 且跟踪被禁用，函数提前返回。`track_top_x_threads` 为负值时禁用 prime 调度但保留跟踪。在 dry-run 模式下仅记录摘要更改消息。

2. **设置跟踪信息** — 如果 `track_top_x_threads != 0`，调用 `prime_core_scheduler.set_tracking_info()` 将进程注册到线程级统计收集中。

3. **按 CPU 时间增量排序线程** — 枚举所有线程，计算每个线程的 CPU 时间增量（`cached_total_time - last_total_time`），按降序排列。如果启用了跟踪，为每个线程快照 `last_system_thread_info`。

4. **构建候选池** — 选择前 N 个候选线程，其中 N = max(prime_count × 4, 逻辑 CPU 数量)，上限为线程总数。同时包含任何先前被固定但可能已掉出顶部候选的线程，确保它们可以通过降级路径正常处理。

5. **计算周期增量** — 对每个候选线程计算 `cached_cycles - last_cycles` 以衡量近期 CPU 活动。

6. **选择** — 调用 [apply_prime_threads_select](apply_prime_threads_select.md)，使用滞后算法确定哪些线程应成为 prime。

7. **提升** — 调用 [apply_prime_threads_promote](apply_prime_threads_promote.md)，将新选中的线程固定到 prime CPU 并提升优先级。

8. **降级** — 调用 [apply_prime_threads_demote](apply_prime_threads_demote.md)，取消不再符合条件的线程的固定并恢复其原始优先级。

9. **句柄清理** — 移除进程快照中已不存在的线程的缓存句柄。`ThreadHandle` 的 `Drop` 实现会自动关闭底层 OS 句柄。

### 候选池大小

候选池故意大于 prime 槽位数量（4 倍或 CPU 数量，取较大者），以便滞后算法有足够数据做出稳定选择。先前被固定的线程无论当前排名如何都始终被包含，以便它们能够顺利通过降级路径。

### 配置

- `config.prime_threads_cpus` — 指定为 prime 核心的 CPU 索引。
- `config.prime_threads_prefixes` — 可选的模块前缀规则，限制哪些线程有资格获得 prime 状态，并支持每前缀的 CPU 和优先级覆盖。
- `config.track_top_x_threads` — 控制跟踪深度。负值禁用 prime 调度但保留跟踪。零同时禁用两者。

### 更改日志

- Dry run：`"Prime CPUs: -> [{cpu_list}]"`

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **行号** | L704–L800 |
| **调用者** | [apply_config](../main.rs/apply_config.md)（main.rs） |
| **调用** | [apply_prime_threads_select](apply_prime_threads_select.md)、[apply_prime_threads_promote](apply_prime_threads_promote.md)、[apply_prime_threads_demote](apply_prime_threads_demote.md) |
| **Windows API** | （委托给子函数） |

## 另请参阅

- [apply_prime_threads_select](apply_prime_threads_select.md)
- [apply_prime_threads_promote](apply_prime_threads_promote.md)
- [apply_prime_threads_demote](apply_prime_threads_demote.md)
- [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)