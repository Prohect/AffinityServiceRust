# PrimeThreadScheduler 结构体 (scheduler.rs)

Prime 线程调度引擎。维护每个进程、每个线程的统计信息，并实现基于迟滞机制的线程选择，以识别和提升 CPU 最活跃的线程（"Prime 线程"）到专用处理器核心。迟滞机制通过要求线程在提升前持续超过进入阈值来防止抖动，并允许线程在保持高于较低的保持阈值时维持已提升状态。

## 语法

```rust
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## 成员

| 名称 | 类型 | 描述 |
|--------|------|-------------|
| `pid_to_process_stats` | `HashMap<u32, ProcessStats>` | 从进程 ID 到 [ProcessStats](ProcessStats.md) 的映射，跟踪每个进程的线程统计、存活状态和调试配置。条目在首次访问时创建，在进程退出时通过 [drop_process_by_pid](#drop_process_by_pid) 移除。 |
| `constants` | `ConfigConstants` | 来自服务配置的迟滞调优常量：`entry_threshold`、`keep_threshold` 和 `min_active_streak`。参见 [ConfigConstants](../config.rs/ConfigConstants.md)。 |

## 方法

### new

```rust
pub fn new(constants: ConfigConstants) -> Self
```

使用给定的迟滞常量和空的进程统计映射创建新的 `PrimeThreadScheduler`。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `constants` | `ConfigConstants` | 控制线程提升/降级行为的迟滞调优参数。 |

**返回值**

`PrimeThreadScheduler` — 准备就绪的新调度器实例。

---

### reset_alive

```rust
pub fn reset_alive(&mut self)
```

通过在每个 [ProcessStats](ProcessStats.md) 条目上设置 `alive = false`，将所有已跟踪的进程标记为已终止。在每次轮询循环迭代开始时调用。仍在运行的进程将在快照处理期间通过 [set_alive](#set_alive) 重新标记为存活。迭代结束后仍然标记为已终止的进程将通过 [drop_process_by_pid](#drop_process_by_pid) 进行清理。

---

### set_alive

```rust
pub fn set_alive(&mut self, pid: u32)
```

将进程标记为当前迭代中存活。如果给定 PID 不存在 [ProcessStats](ProcessStats.md) 条目，则创建一个默认值的新条目。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 要标记为存活的进程标识符。 |

---

### set_tracking_info

```rust
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

设置进程的调试跟踪信息。当进程退出时，如果 `track_top_x_threads` 不为零，将在诊断报告中记录按 CPU 周期排名的前 N 个线程。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 目标进程标识符。 |
| `track_top_x_threads` | `i32` | 进程退出时报告的前 N 个线程数量。`0` 表示禁用跟踪。使用绝对值作为计数。 |
| `process_name` | `String` | 日志输出中的进程显示名称。 |

---

### get_thread_stats

```rust
#[inline]
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

返回给定进程和线程的 [ThreadStats](ThreadStats.md) 可变引用。如果进程或线程条目不存在，则使用默认值创建。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 进程标识符。 |
| `tid` | `u32` | 线程标识符。 |

**返回值**

`&mut ThreadStats` — 线程统计条目的可变引用。

---

### update_active_streaks

```rust
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

根据每个线程相对于所有线程最大值的增量 CPU 周期，更新其 `active_streak` 计数器。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 拥有这些线程的进程标识符。 |
| `tid_with_delta_cycles` | `&[(u32, u64)]` | `(thread_id, delta_cycles)` 对的切片，表示自上次测量以来的 CPU 周期增量。 |

**备注**

连续活跃计数更新算法如下：

1. 计算 `max_cycles` — 切片中所有线程的最高增量。
2. 如果 `max_cycles == 0`，将所有连续计数重置为零并返回。
3. 对于每个线程：
   - **如果线程已有连续计数 > 0：**
     - 如果 `delta < keep_threshold × max_cycles`，将连续计数重置为 0（线程低于保持阈值）。
     - 否则，连续计数加 1，上限为 254。
   - **如果线程没有连续计数：**
     - 如果 `delta >= entry_threshold × max_cycles`，将连续计数设为 1（线程进入活跃区间）。

`entry_threshold` 有意高于 `keep_threshold`，形成一个迟滞带，防止快速的提升/降级循环。`active_streak` 计数器必须达到 `min_active_streak` 后，线程才有资格在 [select_top_threads_with_hysteresis](#select_top_threads_with_hysteresis) 中被提升。

---

### select_top_threads_with_hysteresis

```rust
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
)
```

使用两阶段迟滞算法选择哪些线程应获得 Prime 状态。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 拥有这些线程的进程标识符。 |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | **\[输入/输出\]** `(thread_id, delta_cycles, is_prime)` 元组的可变切片。`is_prime` 字段对被选为 Prime 的线程设为 `true`。`is_prime` 的输入值将被忽略（覆盖）。 |
| `slot_count` | `usize` | 可提升为 Prime 状态的最大线程数（通常为专用 CPU 核心数）。 |
| `is_currently_assigned` | `fn(&ThreadStats) -> bool` | 回调函数，如果线程当前已分配到 Prime 资源（例如具有非空的 `pinned_cpu_set_ids`），则返回 `true`。用于第一阶段识别在任者。 |

**返回值**

此函数不返回值。结果通过 `tid_with_delta_cycles` 中每个元组的 `is_prime` 字段传达。

**备注**

选择分两个阶段进行，输入首先按 `delta_cycles` 降序排序：

| 阶段 | 目的 | 标准 |
|------|---------|----------|
| **第一阶段（保留）** | 保留仍然合格的当前已分配线程 | `is_currently_assigned(stats) == true` 且 `delta >= keep_threshold × max_cycles` |
| **第二阶段（提升）** | 用新候选者填充剩余槽位 | `delta >= entry_threshold × max_cycles` 且 `active_streak >= min_active_streak` 且线程未被选中 |

这种两阶段设计确保在任的 Prime 线程不会因周期的轻微波动而被降级，同时新线程必须展示持续的高活跃度才能被提升。`entry_threshold` 和 `keep_threshold` 之间的差距即为迟滞带。

---

### drop_process_by_pid

```rust
pub fn drop_process_by_pid(&mut self, pid: &u32)
```

从调度器中移除进程及其所有线程统计。关闭所有缓存的线程句柄，丢弃进程的模块地址缓存，并可选地输出前 N 个线程的诊断报告。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `&u32` | 要移除的进程标识符的引用。 |

**备注**

- 如果 [ProcessStats](ProcessStats.md) 条目上的 `track_top_x_threads != 0`，将记录按 `last_cycles` 排名的前 N 个线程的报告。报告包含每个线程的详细信息：TID、周期计数、解析为模块名的起始地址、内核时间、用户时间、创建时间、等待时间、优先级、基础优先级、上下文切换次数、线程状态和等待原因。
- 存储在 [ThreadStats](ThreadStats.md) 中的所有 [ThreadHandle](../winapi.rs/ThreadHandle.md) 实例将被丢弃，这会关闭其底层的 Windows HANDLE。
- 调用 [drop_module_cache](../winapi.rs/drop_module_cache.md) 释放此进程的缓存模块枚举。
- 如果在映射中未找到该 PID，函数立即返回，无任何效果。

## 备注

### 迟滞模型

调度器使用经典的迟滞（施密特触发器）模式来防止 Prime 和非 Prime 状态之间的抖动：

```
                    ┌─────────────────────────────────┐
   entry_threshold  │  ← 线程必须超过此值才能          │
                    │    开始累积连续计数               │
                    ├─────────────────────────────────┤
   keep_threshold   │  ← 线程必须保持在此值之上        │
                    │    才能保持 Prime 状态            │
                    └─────────────────────────────────┘
```

`entry_threshold` 高于 `keep_threshold`。线程必须：
1. 超过 `entry_threshold × max_cycles` 才能开始累积活跃连续计数。
2. 保持连续计数达 `min_active_streak` 次迭代才有资格被提升。
3. 一旦被提升，只有当其周期降至 `keep_threshold × max_cycles` 以下时才会失去 Prime 状态。

### 生命周期

1. **reset_alive** — 在每次循环迭代开始时调用。
2. **set_alive** / **set_tracking_info** — 在快照中发现每个进程时调用。
3. **update_active_streaks** — 在计算周期增量后调用。
4. **select_top_threads_with_hysteresis** — 调用以确定 Prime 线程分配。
5. **drop_process_by_pid** — 对未被标记为存活（已退出）的进程调用。

## 要求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用方** | `src/main.rs` 中的主轮询循环、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) |
| **被调用方** | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md)、[drop_module_cache](../winapi.rs/drop_module_cache.md)、[log_message](../logging.rs/log_message.md) |
| **依赖** | [ConfigConstants](../config.rs/ConfigConstants.md)、[ProcessStats](ProcessStats.md)、[ThreadStats](ThreadStats.md)、[ThreadHandle](../winapi.rs/ThreadHandle.md) |
| **权限** | 无直接权限要求；此调度器使用的线程句柄需要更早获取的权限 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [scheduler.rs](README.md) |
| 进程统计 | [ProcessStats](ProcessStats.md) |
| 线程统计 | [ThreadStats](ThreadStats.md) |
| 理想处理器状态 | [IdealProcessorState](IdealProcessorState.md) |
| 线程句柄 RAII 包装器 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Prime 线程应用 | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| 配置常量 | [ConfigConstants](../config.rs/ConfigConstants.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*