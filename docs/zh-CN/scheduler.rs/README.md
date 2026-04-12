# scheduler.rs 模块 (scheduler.rs)

`scheduler` 模块实现了 Prime 线程调度器，负责动态识别高负载线程并将其分配到指定的高性能 CPU 核心。调度器使用滞后算法（hysteresis）防止线程在 prime 与非 prime 状态之间频繁振荡。

## 概述

Prime 线程调度的核心思路：在每次循环迭代中，采集所有受监控线程的 CPU 周期增量，通过双阈值滞后机制选出最活跃的 N 个线程，将它们"钉"到快速核心上，并提升其线程优先级。当线程不再活跃时，恢复其原始设置。

### 滞后算法

调度器使用两个阈值防止抖动：

- **进入阈值（entry_threshold）**：默认 `0.42` — 线程的周期增量必须达到最高线程的 42% 才有资格被提升为 prime 线程。
- **保持阈值（keep_threshold）**：默认 `0.69` — 已处于 prime 状态的线程只需维持最高线程的 69% 即可保留 prime 状态。

选择过程分为两阶段：

1. **保留阶段**：已分配 prime 状态的线程，只要周期 ≥ keep_threshold，即保留其 prime 状态。
2. **填充阶段**：剩余空位由满足 entry_threshold 且具有足够 `active_streak`（连续活跃计数 ≥ `min_active_streak`）的新线程填充。

### 活跃连胜计数（active_streak）

每次迭代中，若线程周期增量超过进入阈值，其 `active_streak` 递增（上限 254）；若低于保持阈值，则重置为 0。只有连续活跃达到 `min_active_streak` 次的线程才有资格被提升，从而过滤掉偶发的短暂高负载线程。

## 项目

### 结构体

| 名称 | 描述 |
| --- | --- |
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | 管理动态线程-CPU 分配的核心调度器。 |
| [ProcessStats](ProcessStats.md) | 每进程的跟踪数据，包含其所有线程的统计信息。 |
| [IdealProcessorState](IdealProcessorState.md) | 跟踪线程的理想处理器分配状态。 |
| [ThreadStats](ThreadStats.md) | 每线程的跟踪数据，包含周期、句柄、连胜计数和优先级信息。 |

### 函数

| 名称 | 描述 |
| --- | --- |
| [format_100ns](format_100ns.md) | 将 100 纳秒间隔格式化为人类可读的持续时间字符串。 |
| [format_filetime](format_filetime.md) | 将 Windows FILETIME（100ns 间隔）转换为本地日期时间字符串。 |

## 数据流

```text
每次循环迭代:

  prefetch_all_thread_cycles (apply.rs)
      │
      ▼
  PrimeThreadScheduler.get_thread_stats()  ← 缓存/更新周期数据
      │
      ▼
  update_active_streaks()                  ← 更新连胜计数
      │
      ▼
  select_top_threads_with_hysteresis()     ← 两阶段选择
      │
      ├─ 保留阶段: 已有 prime 线程 + cycles ≥ keep(0.69)
      └─ 填充阶段: 新线程 + cycles ≥ entry(0.42) + streak ≥ min
      │
      ▼
  apply_prime_threads_promote (apply.rs)   ← 提升选中线程
  apply_prime_threads_demote (apply.rs)    ← 降级落选线程
      │
      ▼
  close_dead_process_handles()             ← 清理已退出进程
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/scheduler.rs` |
| **调用者** | `src/apply.rs` 中的 [`apply_prime_threads`](../apply.rs/apply_prime_threads.md)、`src/main.rs` |
| **关键依赖** | [`ConfigConstants`](../config.rs/ConfigConstants.md)、[`ThreadPriority`](../priority.rs/ThreadPriority.md)、[`ThreadHandle`](../winapi.rs/ThreadHandle.md) |
| **外部 crate** | `chrono`（时间格式化）、`ntapi`（`SYSTEM_THREAD_INFORMATION`） |

## 另请参阅

- [priority.rs 模块概述](../priority.rs/README.md) — 线程优先级枚举定义
- [process.rs 模块概述](../process.rs/README.md) — 进程和线程枚举
- [apply_prime_threads](../apply.rs/apply_prime_threads.md) — Prime 线程调度的编排入口
- [ProcessConfig](../config.rs/ProcessConfig.md) — 进程配置中的 prime 线程相关字段