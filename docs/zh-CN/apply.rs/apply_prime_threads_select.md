# apply_prime_threads_select 函数 (apply.rs)

使用基于滞后阈值的机制，从候选池中选择排名前 *N* 的线程授予主力线程状态。此函数是 [PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm) 的轻量封装，提供滞后算法所需的"当前已固定"谓词，用于区分已经是主力线程（因而适用更宽松的*保持*阈值）的线程和必须达到更严格的*准入*阈值才能被新晋升的线程。

## 语法

```AffinityServiceRust/src/apply.rs#L807-816
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
| `pid` | `u32` | 目标进程的进程标识符。用作 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 中按进程统计映射的键。 |
| `prime_count` | `usize` | 可用的主力线程槽位数——等于 `config.prime_threads_cpus.len()`。`tid_with_delta_cycles` 中最多有这么多条目的 `is_prime` 标志会被设置为 `true`。 |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | 候选元组 `(tid, delta_cycles, is_prime)` 的可变切片。输入时，每个元素的 `is_prime` 均为 `false`。返回时，最多有 `prime_count` 个元素的 `is_prime` 被设为 `true`。调用方（[apply_prime_threads](apply_prime_threads.md)）应预先按 `delta_cycles` 降序排列该切片，以便滞后算法优先评估 CPU 使用率最高的线程。 |
| `prime_core_scheduler` | `&mut` [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 持久调度器状态。提供对每线程 [ThreadStats](../scheduler.rs/ThreadStats.md)（活跃连续计数器、已固定 CPU 集合 ID）以及定义滞后阈值（`entry_threshold`、`keep_threshold`、`min_active_streak`）的 [ConfigConstants](../config.rs/ConfigConstants.md) 的访问。 |

## 返回值

无（`()`）。函数通过修改 `tid_with_delta_cycles` 中每个元素的 `is_prime` 标志来传达结果。

## 备注

### 滞后算法

该函数完全委托给 `prime_core_scheduler.select_top_threads_with_hysteresis()`，传入以下参数：

- `pid` — 用于查找按进程的 [ProcessStats](../scheduler.rs/ProcessStats.md)。
- `tid_with_delta_cycles` — 带有可变 `is_prime` 标志的候选池。
- `prime_count` — 要选择的最大线程数。
- 闭包 `|thread_stats| !thread_stats.pinned_cpu_set_ids.is_empty()` — 当线程*当前*处于主力状态（即在前一轮周期中已被 [apply_prime_threads_promote](apply_prime_threads_promote.md) 固定到某个 CPU 集合）时，此谓词返回 `true`。

滞后算法（详细文档见 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm)）使用两个阈值来防止频繁的晋升/降级振荡：

| 阈值 | 适用对象 | 条件 |
|------|----------|------|
| **keep_threshold** | *已经*是主力线程的线程（`pinned_cpu_set_ids` 非空） | 当 `delta_cycles >= keep_threshold% × max_delta_cycles`（在候选者中）时，线程保持主力状态。这是一个较低的门槛，提供粘性以防止线程在短暂的 CPU 使用率下降期间被降级。 |
| **entry_threshold** | *当前不是*主力线程的线程 | 仅当 `delta_cycles >= entry_threshold% × max_delta_cycles` **且** `active_streak >= min_active_streak` 时，线程才成为主力线程。这是一个较高的门槛，防止短暂的 CPU 突发使用立即触发晋升。 |

`entry_threshold` 始终大于或等于 `keep_threshold`，形成一个死区以抑制振荡。`min_active_streak` 要求（来自 [ConfigConstants](../config.rs/ConfigConstants.md)）通过要求线程在多个连续轮询周期内保持持续的 CPU 活动，进一步稳定选择。

### 候选排序的重要性

尽管滞后算法通过检查周期增量来决定哪些线程符合条件，但输入切片中候选者的顺序会影响平局决断。调用方（[apply_prime_threads](apply_prime_threads.md)）在调用此函数前按时间增量降序排列，确保 CPU 使用率最高的线程被优先评估。当多个线程超过准入阈值但符合条件的线程数超过 `prime_count` 时，切片中靠前的线程（CPU 使用率更高）会被优先选择。

### 关注点分离

`apply_prime_threads_select` *仅*执行选择步骤。它不打开任何操作系统句柄、不调用任何 Windows API，也不修改任何线程状态。实际的固定和优先级变更由 [apply_prime_threads_promote](apply_prime_threads_promote.md) 和 [apply_prime_threads_demote](apply_prime_threads_demote.md) 根据此处设置的 `is_prime` 标志来处理。这种分离使选择逻辑可以独立于操作系统副作用进行测试和推理。

### 在理想处理器规则中的复用

[apply_ideal_processors](apply_ideal_processors.md) 也使用了相同的 `select_top_threads_with_hysteresis` 方法，但使用不同的"当前已分配"谓词（`|ts| ts.ideal_processor.is_assigned`）。两个调用点共享滞后算法，但在什么构成"已选中"状态方面有所不同。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用方 | [apply_prime_threads](apply_prime_threads.md) |
| 被调用方 | [PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | 无 — 纯 Rust 逻辑；无操作系统调用 |
| 权限 | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主力线程编排 | [apply_prime_threads](apply_prime_threads.md) |
| 选择后的晋升 | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| 未选中线程的降级 | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| 滞后算法与调度器状态 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 每线程统计（活跃连续计数、已固定 ID） | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 滞后常量 | [ConfigConstants](../config.rs/ConfigConstants.md) |
| 理想处理器规则选择（复用相同算法） | [apply_ideal_processors](apply_ideal_processors.md) |
| 周期时间预取（填充 cached_cycles） | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| apply 模块概述 | [apply](README.md) |