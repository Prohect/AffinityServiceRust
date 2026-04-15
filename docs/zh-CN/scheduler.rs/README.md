# scheduler 模块 (AffinityServiceRust)

`scheduler` 模块实现了 AffinityServiceRust 的核心线程调度引擎。它跟踪每个进程和每个线程的 CPU 周期统计信息，管理线程句柄的生命周期，并使用基于滞后（hysteresis）的算法来选择"主力线程"（prime threads）——即那些值得获得优先 CPU 核心分配的线程。该模块还提供了用于线程到核心固定的理想处理器状态跟踪，以及格式化 Windows 内核时间值的工具函数。

## 结构体

| 结构体 | 描述 |
|--------|------|
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | 顶层调度器，拥有每进程的统计信息，并公开用于连续活跃计数跟踪、基于滞后的线程选择和进程清理的方法。 |
| [ProcessStats](ProcessStats.md) | 每进程的簿记信息：存活标志、线程统计映射、顶部线程跟踪计数和进程元数据。 |
| [IdealProcessorState](IdealProcessorState.md) | 跟踪单个线程的当前和先前理想处理器分配（组号 + 编号）。 |
| [ThreadStats](ThreadStats.md) | 每线程的统计信息，包括周期计数器、缓存的时间数据、线程句柄、CPU 集合固定列表、活跃连续计数器、优先级快照和理想处理器状态。 |

## 函数

| 函数 | 描述 |
|------|------|
| [format_100ns](format_100ns.md) | 将 Windows 100 纳秒时间值转换为人类可读的 `秒.毫秒` 字符串。 |
| [format_filetime](format_filetime.md) | 将 Windows `FILETIME` 100 纳秒值转换为本地时间日期时间字符串。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| main 模块 | [main.rs README](../main.rs/README.md) |
| priority 模块 | [priority.rs README](../priority.rs/README.md) |
| apply 模块 | [apply.rs README](../apply.rs/README.md) |
| config 模块 | [config.rs README](../config.rs/README.md) |
| winapi 模块 | [winapi.rs README](../winapi.rs/README.md) |

---
Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
