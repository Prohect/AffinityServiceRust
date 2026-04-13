# PrimeThreadScheduler 结构体 (scheduler.rs)

主力线程管理的核心调度器。跟踪每个进程、每个线程的统计数据，并实现基于滞后算法的选择机制，将最高活跃度的线程提升到性能核心上，同时防止提升/降级抖动。

## 语法

```rust
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `pid_to_process_stats` | `HashMap<u32, ProcessStats>` | 将进程 ID 映射到其每进程统计数据。每个条目包含线程级统计、跟踪配置和存活状态。 |
| `constants` | [ConfigConstants](../config.rs/ConfigConstants.md) | 滞后算法调优参数：`min_active_streak`、`keep_threshold` 和 `entry_threshold`。从配置文件加载，并在配置更改时热重载。 |

## 方法

| 方法 | 描述 |
|------|------|
| [new](#new) | 使用给定的常量创建新的调度器。 |
| [reset_alive](#reset_alive) | 在新一轮扫描之前将所有已跟踪的进程标记为未存活。 |
| [set_alive](#set_alive) | 在当前扫描轮次中将进程标记为存活。 |
| [set_tracking_info](#set_tracking_info) | 设置进程的跟踪深度和显示名称。 |
| [get_thread_stats](#get_thread_stats) | 返回线程统计的可变引用，如果不存在则插入默认值。 |
| [update_active_streaks](#update_active_streaks) | 更新用于滞后算法线程选择的活跃连续计数。 |
| [select_top_threads_with_hysteresis](#select_top_threads_with_hysteresis) | 使用两阶段滞后算法选择前 N 个线程进行主力核心提升。 |
| [drop_process_by_pid](#drop_process_by_pid) | 从调度器中移除进程，关闭句柄并可选择性地记录报告。 |

---

### new

使用给定的滞后算法常量创建一个新的 `PrimeThreadScheduler`。

```rust
pub fn new(constants: ConfigConstants) -> Self
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `constants` | [ConfigConstants](../config.rs/ConfigConstants.md) | 控制线程提升和降级行为的滞后算法调优参数。 |

#### 返回值

一个新的 `PrimeThreadScheduler`，其 `pid_to_process_stats` 映射为空。

---

### reset_alive

将每个已跟踪的进程标记为未存活。在每个扫描循环开始时、重新枚举运行中的进程之前调用。

```rust
pub fn reset_alive(&mut self)
```

#### 备注

调用 `reset_alive` 后，调用方遍历进程快照并对每个匹配的进程调用 [set_alive](#set_alive)。遍历后仍然未存活的进程可通过 [drop_process_by_pid](#drop_process_by_pid) 进行清理。

---

### set_alive

在当前扫描轮次中将特定进程标记为存活。如果该进程尚未被跟踪，则插入一个新的 [ProcessStats](ProcessStats.md) 条目。

```rust
pub fn set_alive(&mut self, pid: u32)
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | Windows 进程 ID。 |

---

### set_tracking_info

设置进程的线程跟踪深度和显示名称。在每个扫描轮次中，对每个配置匹配的进程调用一次。

```rust
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | Windows 进程 ID。 |
| `track_top_x_threads` | `i32` | 退出报告中包含的前 N 个线程数量。值为 `0` 表示禁用报告。允许负值（使用绝对值作为报告数量）。 |
| `process_name` | `String` | 人类可读的进程映像名称，用于日志输出。 |

#### 备注

如果该进程尚未被跟踪，则在设置值之前会插入一个新的 [ProcessStats](ProcessStats.md) 条目。

---

### get_thread_stats

返回特定线程的 [ThreadStats](ThreadStats.md) 的可变引用，如果进程和线程条目不存在则创建它们。

```rust
#[inline]
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | Windows 进程 ID。 |
| `tid` | `u32` | Windows 线程 ID。 |

#### 返回值

线程的 [ThreadStats](ThreadStats.md) 的可变引用。调用方使用它来读取或更新周期计数、句柄、CPU 集合固定、理想处理器状态和活跃连续计数。

---

### update_active_streaks

根据线程的增量周期计数相对于周期计数领先者的比例，更新进程中所有线程的活跃连续计数。

```rust
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | Windows 进程 ID。 |
| `tid_with_delta_cycles` | `&[(u32, u64)]` | `(thread_id, delta_cycles)` 对的切片。`delta_cycles` 是当前与前一扫描轮次之间 `QueryThreadCycleTime` 值的差值。 |

#### 备注

算法计算所有线程中的最大增量周期计数，然后推导出两个阈值：

- **进入阈值** — `max_cycles * constants.entry_threshold`（默认 0.42）。线程必须超过此值才能*开始*累积活跃连续计数。
- **保持阈值** — `max_cycles * constants.keep_threshold`（默认 0.69）。已有连续计数的线程如果低于此值，其连续计数将重置为零。

对于每个线程：

1. 如果线程已有连续计数（`> 0`）：
   - 如果 `delta < keep_min`，连续计数重置为 `0`。
   - 否则，连续计数递增（上限为 `254`）。
2. 如果线程没有连续计数且 `delta >= entry_min`，连续计数设置为 `1`。

此机制防止短暂活跃的线程被提升为主力线程状态。连续计数必须达到 `constants.min_active_streak`（默认 `2`）后，线程才有资格在 [select_top_threads_with_hysteresis](#select_top_threads_with_hysteresis) 中被选中。

---

### select_top_threads_with_hysteresis

使用两阶段滞后算法选择前几个线程进行主力核心提升，以防止提升/降级抖动。

```rust
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
)
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | Windows 进程 ID。 |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | 可变切片，包含 `(thread_id, delta_cycles, is_prime)` 元组。输入时 `is_prime` 应为 `false`。返回时，被选中的线程的 `is_prime` 设置为 `true`。作为副作用，切片按 `delta_cycles` 降序排序。 |
| `slot_count` | `usize` | 最大可提升线程数（通常为配置的性能核心数量）。 |
| `is_currently_assigned` | `fn(&ThreadStats) -> bool` | 回调函数，检查线程是否已被分配主力资源（例如，是否已固定 CPU 集合或理想处理器）。这使"保持"阶段能够保留当前已提升的线程。 |

#### 返回值

无。结果写回 `tid_with_delta_cycles` 切片的 `is_prime` 字段。

#### 备注

算法在按增量周期降序排序后分两个阶段运行：

**第一阶段 — 保留（保持阈值）：**
遍历所有线程。如果线程已被分配（根据 `is_currently_assigned`）且其增量周期达到或超过保持阈值（`max_cycles * constants.keep_threshold`），则保留其主力槽位。这防止线程因轻微的周期计数波动而失去主力状态。

**第二阶段 — 提升（进入阈值）：**
从最高增量周期向下填充剩余槽位。线程仅在满足以下条件时才有资格：
- 尚未被选中（`is_prime == false`）且其 TID 非零。
- 其增量周期达到或超过进入阈值（`max_cycles * constants.entry_threshold`）。
- 其 `active_streak` 至少达到 `constants.min_active_streak`。

进入阈值故意低于保持阈值，形成一个**滞后带**。线程获得提升比维持提升需要更高的活跃度。当两个线程在边界附近具有相似的周期计数时，这消除了快速切换。

**示例：**

当 `keep_threshold = 0.69`、`entry_threshold = 0.42`，且最大增量为 1,000,000 个周期时：
- 当前已提升的线程只要增量 ≥ 690,000 就保持提升状态。
- 新线程必须在至少 `min_active_streak` 个连续轮次中维持 ≥ 420,000 个周期才能获得提升。

---

### drop_process_by_pid

移除进程的所有调度器状态，包括关闭缓存的线程句柄和清除模块地址缓存。可选择性地记录按 CPU 周期排列的前几个线程的诊断报告。

```rust
pub fn drop_process_by_pid(&mut self, pid: &u32)
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `&u32` | 要移除的 Windows 进程 ID 的引用。 |

#### 备注

如果进程的 `track_top_x_threads` 非零，则在清理之前记录报告。报告包括前 N 个线程（按 `last_cycles` 降序排列）的以下信息：

- 线程 ID、总 CPU 周期以及解析为模块名称的起始地址。
- 如果 `last_system_thread_info` 可用：内核时间、用户时间、创建时间、等待时间、客户端 ID、优先级、基础优先级、上下文切换次数、线程状态和等待原因。

时间使用 [format_100ns](format_100ns.md)（持续时间）和 [format_filetime](format_filetime.md)（时间戳）格式化。模块解析使用 `winapi.rs` 中的 `resolve_address_to_module`。

记录日志后，对 PID 调用 `drop_module_cache`，并从 `pid_to_process_stats` 中移除进程条目。线程句柄通过 [ThreadHandle](../winapi.rs/ThreadHandle.md) 的 `Drop` 实现自动关闭。

## 要求

| | |
|---|---|
| **模块** | `scheduler.rs` |
| **调用方** | [apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[update_thread_stats](../apply.rs/update_thread_stats.md)、[hotreload_config](../config.rs/hotreload_config.md)、`main.rs` |
| **被调用方** | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md)、[drop_module_cache](../winapi.rs/drop_module_cache.md)、[log_message](../logging.rs/log_message.md)、[format_100ns](format_100ns.md)、[format_filetime](format_filetime.md) |
| **API** | `QueryThreadCycleTime`（通过调用方） |
| **权限** | `SeDebugPrivilege`（通过调用方，用于线程句柄访问） |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [ProcessStats](ProcessStats.md) | 调度器持有的每进程统计容器。 |
| [ThreadStats](ThreadStats.md) | 每线程统计，包括周期、句柄和理想处理器状态。 |
| [IdealProcessorState](IdealProcessorState.md) | 跟踪每个线程的理想处理器分配状态。 |
| [ConfigConstants](../config.rs/ConfigConstants.md) | 滞后算法调优参数。 |
| [apply_prime_threads](../apply.rs/apply_prime_threads.md) | 驱动主力线程提升/降级管道的入口点。 |
| [ProcessSnapshot](../process.rs/ProcessSnapshot.md) | 提供调度器调用方使用的进程/线程枚举。 |