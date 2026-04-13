# ProcessStats 结构体 (scheduler.rs)

每进程统计容器，用于跟踪主力线程调度所需的线程级数据。每个 `ProcessStats` 实例在 [`PrimeThreadScheduler`](PrimeThreadScheduler.md) 内部以 PID 为键存储，包含该进程中所有已观察线程的 [`ThreadStats`](ThreadStats.md) 集合。

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

| 成员 | 类型 | 描述 |
|------|------|------|
| `alive` | `bool` | 存活标志，用于调度器的标记-清除循环。在每次调度遍历开始时由 [`PrimeThreadScheduler::reset_alive`](PrimeThreadScheduler.md) 设为 `false`，当进程仍存在于快照中时由 [`PrimeThreadScheduler::set_alive`](PrimeThreadScheduler.md) 重新设为 `true`。遍历后仍为 `false` 的进程将作为候选项通过 [`drop_process_by_pid`](PrimeThreadScheduler.md) 进行清理。 |
| `tid_to_thread_stats` | `HashMap<u32, ThreadStats>` | 从线程 ID (TID) 到每线程统计数据的映射。条目在首次通过 [`PrimeThreadScheduler::get_thread_stats`](PrimeThreadScheduler.md) 访问时惰性创建，并持续存在直到整个 `ProcessStats` 被丢弃。 |
| `track_top_x_threads` | `i32` | 控制进程退出时的诊断日志。当非零时，[`drop_process_by_pid`](PrimeThreadScheduler.md) 会记录按 CPU 周期排名的前 *N* 个线程，用于事后分析。绝对值决定数量；符号保留供将来使用。通过 [`PrimeThreadScheduler::set_tracking_info`](PrimeThreadScheduler.md) 设置。 |
| `process_name` | `String` | 进程的小写显示名称（例如 `"game.exe"`）。通过 [`PrimeThreadScheduler::set_tracking_info`](PrimeThreadScheduler.md) 设置，用于日志消息。 |
| `process_id` | `u32` | Windows 进程标识符。存储以供参考；构造后不主动使用。 |

## 备注

### 构造

`ProcessStats` 通过 `ProcessStats::new(pid)` 创建，初始化时 `alive: true`、空的线程统计映射、`track_top_x_threads` 为零以及空的进程名称。

同时提供了 `Default` 实现，调用 `new(0)`。

### 存活标志生命周期

`alive` 字段实现了两阶段标记-清除模式：

1. **标记阶段** — `PrimeThreadScheduler::reset_alive` 将所有已跟踪进程的 `alive` 设为 `false`。
2. **清除阶段** — 当在当前 [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md) 中找到进程时，`PrimeThreadScheduler::set_alive` 将 `alive` 设为 `true`。
3. **清理** — 仍标记为 `alive == false` 的进程已退出，应通过 `drop_process_by_pid` 清理，该方法会关闭线程句柄并释放模块缓存。

### 线程统计映射增长

`tid_to_thread_stats` 映射在进程生命周期内单调增长。已退出的线程**不会**被单独移除；它们的条目保留直到整个进程被丢弃。这避免了逐线程清理的开销，由于短生命周期线程积累的内存可忽略不计，这是可接受的。

### 跟踪信息

`track_top_x_threads` 和 `process_name` 通过 `set_tracking_info` 一起设置，因为它们来源于同一个 [`ProcessConfig`](../config.rs/ProcessConfig.md) 规则。它们仅在进程最终退出且调度器输出诊断信息时才有意义。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 构造者 | [`PrimeThreadScheduler::set_alive`](PrimeThreadScheduler.md)、`ProcessStats::new` |
| 使用者 | [`PrimeThreadScheduler`](PrimeThreadScheduler.md) 的各方法 |
| 依赖 | [`ThreadStats`](ThreadStats.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 父调度器 | [`PrimeThreadScheduler`](PrimeThreadScheduler.md) |
| 每线程数据 | [`ThreadStats`](ThreadStats.md) |
| 滞后算法常量 | [`ConfigConstants`](../config.rs/ConfigConstants.md) |
| 进程快照来源 | [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md) |
| 进程配置 | [`ProcessConfig`](../config.rs/ProcessConfig.md) |