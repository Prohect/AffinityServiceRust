# process 模块 (AffinityServiceRust)

提供基于 RAII 的进程快照机制，构建于 Windows 原生 API 函数 `NtQuerySystemInformation` 的 `SystemProcessInformation` 信息类之上。该模块捕获系统上所有运行进程及其线程的时间点视图，并通过安全的 Rust 抽象层进行公开。快照数据存储在由 `Mutex` 保护的全局缓冲区中，`ProcessSnapshot` 结构体确保在析构时进行清理。单个进程由 `ProcessEntry` 表示，它将原始线程数组延迟解析为 `HashMap` 集合，以实现基于 TID 的高效查找。

## 静态变量

| 名称 | 类型 | 描述 |
|------|------|------|
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | `Lazy<Mutex<Vec<u8>>>` | `NtQuerySystemInformation` 用于接收进程/线程数据的全局字节缓冲区。 |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | `Lazy<Mutex<HashMap<u32, ProcessEntry>>>` | 从进程 ID 到 `ProcessEntry` 的全局映射，在每次快照时填充。 |

## 结构体

| 名称 | 描述 |
|------|------|
| [ProcessSnapshot](ProcessSnapshot.md) | 捕获并拥有进程/线程快照的 RAII 包装器。析构时清除所有数据。 |
| [ProcessEntry](ProcessEntry.md) | 表示单个进程，包含其 `SYSTEM_PROCESS_INFORMATION` 和延迟解析的线程映射。 |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 消费来自 `ProcessEntry` 的线程周期数据，驱动主力线程调度决策。 |
| [ProcessConfig](../config.rs/ProcessConfig.md) | 应用于快照中条目的每进程配置（优先级、亲和性、CPU 集、主力线程）。 |
| [apply_prime_threads](../apply.rs/apply_prime_threads.md) | 从 `ProcessEntry` 读取线程数据并将其馈送到调度器进行提升/降级。 |
| [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) | 遍历 `ProcessEntry` 线程以在调度前查询和缓存周期时间。 |