# ProcessStats 结构体 (scheduler.rs)

[PrimeThreadScheduler](PrimeThreadScheduler.md) 使用的进程级统计容器，用于跟踪单个进程的线程级调度状态、存活标志和调试配置。调度器 `pid_to_process_stats` 映射中的每个条目都是一个 `ProcessStats` 实例。

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
|--------|------|-------------|
| `alive` | `bool` | 当前轮询迭代的存活标志。在每次循环开始时由 [PrimeThreadScheduler::reset_alive](PrimeThreadScheduler.md) 设为 `false`，如果进程仍然存在于快照中，则由 [PrimeThreadScheduler::set_alive](PrimeThreadScheduler.md) 设回 `true`。在快照扫描后仍为 `false` 的进程将由 [drop_process_by_pid](PrimeThreadScheduler.md) 清理。 |
| `tid_to_thread_stats` | `HashMap<u32, ThreadStats>` | 线程 ID → [ThreadStats](ThreadStats.md) 的映射，包含该进程中观察到的每个线程。条目由 [PrimeThreadScheduler::get_thread_stats](PrimeThreadScheduler.md) 延迟创建。线程句柄和调度状态存储在此映射的每个线程条目中。 |
| `track_top_x_threads` | `i32` | 进程终止时退出报告中包含的热门线程数量（按 CPU 周期计数排序）。值为 `0` 时禁用报告。由 [PrimeThreadScheduler::set_tracking_info](PrimeThreadScheduler.md) 从每个进程的配置中设置。接受负值；生成报告时使用绝对值。 |
| `process_name` | `String` | 进程的缓存显示名称（小写），用于日志消息和退出线程报告。由 [PrimeThreadScheduler::set_tracking_info](PrimeThreadScheduler.md) 设置。 |
| `process_id` | `u32` | 此统计条目所属的 PID。在构造时设置，当前标记为 `#[allow(dead_code)]`。 |

## 方法

### new

```rust
pub fn new(process_id: u32) -> Self
```

使用给定的 PID 创建新的 `ProcessStats`。所有字段初始化为默认/空值，`alive` 设为 `true`。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `process_id` | `u32` | 此统计条目跟踪的进程标识符。 |

**返回值**

`ProcessStats` — 一个新实例，包含空的线程统计映射、`track_top_x_threads` 为 `0`、空的 `process_name` 以及 `alive` 为 `true`。

### Default

```rust
impl Default for ProcessStats {
    fn default() -> Self;
}
```

委托给 `ProcessStats::new(0)`。提供零 PID 的默认值，主要为了 HashMap entry API 的使用便利性。

## 备注

### 生命周期

`ProcessStats` 条目在进程 PID 首次在任何调度器方法（`set_alive`、`set_tracking_info`、`get_thread_stats` 或 `update_active_streaks`）中遇到时通过 `HashMap::entry().or_insert()` 创建。它在轮询迭代之间持续存在，直到进程不再存活并调用 [drop_process_by_pid](PrimeThreadScheduler.md)。

### 线程统计增长

`tid_to_thread_stats` 映射在进程的生命周期内单调增长——线程在被观察到时添加，但不会被单独移除。当进程退出时整个映射被丢弃。这与 Windows 线程模型一致，即线程 ID 在单个进程生命周期内不会被重用。

### 退出报告

当 `track_top_x_threads != 0` 且进程退出时，[drop_process_by_pid](PrimeThreadScheduler.md) 会生成一份日志报告，列出按 `last_cycles` 排名的前 N 个线程。报告包含每个线程的 TID、周期计数、起始地址（通过 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) 解析为模块名称）、内核时间、用户时间、创建时间、优先级、上下文切换次数以及来自 `SYSTEM_THREAD_INFORMATION` 的其他字段。

## 要求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用方** | [PrimeThreadScheduler](PrimeThreadScheduler.md)（所有方法） |
| **依赖** | [ThreadStats](ThreadStats.md)、来自 `crate::collections` 的 `HashMap` |
| **权限** | 无（仅数据结构） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [scheduler.rs](README.md) |
| 父级调度器 | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| 线程级状态 | [ThreadStats](ThreadStats.md) |
| 进程快照条目 | [ProcessEntry](../process.rs/ProcessEntry.md) |
| 模块名称解析 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*