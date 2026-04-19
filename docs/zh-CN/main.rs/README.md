# main 模块 (AffinityServiceRust)

`main` 模块是 AffinityServiceRust 的应用程序入口点和顶层协调器。它解析命令行参数、读取配置文件、请求管理员权限，并运行主轮询循环，该循环获取进程快照、将其与配置规则进行匹配，并委托 [apply](../apply.rs/README.md) 模块进行强制执行。它还管理基于 ETW 的响应式睡眠、配置和黑名单文件的热重载，以及发现未管理进程的查找模式。

## 函数

| 名称 | 描述 |
|------|-------------|
| [apply_process_level](apply_process_level.md) | 打开进程句柄并应用所有进程级别设置（优先级、亲和性、CPU 集合、IO 优先级、内存优先级）。 |
| [apply_thread_level](apply_thread_level.md) | 应用所有线程级别设置（Prime 线程调度、理想处理器分配、周期时间跟踪）。 |
| [apply_config](apply_config.md) | 组合入口点，为匹配的进程应用进程级别和线程级别配置。 |
| [log_apply_results](log_apply_results.md) | 格式化并记录 [ApplyConfigResult](../apply.rs/ApplyConfigResult.md)，使用对齐的多行输出。 |
| [process_logs](process_logs.md) | 后处理查找模式日志文件以发现新的未管理进程并定位它们的可执行文件。 |
| [process_find](process_find.md) | 获取进程快照，并记录任何具有默认（完整）CPU 亲和性且不在配置或黑名单中的进程。 |
| [main](main.md) | 应用程序入口点。处理命令行模式、权限提升、ETW 监控器、主循环和优雅关闭。 |

## 参见

| 主题 | 链接 |
|-------|------|
| 应用引擎 | [apply.rs](../apply.rs/README.md) |
| 命令行参数解析 | [CliArgs](../cli.rs/CliArgs.md) |
| 配置类型 | [ConfigResult](../config.rs/ConfigResult.md), [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md), [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 进程快照 | [ProcessSnapshot](../process.rs/ProcessSnapshot.md), [ProcessEntry](../process.rs/ProcessEntry.md) |
| ETW 监控器 | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Win32 辅助工具 | [winapi.rs](../winapi.rs/README.md) |
| 日志记录 | [logging.rs](../logging.rs/README.md) |

*为提交 [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf) 记录*