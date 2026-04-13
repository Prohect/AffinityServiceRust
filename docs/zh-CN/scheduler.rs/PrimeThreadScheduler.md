# PrimeThreadScheduler struct (scheduler.rs)

管理 Prime 线程调度器的核心结构体，负责动态线程到 CPU 的分配。通过滞后算法防止线程在 prime 状态和普通状态之间频繁切换（抖动），确保高负载线程被稳定地钉到高性能核心上。

## 语法

```rust
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## 成员

`pid_to_process_stats`

从进程 ID（PID）到 [ProcessStats](ProcessStats.md) 的映射表。每个被跟踪的进程在此映射中拥有一个条目，包含其所有线程的统计数据和生存状态。

`constants`

类型为 [ConfigConstants](../config.rs/ConfigConstants.md) 的调度常量，包含滞后算法所需的阈值参数：

- `entry_threshold`：默认 `0.42`，新线程进入 prime 状态所需的最低周期比率
- `keep_threshold`：默认 `0.69`，已分配线程保持 prime 状态所需的最低周期比率
- `min_active_streak`：线程在被提升前需要连续活跃的最小迭代次数

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **new** | `pub fn new(constants: ConfigConstants) -> Self` | 使用指定常量创建一个空调度器实例。 |
| **reset_alive** | `pub fn reset_alive(&mut self)` | 将所有已跟踪进程的 `alive` 标志重置为 `false`。在每次循环迭代开始时调用。 |
| **set_alive** | `pub fn set_alive(&mut self, pid: u32)` | 将指定进程标记为存活。若进程不存在则自动创建条目。 |
| **set_tracking_info** | `pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)` | 设置进程的线程跟踪数量和进程名称。 |
| **get_thread_stats** | `pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats` | 获取指定线程的可变引用。若进程或线程条目不存在则自动创建。 |
| **update_active_streaks** | `pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])` | 更新活跃连续计数器，用于滞后线程选择。 |
| **select_top_threads_with_hysteresis** | `pub fn select_top_threads_with_hysteresis(&mut self, pid: u32, tid_with_delta_cycles: &mut [(u32, u64, bool)], slot_count: usize, is_currently_assigned: fn(&ThreadStats) -> bool)` | 使用滞后算法选择顶部线程。 |
| **drop_process_by_pid** | `pub fn drop_process_by_pid(&mut self, pid: &u32)` | 按 PID 移除指定进程，关闭其线程句柄，清除模块缓存，并可选记录顶部线程报告。 |

## 备注

### 滞后算法

Prime 线程调度的核心机制是**两阈值滞后（hysteresis）算法**，用于防止线程在 prime 和非 prime 状态之间频繁切换：

- **进入阈值**（entry threshold = `0.42`）：新线程的 CPU 周期增量必须达到当前最高线程周期增量的 42% 以上，才有资格获得 prime 状态。
- **保持阈值**（keep threshold = `0.69`）：已经处于 prime 状态的线程，只要周期增量仍达到最高值的 69% 以上，就继续保持 prime 状态。

保持阈值高于进入阈值，这意味着线程进入 prime 状态相对容易，但一旦获得 prime 状态后，只有当其活跃度显著下降时才会被降级。这种不对称设计有效抑制了抖动。

### 两阶段选择

[select_top_threads_with_hysteresis](#方法) 使用两阶段选择流程：

1. **第一阶段（保留）：** 优先保留当前已分配 prime 状态的线程，只要它们的周期增量 ≥ `keep_threshold × max_cycles`。
2. **第二阶段（填充）：** 用满足进入条件（周期增量 ≥ `entry_threshold × max_cycles` 且 `active_streak ≥ min_active_streak`）的新线程填充剩余空位。

### 活跃连续计数

`active_streak` 计数器要求线程在连续多次迭代中持续活跃，才能被提升为 prime 线程。这防止了短暂突发活跃的线程被错误提升。计数器上限为 254，当线程的周期增量低于保持阈值时重置为 0。

### 生命周期管理

调度器通过 `reset_alive` / `set_alive` / `drop_process_by_pid` 机制管理进程生命周期：

1. 每次循环开始时调用 `reset_alive()`，将所有进程标记为未存活。
2. 遍历快照中的进程时调用 `set_alive(pid)`，标记仍在运行的进程。
3. 当 ETW 进程退出事件到达或检测到进程已退出时，调用 `drop_process_by_pid(pid)` 按 PID 移除指定进程。

`drop_process_by_pid` 在清理前会可选地记录已退出进程的顶部线程统计信息（当 `track_top_x_threads != 0` 时），包括 CPU 周期、内核/用户时间、创建时间、上下文切换次数等详细数据。线程句柄通过 [ThreadHandle](../winapi.rs/ThreadHandle.md) 的 `Drop` 实现自动关闭。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/scheduler.rs |
| **行号** | L13–L176 |
| **构造** | `PrimeThreadScheduler::new(constants)` |
| **关键依赖** | [ProcessStats](ProcessStats.md)、[ThreadStats](ThreadStats.md)、[ConfigConstants](../config.rs/ConfigConstants.md)、[ThreadHandle](../winapi.rs/ThreadHandle.md) |
| **调用者** | [`apply_prime_threads`](../apply.rs/apply_prime_threads.md)、[`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md)、`main.rs` |

## 另请参阅

- [scheduler.rs 模块概述](README.md)
- [ProcessStats](ProcessStats.md)
- [ThreadStats](ThreadStats.md)
- [IdealProcessorState](IdealProcessorState.md)
- [apply_prime_threads](../apply.rs/apply_prime_threads.md)