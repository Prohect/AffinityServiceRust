# ProcessStats struct (scheduler.rs)

每进程的 Prime 线程调度跟踪数据。由 [PrimeThreadScheduler](PrimeThreadScheduler.md) 为每个受管进程维护一个实例，存储进程的存活状态、所有被跟踪线程的统计信息，以及调试报告配置。

## 语法

```rust
#[derive(Debug)]
pub struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
```

## 成员

`alive`

进程在当前调度周期中是否仍然存活。每次循环开始时由 [`PrimeThreadScheduler::reset_alive`](PrimeThreadScheduler.md) 重置为 `false`，随后由 [`PrimeThreadScheduler::set_alive`](PrimeThreadScheduler.md) 设置为 `true`。当循环结束时仍为 `false` 的条目，将在 [`close_dead_process_handles`](PrimeThreadScheduler.md) 中被清除。

`tid_to_thread_stats`

线程 ID 到 [ThreadStats](ThreadStats.md) 的映射。存储该进程中所有被跟踪线程的周期计数、句柄缓存、活跃连续计数等数据。键为线程的 TID（`u32`），值为对应的 `ThreadStats` 实例。

`track_top_x_threads`

配置文件中指定的 `track_top_x_threads` 值。控制进程退出时的调试报告行为：

- **正值**：进程退出时，按 CPU 周期降序输出前 N 个线程的详细报告。
- **负值**：取绝对值作为报告数量（行为与正值相同）。
- **0**：不输出退出报告。

该值由 [`PrimeThreadScheduler::set_tracking_info`](PrimeThreadScheduler.md) 从 [ProcessConfig](../config.rs/ProcessConfig.md) 中的同名字段同步。

`process_name`

进程映像名称（小写）。由 [`set_tracking_info`](PrimeThreadScheduler.md) 设置，用于日志和退出报告中标识进程。

`process_id`

进程 ID。在构造时传入，用于标识此 `ProcessStats` 所跟踪的进程。标记为 `#[allow(dead_code)]`，主要用于调试目的。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **new** | `pub fn new(process_id: u32) -> Self` | 创建新实例。`alive` 初始化为 `true`，其他字段为空或默认值。 |

## Trait 实现

| Trait | 描述 |
| --- | --- |
| `Debug` | 派生实现，输出所有字段的调试表示。 |
| `Default` | 调用 `Self::new(0)`，返回 `process_id` 为 0 的默认实例。 |

## 备注

`ProcessStats` 作为 [PrimeThreadScheduler](PrimeThreadScheduler.md) 中 `pid_to_process_stats: HashMap<u32, ProcessStats>` 的值类型存在。调度器通过 PID 索引来访问和管理各进程的线程统计数据。

### 生命周期管理

`ProcessStats` 遵循"标记-清除"模式：

1. **标记阶段**：每次循环开始时，`reset_alive()` 将所有条目的 `alive` 标记为 `false`。
2. **更新阶段**：对于快照中仍存在的进程，`set_alive()` 将其标记为 `true`。
3. **清除阶段**：循环结束时，`close_dead_process_handles()` 移除所有 `alive == false` 的条目，同时释放其下所有 [ThreadStats](ThreadStats.md) 中缓存的线程句柄。

### 退出报告

当 `track_top_x_threads != 0` 且进程退出（标记为非存活）时，`close_dead_process_handles` 会生成包含以下信息的详细报告：

- 线程 ID 和 CPU 周期总数
- 起始地址及其解析后的模块名
- 内核时间、用户时间、创建时间
- 上下文切换次数、线程状态、等待原因

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/scheduler.rs |
| **行号** | L178–L204 |
| **拥有者** | [PrimeThreadScheduler](PrimeThreadScheduler.md) 的 `pid_to_process_stats` 字段 |
| **依赖** | [ThreadStats](ThreadStats.md)、`HashMap` |

## 另请参阅

- [PrimeThreadScheduler](PrimeThreadScheduler.md)
- [ThreadStats](ThreadStats.md)
- [IdealProcessorState](IdealProcessorState.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)