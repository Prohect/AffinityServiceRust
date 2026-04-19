# scheduler 模块 (AffinityServiceRust)

`scheduler` 模块实现了 Prime 线程调度引擎——核心算法，用于识别进程中最活跃的线程并跟踪其随时间变化的活动状态，为 CPU 绑定决策提供依据。该模块使用基于迟滞机制的选择策略来防止提升/降级抖动，其中线程必须在超过进入阈值后维持一定的连续迭代次数才能被提升，并且必须降至一个较低的保持阈值以下才会被降级。

该模块跨轮询迭代维护每个进程和每个线程的统计信息，包括周期计数器、连续活跃计数器、线程句柄、理想处理器分配以及缓存的优先级信息。

## 结构体

| 名称 | 描述 |
|------|------|
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | 顶层调度器结构体，拥有每进程统计信息和迟滞常量。提供存活跟踪、连续活跃计数更新以及基于迟滞机制的线程选择方法。 |
| [ProcessStats](ProcessStats.md) | 每进程状态容器，持有线程统计映射、存活标志、跟踪配置和进程元数据。 |
| [IdealProcessorState](IdealProcessorState.md) | 每线程理想处理器分配状态，跟踪当前和先前的处理器组/编号分配。 |
| [ThreadStats](ThreadStats.md) | 每线程状态容器，包含周期/时间计数器、句柄缓存、CPU 集合绑定、活跃连续计数、起始地址、优先级和理想处理器状态。 |

## 函数

| 名称 | 描述 |
|------|------|
| [format_100ns](format_100ns.md) | 将 100 纳秒时间值格式化为人类可读的 `"seconds.milliseconds s"` 字符串。 |
| [format_filetime](format_filetime.md) | 将 Windows FILETIME 值（自 1601-01-01 起的 100ns 单位）转换为本地日期时间字符串。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 阈值配置常量 | [ConfigConstants](../config.rs/ConfigConstants.md) |
| Prime 线程应用逻辑 | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| 线程选择阶段 | [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) |
| 线程句柄管理 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| 进程快照数据 | [ProcessEntry](../process.rs/ProcessEntry.md) |
| 模块地址解析 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*