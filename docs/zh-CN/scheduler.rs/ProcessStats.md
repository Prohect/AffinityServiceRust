# ProcessStats 类型 (scheduler.rs)

保存 [`PrimeThreadScheduler`](PrimeThreadScheduler.md) 用于管理线程级调度决策的每进程簿记状态。每个 `ProcessStats` 实例跟踪进程是否仍然存活、维护每线程统计信息的映射、存储进程名称和 ID，以及记录在进程退出时应记录多少个顶部线程以供诊断使用。

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

| 字段 | 类型 | 描述 |
|------|------|------|
| `alive` | `bool` | 存活标志。在每个轮询迭代开始时，当进程在快照中被观察到时设置为 `true`。在每次扫描之前由 [`PrimeThreadScheduler::reset_alive`](PrimeThreadScheduler.md) 重置为 `false`。扫描后仍为 `false` 的进程被视为已终止，可通过 [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) 进行清理。 |
| `tid_to_thread_stats` | `HashMap<u32, ThreadStats>` | 从 Windows 线程 ID（`u32`）到对应 [`ThreadStats`](ThreadStats.md) 实例的映射。条目在首次观察到线程时由 `PrimeThreadScheduler::get_thread_stats` 惰性创建。当所属进程被清理时，条目会被删除。 |
| `track_top_x_threads` | `i32` | 控制进程退出时的诊断日志记录。当非零时，[`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) 会打印按最后观察到的 CPU 周期计数排序的前 `abs(track_top_x_threads)` 个线程，包括内核/用户时间、起始地址、模块名称和 `SYSTEM_THREAD_INFORMATION` 详细信息。值为 `0` 表示禁用退出日志记录。由 `PrimeThreadScheduler::set_tracking_info` 设置。 |
| `process_name` | `String` | 进程的小写可执行文件名（例如 `"game.exe"`）。由 `PrimeThreadScheduler::set_tracking_info` 填充，用于日志输出。构造时默认为空字符串。 |
| `process_id` | `u32` | Windows 进程 ID（PID）。存储用于日志消息中的标识。在源代码中标记为 `#[allow(dead_code)]`，因为目前仅用于构造和调试。 |

## 备注

- `ProcessStats` 实例**不由**调用者直接创建。它们由 `PrimeThreadScheduler` 的方法（如 `set_alive`、`set_tracking_info` 和 `get_thread_stats`）按需创建，这些方法都使用 `HashMap::entry().or_insert(ProcessStats::new(pid))`。
- `new(process_id)` 构造函数将 `alive` 初始化为 `true`，`track_top_x_threads` 初始化为 `0`，`process_name` 初始化为空字符串，`tid_to_thread_stats` 初始化为空映射。
- 提供了一个 `Default` 实现，它委托给 `Self::new(0)`，生成一个 PID 为 0 的统计条目。这主要用于满足 trait 一致性，不应在生产路径中使用。
- 用于 `tid_to_thread_stats` 的 `HashMap` 类型是项目中 `collections` 模块中自定义的基于栈的 `HashMap`，而非 `std::collections::HashMap`。
- 当 `PrimeThreadScheduler::drop_process_by_pid` 运行时，它首先使用线程统计信息生成可选的诊断报告，然后移除整个 `ProcessStats` 条目。每个 `ThreadStats` 中的 `ThreadHandle` 实例通过其 `Drop` 实现自动关闭。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 调用者 | [`PrimeThreadScheduler`](PrimeThreadScheduler.md)（所有访问 `pid_to_process_stats` 的方法） |
| 被调用者 | [`ThreadStats::new`](ThreadStats.md)（通过映射插入） |
| API | 无直接调用；线程句柄管理委托给 [`ThreadStats`](ThreadStats.md) |
| 权限 | 无 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| IdealProcessorState | [IdealProcessorState](IdealProcessorState.md) |
| scheduler 模块概述 | [README](README.md) |
| main 模块 | [main.rs README](../main.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
