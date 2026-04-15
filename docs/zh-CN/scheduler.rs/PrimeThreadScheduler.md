# PrimeThreadScheduler 类型 (scheduler.rs)

`PrimeThreadScheduler` 是顶层调度引擎，拥有每个进程的统计信息并编排基于滞后的"主线程"选择。它维护一个从进程 ID 到 [`ProcessStats`](ProcessStats.md) 条目的映射，跟踪跨轮询迭代的线程活跃状态，计算活跃连续计数器以实现稳定的线程提升，并执行两遍线程选择以抵抗提升/降级抖动。它还处理进程退出时的清理工作，包括线程句柄关闭、模块缓存清除以及可选的按 CPU 周期排名的顶级线程诊断报告。

## 语法

```AffinityServiceRust/src/scheduler.rs#L15-20
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## 成员

| 字段 | 类型 | 描述 |
|------|------|------|
| `pid_to_process_stats` | `HashMap<u32, ProcessStats>` | 从 Windows 进程 ID（PID）到其对应 [`ProcessStats`](ProcessStats.md) 结构体的映射。条目通过 `set_alive` 或 `get_thread_stats` 延迟创建，并在进程退出时由 `drop_process_by_pid` 移除。 |
| `constants` | `ConfigConstants` | 从配置文件加载的调度器调优常量，包括 `entry_threshold`、`keep_threshold` 和 `min_active_streak`。这些控制主线程选择的滞后行为。 |

## 方法

### `new`

```AffinityServiceRust/src/scheduler.rs#L22-27
pub fn new(constants: ConfigConstants) -> Self
```

构造一个新的 `PrimeThreadScheduler`，具有空的进程映射和给定的配置常量。

### `reset_alive`

```AffinityServiceRust/src/scheduler.rs#L29-31
pub fn reset_alive(&mut self)
```

将每个 [`ProcessStats`](ProcessStats.md) 条目的 `alive` 标志设置为 `false`。在每个轮询迭代开始时调用；随后在快照中观察到的进程将通过 `set_alive` 重新标记为存活。

### `set_alive`

```AffinityServiceRust/src/scheduler.rs#L33-35
pub fn set_alive(&mut self, pid: u32)
```

将给定 PID 的进程标记为存活。如果该 PID 没有 `ProcessStats` 条目，则创建并插入一个新的默认条目。

### `set_tracking_info`

```AffinityServiceRust/src/scheduler.rs#L37-41
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

设置进程 `ProcessStats` 条目上的诊断跟踪元数据。`track_top_x_threads` 控制进程退出时记录多少个线程（0 = 禁用，正数 = 前 N 个，负数 = 前 N 个并包含完整的 `SYSTEM_THREAD_INFORMATION` 转储）。`process_name` 存储用于日志输出。

### `get_thread_stats`

```AffinityServiceRust/src/scheduler.rs#L44-50
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

返回给定 `(pid, tid)` 对的 [`ThreadStats`](ThreadStats.md) 的可变引用，如果 `ProcessStats` 和/或 `ThreadStats` 条目不存在则创建它们。标记为 `#[inline]` 以提高紧密循环中的性能。

### `update_active_streaks`

```AffinityServiceRust/src/scheduler.rs#L57-71
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

更新 `tid_with_delta_cycles` 中提供的所有线程的滞后连续计数器。算法如下：

1. 确定切片中所有线程的**最大增量周期数**。
2. 计算 `entry_min = max_cycles × entry_threshold` 和 `keep_min = max_cycles × keep_threshold`。
3. 对于每个线程：
   - 如果线程已有非零连续计数且其增量低于 `keep_min`，则连续计数**重置为 0**。
   - 如果线程有非零连续计数且满足 `keep_min`，则连续计数**递增**（上限为 254）。
   - 如果线程连续计数为零且其增量达到或超过 `entry_min`，则连续计数**设为 1**。

这种不对称的进入/保持阈值可以防止线程在 CPU 使用率接近边界值时在主线程和非主线程状态之间闪烁。

### `select_top_threads_with_hysteresis`

```AffinityServiceRust/src/scheduler.rs#L80-118
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
)
```

执行两遍主线程选择：

- **第一遍（保留）：** 按增量周期降序遍历线程。已被分配的线程（由 `is_currently_assigned` 回调判断）且其增量达到或超过 `keep_min` 阈值的线程保留其主线程状态。这可以防止因轻微周期波动导致的降级。
- **第二遍（提升）：** 用满足 `entry_min` 阈值**且** `active_streak` 达到或超过 `constants.min_active_streak` 的线程填充剩余槽位。这可以防止短暂活跃的线程被过早提升。

每个元组中的 `is_prime`（第三个元素）布尔值对于被选中的线程设为 `true`。TID 为 0 的线程始终被跳过。`slot_count` 参数限制可选择的线程总数。

### `drop_process_by_pid`

```AffinityServiceRust/src/scheduler.rs#L122-164
pub fn drop_process_by_pid(&mut self, pid: &u32)
```

清理给定进程的所有状态。如果 `track_top_x_threads` 非零，该方法首先按 `last_cycles` 排序构建前 N 个线程的报告，包括每个线程起始地址的模块名解析以及可选的 `SYSTEM_THREAD_INFORMATION` 详细信息（内核时间、用户时间、创建时间、等待时间、上下文切换次数、线程状态、等待原因、优先级）。报告通过 `log_message` 发出。

记录完成后，该方法调用 `drop_module_cache` 释放每个进程的模块解析缓存，然后从映射中移除 `ProcessStats` 条目。所有拥有的 `ThreadHandle` 值通过 Rust 的 `Drop` 实现自动释放，后者关闭底层的 Win32 句柄。

## 备注

- 调度器在 `main` 中实例化一次，在服务的整个运行期间存活。
- 使用的 `HashMap` 类型是项目自定义的 `collections::HashMap`，可能与 `std::collections::HashMap` 不同（例如使用不同的哈希器或内联存储）。
- 存储在 `ThreadStats` 内部的线程句柄在条目被移除时自动关闭。无需手动管理句柄。
- `constants` 字段从解析后的配置克隆而来。如果配置被热重载，可能会使用更新后的常量构造一个新的调度器。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 调用者 | [main](../main.rs/main.md)、[apply_thread_level](../main.rs/apply_thread_level.md)、apply 模块 |
| 被调用者 | `winapi::resolve_address_to_module`、`winapi::drop_module_cache`、`logging::log_message` |
| 依赖 | `ConfigConstants`（config 模块）、[ProcessStats](ProcessStats.md)、[ThreadStats](ThreadStats.md)、[IdealProcessorState](IdealProcessorState.md) |
| 平台 | Windows（内部处理 Win32 线程句柄） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| ProcessStats | [ProcessStats](ProcessStats.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| IdealProcessorState | [IdealProcessorState](IdealProcessorState.md) |
| format_100ns | [format_100ns](format_100ns.md) |
| format_filetime | [format_filetime](format_filetime.md) |
| main 模块 | [main.rs README](../main.rs/README.md) |
| apply_thread_level | [apply_thread_level](../main.rs/apply_thread_level.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
