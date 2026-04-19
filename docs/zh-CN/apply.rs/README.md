# apply 模块 (AffinityServiceRust)

`apply` 模块是 AffinityServiceRust 的核心执行引擎。它将已配置的设置——进程优先级、CPU 亲和性、CPU 集合、IO 优先级、内存优先级、Prime 线程调度以及理想处理器提示——应用到正在运行的 Windows 进程上。每个函数通过 Windows API 读取目标进程或线程的当前状态，将其与期望的配置进行比较，仅在检测到差异时才进行更改。所有变更和错误都会累积到 [ApplyConfigResult](ApplyConfigResult.md) 中，以便进行结构化日志记录。

## 结构体

| 名称 | 描述 |
|------|------|
| [ApplyConfigResult](ApplyConfigResult.md) | 单次应用过程中产生的变更消息和错误消息的累加器。 |

## 函数

| 名称 | 描述 |
|------|------|
| [get_handles](get_handles.md) | 从 [ProcessHandle](../winapi.rs/ProcessHandle.md) 中提取读写 `HANDLE`，优先使用完全访问权限而非受限权限。 |
| [log_error_if_new](log_error_if_new.md) | 仅在首次遇到唯一的 (pid, operation, error_code) 组合时记录错误。 |
| [apply_priority](apply_priority.md) | 读取并可选地设置进程优先级类。 |
| [apply_affinity](apply_affinity.md) | 读取并可选地设置进程 CPU 亲和性掩码。在更改时重置线程理想处理器。 |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | 在亲和性或 CPU 集合更改后，将线程理想处理器重新分配到一组 CPU 上。 |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | 通过 Windows CPU 集合 API 读取并可选地设置进程默认 CPU 集合。 |
| [apply_io_priority](apply_io_priority.md) | 通过 `NtQueryInformationProcess`/`NtSetInformationProcess` 读取并可选地设置进程 IO 优先级。 |
| [apply_memory_priority](apply_memory_priority.md) | 通过 `GetProcessInformation`/`SetProcessInformation` 读取并可选地设置进程内存优先级。 |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | 查询线程周期时间并计算用于 Prime 线程选择的增量。 |
| [apply_prime_threads](apply_prime_threads.md) | Prime 线程调度的顶层协调器：选择、提升和降级。 |
| [apply_prime_threads_select](apply_prime_threads_select.md) | 使用迟滞阈值选择哪些线程获得 Prime 状态。 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 通过 CPU 集合将 Prime 线程固定到专用 CPU，并可选地提升线程优先级。 |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | 取消固定非 Prime 线程，并恢复其原始线程优先级。 |
| [apply_ideal_processors](apply_ideal_processors.md) | 基于模块前缀匹配规则为线程分配理想处理器提示。 |
| [update_thread_stats](update_thread_stats.md) | 将当前周期/时间测量值缓存为"上次"值，用于下一次迭代的增量计算。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 配置类型 | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)、[ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| 进程句柄管理 | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 进程快照数据 | [ProcessEntry](../process.rs/ProcessEntry.md) |
| 错误去重 | [is_new_error](../logging.rs/is_new_error.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md)、[IOPriority](../priority.rs/IOPriority.md)、[MemoryPriority](../priority.rs/MemoryPriority.md)、[ThreadPriority](../priority.rs/ThreadPriority.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*