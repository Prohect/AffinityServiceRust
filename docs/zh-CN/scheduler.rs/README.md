# scheduler 模块 (AffinityServiceRust)

`scheduler` 模块实现了 AffinityServiceRust 服务中基于滞后算法的主力线程调度。它在轮询间隔内跟踪每个进程和每个线程的 CPU 周期统计信息，维护活跃连续计数器以过滤瞬时繁忙的线程，并选择前 N 个线程提升到性能核心。两阶段选择算法（保留 → 填充）通过应用不同的保持阈值和进入阈值来防止提升/降级抖动。该模块还提供了格式化 Windows 时间值的实用函数。

## 结构体

| 结构体 | 描述 |
|--------|------|
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | 中央调度器，拥有每个进程的统计信息映射，并公开基于滞后算法的线程选择。 |
| [ProcessStats](ProcessStats.md) | 每个进程的容器，持有存活状态、线程统计和跟踪配置。 |
| [IdealProcessorState](IdealProcessorState.md) | 跟踪单个线程的当前和之前的理想处理器分配。 |
| [ThreadStats](ThreadStats.md) | 每线程统计信息，包括周期计数、活跃连续计数、线程句柄、CPU 集合固定和理想处理器状态。 |

## 函数

| 函数 | 描述 |
|------|------|
| [format_100ns](format_100ns.md) | 将 100 纳秒间隔计数格式化为人类可读的 `seconds.milliseconds s` 字符串。 |
| [format_filetime](format_filetime.md) | 将 Windows FILETIME 64 位值转换为本地日期时间字符串（`YYYY-MM-DD HH:MM:SS.mmm`）。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 进程快照和枚举 | [process 模块](../process.rs/README.md) |
| 配置常量（阈值） | [ConfigConstants](../config.rs/ConfigConstants.md) |
| 主力线程提升和降级 | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| 线程句柄管理 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| 线程优先级等级 | [ThreadPriority](../priority.rs/ThreadPriority.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd